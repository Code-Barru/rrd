#[derive(Debug, Copy, Clone)]
pub enum OutputType {
    Binary,
    CStyle,
    Hexadecimal,
    PostScript,
}

#[derive(Debug, Clone)]
pub struct RrdOptions {
    pub addr_len: usize,
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

impl Default for RrdOptions {
    fn default() -> RrdOptions {
        Self {
            addr_len: 0,
            color: false,
            columns: Some(16),
            group: Some(2),
            input: None,
            limit: None,
            output: None,
            output_type: Some(OutputType::Hexadecimal),
            reverse: false,
            skip: None,
        }
    }
}
