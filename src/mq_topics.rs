// mq_topics.rs

use std::sync::LazyLock;

const UUID: &str = "42";

pub static TOPICS: LazyLock<Topics> = LazyLock::new(|| Topics {  
    chip_temp: chip_temperature_topic(UUID),
    temp: sensor_temperature_topic(UUID),
    humidity: sensor_humidity_topic(UUID),
    air_pressure: sensor_airpressure_topic(UUID),
    co2: sensor_co2_topic(UUID)
});

pub struct Topics{
    pub chip_temp: String,
    pub temp: String,
    pub humidity: String,
    pub air_pressure: String,
    pub co2: String,
}

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
