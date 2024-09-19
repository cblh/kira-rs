#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub enum FuncType {
    Void,
    Int,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Stmt),
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub struct  Stmt {
    pub num: i32,
}

// #[derive(Debug)]
// #[allow(clippy::enum_variant_names)]
// pub enum Stmt {
//     Return(Return),
// }

#[derive(Debug)]
pub struct Return {
    pub exp: Option<Exp>,
}

#[derive(Debug)]
pub struct Exp {}
