
use std:: {collections::HashMap};
use rusqlite::{Connection, Result};
use rand::seq::{IteratorRandom};

#[derive(Clone,Debug)]
pub struct Lineitem {
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

pub fn create_sample(db_file: &str,sql:&str, sample_fraction: f64) -> Result<(Vec<Lineitem> ,i64), Box<dyn std::error::Error>> {
    // Open the SQLite database connection
    let conn = Connection::open(db_file)?;

    //run the sql query against the database to get the actual YGT
    let count: i64 = conn.query_row(sql, [], |row| row.get(0))?;


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

    Ok((sample,count))
}

//function to create a hashmap of the table lineitem
pub fn lineitem_to_hashmap(lineitems: &[Lineitem], column_names: &Vec<String>) -> Vec<HashMap<String, String>> {
    let mut hashmaps = Vec::new();
    
    for lineitem in lineitems {
        let mut hashmap = HashMap::new();
        for column_name in column_names {
            //matching the column name with the lineitem struct field name and saving the column name as key and lineitem struct value as value
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

//function to calculate the sample ground truth
pub fn sample_ground_truth(query_result: &Vec<HashMap<usize,i64>>, sample_size: f64) -> i64 {
    let mut sum: i64 = 0;
    //summing up all the values in the query result
    for hashmap in query_result {
        sum += hashmap.values().sum::<i64>();
    }

    //calculating the sample ground truth by dividing the sum by the sample size
    let y = sum as f64 / sample_size;
    y as i64
}