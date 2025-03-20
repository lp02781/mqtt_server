set -e
set -x

docker stop mqtt_mosquitto_container || true
docker stop mqtt_node_container || true
docker stop mqtt_actix_container || true

docker rm mqtt_mosquitto_container || true
docker rm mqtt_node_container || true
docker rm mqtt_actix_container || true

docker rmi eclipse-mosquitto || true
docker rmi mqtt_node_image || true
docker rmi mqtt_actix_image || true
docker rmi rustlang/rust:nightly || true
docker rmi python:3.10-slim || true

sudo pkill mosquitto || true


