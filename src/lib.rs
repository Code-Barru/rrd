use std::fs;
pub mod types;

pub fn exit_with_usage() {
    eprintln!("Usage: rrd [options] [input file]");
    std::process::exit(1);
}

pub fn run(options: types::RrdOptions) {
    let contents = match fs::read(options.input.unwrap()) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error reading file");
            std::process::exit(1);
        }
    };
}
