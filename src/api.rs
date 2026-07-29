// api.rs

use std::{collections::HashSet, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sensor {
    pub reference: String,
    pub name: String,
    pub sensor_types: HashSet<SensorType>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum SensorType {
    Temperature,
    Humidity,
    Airpressure,
    BatteryVoltage,
    ChipTemperature,
    Co2,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSensorSimpleMeasuresRange {
    pub sensor_reference: String,
    pub start_datetime: DateTime<Utc>,
    pub end_datetime: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSensorSimpleMeasuresIntervalsRange {
    pub sensor_reference: String,
    pub start_datetime: DateTime<Utc>,
    pub end_datetime: DateTime<Utc>,
    pub interval: Duration
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorSimpleMeasurements {
    pub sensor_reference: String,
    pub measurements: Vec<SimpleMeasurement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorSingleSimpleMeasure {
    pub sensor_reference: String,
    pub measure: SimpleMeasurement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleMeasurement {
    pub measurement: BigDecimal,
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

impl SensorType{
    pub fn to_str(&self) -> &'static str{
        self.into()
    }
}

impl From<&SensorType> for &str {
    fn from(value: &SensorType) -> Self {
        match value {
            SensorType::Temperature => "temperature",
            SensorType::Humidity => "humidity",
            SensorType::Airpressure => "airpressure",
            SensorType::ChipTemperature => "chiptemperature",
            SensorType::BatteryVoltage => "batteryvoltage",
            SensorType::Co2 => "co2",
        }
    }
}

impl FromStr for SensorType{
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
    "temperature" => Ok(SensorType::Temperature),
    "humidity" => Ok(SensorType::Humidity),
    "airpressure" => Ok(SensorType::Airpressure),
    "chiptemperature" => Ok(SensorType::ChipTemperature),
    "co2" => Ok(SensorType::Co2),
    "batteryvoltage" => Ok(SensorType::BatteryVoltage),
    _ => Err(()),
        }
    }
}
