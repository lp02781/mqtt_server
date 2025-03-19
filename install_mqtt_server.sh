set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Install dependencies${NC}"
sudo apt install mosquitto -y
sudo apt install mosquitto-clients -y

echo "${GREEN}Delete existed container${NC}"
docker stop mqtt_mosquitto_container || true
docker rm mqtt_mosquitto_container || true
docker stop mqtt_node_sim_container || true
docker rm mqtt_node_sim_container || true
docker stop mqtt_actix_container || true
docker rm mqtt_actix_container || true

echo "${GREEN}Delete existed image${NC}"
docker rmi eclipse-mosquitto || true
docker rmi mqtt_node_sim_image || true
docker rmi mqtt_actix_image || true
docker rmi rustlang/rust:nightly || true
docker rmi python:3.10-slim || true

echo "${GREEN}Delete existed mosquitto process${NC}"
sudo pkill mosquitto || true

echo "${GREEN}Build and run docker compose${NC}"
docker-compose up -d