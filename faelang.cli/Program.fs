(*
 * Faelang is an experimental programming language, this is the compiler and interpreter.
 * Copyright (C) 2021  Erik Iwarson
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
 *)

open System
open faelang.core.lexer

[<EntryPoint>]
let main argv =
    // have a transpiler to C code for portability and speed
    // have a compiler to bytecode and an interpreter for faster builds
    let tokens = "fn main do +/*- _ident+t9 90i8 another_ident := 1u8, end".ToCharArray()
                 |> Array.toList
                 |> Tokenize
                 |> printfn "Found these tokens: %A"
    
    0 // return an integer exit code
