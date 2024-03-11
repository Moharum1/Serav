#[derive(Debug)]
pub struct Lexer<'a>
{
    content : &'a [char],
}

impl<'a> Lexer<'a>
{
    pub fn new(content: &'a [char]) -> Self
    {
        Self {
            content
        }
    }

    pub fn trim_left(&mut self)
    {
        // Trim left white Spaces
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {

        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        let mut n = 0;
        if self.content[0].is_alphabetic() || self.content[0].is_numeric() {
            while n < self.content.len() && (self.content[n].is_alphanumeric() || self.content[n].is_numeric()) {
                n += 1;
            }
        } else {
            n = 1; // Token is a single character if not alphabetic or numeric
        }

        let token = &self.content[..n];
        self.content = &self.content[n..];
        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a>
{
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}