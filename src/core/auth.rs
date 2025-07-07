#[derive(Clone, Debug)]
pub enum AuthProvider {
    #[allow(unused)]
    Basic(String, String),
    #[allow(unused)]
    KeyQuery(String, String),
    #[allow(unused)]
    KeyHeader(String, String),
    #[allow(unused)]
    KeyCookie(String, String),
    #[allow(unused)]
    Bearer(String),
    #[allow(unused)]
    OAuth2(OAuth2Provider),
}
impl AuthProvider {
    pub async fn add_auth(
        &mut self,
        mut builder: reqwest::RequestBuilder,
    ) -> crate::SdkResult<reqwest::RequestBuilder> {
        match self {
            AuthProvider::Basic(username, password) => {
                builder = builder.basic_auth(username, Some(password));
            }
            AuthProvider::KeyQuery(query_name, key_val) => {
                builder = builder.query(&[(query_name, key_val)]);
            }
            AuthProvider::KeyHeader(header_name, key_val) => {
                builder = builder.header(header_name.to_string(), key_val.to_string());
            }
            AuthProvider::KeyCookie(cookie_name, key_val) => {
                let cookie_val = format!("{cookie_name}={key_val}");
                builder = builder.header(reqwest::header::COOKIE, cookie_val);
            }
            AuthProvider::Bearer(token_val) => builder = builder.bearer_auth(token_val),
            AuthProvider::OAuth2(oauth_provider) => {
                builder = oauth_provider.add_auth(builder).await?;
            }
        };
        Ok(builder)
    }
}
/// OAuth2 authentication props for a password flow
///
/// Details:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.3
#[derive(Clone, Debug, Default)]
pub struct OAuth2Password {
    pub username: String,
    pub password: String,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub grant_type: Option<String>,
    pub scope: Option<Vec<String>>,
    pub token_url: Option<String>,
}
/// OAuth2 authentication props for a password flow
///
/// Details:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.3
#[derive(Clone, Debug, Default)]
pub struct OAuth2ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: Option<String>,
    pub scope: Option<Vec<String>>,
    pub token_url: Option<String>,
}
#[derive(Clone, Debug)]
pub(crate) enum OAuth2Flow {
    #[allow(unused)]
    Password(OAuth2Password),
    #[allow(unused)]
    ClientCredentials(OAuth2ClientCredentials),
}
impl OAuth2Flow {
    fn token_url(&self) -> Option<String> {
        match self {
            OAuth2Flow::Password(pwd) => pwd.token_url.clone(),
            OAuth2Flow::ClientCredentials(creds) => creds.token_url.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) enum OAuth2CredentialsLocation {
    #[allow(unused)]
    RequestBody,
    #[allow(unused)]
    BasicAuthorizationHeader,
}
#[derive(Clone, Debug)]
pub(crate) enum OAuthBodyContentType {
    #[allow(unused)]
    Json,
    #[allow(unused)]
    Form,
}
#[derive(Clone, Debug)]
pub(crate) struct OAuth2ProviderConfig {
    pub(crate) base_url: String,
    pub(crate) default_token_url: String,
    pub(crate) access_token_pointer: String,
    pub(crate) expires_in_pointer: String,
    pub(crate) credentials_location: OAuth2CredentialsLocation,
    pub(crate) body_content: OAuthBodyContentType,
    pub(crate) request_mutator: Box<AuthProvider>,
}
#[derive(Clone, Debug)]
pub(crate) struct OAuth2Provider {
    config: OAuth2ProviderConfig,
    flow: OAuth2Flow,
    access_token: Option<String>,
    expires_at: Option<std::time::SystemTime>,
}
impl OAuth2Provider {
    #[allow(unused)]
    pub fn new(config: OAuth2ProviderConfig, flow: OAuth2Flow) -> Self {
        Self {
            config,
            flow,
            access_token: None,
            expires_at: None,
        }
    }
    async fn refresh(&mut self) -> crate::SdkResult<String> {
        let mut token_url = self
            .flow
            .token_url()
            .unwrap_or(self.config.default_token_url.clone());
        if token_url.starts_with('/') {
            token_url = format!(
                "{base}/{path}", base = & self.config.base_url.trim_end_matches('/'),
                path = token_url.trim_start_matches('/')
            )
                .trim_end_matches('/')
                .into();
        }
        let mut access_token_builder = reqwest::Client::new().post(&token_url);
        let (client_id, client_secret, mut req_data) = match &self.flow {
            OAuth2Flow::Password(pwd) => {
                let mut data = std::collections::BTreeMap::from_iter([
                    (
                        "grant_type",
                        pwd.grant_type.clone().unwrap_or("password".to_string()),
                    ),
                    ("username", pwd.username.clone()),
                    ("password", pwd.password.clone()),
                ]);
                if let Some(s) = &pwd.scope {
                    data.insert("scope", s.join(" ").to_string());
                }
                (pwd.client_id.clone(), pwd.client_secret.clone(), data)
            }
            OAuth2Flow::ClientCredentials(creds) => {
                let mut data = std::collections::BTreeMap::from_iter([
                    (
                        "grant_type",
                        creds
                            .grant_type
                            .clone()
                            .unwrap_or("client_credentials".to_string()),
                    ),
                ]);
                if let Some(s) = &creds.scope {
                    data.insert("scope", s.join(" ").to_string());
                }
                (Some(creds.client_id.clone()), Some(creds.client_secret.clone()), data)
            }
        };
        match &self.config.credentials_location {
            OAuth2CredentialsLocation::RequestBody => {
                if let Some(id) = client_id {
                    req_data.insert("client_id", id);
                }
                if let Some(secret) = client_secret {
                    req_data.insert("client_secret", secret);
                }
            }
            OAuth2CredentialsLocation::BasicAuthorizationHeader => {
                if client_id.is_some() || client_secret.is_some() {
                    access_token_builder = access_token_builder
                        .basic_auth(
                            client_id.unwrap_or_default(),
                            Some(client_secret.unwrap_or_default()),
                        );
                }
            }
        }
        match self.config.body_content {
            OAuthBodyContentType::Json => {
                access_token_builder = access_token_builder.json(&req_data);
                access_token_builder = access_token_builder
                    .header("content-type", "application/json");
            }
            OAuthBodyContentType::Form => {
                access_token_builder = access_token_builder
                    .body(
                        serde_urlencoded::to_string(&req_data)
                            .expect("failed url encoding body"),
                    );
                access_token_builder = access_token_builder
                    .header("content-type", "application/x-www-form-urlencoded");
            }
        }
        let access_token_res = access_token_builder.send().await?;
        if access_token_res.error_for_status_ref().is_err() {
            return Err(
                crate::Error::Api(crate::ApiError::new("POST", access_token_res).await),
            );
        }
        let access_token_res_json = crate::core::response::process_json::<
            serde_json::Value,
        >(access_token_res)
            .await?;
        let access_token = access_token_res_json
            .pointer(&self.config.access_token_pointer)
            .cloned()
            .and_then(|v| serde_json::from_value::<String>(v).ok())
            .unwrap_or_default();
        self.access_token = Some(access_token.clone());
        let expires_in = access_token_res_json
            .pointer(&self.config.expires_in_pointer)
            .cloned()
            .and_then(|v| serde_json::from_value::<u64>(v).ok())
            .unwrap_or(600);
        let expires_at = std::time::SystemTime::now()
            + std::time::Duration::from_secs(expires_in - 60);
        self.expires_at = Some(expires_at);
        Ok(access_token)
    }
    pub async fn add_auth(
        &mut self,
        mut builder: reqwest::RequestBuilder,
    ) -> crate::SdkResult<reqwest::RequestBuilder> {
        let access_token = if let Some(access) = &self.access_token {
            if self
                .expires_at
                .is_some_and(|e| e.duration_since(std::time::SystemTime::now()).is_err())
            {
                self.refresh().await?
            } else {
                access.clone()
            }
        } else {
            self.refresh().await?
        };
        match *self.config.request_mutator.clone() {
            AuthProvider::Basic(_, _) => {
                builder = builder.basic_auth(access_token, None::<String>);
            }
            AuthProvider::KeyQuery(name, _) => {
                builder = builder.query(&[(name, access_token)]);
            }
            AuthProvider::KeyHeader(name, _) => {
                builder = builder.header(name, access_token);
            }
            AuthProvider::KeyCookie(name, _) => {
                let cookie_val = format!("{name}={access_token}");
                builder = builder.header(reqwest::header::COOKIE, cookie_val);
            }
            AuthProvider::Bearer(_) | AuthProvider::OAuth2(_) => {
                builder = builder.bearer_auth(access_token);
            }
        }
        Ok(builder)
    }
}
