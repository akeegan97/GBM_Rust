use std::{f32::consts::E, collections::{ VecDeque}, io::Read};
extern crate csv;
use plotly::{Plot, Scatter, Histogram};
use rand_distr::{Normal, Distribution};
use plotly::histogram::{Bins};
use std::io;



fn main() {
    
    //reading the file into the struct created below fn main
    let bac = make_df();
    let steps:u32 = prediction_steps();

    //DataFrame::read_csv("D:\\Code\\Rust_Things\\GBM_Rust\\BAC.csv", true);
    //getting the close price in an isolated Vec
    let _data_headers = bac.header;
    let price_data = &bac.close;
    //setting the sample data indexes
    let binding = start_date();
    let start_date:&str = binding.as_str(); 
    //"2014-06-02";
    let binding_2 = end_date();
    let end_date:&str = binding_2.as_str();
    //"2018-06-01";
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

    // getting the log daily log returns
    let mut log_returns:VecDeque<f32> = (0..log_training_prices.len())
        .map(|b| log_training_prices[b] - log_training_prices_shifted[b])
        .collect();
    //need to remove first element of vector of log returns
    log_returns.pop_front();
    
    let length_of_log_returns:f32 = log_returns.len() as f32;
    //sum all the log returns

    let summed_log_returns:f32 = log_returns
        .iter()
        .map(|c| *c as f32)
        .sum::<f32>();
    //expected average log return of mu hat 
    let average_training_log_return:f32 = summed_log_returns / length_of_log_returns;

    println!("the expected log return is {} mu hat",average_training_log_return);
    // estimating the sigma and sigma^2
    //getting the square of the difference of each element minus the average log return
    let numerator1:Vec<f32> = (0..log_returns.len())
        .map(|d| (log_returns[d] - average_training_log_return) 
        * (log_returns[d] - average_training_log_return))
        .collect();
    let numerator2:f32 = numerator1
        .iter()
        .map(|e| *e as f32)
        .sum::<f32>();
    let variance:f32 = numerator2 / length_of_log_returns;
    
    let standard_dev = variance.sqrt();
    let normalized_standard_dev = standard_dev * 64.0_f32.sqrt();
    let normalized_variance = normalized_standard_dev.sqrt();

    println!("the standard deviation is {}, \nthe variance is {}",normalized_standard_dev,normalized_variance);
    
    let paths:u32 = 1000;
    
    //64;
    let delta_t:f32 = 1.0 / steps as f32;
    //finished the estimating of the paramaters mu(average log return) and sigma(variance)
    //implementing a for loop to push the answers of the equation to a vector
    let mut big_vec:Vec<Vec<f32>> = Vec::new();
    //testing creating a vector of length of the paths to simulate with vectors as elements that are the length of 
    //the predicting steps

    let first_in_inner_vec = training_prices[training_prices.len()-1];
    for _j in 0..paths{
        let mut inner_vec:Vec<f32> = Vec::new();
        inner_vec.push(first_in_inner_vec);
        let mut abc:u32 = 1;
        while  abc <= steps{
            let index_position: u32 = abc;
            let normal = Normal::new(average_training_log_return, delta_t.sqrt()).unwrap();
            let random_distr_value: f32 = normal
                .sample(&mut rand::thread_rng());
            let value:f32 = inner_vec[index_position as usize -1];
            let operation = value * (E
                .powf(average_training_log_return-(0.5*normalized_standard_dev)*delta_t + normalized_variance * random_distr_value));
            abc +=1;
            inner_vec
                .push(operation);
        };
        big_vec
            .push(inner_vec);
    }
    let gbm = big_vec.clone();
    let histogram_prices = big_vec.clone();

    //getting the x-axis values to plot
    let mut dates = bac.date;
    dates = dates[index_end_training..index_end_training+64]//setting dates equal to the same dates that are in the training set data
        .to_vec();    
    //defining the plot var
    let mut plot = Plot::new();
    //for loop to "pop" each element of the bigvec to be able to plot each path
    for k in big_vec{
        let traces = Scatter::new(dates.clone(),k);
        plot.add_trace(traces);
    }
    //adding the output of the plot to a .html file for viewing
    plot.write_html("out.html");

    //creating a vec of all the last predicted price 
    let mut predicted_price:Vec<f32> = Vec::new();
    for o in gbm{
        let price:f32 = o[o.len()-1];
        predicted_price.push(price);
    }
    let summed_predicted_prices = predicted_price
        .iter()
        .map(|z| *z as f32)
        .sum::<f32>();
    let average_predicted_price = summed_predicted_prices / paths as f32;
    //getting the real value of the price on the predicted price date;
    let real_price:f32 = bac.close[index_end_training + steps as usize];
    //printing out the predicted price and the real price as well as the difference
    println!("The Predicted Price is {}\nThe Real Price was {}\nDifference was {}",
    average_predicted_price
    ,real_price
    ,(average_predicted_price-real_price));

    //making a histogram

    let mut scatter = Plot::new();
    //working on styling the histogram plot
    for y in histogram_prices{
        let traces = Histogram::new(y).name("Prices")
            .auto_bin_x(false)
            .x_bins(Bins::new(0.0,80.0,5.0));
        scatter.add_trace(traces);
    }

    scatter.write_html("Histogram.html");


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
//asks user for the file path and creates a dataframe from the answer
fn make_df() -> DataFrame{
    println!("To Start a GBM simulation please enter the file path of the 
        csv data file MAKE SURE YOU USE DOUBLE BACKSLASHES '\\' : ");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("file path incorrect or does not exist");
    let file_path = file_path.trim();
    //push the data into the dataframe
    let df_all = DataFrame::read_csv(file_path ,true);
    return df_all
}
//ask for start date of training set
fn start_date() -> String {
    println!("Enter the start date of the training set data in the format yyyy-mm-dd: ");
    let mut start_date = String::new();
    io::stdin().read_line(&mut start_date).expect("failed to read");
    let start_date = start_date.trim();
    return start_date.to_string()
}
//ask for ending date of training set
fn end_date() -> String{
    println!("Enter the end date of the training set data in the format yyyy-mm-dd: ");
    let mut end_date = String::new();
    io::stdin().read_line(&mut end_date).expect("failed to read date, check that date is in the correct format and was a trading day");
    let end_date = end_date.trim();
    return end_date.to_string()
}

fn prediction_steps() -> u32{
    println!("How many days do you want to predict the price? Enter: ");
    let mut steps:String = String::new();
    io::stdin().read_line(&mut steps).expect("Error on Step size");
    let steps = steps.trim();
    let step:u32 = steps.parse().unwrap();
    return step
}












