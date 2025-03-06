use clap::Parser;
pub mod args;
pub mod output;

fn main() {
    let args = args::Args::parse();
    // println!("{:?}", args);
    output::run(args);
}
