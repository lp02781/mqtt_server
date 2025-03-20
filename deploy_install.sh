set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Install dependencies${NC}"
sudo apt install mosquitto -y
sudo apt install mosquitto-clients -y

echo "${GREEN}Uninstall first${NC}"
sudo ./uninstall.sh

echo "${GREEN}Build and run docker compose${NC}"
docker-compose -f deploy_compose.yml up -d