use actix_web::get;
use actix_web::Responder;

#[get("/healthz")]
async fn health_handler() -> impl Responder {
  "OK"
}
