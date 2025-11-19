// mq_topics.rs

pub fn chip_temperature_topic(uuid: &str) -> String {
    format!("{}/temperature/chip", uuid)
}

pub fn sensor_temperature_topic(uuid: &str) -> String {
    format!("{}/temperature/sensor", uuid)
}

pub fn sensor_humidity_topic(uuid: &str) -> String {
    format!("{}/humidity/sensor", uuid)
}

pub fn sensor_airpressure_topic(uuid: &str) -> String {
    format!("{}/airpressure/sensor", uuid)
}

pub fn sensor_co2_topic(uuid: &str) -> String {
    format!("{}/co2/sensor", uuid)
}

pub fn hello_topic(uuid: &str) -> String {
    format!("{}/hello", uuid)
}

