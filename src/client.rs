use std::fmt::Display;
#[derive(Clone, Debug)]
pub struct Client {
    base_client: crate::core::base_client::BaseClient,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            base_client: crate::core::base_client::BaseClient {
                base_url: crate::core::base_client::BaseClient::default_base_url(
                    crate::environment::Environment::default(),
                ),
                ..Default::default()
            },
        }
    }
}
impl Client {
    /// Override the default underlying reqwest client
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.base_client.client = client;
        self
    }
    /// Override the default URL environment
    pub fn with_environment(mut self, env: impl Display) -> Self {
        self.base_client.base_url = crate::core::base_client::BaseClient::default_base_url(
            env,
        );
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
    pub fn v1(&mut self) -> crate::resources::v1::resource_client::V1Client<'_> {
        crate::resources::v1::resource_client::V1Client::_new(&mut self.base_client)
    }
}
