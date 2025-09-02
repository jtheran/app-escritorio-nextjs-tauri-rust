use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

const SECRET_KEY: &[u8] = b"clave_secreta_super_segura";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // id o email del usuario
    pub exp: usize,   // fecha de expiración
}

pub fn generar_token(user_id: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        + Duration::from_secs(60 * 60); // expira en 1h

    let claims = Claims {
        sub: user_id,
        exp: expiration.as_secs() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap()
}

pub fn verificar_token(token: &str) -> Result<Claims, String> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        Ok(data) => Ok(data.claims),
        Err(_) => Err("Token inválido".to_string()),
    }
}
