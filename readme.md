# Purpose
The Closed Loop Maf Scaling feature of OpenEJ ( Handled by Service-Lowmaf* ) is the most performance intensive feature of the site at this time.
The Service-Lowmaf feature was originally written in python using fastapi as the web server and pandas for data analysis.
We originally chose python because of its simplicity as a language which made the development process quick and robust, allowing us to quickly push new features.

However it quickly became clear that python was very limited in terms of performance, and when analyzing "large" files, on the size of 1MB or larger, the user would have to wait a substantial ammount of time for their request to be processed. 
We decided that rather try to optimize our python code, which would be systemically limited by the python platform, we would refactor the code into Rust.
Rust gives us vastly higher performance, and still many of the same advantages of python. Moving forward Rust should be the standard for any performance intensive OpenEJ services. We would also like to experiment with concurrency and data analysis libraries such as Polars to further optimize the already ```blazingly fast``` Rust code.

# Architecture 

# Performance
