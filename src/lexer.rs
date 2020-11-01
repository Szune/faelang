use std::str::Chars;
use crate::token::Tokens;
use std::collections::{HashMap,VecDeque};
use std::result::Result;

pub struct Lexer<'a> {
    code: Chars<'a>,
	prev: Option<char>,
	queue: VecDeque<Option<Tokens>>,
	keywords: HashMap<String, Tokens>,
}

impl <'a> Lexer<'a> {
    pub fn new(code: &'a String) -> Lexer<'a> {
        Lexer {
			prev: None,
            code: code.chars(),
            queue: VecDeque::new(),
			keywords: [
				(String::from("let"), Tokens::Let),
				(String::from("fn"), Tokens::Function),
				(String::from("struct"), Tokens::Struct),
			].iter()
			 .cloned()
			 .collect(),
        }
    }

	// rewrite this entire mess
	pub fn next(&mut self) -> Result<Option<Tokens>, String> {
		if self.prev.is_some() {
            let val = self.prev.unwrap();
			self.prev = None;
			Ok(self.match_any(Some(val))?)
		} else if !self.queue.is_empty() {
			Ok(self.queue.pop_front().expect("this shouldn't be possible"))
		} else {
			match self.code.next() {
				Some(next) => Ok(self.match_any(Some(next))?),
				None => Ok(None),
			}
		}
	}

	fn match_any(&mut self, maybe: Option<char>) -> Result<Option<Tokens>, String> {
		return if let Some(mut now) = maybe {
            while now.is_whitespace() {
                let maybe = self.code.next();
                match maybe {
					Some(v) => now = v,
					None => return Ok(None),
				}
			}
			match now {
				'(' => Ok(Some(Tokens::LeftParenthesis)),
				')' => Ok(Some(Tokens::RightParenthesis)),
				'[' => Ok(Some(Tokens::LeftBracket)),
				']' => Ok(Some(Tokens::RightBracket)),
				'{' => Ok(Some(Tokens::LeftBrace)),
				'}' => Ok(Some(Tokens::RightBrace)),
				':' => Ok(Some(Tokens::Semicolon)),
				';' => Ok(Some(Tokens::Colon)),
				',' => Ok(Some(Tokens::Comma)),
				'a'..='z' | 'A'..='Z' | '_' => Ok(Some(self.get_identifier(now)?)),
				'"' => Ok(Some(self.get_string()?)),
				'0'..='9' | '-' => Ok(Some(self.get_number(now)?)),
				_ => Err(format!("Unknown char '{}'", now)),
			}
		} else {
			Ok(None)
		}
	}


	fn get_string(&mut self) -> Result<Tokens, String> {
		let mut t_str = String::with_capacity(10);
		while let Some(t) = self.code.next() {
			match t {
				'\\' => {
					if let Some(next) = self.code.next() {
						match next {
							't' => t_str.push('\t'),
							'n' => t_str.push('\n'),
							'\\' => t_str.push('\\'),
							'"' => t_str.push('"'),
							_ => return Err(format!("Unescaped string: {}", t_str)),
						}
					} else {
						return Err(format!("Unescaped string: {}", t_str));
					}
				},
				'"' => {
					t_str.shrink_to_fit();
					return Ok(Tokens::String(t_str));
				},
				_ => {
					t_str.push(t);
				},
			}
		}

		Err(format!("Unescaped string: {}", t_str))
	}


	fn get_number(&mut self, c: char) -> Result<Tokens, String> {
		let mut t_str = String::with_capacity(10);
		let mut flags = 0;
		t_str.push(c);
		while let Some(t) = self.code.next() {
			match t {
				'0' ..= '9' | '_' => {
					if flags & 1 == 1 {
						flags |= 2;
					}
					t_str.push(t);
				},
				'.' => {
					if flags & 1 == 1 {
						return Err(format!("Two decimal points in number: {}", t_str))
					}
					flags |= 1;
					t_str.push(t);
				},
				next => {
					self.prev = Some(next);
					break;
				}
			}
		}

		t_str.shrink_to_fit();
		if flags & 1 == 1 {
			if flags & 2 == 2 {
				Ok(Tokens::Float(t_str.parse().unwrap()))
			}
			else{
			Err("bad".into())
			}
		} else {
			Ok(Tokens::Integer(t_str.parse().unwrap()))
		}
	}

	fn get_identifier(&mut self, now: char) -> Result<Tokens, String> {
		macro_rules! maybe_keyword(
			($self:ident, $str:ident) => {
				match $self.keywords.get($str.as_str()) {
					Some(keyword) => Ok(keyword.clone()),
					None => Ok(Tokens::Identifier($str.to_owned())),
				}
			}
		);

		let mut t_str = String::with_capacity(10);
		t_str.push(now);
		while let Some(t) = self.code.next() {
			match t {
				'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => {
					t_str.push(t);
				},
				next => {
					t_str.shrink_to_fit();
                    self.prev = Some(next);
                    return maybe_keyword!(self, t_str);
				}
			}
		}

		return maybe_keyword!(self, t_str);
	}
}
