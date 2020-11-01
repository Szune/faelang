#[derive(Debug)]
pub struct Ast {
    pub decls: Vec<Declaration>,
    pub globals: Vec<Expression>
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            decls: Vec::new(),
            globals: Vec::new(),
        }
    }

    pub fn add_decl(&mut self, decl: Declaration) {
        self.decls.push(decl);
    }

    pub fn add_global(&mut self, expr: Expression) {
        self.globals.push(expr);
    }
}

#[derive(Debug)]
pub struct Func {
    name: String,
    args: Vec<String>,
}
impl Func {
    pub fn new(name: String, args: Vec<String>) -> Func {
        Func {
            name,
            args,
        }
    }
}
#[derive(Debug)]
pub struct Trait;
#[derive(Debug)]
pub struct Struct;
#[derive(Debug)]
pub struct Impl;

#[derive(Debug)]
pub enum Declaration {
    Fn(Func),
    Trait(Trait),
    Struct(Struct),
    Impl(Impl),
}

#[derive(Debug)]
pub struct Expr {
    ty: Expression
}

#[derive(Debug)]
pub enum Expression {
    FnCall(String,Vec<Expr>),
    Constant(Constant)
}

/// Constant or literal
#[derive(Debug)]
pub enum Constant {
    Unit,
    String(String),
    True,
    False,
    Integer(i64),
    Float(f64),
}
