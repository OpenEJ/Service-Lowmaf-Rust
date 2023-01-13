use crate::models::{LowmafOutput, Step2Output};
use crate::models::maf_voltages::{usdm02to07wrx_mafvoltages};

// Step 4, matches corrections observed with cooresponding mafv entry
//          match on closest mafvoltage in the table
// calculates a mean of corrections observed for each entry
pub fn calc(data: Vec<Step2Output>) -> Vec<LowmafOutput> {
    let mut output: Vec<LowmafOutput> = vec![];
    for i in 0..usdm02to07wrx_mafvoltages.len(){
        output.push(LowmafOutput::build_trivial())
    }
    return output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data: Vec<Step2Output> = vec![
            Step2Output{
                time: 0,
                af_correction_short: 0.0,
                af_correction_learning: 1.0,
                intake_air_temp: 40,
                mass_airflow_voltage: 1.17,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 1.0,
            },
            Step2Output{
                time: 100,
                af_correction_short: -1.0,
                af_correction_learning: 1.0,
                intake_air_temp: 50,
                mass_airflow_voltage: 1.18,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 0.0
            },
            Step2Output{
                time: 200,
                af_correction_short: 4.0,
                af_correction_learning: 0.8,
                intake_air_temp: 60,
                mass_airflow_voltage: 1.34,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 4.8,
            },
            Step2Output{
                time: 300,
                af_correction_short: 2.0,
                af_correction_learning: 1.5,
                intake_air_temp: 70,
                mass_airflow_voltage: 1.35,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: -3.5,
            }
        ];

        let expected_output: Vec<LowmafOutput> = vec![
            LowmafOutput
            {
                MafVoltage: 0.94,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 0.98,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.02,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.05,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.09,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.13,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.17,
                Correction: 0.5,
                Frequency: 2
            },
            LowmafOutput
            {
                MafVoltage: 1.21,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.25,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.29,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.33,
                Correction: 4.8,
                Frequency: 1
            },
            LowmafOutput
            {
                MafVoltage: 1.37,
                Correction: -3.5,
                Frequency: 1
            },
            LowmafOutput
            {
                MafVoltage: 1.41,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.48,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.56,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.64,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.72,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 1.80,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 1.87,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 1.95,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.03,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.11,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.19,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.27,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.34,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.42,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.54,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.66,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.77,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 2.89,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 3.01,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 3.12,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 3.24,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 3.36,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 3.48,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput {
                MafVoltage: 3.59,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 3.71,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 3.83,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 3.95,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.06,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.18,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.30,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.41,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.49,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.57,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.61,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.65,
                Correction: 0.00,
                Frequency: 0
            },
            LowmafOutput
            {
                MafVoltage: 4.69,
                Correction: 0.00,
                Frequency: 0
            }
        ];

        // run the function and check if output matches expected output
        let output = calc(test_data);

        assert_eq!(expected_output.len(), output.len());
        for i in 0..expected_output.len() {
            assert_eq!(output[i].MafVoltage, expected_output[i].MafVoltage);
            assert_eq!(output[i].Correction, expected_output[i].Correction);
            assert_eq!(output[i].Frequency, expected_output[i].Frequency);
        }
    }
}