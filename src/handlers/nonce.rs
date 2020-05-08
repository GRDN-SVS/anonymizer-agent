use actix_web::{get, web, HttpResponse, Responder};
use sodiumoxide::crypto::box_::gen_nonce;

use crate::{config::State, database::executor::SaveNonce};

#[get("/nonce")]
pub async fn get_nonce(state: web::Data<State>) -> impl Responder {
    let nonce = gen_nonce().0.to_vec();

    println!("{:?}", nonce);

    let db_executor = &state.db;

    let inserted_nonce = db_executor
        .send(SaveNonce { nonce })
        .await
        .unwrap()
        .unwrap();

    HttpResponse::Ok().json(inserted_nonce)
}
