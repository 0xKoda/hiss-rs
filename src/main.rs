use clap::{Command, Arg};
use prettytable::{Table, Row, Cell};
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use chrono::{DateTime, Utc};

const API_BASE_URL: &str = "https://api.mnemonic.no/pdns/v3/";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("DNS Lookup CLI")
        .version("1.0")
        .author("Your Name")
        .about("Performs DNS lookups using the mnemonic.no API")
        .arg(Arg::new("query")
            .help("Domain, IP, or NS to lookup")
            .required(true)
            .index(1))
        .get_matches();

    let query = matches.get_one::<String>("query").unwrap();
    let client = Client::builder()
        .build()?;
    let url = format!("{}{}", API_BASE_URL, query);

    let response = client.get(&url).send()?;
    let json: Value = response.json()?;

    if let Some(data) = json["data"].as_array() {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("First Seen"),
            Cell::new("Last Seen"),
            Cell::new("Query"),
            Cell::new("Answer"),
            Cell::new("Type"),
        ]));

        for item in data {
            let first_seen_timestamp = item["createdTimestamp"].as_i64().unwrap_or(0);
            let last_seen_timestamp = item["lastUpdatedTimestamp"].as_i64().unwrap_or(0);
            
            let first_seen = format_timestamp(first_seen_timestamp);
            let last_seen = format_timestamp(last_seen_timestamp);

            let query = item["query"].as_str().unwrap_or("N/A");
            let answer = item["answer"].as_str().unwrap_or("N/A");
            let record_type = item["rrtype"].as_str().unwrap_or("N/A");

            table.add_row(Row::new(vec![
                Cell::new(&first_seen),
                Cell::new(&last_seen),
                Cell::new(query),
                Cell::new(answer),
                Cell::new(record_type),
            ]));
        }

        table.printstd();
    } else {
        println!("No data found for the given query.");
    }

    Ok(())
}

fn format_timestamp(timestamp: i64) -> String {
    if timestamp == 0 {
        "N/A".to_string()
    } else {
        let datetime: DateTime<Utc> = DateTime::from_timestamp(timestamp / 1000, 0).unwrap();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}