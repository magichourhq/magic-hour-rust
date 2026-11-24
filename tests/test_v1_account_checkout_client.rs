#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
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
    println!("{res:?}");
    assert!(res.is_ok());
}
