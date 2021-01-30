#[derive(Debug,Clone)]
pub enum Tokens {
	LeftParenthesis,
	RightParenthesis,
    LeftBracket,
	RightBracket,
	LeftBrace,
	RightBrace,
    Colon,
	Semicolon,
	Comma,
	Function,
	Struct,
	Let,
	String(String),
	Identifier(String),
	Float(f64),
	Integer(i64),
	True,
	False,
}

impl Tokens {
	pub fn ident(&self) -> Result<String, String> {
		match self {
			Self::Identifier(v) => Ok(v.clone()),
			_ => Err(format!("Expected identifier, was {:?}", self)),
		}
	}

	pub fn str(&self) -> Result<String, String> {
		match self {
			Self::String(v) => Ok(v.clone()),
			_ => Err(format!("Expected string, was {:?}", self)),
		}
	}

	pub fn float(&self) -> Result<f64, String> {
		match self {
			Self::Float(v) => Ok(*v),
			_ => Err(format!("Expected float, was {:?}", self)),
		}
	}

	pub fn int(&self) -> Result<i64, String> {
		match self {
			Self::Integer(v) => Ok(*v),
			_ => Err(format!("Expected int, was {:?}", self)),
		}
	}
}

/*pub struct Token {
	pub ty: Tokens,
	pub line: usize,
	pub col: usize,
}

impl Token {
	pub fn string(s: String, line: usize, col: usize) -> Token {
		Token {
			ty: Tokens::String(s),
			line,
			col
		}
	}

	pub fn new(ty: Tokens, line: usize, col: usize) -> Token {
		Token {
			ty,
			line,
			col
		}
	}
}
*/