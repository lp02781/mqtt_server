set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Install dependencies${NC}"
sudo apt install mosquitto -y
sudo apt install mosquitto-clients -y

echo "${GREEN}Uninstall first${NC}"
sudo ./uninstall_mqtt_server.sh

echo "${GREEN}Build and run docker compose${NC}"
docker-compose -f compose_mqtt_server.yml up -d

echo "${GREEN}Restart container${NC}"
sudo ./restart_mqtt_server.sh