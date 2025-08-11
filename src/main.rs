use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    text: Vec<String>,

    #[arg(short = 'w', long = "words", help = "Reverse words but keep their order")]
    words: bool,
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

    println!("{}", output);
}
