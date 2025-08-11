use clap::Parser;
use std::fs::{self, OpenOptions};
use std::io::{Write, Seek, SeekFrom, Read};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    text: Vec<String>,

    #[arg(short = 'w', long = "words", help = "Reverse words but keep their order")]
    words: bool,

    #[arg(short = 'o', long = "output", help = "Write the reversed text into a file")]
    output: Option<String>,

    #[arg(short = 's', long = "silent", help = "Do not print the reversed text to the console (Use with -o/--output)")]
    silent: bool,

    #[arg(short = 'a', long = "append", help = "Append to the output file instead of overwriting (Only works with -o/--output)")]
    append: bool,
}

fn main() {
    let args = Args::parse();

    if args.append && args.output.is_none() {
        eprintln!("Error: --append requires --output to be set");
        std::process::exit(1);
    }

    let input = args.text.join(" ");

    let output = if args.words {
        input
            .split_whitespace()
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    } else {
        input.chars().rev().collect()
    };

    if let Some(filename) = args.output {
        let result = if args.append {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&filename)
                .unwrap_or_else(|e| {
                    eprintln!("Error opening {}: {}", filename, e);
                    std::process::exit(1);
                });

            let mut need_newline = false;
            if let Ok(metadata) = file.metadata() {
                if metadata.len() > 0 {
                    if file.seek(SeekFrom::End(-1)).is_ok() {
                        let mut last_byte = [0u8];
                        if file.read_exact(&mut last_byte).is_ok() {
                            if last_byte[0] != b'\n' && last_byte[0] != b'\r' {
                                need_newline = true;
                            }
                        }
                    }
                }
            }

            if need_newline {
                file.write_all(b"\n").unwrap_or_else(|e| {
                    eprintln!("Error writing newline to {}: {}", filename, e);
                    std::process::exit(1);
                });
            }

            writeln!(file, "{}", output).unwrap_or_else(|e| {
                eprintln!("Error writing to {}: {}", filename, e);
                std::process::exit(1);
            });

            Ok(())
        } else {
            fs::write(&filename, &output)
        };

        if let Err(e) = result {
            eprintln!("Error writing to {}: {}", filename, e);
            std::process::exit(1);
        }
    }

    if !args.silent {
        println!("{}", output);
    }
}
