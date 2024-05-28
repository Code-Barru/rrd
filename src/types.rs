#[derive(Debug, Copy, Clone)]
pub enum OutputType {
    Binary,
    CStyle,
    Hexadecimal,
    PostScript
}

#[derive(Default, Debug, Clone)]
pub struct RrdOptions {
    pub color: bool,
    pub columns: Option<usize>,
    pub group: Option<usize>,
    pub input: Option<String>,
    pub limit: Option<usize>,
    pub output: Option<String>,
    pub output_type: Option<OutputType>,
    pub reverse: bool,
    pub skip: Option<usize>,
}