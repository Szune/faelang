module faelang.core.parser

open faelang.core.lexer
open faelang.core.ast

type Cont<'t> = {
    Value: 't
    Rest: Tokens list
}

let rec blockParser tokens = //: (Expr) = //: Cont<Root> =
    0

let parseTokens (tokens : Tokens list) : Root =
    let rec parseFunctions tokens =
        match tokens with
        | KW_Function :: rest ->
            // parse arguments as well, but start with func without args
            // parseBlock
            failwith ":("
        | t -> failwithf "Unknown global scope token: %A" t

    let functions = parseFunctions tokens
    {Functions = []}