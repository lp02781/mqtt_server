import paho.mqtt.client as mqtt
import time
import json
import random

broker = "localhost"
port = 1883
topic = "/mqtt_node_2/data"

client = mqtt.Client(protocol=mqtt.MQTTv5)  

def on_connect(client, userdata, flags, rc, properties):
    print(f"Connected with result code {rc}")
    if rc == 0:
        print("Successfully connected to the broker.")
    else:
        print(f"Connection failed with code: {rc}")

client.on_connect = on_connect

print("Connecting to broker...")
client.connect(broker, port, 60)
print("Connection initiated.")

client.loop_start()

try:
    while True:
        
        temperature = random.uniform(15, 40)
        humidity = random.randint(30, 80)
        current = random.uniform(1, 10)
        sensor_data = {
            "id": 2,
            "timestamp": time.time(),
            "data": {  
                "temperature": temperature,
                "humidity": humidity,
                "current": current 
            }
        }

        message = json.dumps(sensor_data)
        client.publish(topic, message)
        print(f"Published: {message}")

        time.sleep(10)
except KeyboardInterrupt:
    print("Stopping MQTT client...")
    client.loop_stop()
    client.disconnect()
