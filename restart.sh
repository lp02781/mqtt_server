set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Restart container${NC}"
docker restart mqtt_mosquitto_container
docker restart mqtt_node_container
docker restart mqtt_actix_container