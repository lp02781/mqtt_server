FROM python:3.10-slim

WORKDIR /app

COPY node_sim_1.py .
COPY node_sim_2.py .
COPY node_sim_3.py .
COPY node_sim_4.py .
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

RUN pip install --upgrade pip
RUN pip install --upgrade paho-mqtt supervisor

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]