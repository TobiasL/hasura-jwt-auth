use jwt_simple::prelude::*;
use std::env;

use super::state;

#[derive(Serialize, Deserialize)]
struct HasuraUserClaim {
    #[serde(rename(serialize = "x-hasura-allowed-roles"))]
    allowed_roles: Vec<String>,
    #[serde(rename(serialize = "x-hasura-default-role"))]
    default_role: String,
    #[serde(rename(serialize = "x-hasura-user-id"))]
    user_id: String,
    #[serde(rename(serialize = "x-hasura-company-id"))]
    company_id: String,
}

#[derive(Serialize, Deserialize)]
struct HasuraClaim {
    #[serde(rename(serialize = "https://hasura.io/jwt/claims"))]
    claim_url: HasuraUserClaim,
}

pub async fn sign(_req: tide::Request<state::State>) -> tide::Result {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let key = HS256Key::from_bytes(jwt_secret.as_bytes());

    let my_claim = HasuraClaim {
        claim_url: HasuraUserClaim {
            allowed_roles: vec!["user".to_string()],
            default_role: "user".to_string(),
            user_id: "3a2278d5-8c34-46ec-ab90-9eec6262df91".to_string(),
            company_id: "30828f14-17ea-42e9-a0dc-9fb912b26418".to_string(),
        },
    };

    let claims = Claims::with_custom_claims(my_claim, Duration::from_hours(2));
    let token = key.authenticate(claims)?;

    Ok(tide::Response::builder(200).body(token).build())
}
