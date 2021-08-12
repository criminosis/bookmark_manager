// Enable cargo build --fail-on-warnings
// https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/e087nj8?utm_source=share&utm_medium=web2x&context=3
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::{env, process, io::Write};
use std::path::Path;
use clap::Clap;
use anyhow::{ensure, Result, Context};
use config::{Command, Add, Search};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead};
use ansi_term::Colour::Red;

mod config;

const ENV_CSV: &str = "BOOKMARK_MANAGER_CSV";

fn main() -> Result<()> {
    let opt = config::Opts::parse();

    let csv = env::var(ENV_CSV)
        .expect(&*format!("Environmental variable {} must be set", ENV_CSV));
    ensure!(Path::new(csv.as_str()).exists(), "CSV file does not exist");

    match opt.cmd {
        Command::Add(add_opts) => add(&add_opts, &csv)?,
        Command::Search(search_opts) => search(&search_opts, &csv)?,
    }

    Ok(())
}

fn exit_error(msg: &str) {
    eprintln!("{}", Red.paint(msg));
    process::exit(1);
}

fn add(add_opts: &Add, csv: &String) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&csv)
        .unwrap();

    if url_exists(add_opts.url.as_str(), csv)? {
        exit_error(format!("{} has already been book marked", add_opts.url).as_str());
    }

    writeln!(f, "{}|{}|{}", add_opts.url, add_opts.description, add_opts.tags.join(",")).context("Could not add bookmark")?;

    Ok(())
}

/// Check if URL already exists
fn url_exists(url: &str, csv: &String) -> Result<bool> {
    let f = File::open(&csv).context("Could not open CSV file")?;
    let reader = BufReader::new(f);

    for line_result in reader.lines() {
        let line = line_result.context("Could not read line from CSV")?;
        let line_parts = line.split("|").collect::<Vec<&str>>();
        ensure!(line_parts.len() == 3, format!("CSV line has more than 3 columns: {}", line));

        let line_url = line_parts[0];

        if line_url == url {
            return Ok(true);
        }
    }

    Ok(false)
}

fn search(search_opts: &Search, csv: &String) -> Result<()> {
    let f = File::open(&csv).context("Could not open CSV file")?;
    let reader = BufReader::new(f);

    for line_result in reader.lines() {
        let line = line_result.context("Could not read line from CSV")?;
        let line_parts = line.split("|").collect::<Vec<&str>>();
        ensure!(line_parts.len() == 3, format!("CSV line has more than 3 columns: {}", line));

        let url = line_parts[0];
        let description = line_parts[1];
        let tags_all = line_parts[2];

        let tags = tags_all.split(",").collect::<Vec<&str>>();

        // Make sure the line has all tags
        if !search_opts.tags.iter().all(|tag| tags.contains(&&**tag)) {
            continue;
        }

        if url.contains(&search_opts.query) || description.contains(&search_opts.query) {
            println!("{}", line);
        }
    }

    Ok(())
}