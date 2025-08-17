#[derive(Debug)]
pub struct FilesClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> FilesClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    pub fn upload_urls(
        &mut self,
    ) -> crate::resources::v1::files::upload_urls::resource_client::UploadUrlsClient<
        '_,
    > {
        crate::resources::v1::files::upload_urls::resource_client::UploadUrlsClient::_new(
            self.base_client,
        )
    }
}
