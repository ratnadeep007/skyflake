mod cli;
mod snowcore;

use datafusion::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let mut ctx = ExecutionContext::new();
    // ctx.register_csv("test", "test_data/test.csv", CsvReadOptions::new())
    //     .await?;

    // let df = ctx
    //     .sql("SELECT * FROM test where Price > 100 and Rating is not null")
    //     .await?;
    // df.show().await?;

    // Ok(())
    let args = env::args().collect();
    let mut tables: Vec<String> = Vec::new();

    cli::parser::parse_cli(args, &mut ctx, &mut tables).await;
    cli::repl::start_repl(&mut ctx, &mut tables).await;
}
