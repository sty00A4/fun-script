use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ID(String), Number(f64), Bool(bool), String(String),
    ExprIn, ExprOut
}
impl Token {
    pub fn from_id(id: String) -> Self {
        match id.as_str() {
            "true" => Self::Bool(true),
            "false" => Self::Bool(false),
            _ => Self::ID(id)
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::ID(_) => format!("identifier"),
            Self::Number(_) => format!("number"),
            Self::Bool(_) => format!("boolean"),
            Self::String(_) => format!("string"),
            _ => format!("'{self}'")
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ID(id) => write!(f, "{id}"),
            Self::Number(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v:?}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::ExprIn => write!(f, "("),
            Self::ExprOut => write!(f, ")"),
        }
    }
}