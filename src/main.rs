// Enable cargo build --fail-on-warnings
// https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/e087nj8?utm_source=share&utm_medium=web2x&context=3
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::env;
use clap::Clap;
use anyhow::Result;

use config::Command;
use crate::commands::add::add;
use crate::commands::search::search;
use crate::commands::tags::tags;

mod config;
mod format_output;
mod csv;
mod commands;
mod output_utils;

const ENV_CSV: &str = "BOOKMARK_MANAGER_CSV";

fn main() -> Result<()> {
    let opt = config::Opts::parse();

    let csv = env::var(ENV_CSV)
        .expect(&*format!("Environmental variable {} must be set", ENV_CSV));
    csv::create_csv(csv.as_str())?;

    match opt.cmd {
        Command::Add(add_opts) => add(&add_opts, &csv)?,
        Command::Search(search_opts) => search(&search_opts, &csv)?,
        Command::Tags(tags_opts) => tags(&tags_opts, &csv)?,
    }

    Ok(())
}