use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String), Number(f64), Bool(bool), String(String),
    End, Sep, Rep, Equal,
    ExprIn, ExprOut,
    CondIn, CondOut,
    StatIn, StatOut,
    BodyIn, BodyOut,
}
impl Token {
    pub fn from_word(word: String) -> Self {
        match word.as_str() {
            "true" => Self::Bool(true),
            "false" => Self::Bool(false),
            _ => Self::Word(word)
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::Word(_) => format!("identifier"),
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
            Self::Word(word) => write!(f, "{word}"),
            Self::Number(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v:?}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::End => write!(f, ";"),
            Self::Sep => write!(f, ","),
            Self::Rep => write!(f, ":"),
            Self::Equal => write!(f, "="),
            Self::ExprIn => write!(f, "("),
            Self::ExprOut => write!(f, ")"),
            Self::CondIn => write!(f, "<"),
            Self::CondOut => write!(f, ">"),
            Self::StatIn => write!(f, "["),
            Self::StatOut => write!(f, "]"),
            Self::BodyIn => write!(f, "{{"),
            Self::BodyOut => write!(f, "}}"),
        }
    }
}