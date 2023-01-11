use crate::models::{LowmafInput, LowmafOutput}; 
mod step1_dmafdt; 
mod step2_outliers; 
mod step3_correction; 
mod step4_matchmaf;


// calls main functions for each step
pub fn begin(data: Vec<LowmafInput>) -> Vec<LowmafInput> {
    step1_dmafdt::calc();
    step2_outliers::calc();
    step3_correction::calc();
    step4_matchmaf::calc();
    
    data 
}