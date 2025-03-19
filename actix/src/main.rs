use actix_web::{App, HttpServer};

mod mqtt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    tokio::spawn(async {mqtt::start_mqtt_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_publisher().await;});

    HttpServer::new(move || {
        App::new()
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
