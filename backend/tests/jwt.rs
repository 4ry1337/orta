use backend::{
    models::enums::Role,
    utils::jwt::{AccessToken, AccessTokenPayload, JWT},
};

#[tokio::test]
async fn jwt_tokens() {
    let payload = AccessTokenPayload {
        user_id: 1,
        role: Role::User,
        username: "asd".to_string(),
        email: "asd@gmail.com".to_string(),
        image: None,
    };

    let access_token = AccessToken::generate(payload.clone()).unwrap();
    let refresh_token = AccessToken::validate(&access_token).unwrap();

    // Assert
    assert_eq!(payload.user_id, refresh_token.payload.user_id);
}
