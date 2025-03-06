use crate::args::{Args, OutputFormat};
use std::fs;

fn print_line(bytes: &[u8], options: &Args, addr_len: &usize) -> usize {
    let mut addr_len = *addr_len;
    let line_lenght: usize;
    let mut current_line_lenght;

    match options.format {
        OutputFormat::Hexadecimal => {
            line_lenght = 11 + (options.columns * 2) + (options.columns / options.group);
            current_line_lenght = 0;
            print!("{:08x}: ", addr_len);
            current_line_lenght += 10;

            let mut i = 1;
            for byte in bytes {
                print!("{:02x}", byte);
                if i % options.group == 0 {
                    print!(" ");
                    current_line_lenght += 1;
                    i = 1;
                } else {
                    i += 1;
                }
                addr_len += 1;
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
        OutputFormat::CStyle => {}
        OutputFormat::PostScript => {
            for byte in bytes {
                print!("{:02x}", byte);
            }
        }
        OutputFormat::Binary => {
            line_lenght = 4 + (options.columns * 9) + (options.columns / options.group);
            current_line_lenght = 0;

            print!("{:08x}: ", addr_len);
            current_line_lenght += 10;

            let mut i = 1;
            for byte in bytes {
                print!("{:08b}", byte);
                if i % options.group == 0 {
                    print!(" ");
                    current_line_lenght += 1;
                    i = 1;
                } else {
                    i += 1;
                }
                addr_len += 1;
                current_line_lenght += 8
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
    }
    return addr_len;
}

pub fn run(args: Args) {
    let contents = match fs::read(&args.input) {
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
        let line = &contents[i..(std::cmp::min(i + args.columns, contents.len()))];
        addr_len = print_line(line, &args, &addr_len);
        i += args.columns;
    }
}
