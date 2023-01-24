use crate::models::{LowmafOutput, Step2Output};
use crate::models::maf_voltages::{usdm02to07wrx_mafvoltages};

// Step 4, matches corrections observed with cooresponding mafv entry
//          match on closest mafvoltage in the table
// calculates a mean of corrections observed for each entry
pub fn calc(data: Vec<Step2Output>) -> Vec<LowmafOutput> {
    let mut output: Vec<LowmafOutput> = usdm02to07wrx_mafvoltages.into_iter().map(
        |x| LowmafOutput{MafVoltage: x, Correction: 0.0, Frequency: 0}
    ).collect();
    for x in data {
        let matched_index = find_nearest_maf(x.mass_airflow_voltage, usdm02to07wrx_mafvoltages);
        output[matched_index].Correction = ((output[matched_index].Correction * output[matched_index].Frequency as f64) + x.correction)/ (output[matched_index].Frequency + 1) as f64;
        output[matched_index].Correction = format!("{:.2}", output[matched_index].Correction).parse::<f64>().unwrap(); // round to nearest hundreth
        output[matched_index].Frequency += 1;
    }
    output
}

// binary search 
// given a val and a sorted list of voltages, find a val that is closest to a val in the list 
// return the index of that value
pub fn find_nearest_maf(val: f64, voltages: [f64; 48]) -> usize {
    //conduct binary search until left and right are next to eachother
    let mut left = 0;
    let mut right = voltages.len()-1;
    while (right - left) > 1 {
        let middle = (left + right)/2; 
        if voltages[middle] == val {
            return middle; 
        }
        else if voltages[middle] > val {
            right = middle; 
        }
        else {
            left = middle; 
        }
    }
    // we are left with 2 values, lets find which one is closet to val
    let dist_to_left = (voltages[left] - val).abs();
    let dist_to_right = (voltages[right] - val).abs();
    if dist_to_left < dist_to_right {
        return left;
    }
    return right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_nearest_maf_works() {
        let v1: f64 = 1.10;
        let v2: f64 = 1.17;
        let v3: f64 = 0.5;
        let v4: f64 = 5.5;
        let v5: f64 = -1000.0;
        let v6: f64 = 1.0;

        assert_eq!(find_nearest_maf(v1, usdm02to07wrx_mafvoltages), 4);
        assert_eq!(find_nearest_maf(v2, usdm02to07wrx_mafvoltages), 6);
        assert_eq!(find_nearest_maf(v3, usdm02to07wrx_mafvoltages), 0);
        assert_eq!(find_nearest_maf(v4, usdm02to07wrx_mafvoltages), 47);
        assert_eq!(find_nearest_maf(v5, usdm02to07wrx_mafvoltages), 0);
        assert_eq!(find_nearest_maf(v6, usdm02to07wrx_mafvoltages), 2);
    }

    #[test]
    fn calc_works() {
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