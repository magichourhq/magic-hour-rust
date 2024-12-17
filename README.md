
# Magic Hour API Rust SDK

## Overview

# Introduction 

Magic Hour provides an API (beta) that can be integrated into your own application to generate videos using AI. 

Webhook documentation can be found [here](https://magichour.ai/docs/webhook).

If you have any questions, please reach out to us via [discord](https://discord.gg/JX5rgsZaJp).

# Authentication

Every request requires an API key.

To get started, first generate your API key [here](https://magichour.ai/settings/developer).

Then, add the `Authorization` header to the request.

| Key | Value |
|-|-|
| Authorization | Bearer mhk_live_apikey |

> **Warning**: any API call that renders a video will utilize frames in your account.


### Example Client Initialization

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
```

## Module Documentation and Snippets

### [v1.ai_headshot_generator](src/resources/v1/ai_headshot_generator/README.md)

* [create](src/resources/v1/ai_headshot_generator/README.md#create) - Create AI Headshots

### [v1.ai_image_generator](src/resources/v1/ai_image_generator/README.md)

* [create](src/resources/v1/ai_image_generator/README.md#create) - Create AI Images

### [v1.ai_image_upscaler](src/resources/v1/ai_image_upscaler/README.md)

* [create](src/resources/v1/ai_image_upscaler/README.md#create) - Create Upscaled Image

### [v1.ai_photo_editor](src/resources/v1/ai_photo_editor/README.md)

* [create](src/resources/v1/ai_photo_editor/README.md#create) - AI Photo Editor

### [v1.ai_qr_code_generator](src/resources/v1/ai_qr_code_generator/README.md)

* [create](src/resources/v1/ai_qr_code_generator/README.md#create) - Create AI QR Code

### [v1.animation](src/resources/v1/animation/README.md)

* [create](src/resources/v1/animation/README.md#create) - Create Animation

### [v1.face_swap](src/resources/v1/face_swap/README.md)

* [create](src/resources/v1/face_swap/README.md#create) - Create Face Swap video

### [v1.face_swap_photo](src/resources/v1/face_swap_photo/README.md)

* [create](src/resources/v1/face_swap_photo/README.md#create) - Create Face Swap Photo

### [v1.files.upload_urls](src/resources/v1/files/upload_urls/README.md)

* [create](src/resources/v1/files/upload_urls/README.md#create) - Generate asset upload urls

### [v1.image_background_remover](src/resources/v1/image_background_remover/README.md)

* [create](src/resources/v1/image_background_remover/README.md#create) - Image Background Remover

### [v1.image_projects](src/resources/v1/image_projects/README.md)

* [delete](src/resources/v1/image_projects/README.md#delete) - Delete image
* [get](src/resources/v1/image_projects/README.md#get) - Get image project details

### [v1.image_to_video](src/resources/v1/image_to_video/README.md)

* [create](src/resources/v1/image_to_video/README.md#create) - Create Image-to-Video

### [v1.lip_sync](src/resources/v1/lip_sync/README.md)

* [create](src/resources/v1/lip_sync/README.md#create) - Create Lip Sync video

### [v1.text_to_video](src/resources/v1/text_to_video/README.md)

* [create](src/resources/v1/text_to_video/README.md#create) - Create Text-to-Video

### [v1.video_projects](src/resources/v1/video_projects/README.md)

* [delete](src/resources/v1/video_projects/README.md#delete) - Delete video
* [get](src/resources/v1/video_projects/README.md#get) - Get video details

### [v1.video_to_video](src/resources/v1/video_to_video/README.md)

* [create](src/resources/v1/video_to_video/README.md#create) - Create Video-to-Video

<!-- MODULE DOCS END -->
