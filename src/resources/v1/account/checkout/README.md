# v1.account.checkout

## Module Functions

### Create checkout <a name="create"></a>

Create a Stripe Checkout link for the account that owns the API key. Nothing is charged until the checkout is completed in a browser.

**API Endpoint**: `POST /v1/account/checkout`

#### Parameters

| Parameter          | Required | Description                                                                                                                                                 | Example                                           |
| ------------------ | :------: | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `type_`            |    ✓     | What to buy. `credit_pack` is a one-time credit purchase.                                                                                                   | `V1AccountCheckoutCreateBodyTypeEnum::CreditPack` |
| `credit_shortfall` |    ✗     | Credits a blocked request is missing, e.g. `credit_shortfall` from a 402 response. The checkout starts with enough packs to cover it. Defaults to one pack. | `150.0`                                           |
| `pack_count`       |    ✗     | Number of credit packs the checkout starts with. Takes priority over `credit_shortfall`.                                                                    | `3`                                               |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .account()
    .checkout()
    .create(magic_hour::resources::v1::account::checkout::CreateRequest {
        credit_shortfall: Some(150.0),
        pack_count: Some(3),
        type_: magic_hour::models::V1AccountCheckoutCreateBodyTypeEnum::CreditPack,
    })
    .await;
```

#### Response

##### Type

[V1AccountCheckoutCreateResponse](/src/models/v1_account_checkout_create_response.rs)

##### Example

```rust
V1AccountCheckoutCreateResponse {checkout_url: "https://pay.magichour.ai/c/pay/cs_live_...".to_string()}
```
