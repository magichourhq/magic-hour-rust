
### AI QR Code <a name="create"></a>

Create an AI QR code. Each QR code costs 20 credits.

**API Endpoint**: `POST /v1/ai-qr-code-generator`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `content` | ✓ | The content of the QR code. | `"https://magichour.ai".to_string()` |
| `style` | ✓ |  | `V1AiQrCodeGeneratorCreateBodyStyle {art_style: "Watercolor".to_string()}` |
| `name` | ✗ | The name of image | `"Qr Code image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_qr_code_generator()
    .create(magic_hour::resources::v1::ai_qr_code_generator::CreateRequest {
        content: "https://magichour.ai".to_string(),
        name: Some("Qr Code image".to_string()),
        style: magic_hour::models::V1AiQrCodeGeneratorCreateBodyStyle {
            art_style: "Watercolor".to_string(),
        },
    })
    .await;
```

#### Response

##### Type
[V1AiQrCodeGeneratorCreateResponse](/src/models/v1_ai_qr_code_generator_create_response.rs)

##### Example
`V1AiQrCodeGeneratorCreateResponse {credits_charged: 20, frame_cost: 20, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
