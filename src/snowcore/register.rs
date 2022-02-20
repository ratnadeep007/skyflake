use datafusion::prelude::*;
use std::{fs, io, str::FromStr};

pub async fn bulk_register_csv(ctx: &mut ExecutionContext, dir: String, tables: &mut Vec<String>) {
    let entries = fs::read_dir(dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    for entry in entries {
        let os_str = entry.as_os_str().to_str().unwrap();
        let file_or_path = String::from_str(os_str).unwrap();
        let filename_vec: Vec<&str> = file_or_path.split("/").collect();
        let filename = filename_vec[1];
        let tablename_vec: Vec<&str> = filename.split(".").collect();
        let tablename = tablename_vec[0];
        tables.push(String::from(tablename));

        register_csv(ctx, tablename, &file_or_path.as_str()).await;

        println!("{} database registered", tablename);

        // if fs::metadata(Path::from(filename)).unwrap().is_file() {}
    }
}

pub async fn register_csv(ctx: &mut ExecutionContext, tablename: &str, uri: &str) {
    let _ = &ctx
        .register_csv(tablename, uri, CsvReadOptions::new())
        .await
        .unwrap();
}
