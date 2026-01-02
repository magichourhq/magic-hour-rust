# v1.ai_meme_generator

## Module Functions

### AI Meme Generator <a name="create"></a>

Create an AI generated meme. Each meme costs 10 credits.

**API Endpoint**: `POST /v1/ai-meme-generator`

#### Parameters

| Parameter       | Required | Description                                           | Example                                                                                                                                                                                   |
| --------------- | :------: | ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `style`         |    ✓     |                                                       | `V1AiMemeGeneratorCreateBodyStyle {search_web: Some(false), template: V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DrakeHotlineBling, topic: "When the code finally works".to_string()}` |
| `└─ search_web` |    ✗     | Whether to search the web for meme content.           | `false`                                                                                                                                                                                   |
| `└─ template`   |    ✓     | To use our templates, pass in one of the enum values. | `V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DrakeHotlineBling`                                                                                                                         |
| `└─ topic`      |    ✓     | The topic of the meme.                                | `"When the code finally works".to_string()`                                                                                                                                               |
| `name`          |    ✗     | The name of the meme.                                 | `"My Funny Meme".to_string()`                                                                                                                                                             |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_meme_generator()
    .create(magic_hour::resources::v1::ai_meme_generator::CreateRequest {
        name: Some("My Funny Meme".to_string()),
        style: magic_hour::models::V1AiMemeGeneratorCreateBodyStyle {
            search_web: Some(false),
            template: magic_hour::models::V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DrakeHotlineBling,
            topic: "When the code finally works".to_string(),
        },
    })
    .await;
```

#### Response

##### Type

[V1AiMemeGeneratorCreateResponse](/src/models/v1_ai_meme_generator_create_response.rs)

##### Example

```rust
V1AiMemeGeneratorCreateResponse {credits_charged: 10, frame_cost: 10, id: "cuid-example".to_string()}
```
