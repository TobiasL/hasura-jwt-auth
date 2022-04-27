use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use std::env;

#[derive(Serialize, Deserialize)]
struct HasuraUserClaim {
    #[serde(rename(serialize = "x-hasura-allowed-roles"))]
    allowed_roles: Vec<String>,
    #[serde(rename(serialize = "x-hasura-default-role"))]
    default_role: String,
    #[serde(rename(serialize = "x-hasura-user-id"))]
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct HasuraClaim {
    #[serde(rename(serialize = "https://hasura.io/jwt/claims"))]
    claim_url: HasuraUserClaim,
}

pub fn create_token(user_id: Uuid, default_role: String) -> Result<String, jwt_simple::Error> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let key = HS256Key::from_bytes(jwt_secret.as_bytes());

    let my_claim = HasuraClaim {
        claim_url: HasuraUserClaim {
            allowed_roles: vec![default_role.to_string()],
            default_role: default_role.to_string(),
            user_id: user_id.to_string(),
        },
    };

    let claims = Claims::with_custom_claims(my_claim, Duration::from_hours(2));

    key.authenticate(claims)
}
