use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    size: usize,
    file: PathBuf,
}

fn parse_args() -> Cli {
    Cli::parse()
}

fn calculate_file_size_in_bytes(size_in_mb: usize) -> usize {
    size_in_mb * 1024 * 1024
}

fn create_file(file_path: &PathBuf) -> io::Result<File> {
    File::create(file_path)
}

fn write_to_file(mut file: File, size_in_bytes: usize) -> io::Result<()> {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let mut written_bytes = 0;

    while written_bytes < size_in_bytes {
        let remaining_bytes = size_in_bytes - written_bytes;
        let text_to_write = if lorem.len() <= remaining_bytes {
            lorem
        } else {
            &lorem[..remaining_bytes]
        };
        file.write_all(text_to_write.as_bytes())?;
        written_bytes += text_to_write.len();
    }

    file.set_len(size_in_bytes as u64)?;
    Ok(())
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
