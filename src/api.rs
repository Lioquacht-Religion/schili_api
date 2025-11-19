// api.rs

use std::collections::HashSet;

use bigdecimal::BigDecimal;
use chrono::{serde::ts_seconds, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sensor {
    pub reference: String,
    pub name: String,
    pub sensor_types: HashSet<SensorType>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SensorType {
    Temperature,
    Humidity,
    Airpressure,
    Co2,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorTempMeasurements {
    pub sensor_reference: String,
    pub temp_measurements: Vec<TemperatureMeasurement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorSingleTempMeasure {
    pub sensor_reference: String,
    pub temp_measure: TemperatureMeasurement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemperatureMeasurement {
    pub temp_celsius: BigDecimal,
    #[serde(with = "ts_seconds")]
    pub measure_time: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorSingleCo2Measure {
    pub sensor_reference: String,
    pub co2_measure: Co2Measurement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Co2Measurement {
    pub co2_ppm: BigDecimal,
    pub res0: BigDecimal,
    pub adc_val: i32,
    #[serde(with = "ts_seconds")]
    pub measure_time: chrono::DateTime<Utc>,
}

impl Sensor {
    pub fn new(reference: &str, name: &str, sensor_types: HashSet<SensorType>) -> Self {
        Self {
            reference: reference.into(),
            name: name.into(),
            sensor_types,
        }
    }
}

impl From<&SensorType> for &str {
    fn from(value: &SensorType) -> Self {
        match value {
            SensorType::Temperature => "temperature",
            SensorType::Humidity => "humidity",
            SensorType::Airpressure => "airpressure",
            SensorType::Co2 => "co2",
        }
    }
}
