use actix_web::{get, web, HttpResponse, Responder};

use crate::{config::State, database::executor::SaveNonce};

#[get("/nonce")]
pub async fn get_nonce(state: web::Data<State>) -> impl Responder {
    let nonce = textnonce::TextNonce::sized_urlsafe(32)
        .unwrap()
        .into_string();

    let db_executor = &state.db;

    let inserted_nonce = db_executor
        .send(SaveNonce { nonce })
        .await
        .unwrap()
        .unwrap();

    HttpResponse::Ok().json(inserted_nonce)
}
