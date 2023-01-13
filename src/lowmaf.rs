use crate::models::{LowmafInput, LowmafOutput}; 
mod step1_dmafdt; 
mod step2_outliers; 
mod step3_correction; 
mod step4_matchmaf;

// use crate::lowmaf::step1_dmafdt::Step1Output;


// calls main functions for each step
pub fn begin(data: Vec<LowmafInput>) -> Vec<LowmafOutput> {
    let output1 = step1_dmafdt::calc(data);
    step2_outliers::calc();
    step3_correction::calc();
    step4_matchmaf::calc();
    
    let test_output = vec![LowmafOutput::new(0.0, 0.0, 0)];
    test_output
}