// mq_topics.rs

use std::{str::FromStr, sync::LazyLock};

const UUID: &str = "42";

pub static TOPICS: LazyLock<Topics> = LazyLock::new(|| Topics {  
    chip_temp: chip_temperature_topic(UUID, Dataformat::Json.into()),
    chip_temp_msgpack: chip_temperature_topic(UUID, Dataformat::MsgPack.into()),
    temp: sensor_temperature_topic(UUID, Dataformat::Json.into()),
    temp_msgpack: sensor_temperature_topic(UUID, Dataformat::MsgPack.into()),
    humidity: sensor_humidity_topic(UUID, Dataformat::Json.into()),
    humidity_msgpack: sensor_humidity_topic(UUID, Dataformat::MsgPack.into()),
    air_pressure: sensor_airpressure_topic(UUID, Dataformat::Json.into()),
    air_pressure_msgpack: sensor_airpressure_topic(UUID, Dataformat::MsgPack.into()),
    light_intensity: sensor_lightintensity_topic(UUID, Dataformat::Json.into()),
    light_intensity_msgpack: sensor_lightintensity_topic(UUID, Dataformat::MsgPack.into()),
    co2: sensor_co2_topic(UUID, Dataformat::Json.into()),
    co2_msgpack: sensor_co2_topic(UUID, Dataformat::MsgPack.into()),
    battery_voltage: sensor_battery_voltage_topic(UUID, Dataformat::Json.into()),
    battery_voltage_msgpack: sensor_battery_voltage_topic(UUID, Dataformat::MsgPack.into()),
    measurement_bundle: sensor_measurements_bundle_topic(UUID, Dataformat::Json.into()),
    measurement_bundle_msgpack: sensor_measurements_bundle_topic(UUID, Dataformat::MsgPack.into()),
    error: sensor_error_topic(UUID, Dataformat::Json.into()),
    error_msgpack: sensor_error_topic(UUID, Dataformat::MsgPack.into()),
});

pub enum Dataformat{
    Json,
    MsgPack
}

impl FromStr for Dataformat{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Dataformat::Json),
            "msgpack" => Ok(Dataformat::MsgPack),
            _ => Err(())
        }
    }
}

impl From<Dataformat> for &'static str{
    fn from(value: Dataformat) -> Self {
        match value {
             Dataformat::Json => "json",
             Dataformat::MsgPack => "msgpack",
        }
    }
}

pub struct Topics{
    pub chip_temp: String,
    pub chip_temp_msgpack: String,
    pub temp: String,
    pub temp_msgpack: String,
    pub humidity: String,
    pub humidity_msgpack: String,
    pub air_pressure: String,
    pub air_pressure_msgpack: String,
    pub light_intensity: String,
    pub light_intensity_msgpack: String,
    pub co2: String,
    pub co2_msgpack: String,
    pub battery_voltage: String,
    pub battery_voltage_msgpack: String,
    pub measurement_bundle: String,
    pub measurement_bundle_msgpack: String,
    pub error: String,
    pub error_msgpack: String,
}

pub fn chip_temperature_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/temperature/chip", uuid, dataformat)
}

pub fn sensor_temperature_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/temperature/sensor", uuid, dataformat)
}

pub fn sensor_humidity_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/humidity/sensor", uuid, dataformat)
}

pub fn sensor_airpressure_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/airpressure/sensor", uuid, dataformat)
}

pub fn sensor_lightintensity_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/lightintensity/sensor", uuid, dataformat)
}

pub fn sensor_co2_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/co2/sensor", uuid, dataformat)
}

pub fn sensor_battery_voltage_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/battery/voltage/sensor", uuid, dataformat)
}

pub fn sensor_measurements_bundle_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/measurement/bundle/sensor", uuid, dataformat)
}

pub fn sensor_error_topic(uuid: &str, dataformat: &str) -> String {
    format!("{}/{}/error/sensor", uuid, dataformat)
}
