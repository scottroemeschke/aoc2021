use std::{fs, env};
use anyhow::{Context, Result};

pub fn read_input() -> Result<String> {
    fs::read_to_string("input")
        .with_context(|| format!("failed to read input file at working directory: {:?}", env::current_dir()))
}