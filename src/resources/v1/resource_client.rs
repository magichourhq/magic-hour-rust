#[derive(Debug)]
pub struct V1Client<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> V1Client<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    pub fn image_projects(
        &mut self,
    ) -> crate::resources::v1::image_projects::resource_client::ImageProjectsClient {
        crate::resources::v1::image_projects::resource_client::ImageProjectsClient::_new(
            self.base_client,
        )
    }
    pub fn video_projects(
        &mut self,
    ) -> crate::resources::v1::video_projects::resource_client::VideoProjectsClient {
        crate::resources::v1::video_projects::resource_client::VideoProjectsClient::_new(
            self.base_client,
        )
    }
    pub fn ai_clothes_changer(
        &mut self,
    ) -> crate::resources::v1::ai_clothes_changer::resource_client::AiClothesChangerClient {
        crate::resources::v1::ai_clothes_changer::resource_client::AiClothesChangerClient::_new(
            self.base_client,
        )
    }
    pub fn ai_face_editor(
        &mut self,
    ) -> crate::resources::v1::ai_face_editor::resource_client::AiFaceEditorClient {
        crate::resources::v1::ai_face_editor::resource_client::AiFaceEditorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_headshot_generator(
        &mut self,
    ) -> crate::resources::v1::ai_headshot_generator::resource_client::AiHeadshotGeneratorClient {
        crate::resources::v1::ai_headshot_generator::resource_client::AiHeadshotGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_image_generator(
        &mut self,
    ) -> crate::resources::v1::ai_image_generator::resource_client::AiImageGeneratorClient {
        crate::resources::v1::ai_image_generator::resource_client::AiImageGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_image_upscaler(
        &mut self,
    ) -> crate::resources::v1::ai_image_upscaler::resource_client::AiImageUpscalerClient {
        crate::resources::v1::ai_image_upscaler::resource_client::AiImageUpscalerClient::_new(
            self.base_client,
        )
    }
    pub fn ai_meme_generator(
        &mut self,
    ) -> crate::resources::v1::ai_meme_generator::resource_client::AiMemeGeneratorClient {
        crate::resources::v1::ai_meme_generator::resource_client::AiMemeGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_photo_editor(
        &mut self,
    ) -> crate::resources::v1::ai_photo_editor::resource_client::AiPhotoEditorClient {
        crate::resources::v1::ai_photo_editor::resource_client::AiPhotoEditorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_qr_code_generator(
        &mut self,
    ) -> crate::resources::v1::ai_qr_code_generator::resource_client::AiQrCodeGeneratorClient {
        crate::resources::v1::ai_qr_code_generator::resource_client::AiQrCodeGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_talking_photo(
        &mut self,
    ) -> crate::resources::v1::ai_talking_photo::resource_client::AiTalkingPhotoClient {
        crate::resources::v1::ai_talking_photo::resource_client::AiTalkingPhotoClient::_new(
            self.base_client,
        )
    }
    pub fn animation(
        &mut self,
    ) -> crate::resources::v1::animation::resource_client::AnimationClient {
        crate::resources::v1::animation::resource_client::AnimationClient::_new(
            self.base_client,
        )
    }
    pub fn face_swap(
        &mut self,
    ) -> crate::resources::v1::face_swap::resource_client::FaceSwapClient {
        crate::resources::v1::face_swap::resource_client::FaceSwapClient::_new(
            self.base_client,
        )
    }
    pub fn face_swap_photo(
        &mut self,
    ) -> crate::resources::v1::face_swap_photo::resource_client::FaceSwapPhotoClient {
        crate::resources::v1::face_swap_photo::resource_client::FaceSwapPhotoClient::_new(
            self.base_client,
        )
    }
    pub fn files(
        &mut self,
    ) -> crate::resources::v1::files::resource_client::FilesClient {
        crate::resources::v1::files::resource_client::FilesClient::_new(self.base_client)
    }
    pub fn image_background_remover(
        &mut self,
    ) -> crate::resources::v1::image_background_remover::resource_client::ImageBackgroundRemoverClient {
        crate::resources::v1::image_background_remover::resource_client::ImageBackgroundRemoverClient::_new(
            self.base_client,
        )
    }
    pub fn image_to_video(
        &mut self,
    ) -> crate::resources::v1::image_to_video::resource_client::ImageToVideoClient {
        crate::resources::v1::image_to_video::resource_client::ImageToVideoClient::_new(
            self.base_client,
        )
    }
    pub fn lip_sync(
        &mut self,
    ) -> crate::resources::v1::lip_sync::resource_client::LipSyncClient {
        crate::resources::v1::lip_sync::resource_client::LipSyncClient::_new(
            self.base_client,
        )
    }
    pub fn text_to_video(
        &mut self,
    ) -> crate::resources::v1::text_to_video::resource_client::TextToVideoClient {
        crate::resources::v1::text_to_video::resource_client::TextToVideoClient::_new(
            self.base_client,
        )
    }
    pub fn video_to_video(
        &mut self,
    ) -> crate::resources::v1::video_to_video::resource_client::VideoToVideoClient {
        crate::resources::v1::video_to_video::resource_client::VideoToVideoClient::_new(
            self.base_client,
        )
    }
}
