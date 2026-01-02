# Magic Hour Rust SDK

[![Crates.io Version](https://img.shields.io/crates/v/magic_hour)](https://crates.io/crates/magic_hour)

Magic Hour provides an API (beta) that can be integrated into your own application to generate videos and images using AI.

Webhook documentation can be found [here](https://magichour.ai/docs/webhook).

If you have any questions, please reach out to us via [discord](https://discord.gg/JX5rgsZaJp).

## Install

```
cargo add magic_hour
```

## Usage

```rust
use magic_hour;

let client = magic_hour::Client::default()
    // generate your API Key at https://magichour.ai/developer
    .with_bearer_auth("my api token");
```

> [!WARNING]
> Any API call that renders a video will utilize frames in your account.

## Module Documentation and Snippets

### [v1.ai_clothes_changer](src/resources/v1/ai_clothes_changer/README.md)

- [create](src/resources/v1/ai_clothes_changer/README.md#create) - AI Clothes Changer

### [v1.ai_face_editor](src/resources/v1/ai_face_editor/README.md)

- [create](src/resources/v1/ai_face_editor/README.md#create) - AI Face Editor

### [v1.ai_gif_generator](src/resources/v1/ai_gif_generator/README.md)

- [create](src/resources/v1/ai_gif_generator/README.md#create) - AI GIF Generator

### [v1.ai_headshot_generator](src/resources/v1/ai_headshot_generator/README.md)

- [create](src/resources/v1/ai_headshot_generator/README.md#create) - AI Headshot Generator

### [v1.ai_image_editor](src/resources/v1/ai_image_editor/README.md)

- [create](src/resources/v1/ai_image_editor/README.md#create) - AI Image Editor

### [v1.ai_image_generator](src/resources/v1/ai_image_generator/README.md)

- [create](src/resources/v1/ai_image_generator/README.md#create) - AI Image Generator

### [v1.ai_image_upscaler](src/resources/v1/ai_image_upscaler/README.md)

- [create](src/resources/v1/ai_image_upscaler/README.md#create) - AI Image Upscaler

### [v1.ai_meme_generator](src/resources/v1/ai_meme_generator/README.md)

- [create](src/resources/v1/ai_meme_generator/README.md#create) - AI Meme Generator

### [v1.ai_photo_editor](src/resources/v1/ai_photo_editor/README.md)

- [create](src/resources/v1/ai_photo_editor/README.md#create) - AI Photo Editor

### [v1.ai_qr_code_generator](src/resources/v1/ai_qr_code_generator/README.md)

- [create](src/resources/v1/ai_qr_code_generator/README.md#create) - AI QR Code Generator

### [v1.ai_talking_photo](src/resources/v1/ai_talking_photo/README.md)

- [create](src/resources/v1/ai_talking_photo/README.md#create) - AI Talking Photo

### [v1.ai_voice_cloner](src/resources/v1/ai_voice_cloner/README.md)

- [create](src/resources/v1/ai_voice_cloner/README.md#create) - AI Voice Cloner

### [v1.ai_voice_generator](src/resources/v1/ai_voice_generator/README.md)

- [create](src/resources/v1/ai_voice_generator/README.md#create) - AI Voice Generator

### [v1.animation](src/resources/v1/animation/README.md)

- [create](src/resources/v1/animation/README.md#create) - Animation

### [v1.audio_projects](src/resources/v1/audio_projects/README.md)

- [delete](src/resources/v1/audio_projects/README.md#delete) - Delete audio
- [get](src/resources/v1/audio_projects/README.md#get) - Get audio details

### [v1.auto_subtitle_generator](src/resources/v1/auto_subtitle_generator/README.md)

- [create](src/resources/v1/auto_subtitle_generator/README.md#create) - Auto Subtitle Generator

### [v1.face_detection](src/resources/v1/face_detection/README.md)

- [create](src/resources/v1/face_detection/README.md#create) - Face Detection
- [get](src/resources/v1/face_detection/README.md#get) - Get face detection details

### [v1.face_swap](src/resources/v1/face_swap/README.md)

- [create](src/resources/v1/face_swap/README.md#create) - Face Swap Video

### [v1.face_swap_photo](src/resources/v1/face_swap_photo/README.md)

- [create](src/resources/v1/face_swap_photo/README.md#create) - Face Swap Photo

### [v1.files.upload_urls](src/resources/v1/files/upload_urls/README.md)

- [create](src/resources/v1/files/upload_urls/README.md#create) - Generate asset upload urls

### [v1.image_background_remover](src/resources/v1/image_background_remover/README.md)

- [create](src/resources/v1/image_background_remover/README.md#create) - Image Background Remover

### [v1.image_projects](src/resources/v1/image_projects/README.md)

- [delete](src/resources/v1/image_projects/README.md#delete) - Delete image
- [get](src/resources/v1/image_projects/README.md#get) - Get image details

### [v1.image_to_video](src/resources/v1/image_to_video/README.md)

- [create](src/resources/v1/image_to_video/README.md#create) - Image-to-Video

### [v1.lip_sync](src/resources/v1/lip_sync/README.md)

- [create](src/resources/v1/lip_sync/README.md#create) - Lip Sync

### [v1.photo_colorizer](src/resources/v1/photo_colorizer/README.md)

- [create](src/resources/v1/photo_colorizer/README.md#create) - Photo Colorizer

### [v1.text_to_video](src/resources/v1/text_to_video/README.md)

- [create](src/resources/v1/text_to_video/README.md#create) - Text-to-Video

### [v1.video_projects](src/resources/v1/video_projects/README.md)

- [delete](src/resources/v1/video_projects/README.md#delete) - Delete video
- [get](src/resources/v1/video_projects/README.md#get) - Get video details

### [v1.video_to_video](src/resources/v1/video_to_video/README.md)

- [create](src/resources/v1/video_to_video/README.md#create) - Video-to-Video

<!-- MODULE DOCS END -->
