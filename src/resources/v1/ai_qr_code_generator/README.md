
### create <a name="create"></a>
Create AI QR Code

Create an AI QR code. Each QR code costs 20 frames.

**API Endpoint**: `POST /v1/ai-qr-code-generator`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_qr_code_generator()
    .create(magic_hour::resources::v1::ai_qr_code_generator::CreateRequest {
        data: magic_hour::models::PostV1AiQrCodeGeneratorBody {
            content: "https://magichour.ai".to_string(),
            name: Some("Qr Code image".to_string()),
            style: magic_hour::models::PostV1AiQrCodeGeneratorBodyStyle {
                art_style: "Watercolor".to_string(),
            },
        },
    })
    .await;
```

**Upgrade to see all examples**