#[derive(Debug)]
pub struct EstimateClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> EstimateClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    pub fn image(
        &mut self,
    ) -> crate::resources::v1::estimate::image::resource_client::ImageClient<'_> {
        crate::resources::v1::estimate::image::resource_client::ImageClient::_new(
            self.base_client,
        )
    }
}
