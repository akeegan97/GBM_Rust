use std::{f32::consts::E, result, collections::HashMap};
extern crate csv;
#[allow(non_snake_case)]
use csv::StringRecord;
//use std::error::Error;


//Geometric Brownian Motion Equation: St = S0exp(mu-1/2(sigma^2)*DeltaT + Sigma*Wt ~ N(0,sqrt(DeltaT)))
//Mu to be estimated as the mean of sample log returns
//sigma^2 as the mean of the variance of the sample log returns
//T how long to predict

fn main() {
    
    //getting familiar with the granular operations of the vectors
    /* 
    let array:Vec<f32>=[1.0,2.0,3.5].to_vec();
    
    let log_array:Vec<f32> = array.iter().map(|i| i.log(E)).collect();

    
    let mut log_array_shifted:Vec<f32>=log_array.clone();
    log_array_shifted.insert(0,0.0);
    log_array_shifted.pop();
    

    let diff:Vec<f32> = (0..log_array.len()).map(|i| log_array[i] - log_array_shifted[i] ).collect();

    let x:f32 = diff.len() as f32;

   

    let average_log_return:f32 = diff.iter().map(|&i| i as f32).sum::<f32>() / x;
    

    let numerator1:Vec<f32> = (0..diff.len()).map(|i|(diff[i] - average_log_return)*
        (diff[i] -average_log_return)).collect();
    
    let numerator2:f32 = numerator1.iter().map(|i| *i as f32).sum::<f32>();

    let std_dev1 = numerator2 / x;

    let std_dev = std_dev1.sqrt();*/
    
    //got the mean of the difference of logs ie log returns and the std_deviation of the log returns;

    //next is to implement reading csv price data into a vector of floats to replace the "array" defined

    
    
    
    //reading the file into the struct created below fn main
    let bac = DataFrame::read_csv("D:\\Code\\Rust_Things\\GBM_Rust\\BAC.csv", true);
    //getting the close price in an isolated Vec
    let price_data = &bac.close;
    //setting the sample data indexes
    let start_date:&str = "2014-06-02";
    let end_date:&str = "2018-06-01";
    //getting the index number of the dates above for the sample price data
    let index_start_training = bac.date  
        .iter()
        .position(|a| a == start_date)
        .unwrap();
    let index_end_training = bac.date
        .iter()
        .position(|b|  b == end_date)
        .unwrap();
    // breaking the original dataframe into a smaller chunk indexed with the above index numbers

    let training_prices:Vec<f32> = (&price_data[index_start_training..index_end_training]).to_vec();
    
    //taking the natural log of each element in the training price vector
    let log_training_prices:Vec<f32> = training_prices
        .iter()
        .map(|a| a.log(E))
        .collect();
    //creating a second vector that is shifter up by 1 index place
    let mut log_training_prices_shifted:Vec<f32> = log_training_prices
        .clone();
    log_training_prices_shifted.insert(0, 0.0);
    log_training_prices_shifted.pop();
    //testing to see if both lengths are the same

    //println!("{},{}",log_training_prices.len(),log_training_prices_shifted.len());
    //both have the same length

    // getting the log daily log returns
    let log_returns:Vec<f32> = (0..log_training_prices.len())
        .map(|b| log_training_prices[b] - log_training_prices_shifted[b])
        .collect();
    let length_of_log_returns:f32 = log_returns.len() as f32;
    //sum all the log returns

    let summed_log_returns:f32 = log_returns
        .iter()
        .map(|c| *c as f32)
        .sum::<f32>();
    println!("{} = summed log returns of the training data set",summed_log_returns);

    //expected average log return of mu hat 

    let average_training_log_return:f32 = summed_log_returns / length_of_log_returns;

    println!("the expected log return is {} mu hat",average_training_log_return);

    


    





    
    
    //println!("length of training data: {:?}",training_log_diff);




   


}


#[derive(Debug)]
struct DataFrame{
    header:csv::StringRecord,
    date:Vec<String>,
    open:Vec<f32>,
    high:Vec<f32>,
    low:Vec<f32>,
    close:Vec<f32>,
    adj_close:Vec<f32>,
    volume:Vec<i32>,
}

impl DataFrame{
    fn new()-> DataFrame{
        DataFrame { header:csv::StringRecord::new(),
            date:Vec::new(), 
            open: Vec::new(), 
            high: Vec::new(), 
            low: Vec::new(), 
            close: Vec::new(), 
            adj_close: Vec::new(), 
            volume:Vec::new() 
        }
    }
    fn read_csv(filepath:&str, has_headers :bool)->DataFrame{
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);
        let mut data_frame = DataFrame::new();
        for i in rdr.records().into_iter(){
            let record = i.unwrap();
            data_frame.push(&record);
        }
        return data_frame;
    }

    fn push(&mut self, row:&csv::StringRecord){
        self.date.push(row[0].to_string());
        self.open.push(row[1].parse().unwrap());
        self.high.push(row[2].parse().unwrap());
        self.low.push(row[3].parse().unwrap());
        self.close.push(row[4].parse().unwrap());
        self.adj_close.push(row[5].parse().unwrap());
        self.volume.push(row[6].parse().unwrap());
    }

}








