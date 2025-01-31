#[derive(Debug, Clone)]
pub struct Loc {
    pub pos: usize,
    pub col: usize,
    pub ln: usize,
}

pub type ID = usize;

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub id: ID,
}

#[derive(Debug, Clone)]
pub struct Param<T> {
    pub var: Var,
    pub r#type: T,
}

#[derive(Debug, Clone)]
pub struct Def<T> {
    pub loc: Loc,
    pub name: Var,
    pub params: Vec<Param<T>>,
    pub ret_type: T,
    pub body: T,
}
