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

    /// Upload a file to Magic Hour storage
    ///
    /// This method handles uploading a local file to Magic Hour's storage and returns
    /// a file path that can be used in other API calls for video_file_path, image_file_path,
    /// or audio_file_path parameters.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the local file to upload
    ///
    /// # Returns
    ///
    /// Returns a `SdkResult<String>` containing the file path to use in API calls
    ///
    /// # Example
    ///
    /// ```ignore
    /// let file_path = client.v1().files().upload_file("/path/to/image.jpg").await?;
    /// // Now use file_path in other API calls
    /// ```
    pub async fn upload_file(&mut self, file_path: &str) -> crate::SdkResult<String> {
        // Determine file extension and type
        let path = std::path::Path::new(file_path);
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| crate::core::error::Error::Custom("Unable to determine file extension".to_string()))?;

        let file_type = Self::determine_file_type(extension)?;

        // Create request for upload URL
        let request = super::upload_urls::request_types::CreateRequest {
            items: vec![crate::models::V1FilesUploadUrlsCreateBodyItemsItem {
                extension: extension.to_string(),
                type_: file_type,
            }],
        };

        // Get upload URL
        let response = self.upload_urls().create(request).await?;

        if response.items.is_empty() {
            return Err(crate::core::error::Error::Custom("No upload URL received".to_string()));
        }

        let upload_item = &response.items[0];

        // Read file content
        let file_content = std::fs::read(file_path)
            .map_err(|e| crate::core::error::Error::Custom(format!("Failed to read file: {}", e)))?;

        // Upload file using PUT request
        let client = reqwest::Client::new();
        let put_response = client
            .put(&upload_item.upload_url)
            .body(file_content)
            .send()
            .await?;

        if !put_response.status().is_success() {
            return Err(crate::core::error::Error::Custom(format!("Upload failed with status: {}", put_response.status())));
        }

        // Return the file path for use in other API calls
        Ok(upload_item.file_path.clone())
    }

    fn determine_file_type(extension: &str) -> crate::SdkResult<crate::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum> {
        match extension.to_lowercase().as_str() {
            // Video extensions
            "mp4" | "m4v" | "mov" | "webm" | "avi" | "wmv" | "flv" | "mkv" | "mpg" | "mpeg" => {
                Ok(crate::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Video)
            },
            // Audio extensions
            "mp3" | "wav" | "aac" | "aiff" | "flac" | "ogg" | "wma" | "m4a" => {
                Ok(crate::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Audio)
            },
            // Image extensions
            "png" | "jpg" | "jpeg" | "webp" | "avif" | "jp2" | "tiff" | "bmp" | "gif" | "svg" => {
                Ok(crate::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Image)
            },
            _ => Err(crate::core::error::Error::Custom(format!("Unsupported file extension: {}", extension))),
        }
    }
}
