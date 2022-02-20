use datafusion::prelude::ExecutionContext;
use linefeed::{Interface, ReadResult};

pub async fn start_repl(ctx: &mut ExecutionContext, tables: &mut Vec<String>) {
    let interface = Interface::new("snowflake").unwrap();
    interface.set_prompt("skyflake> ").unwrap();

    // let start = Instant::now();

    loop {
        if let ReadResult::Input(res) = interface.read_line().unwrap() {
            if res == "\\exit" {
                break;
            } else if res == "\\tables" {
                for table in tables.iter() {
                    println!("{}", table);
                }
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
