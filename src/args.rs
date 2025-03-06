use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Args {
    #[arg(help = "The input file to read")]
    pub input: String,

    #[arg(long, short, help = "The output file", default_value = "0")]
    pub addr_len: usize,

    #[arg(long, short, help = "The number of columns", default_value = "16")]
    pub columns: usize,

    #[arg(
        long,
        short,
        help = "The number of bytes to group",
        default_value_t = 2
    )]
    pub group: usize,

    #[arg(long, short, help = "Stop after N bytes", default_value = "0")]
    pub len: usize,

    pub output: Option<String>,

    #[arg(long, short, value_enum, help = "Output format", default_value_t = OutputFormat::Hexadecimal)]
    pub format: OutputFormat,

    #[arg(long, short, help = "Reverse the output", default_value_t = false)]
    pub reverse: bool,

    #[arg(long, short, help = "Skip N bytes", default_value_t = 0)]
    pub skip: usize,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum OutputFormat {
    #[clap(name = "b", alias = "binary")]
    Binary,
    #[clap(name = "c")]
    CStyle,
    #[clap(name = "h", alias = "hex")]
    Hexadecimal,
    #[clap(name = "ps")]
    PostScript,
}
