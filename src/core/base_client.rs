#[derive(Clone, Debug, Default)]
pub(crate) struct BaseClient {
    pub(crate) base_url: crate::environment::BaseUrl,
    pub(crate) client: reqwest::Client,
    pub(crate) auth: std::collections::BTreeMap<String, super::auth::AuthProvider>,
}
impl BaseClient {
    pub fn build_url(&self, endpoint: &str) -> String {
        let base = self.base_url.to_string();
        format!("{}/{}", base.trim_end_matches('/'), endpoint.trim_start_matches('/'))
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
