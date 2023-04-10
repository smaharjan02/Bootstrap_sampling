#[allow(dead_code)]
mod data_sampling;
mod parser;
mod bootstrap_sampling;

use data_sampling::{create_sample, lineitem_to_hashmap,sample_ground_truth};
use bootstrap_sampling::{bootstrap_sums, random_sample_with_replacement,calculate_confidence_interval, calculate_mean,calculate_std_error};
use parser::{SelectStatement, parse_select_statement};
use regex::Regex;
use std::{ fs, collections::HashMap};



//Getting all the column name from create table schema
fn parse_column_names(sql: &str) -> Vec<String> {
    let column_re = Regex::new(r"(?i)(\w+)\s+[\w\(\),]+(\s+NOT NULL)?").unwrap();
    column_re
        .captures_iter(sql)
        .map(|cap| cap[1].to_string())
        .filter(|name| name.to_uppercase() != "CREATE" && name.to_uppercase() != "LINEITEM")
        .collect()
}

/// This function takes in a vector of HashMaps and a SelectStatement and returns a vector of i64 which will be our sample query result Ys 
fn get_query_result(data: &Vec<HashMap<String, String>>, select: &SelectStatement) -> Vec<HashMap<usize, i64>> {
    let mut results: Vec<HashMap<usize, i64>> = Vec::new();
    let mut counter: usize = 0;

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

        let mut result = HashMap::new();
        //inserting 1 if the where condition and and condition are true else inserting 0
        result.insert(counter, if where_cond_result && and_cond_result { 1 } else { 0 });
        results.push(result);
        counter += 1;
    }

    results
}



fn main(){
    //get the database file name and sample fraction from command line as arguments and 
    let db_file = "table100k.db";
    let sample_fraction = 0.1;

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

    //number of bootstrap samples TODO: make this a command line argument
    let bootstrap_size = 1000;
    let bootstrap_sampling = random_sample_with_replacement(&sample_query_result, sample_query_result.len());
    println!("Bootstrap sampling: {:#?}", bootstrap_sampling.len());

    let (bootstrap_sums, elapsed_time) = bootstrap_sums(&bootstrap_sampling, bootstrap_size,sample_fraction);
    println!("Bootstrap sums: {:#?}", bootstrap_sums.len());
    println!("Time taken: {:#?} seconds", elapsed_time);

    //calculating the bootstrap mean and std error 
    let bootstrap_mean =  calculate_mean(&bootstrap_sums);
    //println!("Bootstrap mean: {}", bootstrap_mean);
    let bootstrap_std = calculate_std_error(&bootstrap_sums, bootstrap_mean);
    println!("Bootstrap std: {:.2}", bootstrap_std);

    //z_score for 95% confidence interval
    let z = 1.960;
    let (lower_bound, upper_bound) = calculate_confidence_interval(bootstrap_std, z) ;
    println!("Confidence interval: [{:.2}, {:.2}]", lower_bound, upper_bound);
    //calculating the lower_bound and upper_bound
    let lower_range = sample_ground_truth as f64 - lower_bound;
    let upper_range = sample_ground_truth as f64 + upper_bound;

    println!("Lower range: {:.2} and Upper range {:.2}", lower_range,upper_range);

    //checking if the database ground truth is within the lower and upper range 
    if (database_ground_truth as f64) > lower_range && (database_ground_truth as f64) < upper_range {
        println!("The database ground truth {} is within the Lower range {:.2} and Upper range {:.2}", database_ground_truth, lower_range, upper_range);
    } else {
        println!("The database ground truth {} is NOT!! within the Lower range {:.2} and Upper range {:.2}", database_ground_truth, lower_range, upper_range);
    }

}



