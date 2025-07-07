use std::fmt::Display;
pub(crate) static _DEFAULT_SERVICE_NAME: &str = "__default_service__";
#[derive(Clone, Debug, Default)]
pub(crate) struct BaseClient {
    pub(crate) base_url: std::collections::HashMap<String, String>,
    pub(crate) client: reqwest::Client,
    pub(crate) auth: std::collections::HashMap<String, super::auth::AuthProvider>,
}
impl BaseClient {
    #[allow(unused)]
    pub fn default_base_url(
        base_url: impl Display,
    ) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::from_iter([
            (_DEFAULT_SERVICE_NAME.to_string(), base_url.to_string()),
        ])
    }
    #[allow(unused)]
    pub fn build_url(&self, endpoint: &str, service: Option<String>) -> String {
        let base = self
            .base_url
            .get(&service.unwrap_or(_DEFAULT_SERVICE_NAME.to_string()))
            .cloned()
            .unwrap_or_default();
        format!("{}/{}", base.trim_end_matches('/'), endpoint.trim_start_matches('/'))
            .trim_end_matches('/')
            .into()
    }
    #[allow(unused)]
    pub async fn apply_auths_to_builder(
        &mut self,
        mut builder: reqwest::RequestBuilder,
        auth_names: &[&str],
    ) -> crate::SdkResult<reqwest::RequestBuilder> {
        for name in auth_names {
            if let Some(provider) = self.auth.get_mut(*name) {
                builder = provider.add_auth(builder).await?;
            }
        }
        Ok(builder)
    }
    #[allow(unused)]
    pub async fn error_for_status(
        &self,
        method: &str,
        res: reqwest::Response,
    ) -> crate::SdkResult<reqwest::Response> {
        if res.error_for_status_ref().is_err() {
            Err(crate::Error::Api(crate::ApiError::new(method, res).await))
        } else {
            Ok(res)
        }
    }
}
