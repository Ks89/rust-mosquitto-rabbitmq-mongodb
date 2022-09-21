# rust-mosquitto-and-rabbitmq

## Mosquitto

docker pull eclipse-mosquitto

docker run -it --name mosquitto -p 1883:1883 -p 9001:9001 --rm -v $PWD/mosquitto/mosquitto-no-security.conf:/mosquitto/config/mosquitto.conf -v /mosquitto/data -v /mosquitto/log eclipse-mosquitto

mosquitto_sub -t sensors/+/temperature
mosquitto_pub -m "{\"uuid\": \"uuid1\",\"profileToken\": \"profiletoken-1\",\"temperature\": 12.712}" -t sensors/uuid1/temperature


## Mondodb

Install mongodb in docker with:

```
docker run -d --name mongodb -v ~/mongodb:/data/db -p 27017:27017 mongo:5
```