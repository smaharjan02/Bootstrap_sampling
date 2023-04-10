#[allow(dead_code)]
mod data_sampling;
mod parser;
mod bootstrap_sampling;

use data_sampling::{create_sample, lineitem_to_hashmap,sample_ground_truth};
use bootstrap_sampling::{bootstrap_sums, random_sample_with_replacement,calculate_confidence_interval, calculate_mean,calculate_std_error};
use parser::{SelectStatement, parse_select_statement};
use regex::Regex;
use std::{ fs, collections::HashMap, env};




//Getting all the column name from create table schema
fn parse_column_names(sql: &str) -> Vec<String> {
    let column_re = Regex::new(r"(?i)(\w+)\s+[\w\(\),]+(\s+NOT NULL)?").unwrap();
    column_re
        .captures_iter(sql)
        .map(|cap| cap[1].to_string())
        .filter(|name| name.to_uppercase() != "CREATE" && name.to_uppercase() != "LINEITEM")
        .collect()
}

//checking against the where condition and and condition and returning the result as 1 and 0
fn get_query_result(data: &Vec<HashMap<String, String>>, select: &SelectStatement) -> Vec<i64> {
    let mut results = Vec::with_capacity(data.len());

    for row in data {
        //checking for the condtion in where clause
        let mut where_cond_result = false;
        let mut and_cond_result = false;

        for (column_name, column_value) in row.iter() {
            if column_name == &select.where_cond_column() && column_name == &select.and_cond_column() {
                let column_value = column_value.parse::<f64>().unwrap_or(0.0); // parsing string as f64 so that it works on both int and float data types
                let where_cond_value = select.where_cond_value().parse::<f64>().unwrap_or(0.0);
                let and_cond_value = select.and_cond_value().parse::<f64>().unwrap_or(0.0);
                //matching the comparator and returning true or false based on the condition values
                where_cond_result = match select.where_cond_comparator() {
                    "<" => column_value < where_cond_value,
                    ">" => column_value > where_cond_value,
                    _ => false,
                };

                and_cond_result = match select.and_cond_comparator() {
                    "<" => column_value < and_cond_value,
                    ">" => column_value > and_cond_value,
                    _ => false,
                };
            }
        }

        //inserting 1 if the where condition and and condition are true else inserting 0
        let result = if where_cond_result && and_cond_result { 1 } else { 0 };
        results.push(result);
    }

    results
}


fn main(){
    //get the database file name and sample fraction from command line as arguments and 
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} -d <database> -s <sample_fraction>", args[0]);
        std::process::exit(1);
    }

    let db_file = args.iter().position(|arg| arg == "-d").map(|i| &args[i + 1]).expect("Missing -d <database> argument");
    let sample_fraction = args.iter().position(|arg| arg == "-s").map(|i| &args[i + 1]).expect("Missing -s <sample_fraction> argument");
    let sample_fraction = sample_fraction.parse::<f64>().expect("Sample fraction must be a valid floating-point number");

    //parsing the select statement
    let select = fs::read_to_string("query.txt")
        .expect("Unable to read file");

    let (_,select_statement) = parse_select_statement(select.as_str()).unwrap();

    //creating the sample from the database
    let (sample, database_ground_truth) = create_sample(db_file,select.as_str(), sample_fraction).unwrap();
    println!("Sample size: {} ", sample.len());
    //running the query directly on the database to get the ground truth
    print!("Database ground truth: {} \n", database_ground_truth);


    //parsing the create table schema to get the column names
    let create_table_column = fs::read_to_string("create_table.txt")
        .expect("Unable to read file");
    let column_names = parse_column_names(&create_table_column);

    
    //converting the sample to a vector of HashMaps
    let hashed_table = lineitem_to_hashmap(&sample, &column_names);

    //getting the sample query result 
    let sample_query_result = get_query_result(&hashed_table, &select_statement);

    println!("Sample query result: {:#?}", sample_query_result.len());
    //calculating the sample ground truth
    let sample_ground_truth = sample_ground_truth(&sample_query_result, sample_fraction);

    println!("Sample ground truth: {}", sample_ground_truth);

    //Bootstrap sampling and error estimations
    let z = 1.960;
    let bootstrap_size = 1000;

    //bootstrap sampling takes in the sample query result, bootstrap size and sample fraction
    let (bootstrap_sampling,elapsed_time) = bootstrap_sums(&sample_query_result, bootstrap_size,sample_fraction);
    println!("Bootstrap sampling: {:#?}", bootstrap_sampling.len());
    println!("Time taken: {:#?} seconds", elapsed_time);

    let bootstrap_mean = calculate_mean(&bootstrap_sampling);
    println!("Bootstrap mean: {:#?}", bootstrap_mean);

    let bootstrap_std_error = calculate_std_error(&bootstrap_sampling,bootstrap_mean);
    println!("Bootstrap standard error: {:#?}", bootstrap_std_error);

    let (lower_bound, upper_bound) = calculate_confidence_interval(bootstrap_std_error,z);
    println!("Confidence interval: [{:#?}, {:#?}]", lower_bound, upper_bound);

    let lower_range = sample_ground_truth as f64 - lower_bound;
    let upper_range = sample_ground_truth as f64 + upper_bound;

    println!("Lower range: [{:.2}, Upper range {:.2}]", lower_range, upper_range);

    if (database_ground_truth as f64) > lower_range && (database_ground_truth as f64) < upper_range {
        println!("The database ground truth {} is within the confidence interval[{:.2} ,{:.2}]", database_ground_truth, lower_range, upper_range);
    } else {
        println!("The database ground truth is not within the confidence interval");
    }


}



