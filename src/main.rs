use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    text: Vec<String>,

    #[arg(short = 'w', long = "words", help = "Reverse words but keep their order")]
    words: bool,

    #[arg(short = 'o', long = "output", help = "Write the reversed text into a file")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

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
        if let Err(e) = fs::write(&filename, &output) {
            eprintln!("Error writing to {}: {}", filename, e);
            std::process::exit(1);
        }
    } else {
        println!("{}", output);
    }
}
