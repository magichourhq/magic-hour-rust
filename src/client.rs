#[derive(Clone, Debug, Default)]
pub struct Client {
    base_client: crate::core::base_client::BaseClient,
}
impl Client {
    /// Override the default URL environment
    pub fn with_environment(mut self, env: crate::environment::Environment) -> Self {
        self.base_client.base_url = crate::environment::BaseUrl::Env(env);
        self
    }
    /// Override the default URL with a custom base URL
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_client.base_url = crate::environment::BaseUrl::Custom(base_url.into());
        self
    }
    /// Override the default underlying reqwest client
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.base_client.client = client;
        self
    }
    pub fn with_bearer_auth(mut self, token: &str) -> Self {
        self.base_client
            .auth
            .insert(
                "bearerAuth".into(),
                crate::core::auth::AuthProvider::Bearer(token.into()),
            );
        self
    }
    pub fn v1(&mut self) -> crate::resources::v1::resource_client::V1Client {
        crate::resources::v1::resource_client::V1Client::_new(&mut self.base_client)
    }
}
