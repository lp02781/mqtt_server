use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod mqtt;
mod json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    tokio::spawn(async {mqtt::start_mqtt_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_publisher().await;});

    HttpServer::new(move || {
        App::new()
        .route("/mqtt/data", web::post().to(post_mqtt_data))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn post_mqtt_data(payload: web::Json<json::MqttPayload>) -> impl Responder {
    HttpResponse::Ok().json(payload)
}