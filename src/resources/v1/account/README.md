# v1.account

## Module Functions

### Get account details <a name="list"></a>

Get the current credit balance and subscription details of the account that owns the API key.

**API Endpoint**: `GET /v1/account`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client.v1().account().list().await;
```

#### Response

##### Type

[V1AccountListResponse](/src/models/v1_account_list_response.rs)

##### Example

```rust
V1AccountListResponse {credits: 12500, email: Some("user@example.com".to_string()), id: "cuid-example".to_string(), subscription: Some(V1AccountListResponseSubscription {billing_interval: Some(V1AccountListResponseSubscriptionBillingIntervalEnum::Month), cancel_at_period_end: false, current_period_end: Some("2026-10-01T00:00:00.000Z".to_string()), discount: Some(V1AccountListResponseSubscriptionDiscount {amount_off: Some(123), percent_off: Some(20.0)}), name: Some("Pro".to_string()), price: V1AccountListResponseSubscriptionPrice {amount: 4900, currency: "usd".to_string()}, status: V1AccountListResponseSubscriptionStatusEnum::Active}), tier: V1AccountListResponseTierEnum::Pro}
```
