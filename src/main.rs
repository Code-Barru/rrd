use rrd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        dbg!(args.len());
        rrd::exit_with_usage();
    }
    let mut options: rrd::types::RrdOptions = rrd::types::RrdOptions {
        ..Default::default()
    };

    let mut i = 1;
    while let Some(arg) = args.get(i) {
        match arg.as_str() {
            "-b" | "--binary" => {
                if options.output_type.is_none() {
                    rrd::exit_with_usage();
                }
                options.output_type = Some(rrd::types::OutputType::Binary);
                options.group = Some(1);
                options.columns = Some(6);
            }
            "-c" | "--columns" => {
                options.columns = Some(match args.get(i + 1) {
                    Some(value) => match value.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            rrd::exit_with_usage();
                            0
                        }
                    },
                    None => {
                        rrd::exit_with_usage();
                        0
                    }
                });
            }
            "-C" | "--color" => {
                options.color = true;
            }
            "-g" | "--group" => {
                options.group = Some(match args.get(i + 1) {
                    Some(value) => match value.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            rrd::exit_with_usage();
                            0
                        }
                    },
                    None => {
                        rrd::exit_with_usage();
                        0
                    }
                });
                i += 1;
            }
            "-h" | "--help" => {
                rrd::exit_with_usage();
            }
            "-i" | "--include" => {
                if !options.output_type.is_none() {
                    options.output_type = Some(rrd::types::OutputType::CStyle);
                } else {
                    rrd::exit_with_usage();
                }
            }
            "-l" | "--limit" => {
                options.limit = Some(match args.get(i + 1) {
                    Some(value) => match value.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            rrd::exit_with_usage();
                            0
                        }
                    },
                    None => {
                        rrd::exit_with_usage();
                        0
                    }
                });
                i += 1;
            }
            "-o" | "--output" => {
                options.output = Some(args.get(i + 1).unwrap().to_string());
                i += 1;
            }
            "-ps" | "--postscript" => {
                if !options.output_type.is_none() {
                    options.output_type = Some(rrd::types::OutputType::PostScript);
                } else {
                    rrd::exit_with_usage();
                }
            }
            "-r" | "--reverse" => {
                options.reverse = true;
            }
            "-s" | "--skip" => {
                options.skip = Some(match args.get(i + 1) {
                    Some(value) => match value.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            rrd::exit_with_usage();
                            0
                        }
                    },
                    None => {
                        rrd::exit_with_usage();
                        0
                    }
                });
                i += 1;
            }
            _ => {
                options.input = Some(arg.to_string());
            }
        }
        i += 1;
    }

    if options.input.is_none() {
        rrd::exit_with_usage();
    }

    rrd::run(options);
}
