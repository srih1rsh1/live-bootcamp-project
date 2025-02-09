use auth_service::{routes::SignupResponse, ErrorResponse};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn signup_auth_ui() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // Check the response on /signup
    let test_cases = [
        serde_json::json!({
            "password": "123444565",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": false
        }),
    ];

    for test_case in test_cases {
        let response = app.signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let test_case = serde_json::json!({
        "email": random_email,
        "password": "123456789",
        "requires2FA": true
    });
    let response = app.signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]

async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [serde_json::json!({
        "email": random_email,
        "password": "67hey67",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Cloud not deserialize response body to ErrorResponse")
                .error,
            "Bad credentials".to_owned()
        );
    }
}

#[tokio::test]

async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = "hey@clickup.com".to_string();
    let test_case = serde_json::json!({
        "email": random_email,
        "password": "67hey678",
        "requires2FA": true
    });
    let _response = app.signup(&test_case).await;
    let response2 = app.signup(&test_case).await;

    assert_eq!(response2.status().as_u16(), 409);

    assert_eq!(
        response2
            .json::<ErrorResponse>()
            .await
            .expect("Cloud not deserialize body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
