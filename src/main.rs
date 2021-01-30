/*
 * Faelang is an experimental programming language, this is the compiler and interpreter.
 * Copyright (C) 2020  Erik Iwarson
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
mod token;
mod lexer;
mod parser;
mod compiler;
mod values;
mod ast;

fn main() -> Result<(), String> {
	// use Cow<'a, str> for string cache (string internalization)
	// Faelang might one day be a dynamic functional programming language and more semi-vague words
    // TODO: add comments //
    let code = "fn main(args) { }".to_string();
    let lexer = lexer::Lexer::new(&code);
    let ast = parser::Parser::new(lexer).parse()?;
    println!("{:#?}", ast);
    //let next = lexer.next();
    //println!("Hello, token: {:?}", next);
    Ok(())
}