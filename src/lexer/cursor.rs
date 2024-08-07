// Mostly taken from `rustc_lexer` and adapted to suit the project.

/// Peekable iterator over a char sequence.
pub struct Cursor<'a> {
    len_remaining: usize,
    curr_pt: usize,
    /// Iterator over chars in a &str
    chars: &'a str,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            curr_pt: 0,
            chars: input,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.is_empty()
    }

    /// Return slice of input starting at the current point of the cursor
    pub fn at_curr_pt(&self) -> &'a str {
        &self.chars[self.curr_pt..]
    }

    /// Move cursor ahead in the input by given amount
    pub fn advance(&mut self, amt: usize) {
        self.curr_pt += amt;
        self.len_remaining -= amt;
    }

    /// Returns current cursor position
    pub fn curr_pt(&self) -> usize {
        self.curr_pt
    }
}
