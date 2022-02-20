use datafusion::prelude::ExecutionContext;
use linefeed::{Interface, ReadResult};

use std::str::FromStr;

use crate::snowcore;

pub async fn start_repl(ctx: &mut ExecutionContext, tables: &mut Vec<String>) {
    let interface = Interface::new("snowflake").unwrap();
    interface.set_prompt("skyflake> ").unwrap();

    // let start = Instant::now();

    loop {
        if let ReadResult::Input(res) = interface.read_line().unwrap() {
            if res == "\\exit" {
                break;
            } else if res == "\\tables" {
                if tables.len() < 1 {
                    println!("");
                    println!("No tables registered.");
                    println!("Use \\load command to register csv.");
                    println!("\\load <csv_file_path>");
                    println!("");
                }
                for table in tables.iter() {
                    println!("{}", table);
                }
            } else if res.contains("\\load") {
                let file_path: Vec<&str> = res.split(" ").collect();
                if file_path.len() != 2 {
                    println!("load command requires file path");
                } else {
                    let os_str = file_path[1];
                    let file_or_path = String::from_str(os_str).unwrap();
                    let filename_vec: Vec<&str> = file_or_path.split("/").collect();
                    let filename = filename_vec[1];
                    let tablename_vec: Vec<&str> = filename.split(".").collect();
                    let tablename = tablename_vec[0];
                    tables.push(String::from(tablename));

                    snowcore::register::register_csv(ctx, tablename, &file_or_path.as_str()).await;

                    println!("{} database registered", tablename);
                }
            } else if res == "\\help" {
                help_text(true);
            } else {
                let df = ctx.sql(&res).await;
                let _f = match df {
                    Ok(dframe) => dframe.show().await.unwrap(),
                    Err(error) => {
                        println!("Error occured while executing sql: {}", error.to_string())
                    }
                };
            }
        }
    }
}

pub fn help_text(is_repl_on: bool) {
    if !is_repl_on {
        println!("");
        println!("Snowflake - A small single executable software to use SQL in your CSVs");
        println!("CLI arguments:");
        println!("--dir <dir_path>: load CSVs from a directory");
        println!("");
    } else {
        println!("");
        println!("REPL Commands:");
        println!("\\tables: List all tables");
        println!("\\load: Load a single csv");
        println!("\\exit: Quit repl");
        println!("");
    }
}
