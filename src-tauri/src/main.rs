mod models;
mod auth;
mod entities;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::env;
use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString};
use rand_core::OsRng;

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use jsonwebtoken::errors::ErrorKind;

use uuid::Uuid;
use log::info;
use chrono::{Utc, Duration};

use entities::prelude::*; 
use entities::users; // <- aquí van tus modelos generados con sea-orm-cli


#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    jwt_secret: String,
}

// ==================== MODELOS ====================

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// ==================== HANDLERS ====================

async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = users::ActiveModel {
    id: Set(Uuid::new_v4().to_string()),
    email: Set(payload.email.clone()),
    password: Set(password_hash),
    ..Default::default()
};

    let _ = Users::insert(user)
        .exec(&state.db)
        .await
        .unwrap();

    Json("Usuario registrado con éxito".to_string())
}


async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {
    // Buscar usuario por email
    let user_option = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&state.db)
        .await
        .unwrap();

    if let Some(ref user) = user_option {
        println!("Usuario encontrado: {:?}", user);

        // Verificar contraseña
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        let argon2 = Argon2::default();

        if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
            // Crear JWT
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .unwrap()
                .timestamp() as usize;

            let claims = crate::Claims {
                sub: user.id.clone(),
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
            )
            .unwrap();

            return Json(AuthResponse { token });
        }
    } else {
        println!("No existe un usuario con ese email");
    }

    // Credenciales inválidas
    Json(AuthResponse {
        token: "Credenciales inválidas".to_string(),
    })
}

// ==================== MAIN ====================

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Conexión DB (SQLite)
    let db = Database::connect("sqlite://app.db").await.unwrap();

    let state = AppState {
        db,
        jwt_secret: "supersecreto123".to_string(), // cambia esto en prod
    };

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/health", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Servidor corriendo en http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
