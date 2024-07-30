use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    size: usize,
    file: PathBuf,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Erro: {}", e);
    }
}

fn run() -> io::Result<()> {
    let args = parse_args();
    let file_size_in_bytes = calculate_file_size_in_bytes(args.size);

    let file = create_file(&args.file)?;
    write_to_file(file, file_size_in_bytes)?;

    println!(
        "Arquivo {} com tamanho de {} MB foi criado.",
        args.file.display(),
        args.size
    );

    Ok(())
}
