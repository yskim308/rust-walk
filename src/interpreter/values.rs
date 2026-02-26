#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Value::Nil => false,
            Value::Boolean(b) => b,
            _ => true,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_stringy(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn as_number(&self) -> f64 {
        match *self {
            Value::Number(n) => n,
            _ => panic!("as number called on non number type: {:?}", self),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Nil => "NIL".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
        }
    }
}
