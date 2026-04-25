#[derive(Clone, PartialEq, Debug)]
pub struct TokenPosition {
    line: usize,
    col: usize,
}

impl std::fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, col {}", self.line, self.col)
    }
}

impl TokenPosition {
    pub fn origin() -> TokenPosition {
        TokenPosition { line: 1, col: 1 }
    }
    #[allow(dead_code)] // this method is heavily used in testing
    pub fn from((line, col): (usize, usize)) -> TokenPosition {
        TokenPosition { line, col }
    }
    pub fn adv_line(&mut self) {
        self.line += 1;
    }

    pub fn adv_col(&mut self) {
        self.col += 1;
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    pub fn set_col(&mut self, col: usize) {
        self.col = col;
    }
}
