FROM python:3.10-slim

WORKDIR /app

COPY mqtt_node_1.py .
COPY mqtt_node_2.py .
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

RUN pip install --upgrade pip
RUN pip install --upgrade paho-mqtt supervisor

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]