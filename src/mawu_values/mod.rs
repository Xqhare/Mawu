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
            MawuValue::Null => None,
            _ => Some(&())
        }
    }

}

#[test]
fn test_mawu_value_from_string() {
    let mawu_string_value = MawuValue::from("test".to_string());
    assert_eq!(mawu_string_value, MawuValue::String("test".to_string()));
    assert_eq!(mawu_string_value.is_string(), true);
    assert_eq!(mawu_string_value.as_string(), Some(&"test".to_string()));

    let mawu_int_value = MawuValue::from("123".to_string());
    assert_eq!(mawu_int_value, MawuValue::Int(123));
    assert_eq!(mawu_int_value.is_int(), true);
    assert_eq!(mawu_int_value.as_int(), Some(&123));

    let mawu_float_value = MawuValue::from("123.456".to_string());
    assert_eq!(mawu_float_value, MawuValue::Float(123.456));
    assert_eq!(mawu_float_value.is_float(), true);
    assert_eq!(mawu_float_value.as_float(), Some(&123.456));

    let mawu_bool_true_value = MawuValue::from("true".to_string());
    assert_eq!(mawu_bool_true_value, MawuValue::Bool(true));
    assert_eq!(mawu_bool_true_value.is_bool(), true);
    assert_eq!(mawu_bool_true_value.as_bool(), Some(&true));

    let mawu_bool_false_value = MawuValue::from("false".to_string());
    assert_eq!(mawu_bool_false_value, MawuValue::Bool(false));
    assert_eq!(mawu_bool_false_value.is_bool(), true);
    assert_eq!(mawu_bool_false_value.as_bool(), Some(&false));

    let mawu_null_value = MawuValue::from("".to_string());
    assert_eq!(mawu_null_value, MawuValue::Null);
    assert_eq!(mawu_null_value.is_null(), true);
    assert_eq!(mawu_null_value.as_null(), None);

}

#[test]
fn test_mawu_value_constructed() {
    let mawu_object_value = MawuValue::Object(HashMap::new());
    let mawu_array_value = MawuValue::Array(vec![]);
    let mawu_csv_object_value = MawuValue::CSVObject(vec![HashMap::new()]);
    let mawu_csv_array_value = MawuValue::CSVArray(vec![vec![]]);

    assert_eq!(mawu_object_value.is_object(), true);
    assert_eq!(mawu_array_value.is_array(), true);
    assert_eq!(mawu_csv_object_value.is_csv_object(), true);
    assert_eq!(mawu_csv_array_value.is_csv_array(), true);

    assert_eq!(mawu_object_value.as_object(), Some(&HashMap::new()));
    assert_eq!(mawu_array_value.as_array(), Some(&vec![]));
    assert_eq!(mawu_csv_object_value.as_csv_object(), Some(&vec![HashMap::new()]));
    assert_eq!(mawu_csv_array_value.as_csv_array(), Some(&vec![vec![]]));
}
