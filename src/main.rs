#[allow(dead_code)]

mod parser;
use std::{fs::File, io::{BufReader, BufRead}};
use rusqlite::{Connection, Result};
use rand::seq::IteratorRandom;

#[derive(Clone,Debug)]#[allow(dead_code)]

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

// fn query_result(sample:&Vec<Lineitem>, query: &str) -> f64 {
//     let mut result = 0.0;
//     for row in sample {
//         if query == "l_quantity" {
//             result += row.l_quantity;
//         }
//     }
//     result
// }

// fn is_column_in_file(column_name: &str, file_path: &str) -> Option<String> {
//     let contents = fs::read_to_string(file_path).expect("Failed to read file");

//     contents.lines().enumerate().find_map(|(_, line)| {
//         if line.contains(column_name) {
//             Some(column_name.to_string())
//         } else {
//             None
//         }
//     })
// }


fn main(){
    let sample = create_sample("table100m.db", 0.1).unwrap();

    println!("Sample size: {:#?}", sample.len());

    let file = File::open("query.sql"). expect("Unable to open file");
    let reader = BufReader::new(file);

    for (index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (_, statement) = parser::parse_select_statement(&line).unwrap();
        println!("Line number {} \n {:#?}", index+1, statement);
}
}


