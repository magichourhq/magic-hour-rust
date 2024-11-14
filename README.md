
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

### SDK Usage 
 See [SDK Examples](SDK_EXAMPLES.md) for example usage of all SDK functionality