use crate::models::{Step1Output, Step2Output};

// Step 2
// Filter out OL values (8 is CL, 10 is OL)
// Filter out IAT values > threshold ( FOR NOW DONT WORRY )
// Calculate Overall correction = af_correction_short + af_correction_learning
pub fn calc(data: Vec<Step1Output>) -> Vec<Step2Output> {
    let output: Vec<Step2Output> = data.into_iter().filter_map(|x| 
        if x.cl_ol_status == 8 {
            Some(Step2Output::build(&x)) // correction is calculated inside the build()
        } else { None } )
        .collect();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let test_data: Vec<Step1Output> = vec![
            Step1Output{
                time: 0,
                af_correction_short: 0.0,
                af_correction_learning: 1.0,
                intake_air_temp: 40,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
            },
            Step1Output{
                time: 100,
                af_correction_short: -1.0,
                af_correction_learning: 1.0,
                intake_air_temp: 50,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
            },
            Step1Output{
                time: 200,
                af_correction_short: 4.0,
                af_correction_learning: 0.8,
                intake_air_temp: 60,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
            },
            Step1Output{
                time: 300,
                af_correction_short: 2.0,
                af_correction_learning: 1.5,
                intake_air_temp: 70,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
            },
            Step1Output{
                time: 400,
                af_correction_short: -5.0,
                af_correction_learning: 1.8,
                intake_air_temp: 80,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 10, // should be filtered out
                dmafdt: 0.15,
            }
        ];

        let expected_output: Vec<Step2Output> = vec![
            Step2Output{
                time: 0,
                af_correction_short: 0.0,
                af_correction_learning: 1.0,
                intake_air_temp: 40,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 1.0,
            },
            Step2Output{
                time: 100,
                af_correction_short: -1.0,
                af_correction_learning: 1.0,
                intake_air_temp: 50,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 0.0
            },
            Step2Output{
                time: 200,
                af_correction_short: 4.0,
                af_correction_learning: 0.8,
                intake_air_temp: 60,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 4.8,
            },
            Step2Output{
                time: 300,
                af_correction_short: 2.0,
                af_correction_learning: 1.5,
                intake_air_temp: 70,
                mass_airflow_voltage: 3.0,
                cl_ol_status: 8,
                dmafdt: 0.15,
                correction: 3.5,
            }
        ];

        // run the function and check if output matches expected output
        let output = calc(test_data);

        assert_eq!(expected_output.len(), output.len());
        for i in 0..expected_output.len() {
            assert_eq!(output[i].cl_ol_status, expected_output[i].cl_ol_status);
            assert_eq!(output[i].correction, expected_output[i].correction);
        }
    }
}