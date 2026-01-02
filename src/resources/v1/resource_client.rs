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
    pub fn audio_projects(
        &mut self,
    ) -> crate::resources::v1::audio_projects::resource_client::AudioProjectsClient<'_> {
        crate::resources::v1::audio_projects::resource_client::AudioProjectsClient::_new(
            self.base_client,
        )
    }
    pub fn image_projects(
        &mut self,
    ) -> crate::resources::v1::image_projects::resource_client::ImageProjectsClient<'_> {
        crate::resources::v1::image_projects::resource_client::ImageProjectsClient::_new(
            self.base_client,
        )
    }
    pub fn video_projects(
        &mut self,
    ) -> crate::resources::v1::video_projects::resource_client::VideoProjectsClient<'_> {
        crate::resources::v1::video_projects::resource_client::VideoProjectsClient::_new(
            self.base_client,
        )
    }
    pub fn face_detection(
        &mut self,
    ) -> crate::resources::v1::face_detection::resource_client::FaceDetectionClient<'_> {
        crate::resources::v1::face_detection::resource_client::FaceDetectionClient::_new(
            self.base_client,
        )
    }
    pub fn ai_clothes_changer(
        &mut self,
    ) -> crate::resources::v1::ai_clothes_changer::resource_client::AiClothesChangerClient<
        '_,
    > {
        crate::resources::v1::ai_clothes_changer::resource_client::AiClothesChangerClient::_new(
            self.base_client,
        )
    }
    pub fn ai_face_editor(
        &mut self,
    ) -> crate::resources::v1::ai_face_editor::resource_client::AiFaceEditorClient<'_> {
        crate::resources::v1::ai_face_editor::resource_client::AiFaceEditorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_gif_generator(
        &mut self,
    ) -> crate::resources::v1::ai_gif_generator::resource_client::AiGifGeneratorClient<
        '_,
    > {
        crate::resources::v1::ai_gif_generator::resource_client::AiGifGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_headshot_generator(
        &mut self,
    ) -> crate::resources::v1::ai_headshot_generator::resource_client::AiHeadshotGeneratorClient<
        '_,
    > {
        crate::resources::v1::ai_headshot_generator::resource_client::AiHeadshotGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_image_editor(
        &mut self,
    ) -> crate::resources::v1::ai_image_editor::resource_client::AiImageEditorClient<
        '_,
    > {
        crate::resources::v1::ai_image_editor::resource_client::AiImageEditorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_image_generator(
        &mut self,
    ) -> crate::resources::v1::ai_image_generator::resource_client::AiImageGeneratorClient<
        '_,
    > {
        crate::resources::v1::ai_image_generator::resource_client::AiImageGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_image_upscaler(
        &mut self,
    ) -> crate::resources::v1::ai_image_upscaler::resource_client::AiImageUpscalerClient<
        '_,
    > {
        crate::resources::v1::ai_image_upscaler::resource_client::AiImageUpscalerClient::_new(
            self.base_client,
        )
    }
    pub fn ai_meme_generator(
        &mut self,
    ) -> crate::resources::v1::ai_meme_generator::resource_client::AiMemeGeneratorClient<
        '_,
    > {
        crate::resources::v1::ai_meme_generator::resource_client::AiMemeGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_photo_editor(
        &mut self,
    ) -> crate::resources::v1::ai_photo_editor::resource_client::AiPhotoEditorClient<
        '_,
    > {
        crate::resources::v1::ai_photo_editor::resource_client::AiPhotoEditorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_qr_code_generator(
        &mut self,
    ) -> crate::resources::v1::ai_qr_code_generator::resource_client::AiQrCodeGeneratorClient<
        '_,
    > {
        crate::resources::v1::ai_qr_code_generator::resource_client::AiQrCodeGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn ai_talking_photo(
        &mut self,
    ) -> crate::resources::v1::ai_talking_photo::resource_client::AiTalkingPhotoClient<
        '_,
    > {
        crate::resources::v1::ai_talking_photo::resource_client::AiTalkingPhotoClient::_new(
            self.base_client,
        )
    }
    pub fn ai_voice_cloner(
        &mut self,
    ) -> crate::resources::v1::ai_voice_cloner::resource_client::AiVoiceClonerClient<
        '_,
    > {
        crate::resources::v1::ai_voice_cloner::resource_client::AiVoiceClonerClient::_new(
            self.base_client,
        )
    }
    pub fn ai_voice_generator(
        &mut self,
    ) -> crate::resources::v1::ai_voice_generator::resource_client::AiVoiceGeneratorClient<
        '_,
    > {
        crate::resources::v1::ai_voice_generator::resource_client::AiVoiceGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn animation(
        &mut self,
    ) -> crate::resources::v1::animation::resource_client::AnimationClient<'_> {
        crate::resources::v1::animation::resource_client::AnimationClient::_new(
            self.base_client,
        )
    }
    pub fn auto_subtitle_generator(
        &mut self,
    ) -> crate::resources::v1::auto_subtitle_generator::resource_client::AutoSubtitleGeneratorClient<
        '_,
    > {
        crate::resources::v1::auto_subtitle_generator::resource_client::AutoSubtitleGeneratorClient::_new(
            self.base_client,
        )
    }
    pub fn face_swap(
        &mut self,
    ) -> crate::resources::v1::face_swap::resource_client::FaceSwapClient<'_> {
        crate::resources::v1::face_swap::resource_client::FaceSwapClient::_new(
            self.base_client,
        )
    }
    pub fn face_swap_photo(
        &mut self,
    ) -> crate::resources::v1::face_swap_photo::resource_client::FaceSwapPhotoClient<
        '_,
    > {
        crate::resources::v1::face_swap_photo::resource_client::FaceSwapPhotoClient::_new(
            self.base_client,
        )
    }
    pub fn files(
        &mut self,
    ) -> crate::resources::v1::files::resource_client::FilesClient<'_> {
        crate::resources::v1::files::resource_client::FilesClient::_new(self.base_client)
    }
    pub fn image_background_remover(
        &mut self,
    ) -> crate::resources::v1::image_background_remover::resource_client::ImageBackgroundRemoverClient<
        '_,
    > {
        crate::resources::v1::image_background_remover::resource_client::ImageBackgroundRemoverClient::_new(
            self.base_client,
        )
    }
    pub fn image_to_video(
        &mut self,
    ) -> crate::resources::v1::image_to_video::resource_client::ImageToVideoClient<'_> {
        crate::resources::v1::image_to_video::resource_client::ImageToVideoClient::_new(
            self.base_client,
        )
    }
    pub fn lip_sync(
        &mut self,
    ) -> crate::resources::v1::lip_sync::resource_client::LipSyncClient<'_> {
        crate::resources::v1::lip_sync::resource_client::LipSyncClient::_new(
            self.base_client,
        )
    }
    pub fn photo_colorizer(
        &mut self,
    ) -> crate::resources::v1::photo_colorizer::resource_client::PhotoColorizerClient<
        '_,
    > {
        crate::resources::v1::photo_colorizer::resource_client::PhotoColorizerClient::_new(
            self.base_client,
        )
    }
    pub fn text_to_video(
        &mut self,
    ) -> crate::resources::v1::text_to_video::resource_client::TextToVideoClient<'_> {
        crate::resources::v1::text_to_video::resource_client::TextToVideoClient::_new(
            self.base_client,
        )
    }
    pub fn video_to_video(
        &mut self,
    ) -> crate::resources::v1::video_to_video::resource_client::VideoToVideoClient<'_> {
        crate::resources::v1::video_to_video::resource_client::VideoToVideoClient::_new(
            self.base_client,
        )
    }
}
