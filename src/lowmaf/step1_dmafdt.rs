use crate::models::LowmafInput;

pub struct Step1Output {
    pub time: u64,
    pub af_correction_short: f64,
    pub af_correction_learning: f64,
    pub intake_air_temp: u64,
    pub mass_airflow_voltage: f64,
    pub cl_ol_status: u8, 
    pub dmafdt: f64,
}

// step 1
// construct dmaf/dt column for each data entry
// filter out values >.3
pub fn calc(data: Vec<LowmafInput>) -> Vec<Step1Output>{
    //TODO
    let mut output: Vec<Step1Output> = vec![];
    for d in data{
        let new_val = Step1Output {
            time: d.time,
            af_correction_short: d.af_correction_short,
            af_correction_learning: d.af_correction_learning,
            intake_air_temp: d.intake_air_temp,
            mass_airflow_voltage: d.mass_airflow_voltage,
            cl_ol_status: d.cl_ol_status,
            dmafdt: 0.0,
        };
        output.push(new_val);
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
            mass_airflow_voltage: 3.3,
            cl_ol_status: 8, 
        };
        let test_data: Vec<LowmafInput> = vec![test_row1, test_row2];

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
        
        // run the function and check if output matches expected output
        let output = calc(test_data);

        assert_eq!(expected_output.len(), output.len());
        for i in 0..expected_output.len() {
            assert_eq!(output[i].time, expected_output[i].time);
            assert_eq!(output[i].dmafdt, expected_output[i].dmafdt);
        }
    }
}