use std::{f32::consts::E, result};
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
    let array:Vec<f32>=[1.0,2.0,3.5].to_vec();
    
    let log_array:Vec<f32> = array.iter().map(|i| i.log(E)).collect();

    println!("{:?}",log_array);
    let mut log_array_shifted:Vec<f32>=log_array.clone();
    log_array_shifted.insert(0,0.0);
    log_array_shifted.pop();
    println!("{:?}",log_array_shifted);

    let diff:Vec<f32> = (0..log_array.len()).map(|i| log_array[i] - log_array_shifted[i] ).collect();

    let x:f32 = diff.len() as f32;

    println!("difference of log returns:{:?}",diff);

    let average_log_return:f32 = diff.iter().map(|&i| i as f32).sum::<f32>() / x;
    println!("{:?}",average_log_return);

    let numerator1:Vec<f32> = (0..diff.len()).map(|i|(diff[i] - average_log_return)*
        (diff[i] -average_log_return)).collect();
    
    let numerator2:f32 = numerator1.iter().map(|i| *i as f32).sum::<f32>();

    let std_dev1 = numerator2 / x;

    let std_dev = std_dev1.sqrt();

    println!("{}",std_dev);
    //got the mean of the difference of logs ie log returns and the std_deviation of the log returns;

    //next is to implement reading csv price data into a vector of floats to replace the "array" defined

    
    println!("this is the mean log return ie Mu Hat :{}",average_log_return);
    println!("this is the standard deviation of log returns ie Sigma Hat :{}",std_dev);
    
    let bac = DataFrame::read_csv("D:\\Code\\Rust_Things\\GBM_Rust\\BAC.csv", true);

    let price_data = &bac.close;
    //starting the daily log return vec calculations
    let log_array_price_data:Vec<f32> = price_data.iter().map(|i| i.log(E)).collect();
    println!("{:?}",log_array_price_data);
    
    
   


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








