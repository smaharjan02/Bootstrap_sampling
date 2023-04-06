#[allow(dead_code)]

mod parser;
use regex::Regex;
use std::collections::HashMap;
use std::{fs::{File, self}, io::{BufReader, BufRead}};
use rusqlite::{Connection, Result};
use rand::seq::IteratorRandom;

#[derive(Clone,Debug)]
struct Lineitem {
    l_orderkey: i32,
    l_partkey: i32,
    l_suppkey: i32,
    l_linenumber: i32,
    l_quantity: f64,
    l_extendedprice: f64,
    l_discount: f64,
    l_tax: f64,
    l_returnflag: String,
    l_linestatus: String,
    l_shipdate: String,
    l_commitdate: String,
    l_receiptdate: String,
    l_shipinstruct: String,
    l_shipmode: String,
    l_comment: String,
}

impl Lineitem {
    fn from_row(row: &rusqlite::Row) -> Result<Self> {
        Ok(Lineitem {
            l_orderkey: row.get(0)?,
            l_partkey: row.get(1)?,
            l_suppkey: row.get(2)?,
            l_linenumber: row.get(3)?,
            l_quantity: row.get(4)?,
            l_extendedprice: row.get(5)?,
            l_discount: row.get(6)?,
            l_tax: row.get(7)?,
            l_returnflag: row.get(8)?,
            l_linestatus: row.get(9)?,
            l_shipdate: row.get(10)?,
            l_commitdate: row.get(11)?,
            l_receiptdate: row.get(12)?,
            l_shipinstruct: row.get(13)?,
            l_shipmode: row.get(14)?,
            l_comment: row.get(15)?,
        })
    }
}

fn create_sample(db_file: &str, sample_fraction: f64) -> Result<Vec<Lineitem>, Box<dyn std::error::Error>> {
    // Open the SQLite database connection
    let conn = Connection::open(db_file)?;

    // Define the SQL query to retrieve all rows from the lineitem table
    let query = "SELECT * FROM lineitem;";

    // Execute the query and get all the rows
    let mut stmt = conn.prepare(query)?;
    let all_rows = stmt.query_map([], Lineitem::from_row)?.collect::<Result<Vec<Lineitem>, _>>()?;


    // Calculate the sample size
    let sample_size = (all_rows.len() as f64 * sample_fraction).floor() as usize;

    // Randomly select the sample without replacement
    let mut rng = rand::thread_rng();
    let sample = all_rows.iter().cloned().choose_multiple(&mut rng, sample_size);

    // Close the database connection
    drop(stmt);
    drop(conn);

    // // Calculate the ground truth of the sample and database
    // let sample_ground_truth = sample.iter().map(|row| row.l_quantity).sum::<f64>();
    // let database_ground_truth = all_rows.iter().map(|row| row.l_quantity).sum::<f64>();

    Ok(sample)
}


fn parse_column_names(sql: &str) -> Vec<String> {
    let column_re = Regex::new(r"(?i)(\w+)\s+[\w\(\),]+(\s+NOT NULL)?").unwrap();
    column_re
        .captures_iter(sql)
        .map(|cap| cap[1].to_string())
        .filter(|name| name.to_uppercase() != "CREATE" && name.to_uppercase() != "LINEITEM")
        .collect()
}

//Creating a hashmap using column names as keys and lineitem struct values as values
fn lineitem_to_hashmap(lineitems: &[Lineitem], column_names: &Vec<String>) -> Vec<HashMap<String, String>> {
    let mut hashmaps = Vec::new();
    
    for lineitem in lineitems {
        let mut hashmap = HashMap::new();
        for column_name in column_names {
            match column_name.to_lowercase().as_str() {
                "l_orderkey" => {
                    hashmap.insert(column_name.clone(), lineitem.l_orderkey.to_string());
                }
                "l_partkey" => {
                    hashmap.insert(column_name.clone(), lineitem.l_partkey.to_string());
                }
                "l_suppkey" => {
                    hashmap.insert(column_name.clone(), lineitem.l_suppkey.to_string());
                }
                "l_linenumber" => {
                    hashmap.insert(column_name.clone(), lineitem.l_linenumber.to_string());
                }
                "l_quantity" => {
                    hashmap.insert(column_name.clone(), lineitem.l_quantity.to_string());
                }
                "l_extendedprice" => {
                    hashmap.insert(column_name.clone(), lineitem.l_extendedprice.to_string());
                }
                "l_discount" => {
                    hashmap.insert(column_name.clone(), lineitem.l_discount.to_string());
                }
                "l_tax" => {
                    hashmap.insert(column_name.clone(), lineitem.l_tax.to_string());
                }
                "l_returnflag" => {
                    hashmap.insert(column_name.clone(), lineitem.l_returnflag.to_string());
                }
                "l_linestatus" => {
                    hashmap.insert(column_name.clone(), lineitem.l_linestatus.to_string());
                }
                "l_shipdate" => {
                    hashmap.insert(column_name.clone(), lineitem.l_shipdate.to_string());
                }
                "l_commitdate" => {
                    hashmap.insert(column_name.clone(), lineitem.l_commitdate.to_string());
                }
                "l_receiptdate" => {
                    hashmap.insert(column_name.clone(), lineitem.l_receiptdate.to_string());
                }
                "l_shipinstruct" => {
                    hashmap.insert(column_name.clone(), lineitem.l_shipinstruct.to_string());
                }
                "l_shipmode" => {
                    hashmap.insert(column_name.clone(), lineitem.l_shipmode.to_string());
                }
                "l_comment" => {
                    hashmap.insert(column_name.clone(), lineitem.l_comment.to_string());
                }
                _ => {
                    println!("Not found");
                }            
            };
        }
        hashmaps.push(hashmap);
    }
    hashmaps
}


fn main(){
    let sample = create_sample("table100m.db", 0.1).unwrap();

    //parsing the create table schema to get the column names
    let create_table_column = fs::read_to_string("create_table.txt")
        .expect("Unable to read file");
    let column_names = parse_column_names(&create_table_column);


    let hashed_table = lineitem_to_hashmap(&sample, &column_names);

    println!("{:#?}", hashed_table.len());
    // for hashmap in hashed_table {
    //     println!("{:#?}", hashmap.len());
    // }

    

    
    // println!("Sample size: {:#?}", sample.len());

    // let file = File::open("query.txt"). expect("Unable to open file");
    // let reader = BufReader::new(file);

    // for (index,line) in reader.lines().enumerate() {
    //     let line = line.unwrap();
    //     let (_, statement) = parser::parse_select_statement(&line).unwrap();
    //     println!("Line number {} \n {:#?}", index+1, statement );
    // }

    
}




