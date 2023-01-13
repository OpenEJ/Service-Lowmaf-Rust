use crate::models::{LowmafInput, LowmafOutput, Step1Output, Step2Output}; 
mod step1_dmafdt; 
mod step2_outliers; 
mod step3_matchmaf; 

// calls main functions for each step
pub fn begin(data: Vec<LowmafInput>) -> Vec<LowmafOutput> {
    let output1: Vec<Step1Output> = step1_dmafdt::calc(data);
    let output2: Vec<Step2Output> = step2_outliers::calc(output1);
    let output3: Vec<LowmafOutput> = step3_matchmaf::calc(output2);   
    output3
}