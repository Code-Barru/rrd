use std::fs;
pub mod types;

pub fn exit_with_usage() {
    eprintln!("Usage: rrd [options] [input file]");
    std::process::exit(1);
}

fn print_line(bytes: &[u8], options: &types::RrdOptions, addr_len: &usize) -> usize {
    let mut addr_len = *addr_len;
    let line_lenght: usize = 3 + (options.group.unwrap() + 1) * options.columns.unwrap();
    let mut current_line_lenght;

    match options.output_type.unwrap() {
        types::OutputType::Hexadecimal => {
            current_line_lenght = 0;
            print!("{:08x}: ", addr_len);
            addr_len += 1;
            current_line_lenght += 10;

            let mut i = 1;
            for byte in bytes {
                print!("{:02x}", byte);
                if i % options.group.unwrap() == 0 {
                    print!(" ");
                    current_line_lenght += 1;
                    i = 1;
                } else {
                    i += 1;
                }
                current_line_lenght += 2;
            }
            let nb_spaces = if (line_lenght - current_line_lenght) > 0 {
                line_lenght - current_line_lenght
            } else {
                0
            };
            print!("{}", " ".repeat(nb_spaces));

            for byte in bytes {
                if *byte >= 32 && *byte <= 126 {
                    print!("{}", *byte as char);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        types::OutputType::CStyle => {}
        types::OutputType::PostScript => {
            for byte in bytes {
                print!("{:02x}", byte);
            }
        }
        types::OutputType::Binary => {}
    }
    return addr_len;
}

pub fn run(options: types::RrdOptions) {
    let contents = match fs::read(options.clone().input.unwrap()) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error reading file");
            std::process::exit(1);
        }
    };
    let mut i = 0;
    let mut addr_len = 0;

    while i < contents.len() {
        // Only takes the next `columns` bytes or the remaining bytes if less than `columns` remain
        let line = &contents[i..(std::cmp::min(i + options.columns.unwrap(), contents.len()))];
        addr_len = print_line(line, &options, &addr_len);
        i += options.columns.unwrap();
    }
}
