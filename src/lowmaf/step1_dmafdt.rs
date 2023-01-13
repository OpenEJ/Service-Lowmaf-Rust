use crate::models::{LowmafInput, Step1Output};

// step 1
// construct dmaf/dt column for each data entry
// filter out values >.3
pub fn calc(data: Vec<LowmafInput>) -> Vec<Step1Output>{
    // first index is trivial (has dmafdt of 0), since there is no value before it to compare 
    let mut output: Vec<Step1Output> = vec![Step1Output::build_trivial(&data[0])];
    for d in 1..data.len() {
        let mut new_val = Step1Output::build_trivial(&data[d]);
        // calculate the dmafdt
        let dmafdt: f64 = 1000.0*(data[d].mass_airflow_voltage - data[d-1].mass_airflow_voltage)/(data[d].time - data[d-1].time) as f64;
        let dmafdt_rounded = format!("{:.2}", dmafdt).parse::<f64>().unwrap(); // round to nearest hundreth
        // filter out values > 0.3 
        if dmafdt_rounded < 0.3 {
            new_val.dmafdt = dmafdt_rounded;
            output.push(new_val);
        }
    }
    output
}

// Test Driven Development
#[cfg(test)]
mod tests {
    use super::*; // put Step1Output into scope

    // let's write a failing test
    #[test]
    fn it_works() {
        // initialize test input data
        let test_row1 = LowmafInput {
            time: 0,
            af_correction_short: 3.0,
            af_correction_learning: -3.0,
            intake_air_temp: 40,
            mass_airflow_voltage: 3.1,
            cl_ol_status: 8, 
        };
        let test_row2 = LowmafInput {
            time: 100,
            af_correction_short: 3.0,
            af_correction_learning: -3.0,
            intake_air_temp: 40,
            mass_airflow_voltage: 3.105,
            cl_ol_status: 8, 
        };
        let test_row3 = LowmafInput {
            time: 200,
            af_correction_short: 3.0,
            af_correction_learning: -3.0,
            intake_air_temp: 40,
            mass_airflow_voltage: 3.7, //this is a big jump in mafv, this should get filtered out
            cl_ol_status: 8, 
        };
        let test_data: Vec<LowmafInput> = vec![test_row1, test_row2, test_row3];

        // initialize test output data
        let expected_output: Vec<Step1Output> = vec![ Step1Output {
            time: 0,
            af_correction_short: 3.0,
            af_correction_learning: -3.0,
            intake_air_temp: 40,
            mass_airflow_voltage: 3.1,
            cl_ol_status: 8, 
            dmafdt: 0.0, // no previous value, so we set it to 0
                       // dmafdt < .3, so we keep it
        }, Step1Output {
            time: 100,
            af_correction_short: 3.0,
            af_correction_learning: -3.0,
            intake_air_temp: 40,
            mass_airflow_voltage: 3.105,
            cl_ol_status: 8, 
            dmafdt: 0.05, // = 1000*(maf2-maf1)/(time2-time1)
                          // = 1000*(3.105-3.1)/(100-0)
                          // = .05
                          // .05 < .3, so we keep it
        }];
        // row3 should have been filtered out
        
        // run the function and check if output matches expected output
        let output = calc(test_data);

        assert_eq!(expected_output.len(), output.len());
        for i in 0..expected_output.len() {
            assert_eq!(output[i].time, expected_output[i].time);
            assert_eq!(output[i].dmafdt, expected_output[i].dmafdt);
        }
    }
}