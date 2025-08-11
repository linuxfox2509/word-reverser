use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]

struct Args {
    text: String,
}

fn main() {
    let args = Args::parse();
    let reversed: String = args.text.chars().rev().collect();
    println!("{}", reversed);
}