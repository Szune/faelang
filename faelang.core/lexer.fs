module faelang.core.lexer

open System.Text

type Number =
    | I64 of int64
    | I32 of int32
    | I16 of int16
    | I8 of int8
    | U64 of uint64
    | U32 of uint32
    | U16 of uint16
    | U8 of uint8

type Tokens =
    | NoToken
    | Plus
    | Minus
    | Asterisk
    | Slash
    | LeftParens
    | RightParens
    | Comma
    | Colon
    | Dot
    | DoubleDot
    | DoubleDotEquals
    | Equals
    | Walrus
    | Ident of string
    | String of string
    | Number of Number
    | KW_Type
    | KW_Extend
    | KW_Function
    | KW_If
    | KW_ElseIf
    | KW_While
    | KW_Is
    | KW_Do
    | KW_End
    | KW_Constructor
    | KW_Operator
    | KW_Return
    | KW_Array
    | KW_Map

let keywords =
    Map [ ("fn", KW_Function)
          ("if", KW_If)
          ("elif", KW_ElseIf)
          ("while", KW_While)
          ("is", KW_Is)
          ("do", KW_Do)
          ("end", KW_End)
          ("type", KW_Type)
          ("extend", KW_Extend)
          ("ctor", KW_Constructor)
          ("op", KW_Operator)
          ("return", KW_Return)
          ("array", KW_Array)
          ("map", KW_Map) ]

let inRange it low high = (it >= low && it <= high)

let isNum c = inRange c '0' '9'

let isIdentStart c =
    inRange c 'a' 'z' || inRange c 'A' 'Z' || c = '_'

let isIdentMid c = isIdentStart c || inRange c '0' '9'

// TODO: rewrite this match mess?
let basicTokenParser (tokens, chars) =
    let token =
        match chars with // following ordering is very important
        | '+' :: rest -> (Plus, rest)
        | '-' :: rest -> (Minus, rest)
        | '*' :: rest -> (Asterisk, rest)
        | '/' :: rest -> (Slash, rest)
        | '.' :: '.' :: '=' :: rest -> (DoubleDotEquals, rest)
        | '=' :: rest -> (Equals, rest)
        | '(' :: rest -> (LeftParens, rest)
        | ')' :: rest -> (RightParens, rest)
        | '.' :: '.' :: rest -> (DoubleDot, rest)
        | '.' :: rest -> (Dot, rest)
        | ',' :: rest -> (Comma, rest)
        | ':' :: '=' :: rest -> (Walrus, rest)
        | ':' :: rest -> (Colon, rest)
        | _ -> (NoToken, chars)

    match token with
    | (NoToken, chars) -> (tokens, chars, false)
    | (tok, rest) -> (tok :: tokens, rest, true)

let keywordMatcher word = keywords |> Map.tryFind word

let identifierAndKeywordParser (tokens, chars) =
    let rec inner identCharCount remaining =
        match remaining with
        | c :: rest when isIdentMid c -> inner (identCharCount + 1) rest
        | _ -> identCharCount

    match chars with
    | curChar :: rest when isIdentStart curChar ->
        let charCount = inner 0 rest

        let identStr =
            (curChar :: rest |> List.take (charCount + 1))
            |> Array.ofList
            |> System.String

        let resultingToken =
            keywordMatcher identStr
            |> function
            | Some s -> s
            | None -> Ident identStr

        (resultingToken :: tokens, rest |> List.skip charCount, true)
    | _ -> (tokens, chars, false)

let numberParser (tokens, chars) =
    let rec innerNumberParser (chars : StringBuilder) remaining =
        match remaining with
        | c :: rest when isNum c -> innerNumberParser (chars.Append(c)) rest
        | '_' :: rest -> innerNumberParser chars rest
        | 'i' :: rest ->
            match rest with
            | '8' :: rest -> (I8 (System.SByte.Parse(chars.ToString())), rest)
            | '1' :: '6' :: rest -> (I16 (System.Int16.Parse(chars.ToString())), rest)
            | '3' :: '2' :: rest -> (I32 (System.Int32.Parse(chars.ToString())), rest)
            | '6' :: '4' :: rest -> (I64 (System.Int64.Parse(chars.ToString())), rest)
            | _ -> failwith "Missing bit count on integer"
        | 'u' :: rest ->
            match rest with
            | '8' :: rest -> (U8 (System.Byte.Parse(chars.ToString())), rest)
            | '1' :: '6' :: rest -> (U16 (System.UInt16.Parse(chars.ToString())), rest)
            | '3' :: '2' :: rest -> (U32 (System.UInt32.Parse(chars.ToString())), rest)
            | '6' :: '4' :: rest -> (U64 (System.UInt64.Parse(chars.ToString())), rest)
            | _ -> failwith "Missing bit count on unsigned integer"
        | _ -> failwith "Missing type on number"


    match chars with
    | c :: rest when isNum c || c = '-' ->
        let (number, remaining) = innerNumberParser (StringBuilder().Append(c)) rest
        ((Tokens.Number number) :: tokens, remaining, true)
    | _ -> (tokens, chars, false)

type StringState =
    | Adding
    | Escaping

let stringParser (tokens, chars) =
    let rec innerStringParser (sb: StringBuilder) remaining (state: StringState) =
        match state with
        | Adding ->
            match remaining with
            // only exit point for a valid string:
            | '"' :: rest -> (String(sb.ToString()) :: tokens, rest, true)
            | '\\' :: rest -> innerStringParser sb rest Escaping
            | c :: rest -> innerStringParser (sb.Append(c)) rest Adding
            | [] -> failwith "Unterminated string"
        | Escaping ->
            match remaining with
            | c :: rest when c = '"' || c = '\\' -> innerStringParser (sb.Append(c)) rest Adding
            | 'n' :: rest -> innerStringParser (sb.Append('\n')) rest Adding
            | 'r' :: rest -> innerStringParser (sb.Append('\r')) rest Adding
            | 't' :: rest -> innerStringParser (sb.Append('\t')) rest Adding
            | [] -> failwith "Missing escape character after backslash"
            | _ -> failwith "Unknown escape character"


    match chars with
    | '"' :: rest -> innerStringParser (StringBuilder()) rest Adding
    | _ -> (tokens, chars, false)

let whitespaceParser (tokens, chars) =
    match chars with
    | c :: rest when c = ' ' || c = '\n' || c = '\r' || c = '\t' -> (tokens, rest, true)
    | _ -> (tokens, chars, false)


let parse_chain (all_fns) (tokens, chars) =
    let rec inner fns tokens chars =
        match fns with
        | [] -> (tokens, chars, false)
        | fn :: rest ->
            match fn (tokens, chars) with
            | (_, _, false) -> inner rest tokens chars
            | (a, b, true) -> (a, b, true)

    inner all_fns tokens chars


let Tokenize text =
    let rec inner tokens remaining =
        match remaining with
        | [] -> tokens |> List.rev
        | _ ->
            match (tokens, remaining)
                  |> parse_chain [ 
                                   basicTokenParser
                                   identifierAndKeywordParser
                                   numberParser
                                   stringParser
                                   whitespaceParser ] with
            | (tokens, rest, true) -> inner tokens rest
            | (_, _, false) -> failwithf "Encountered unknown token at %A" remaining

    inner [] text
