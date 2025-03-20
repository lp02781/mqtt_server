set -e
set -x

docker stop mqtt_mosquitto_container || true
docker stop mqtt_node_container || true
docker stop mqtt_actix_container || true

docker rm mqtt_mosquitto_container || true
docker rm mqtt_node_container || true
docker rm mqtt_actix_container || true

docker rmi eclipse-mosquitto || true
docker rmi lp02781/mqtt_node_image || true
docker rmi lp02781/mqtt_actix_image || true

sudo pkill mosquitto || true


