// mq_topics.rs

use std::sync::LazyLock;

const UUID: &str = "42";

pub static TOPICS: LazyLock<Topics> = LazyLock::new(|| Topics {  
    chip_temp: chip_temperature_topic(UUID),
    temp: sensor_temperature_topic(UUID),
    humidity: sensor_humidity_topic(UUID),
    air_pressure: sensor_airpressure_topic(UUID),
    light_intensity: sensor_lightintensity_topic(UUID),
    co2: sensor_co2_topic(UUID),
    battery_voltage: sensor_battery_voltage_topic(UUID),
    measurement_bundle: sensor_measurements_bundle_topic(UUID),
    error: sensor_error_topic(UUID),
});

pub struct Topics{
    pub chip_temp: String,
    pub temp: String,
    pub humidity: String,
    pub air_pressure: String,
    pub light_intensity: String,
    pub co2: String,
    pub battery_voltage: String,
    pub measurement_bundle: String,
    pub error: String,
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

pub fn sensor_lightintensity_topic(uuid: &str) -> String {
    format!("{}/lightintensity/sensor", uuid)
}

pub fn sensor_co2_topic(uuid: &str) -> String {
    format!("{}/co2/sensor", uuid)
}

pub fn sensor_battery_voltage_topic(uuid: &str) -> String {
    format!("{}/battery/voltage/sensor", uuid)
}

pub fn sensor_measurements_bundle_topic(uuid: &str) -> String {
    format!("{}/measurement/bundle/sensor", uuid)
}

pub fn sensor_error_topic(uuid: &str) -> String {
    format!("{}/error/sensor", uuid)
}
