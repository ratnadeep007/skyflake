mod cli;
mod snowcore;

use datafusion::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let mut ctx = ExecutionContext::new();
    let args: Vec<String> = env::args().collect();
    let mut tables: Vec<String> = Vec::new();

    cli::parser::parse_cli(args, &mut ctx, &mut tables).await;
    cli::repl::start_repl(&mut ctx, &mut tables).await;
}
