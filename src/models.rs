use serde::{Deserialize, Serialize};
use actix_web::{error};
use derive_more::{Display, Error};

// --- Main INPUT/OUTPUT ---

#[derive(Deserialize, Serialize, Debug)]
pub struct LowmafInput {
    pub time: u64,
    pub af_correction_short: f64,
    pub af_correction_learning: f64,
    pub intake_air_temp: u64,
    pub mass_airflow_voltage: f64,
    pub cl_ol_status: u8,
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

#[derive(Serialize, Debug, Deserialize)]
pub struct LowmafOutput {
    pub MafVoltage: f64,
    pub Correction: f64,
    pub Frequency: u64, 
}

impl LowmafOutput {
    pub fn new(MafVoltage: f64, Correction: f64, Frequency: u64) -> LowmafOutput {
        LowmafOutput {
            MafVoltage,
            Correction,
            Frequency
        }
    }
    pub fn build_trivial() -> LowmafOutput {
        LowmafOutput {
            MafVoltage: 0.0,
            Correction: 0.0,
            Frequency: 0
        }
    }    
}

// --- STEP 1 ---

pub struct Step1Output {
    pub time: u64,
    pub af_correction_short: f64,
    pub af_correction_learning: f64,
    pub intake_air_temp: u64,
    pub mass_airflow_voltage: f64,
    pub cl_ol_status: u8, 
    pub dmafdt: f64,
}
impl Step1Output {
    // build trivial output with dmafdt of 0.0
    pub fn build_trivial(val: &LowmafInput) -> Step1Output {
        Step1Output {
            time: val.time,
            af_correction_short: val.af_correction_short,
            af_correction_learning: val.af_correction_learning,
            intake_air_temp: val.intake_air_temp,
            mass_airflow_voltage: val.mass_airflow_voltage,
            cl_ol_status: val.cl_ol_status,
            dmafdt: 0.0,
        }
    }
}

// --- Step 2 ---

pub struct Step2Output {
    pub time: u64,
    pub af_correction_short: f64,
    pub af_correction_learning: f64,
    pub intake_air_temp: u64,
    pub mass_airflow_voltage: f64,
    pub cl_ol_status: u8, 
    pub dmafdt: f64,
    pub correction: f64,
}
impl Step2Output {
    // builds a new output struct, with a calculated overall correction
    pub fn build(val: &Step1Output) -> Step2Output {
        Step2Output {
            time: val.time,
            af_correction_short: val.af_correction_short,
            af_correction_learning: val.af_correction_learning,
            intake_air_temp: val.intake_air_temp,
            mass_airflow_voltage: val.mass_airflow_voltage,
            cl_ol_status: val.cl_ol_status,
            dmafdt: val.dmafdt,
            correction: val.af_correction_short + val.af_correction_learning,
        }
    }
}

// --- Step 3 ---
pub mod maf_voltages {
    pub static usdm02to07wrx_mafvoltages: [f64; 48] = [0.94, 0.98, 1.02, 1.05, 1.09, 1.13, 1.17, 1.21, 1.25, 1.29, 1.33, 1.37, 1.41, 1.48, 1.56, 1.64, 1.72, 1.80, 1.87, 1.95, 2.03, 2.11, 2.19, 2.27, 2.34, 2.42, 2.54, 2.66, 2.77, 2.89, 3.01, 3.12, 3.24, 3.36, 3.48, 3.59, 3.71, 3.83, 3.95, 4.06, 4.18, 4.30, 4.41, 4.49, 4.57, 4.61, 4.65, 4.69];
}

// --- ERRORS ---
#[derive(Debug, Display, Error)]
#[display(fmt = "bad data: {}", message)]
pub struct BadData {
    message: &'static str,
}
// use default implementation for `error_response()` method
impl error::ResponseError for BadData {}