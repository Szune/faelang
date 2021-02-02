module faelang.core.ast

type Function = {
    Name: string
}

type Expr =
    | Block of Expr list
    | Return
    | ReturnValue of Expr

type Root = {
    Functions: Function list
}

