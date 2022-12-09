use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LowmafInput {
    pub time: u64,
    pub af_correction_short: f64,
    pub af_correction_learning: f64,
    pub intake_air_temp: u64,
    pub mass_airflow_voltage: f64,
    pub cl_ol_status: u8
}

impl std::fmt::Display for LowmafInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("time: {}, af_correction_short: {}, af_correction_learning: {}, intake_air_temperature: {}, mass_airflow_voltage: {}, cl_ol_fueling_status: {}",
                self.time, 
                self.af_correction_short, 
                self.af_correction_learning, 
                self.intake_air_temp, 
                self.mass_airflow_voltage, 
                self.cl_ol_status)
        .fmt(f)
    }
}

#[derive(Serialize)]
pub struct LowmafOutput {
    pub MafVoltage: f64,
    pub Correction: f64,
    pub Frequency: u64 
}

impl LowmafOutput {
    pub fn new(MafVoltage: f64, Correction: f64, Frequency: u64) -> LowmafOutput {
        LowmafOutput {
            MafVoltage,
            Correction,
            Frequency
        }
    }    
}