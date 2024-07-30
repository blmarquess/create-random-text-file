use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    size: usize,
    file: PathBuf,
}
