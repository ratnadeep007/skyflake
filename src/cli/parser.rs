use std::process;

use datafusion::prelude::ExecutionContext;

use crate::snowcore;

use super::repl::help_text;

pub async fn parse_cli(
    arguments: Vec<String>,
    ctx: &mut ExecutionContext,
    tables: &mut Vec<String>,
) {
    for (idx, arg) in arguments.iter().enumerate() {
        let arg_str = arg.as_str();
        if arg_str == "--dir" {
            let dir = &arguments[idx + 1];
            snowcore::register::bulk_register_csv(ctx, dir.to_string(), tables).await;
        } else if arg_str == "--help" {
            help_text(false);
            process::exit(0);
        }
    }
}
