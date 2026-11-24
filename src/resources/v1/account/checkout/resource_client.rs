#[derive(Debug)]
pub struct CheckoutClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> CheckoutClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Create checkout
    ///
    /// Create a Stripe Checkout link for the account that owns the API key. Nothing is charged until the checkout is completed in a browser.
    ///
    /// POST /v1/account/checkout
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AccountCheckoutCreateResponse> {
        let url = self.base_client.build_url("/v1/account/checkout", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AccountCheckoutCreateBody {
                    credit_shortfall: request.credit_shortfall,
                    pack_count: request.pack_count,
                    type_: request.type_,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AccountCheckoutCreateResponse,
        >(response)
            .await
    }
}
