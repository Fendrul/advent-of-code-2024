#[derive(Debug, PartialEq)]
pub enum XmasSymbol {
    X,
    M,
    A,
    S,
    Point,
}

impl From<char> for XmasSymbol {
    fn from(c: char) -> Self {
        match c {
            'X' => XmasSymbol::X,
            'M' => XmasSymbol::M,
            'A' => XmasSymbol::A,
            'S' => XmasSymbol::S,
            _ => panic!("Invalid character: {c}"),
        }
    }
}
