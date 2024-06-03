use std::collections::HashMap;


#[derive(Clone, Debug, PartialEq)]
pub enum MawuValue {
    CSVObject(Vec<HashMap<String, MawuValue>>),
    CSVArray(Vec<Vec<MawuValue>>),
    Object(HashMap<String, MawuValue>),
    Array(Vec<MawuValue>),
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Null,
}

impl From<String> for MawuValue {
    fn from(value: String) -> Self {
        if value.is_empty() {
            MawuValue::Null
        } else if value.parse::<i32>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f32>().is_ok() {
            MawuValue::Float(value.parse().unwrap())
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value)
        }
    }
}

impl MawuValue {
    pub fn is_csv_object(&self) -> bool {
        match self {
            MawuValue::CSVObject(_) => true,
            _ => false
        }
    }

    pub fn is_csv_array(&self) -> bool {
        match self {
            MawuValue::CSVArray(_) => true,
            _ => false
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            MawuValue::Object(_) => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            MawuValue::Array(_) => true,
            _ => false
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            MawuValue::String(_) => true,
            _ => false
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            MawuValue::Int(_) => true,
            _ => false
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            MawuValue::Float(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            MawuValue::Bool(_) => true,
            _ => false
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            MawuValue::Null => true,
            _ => false
        }
    }

    pub fn as_csv_object(&self) -> Option<&Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v),
            _ => None
        }
    }

    pub fn as_csv_array(&self) -> Option<&Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v),
            _ => None
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v),
            _ => None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<MawuValue>> {
        match self {
            MawuValue::Array(v) => Some(v),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            MawuValue::String(v) => Some(v),
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<&i32> {
        match self {
            MawuValue::Int(v) => Some(v),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<&f32> {
        match self {
            MawuValue::Float(v) => Some(v),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            MawuValue::Bool(v) => Some(v),
            _ => None
        }
    }

    pub fn as_null(&self) -> Option<&() > {
        match self {
            MawuValue::Null => Some(&()),
            _ => None
        }
    }

}
