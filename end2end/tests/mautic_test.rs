use mockito::Server;
use wmw::components::utils::add_mautic_contact;
use wmw::models::CustomerData;

#[tokio::test]
async fn test_add_mautic_succesfully() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let mut server = Server::new_async().await;
            let mock_url = server.url();
            let mock = server
                .mock("POST", "/contacts/new")
                .with_status(201)
                .with_header("content-type", "application/json")
                .with_body(r#"{"id": 55}"#)
                .create_async()
                .await;
            std::env::set_var("MAUTIC_URL", &mock_url);
            std::env::set_var("MAUTIC_LOGIN", "admin");
            std::env::set_var("MAUTIC_PASSWORD", "password123");

            let data = CustomerData {
                first_name: "Patryk".into(),
                email: "patryk@test.com".into(),
            };

            let result = add_mautic_contact(data).await;

            assert!(result.is_ok());
            mock.assert_async().await;
        })
        .await;
}
