use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum MawuValue {
    CSVObject(Vec<HashMap<String, MawuValue>>),
    CSVArray(Vec<Vec<MawuValue>>),
    Object(HashMap<String, MawuValue>),
    Array(Vec<MawuValue>),
    Uint(u64),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl From<String> for MawuValue {
    fn from(value: String) -> Self {
        if value.is_empty() {
            MawuValue::None
        } else if value.parse::<u64>().is_ok() {
            MawuValue::Uint(value.parse().unwrap())
        } else if value.parse::<i64>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f64>().is_ok() {
            MawuValue::Float(value.parse().unwrap())
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value)
        }
    }
}

impl From<&String> for MawuValue {
    fn from(value: &String) -> Self {
         if value.is_empty() {
            MawuValue::None
        } else if value.parse::<u64>().is_ok() {
            MawuValue::Uint(value.parse().unwrap())
        } else if value.parse::<i64>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f64>().is_ok() {
            MawuValue::Float(value.parse().unwrap())
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value.to_string())
        }
    }
}

impl From<&str> for MawuValue {
    fn from(value: &str) -> Self {
         if value.is_empty() {
            MawuValue::None
        } else if value.parse::<u64>().is_ok() {
            MawuValue::Uint(value.parse().unwrap())
        } else if value.parse::<i64>().is_ok() {
            MawuValue::Int(value.parse().unwrap())
        } else if value.parse::<f64>().is_ok() {
            MawuValue::Float(value.parse().unwrap())
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value.to_string())
        }
    }
}

impl MawuValue {
    /// Check if the value is an `CSV-Object`
    ///
    /// ## Returns
    /// `true` if the value is an `CSV-Object`, `false` otherwise.
    pub fn is_csv_object(&self) -> bool {
        match self {
            MawuValue::CSVObject(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an `CSV-Array`
    ///
    /// ## Returns
    /// `true` if the value is an `CSV-Array`, `false` otherwise.
    pub fn is_csv_array(&self) -> bool {
        match self {
            MawuValue::CSVArray(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an object
    ///
    /// ## Returns
    /// `true` if the value is an object, `false` otherwise.
    pub fn is_object(&self) -> bool {
        match self {
            MawuValue::Object(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an array
    ///
    /// ## Returns
    /// `true` if the value is an array, `false` otherwise.
    pub fn is_array(&self) -> bool {
        match self {
            MawuValue::Array(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a string
    ///
    /// ## Returns
    /// `true` if the value is a string, `false` otherwise.
    pub fn is_string(&self) -> bool {
        match self {
            MawuValue::String(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an unsigned integer
    ///
    /// ## Returns
    /// `true` if the value is an unsigned integer, `false` otherwise.
    pub fn is_uint(&self) -> bool {
        match self {
            MawuValue::Uint(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an integer
    ///
    /// ## Returns
    /// `true` if the value is an integer, `false` otherwise.
    pub fn is_int(&self) -> bool {
        match self {
            MawuValue::Int(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a float
    /// 
    /// ## Returns
    /// `true` if the value is a float, `false` otherwise.
    pub fn is_float(&self) -> bool {
        match self {
            MawuValue::Float(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a boolean
    ///
    /// ## Returns
    /// `true` if the value is a boolean, `false` otherwise.
    pub fn is_bool(&self) -> bool {
        match self {
            MawuValue::Bool(_) => true,
            _ => false,
        }
    }

    /// Simple convenience method to check if the value is a boolean and `true`.
    pub fn is_true(&self) -> bool {
        match self {
            MawuValue::Bool(v) => match v {
                true => true,
                false => false,
            },
            _ => false,
        }
    }

    /// Simple convenience method to check if the value is a boolean and `false`.
    pub fn is_false(&self) -> bool {
        match self {
            MawuValue::Bool(v) => match v {
                true => false,
                false => true,
            },
            _ => false,
        }
    }

    /// Simple convenience method to check if the value is `None`.
    ///
    /// ## Returns
    /// `true` if the value is `None`, `false` otherwise.
    pub fn is_none(&self) -> bool {
        match self {
            MawuValue::None => true,
            _ => false,
        }
    }

    /// Returns `Some(&Vec<HashMap<String, MawuValue>>)` if the value is an `CSV-Object`, `None` otherwise.
    pub fn as_csv_object(&self) -> Option<&Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<Vec<MawuValue>>)` if the value is an `CSV-Array`, `None` otherwise.
    pub fn as_csv_array(&self) -> Option<&Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&HashMap<String, MawuValue>)` if the value is an object, `None` otherwise.
    pub fn as_object(&self) -> Option<&HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<MawuValue>)` if the value is an array, `None` otherwise.
    pub fn as_array(&self) -> Option<&Vec<MawuValue>> {
        match self {
            MawuValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&String)` if the value is a String, `None` otherwise.
    pub fn as_string(&self) -> Option<&String> {
        match self {
            MawuValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&str)` if the value is a String, `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MawuValue::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    /// Returns `Some(&u64)` if the value is an integer, `None` otherwise.
    pub fn as_uint(&self) -> Option<&u64> {
        match self {
            MawuValue::Uint(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&i64)` if the value is an integer, `None` otherwise.
    pub fn as_int(&self) -> Option<&i64> {
        match self {
            MawuValue::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&f64)` if the value is a float, `None` otherwise.
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            MawuValue::Float(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&bool)` if the value is a boolean, `None` otherwise.
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            MawuValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(())` if the value is not `None`, `None` otherwise.
    pub fn as_none(&self) -> Option<()> {
        match self {
            MawuValue::None => None,
            _ => Some(()),
        }
    }

    /// Returns a owned copy of the value as an `Vec<HashMap<String, MawuValue>>`.
    /// Returns `None` if the value is not an `CSV-Object`.
    pub fn to_csv_object(&self) -> Option<Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<Vec<MawuValue>>`.
    /// Returns `None` if the value is not a `CSV-Array`.
    pub fn to_csv_array(&self) -> Option<Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `HashMap<String, MawuValue>`.
    /// Returns `None` if the value is not an `Object`.
    pub fn to_object(&self) -> Option<HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<MawuValue>`.
    /// Casts any primitive type representable as an `Array` to an `Array`.
    /// Returns `None` if the value is not a primitive type.
    pub fn to_array(&self) -> Option<Vec<MawuValue>> {
        match self {
            MawuValue::Array(v) => Some(v.clone()),
            MawuValue::String(v) => Some(vec![MawuValue::String(v.clone())]),
            MawuValue::None => Some(vec![MawuValue::None]),
            MawuValue::Int(v) => Some(vec![MawuValue::Int(*v)]),
            MawuValue::Uint(v) => Some(vec![MawuValue::Uint(*v)]),
            MawuValue::Float(v) => Some(vec![MawuValue::Float(*v)]),
            MawuValue::Bool(v) => Some(vec![MawuValue::Bool(*v)]),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `String`.
    /// Casts any other primitive type representable as a `String` to a `String`.
    /// Returns `None` if the value is not a primitive type.
    pub fn to_string(&self) -> Option<String> {
        match self {
            MawuValue::String(v) => Some(v.to_string()),
            MawuValue::None => Some("".to_string()),
            MawuValue::Int(v) => Some(v.to_string()),
            MawuValue::Uint(v) => Some(v.to_string()),
            MawuValue::Float(v) => Some(v.to_string()),
            MawuValue::Bool(v) => Some(v.to_string()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `u64`.
    /// Casts any other primitive type representable as a `u64` to a `u64`.
    /// Returns `None` if the value is not a number.
    pub fn to_uint(&self) -> Option<u64> {
        match self {
            MawuValue::Uint(v) => Some(*v),
            MawuValue::Int(v) => {
                if v.is_positive() {
                    let tmp = v.to_string().parse::<u64>();
                    if tmp.is_ok() {
                        Some(tmp.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            MawuValue::Float(v) => {
                if v.is_normal() {
                    let tmp = v.to_string().parse::<u64>();
                    if tmp.is_ok() {
                        Some(tmp.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `i64`.
    /// Casts any other primitive type representable as an `i64` to an `i64`.
    /// Returns `None` if the value is not a number.
    pub fn to_int(&self) -> Option<i64> {
        match self {
            MawuValue::Int(v) => Some(*v),
            MawuValue::Uint(v) => {
                let tmp = v.to_string().parse::<i64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            },
            MawuValue::Float(v) => {
                if v.is_normal() {
                    let tmp = v.to_string().parse::<i64>();
                    if tmp.is_ok() {
                        Some(tmp.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `f64`.
    /// Casts any other primitive type representable as a `f64` to a `f64`.
    /// Returns `None` if the value is not a number.
    pub fn to_float(&self) -> Option<f64> {
        match self {
            MawuValue::Float(v) => Some(*v),
            MawuValue::Int(v) => {
                let tmp = v.to_string().parse::<f64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            },
            MawuValue::Uint(v) => {
                let tmp = v.to_string().parse::<f64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// ## Returns
    /// A owned copy of the value as a `bool`.
    /// `None` if the value is not a boolean and could not be represented as one.
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            MawuValue::Bool(v) => Some(*v),
            // I don't think that this code will ever actually return anything besides `None`
            _ => {
                let tmp = self.to_string();
                if tmp.is_some() {
                    let tmp2 = tmp.unwrap().parse::<bool>();
                    if tmp2.is_ok() {
                        Some(tmp2.unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        }
    }

    

}

// While not 100% test coverage, it's a decent sanity check

#[test]
fn test_convenience_boolean_methods() {
    let bool_true = MawuValue::Bool(true);
    assert!(bool_true.is_true());
    assert!(!bool_true.is_false());

    let bool_false = MawuValue::Bool(false);
    assert!(!bool_false.is_true());
    assert!(bool_false.is_false());

    let not_bool = MawuValue::from("test");
    assert!(!not_bool.is_true());
    assert!(!not_bool.is_false());
}

#[test]
fn test_to_primitive() {
    let mawu = MawuValue::from("test").to_string().unwrap();
    assert_eq!(mawu, "test".to_string());
    let bool_true = MawuValue::from("true").to_bool().unwrap();
    assert_eq!(bool_true, true);
    let bool_false = MawuValue::from("false").to_bool().unwrap();
    assert_eq!(bool_false, false);
}

#[test]
fn test_as_primitive() {
    let tmp = MawuValue::from("test");
    let mawu_str = tmp.as_str().unwrap();
    assert_eq!(mawu_str, "test");
    let mawu = tmp.as_string().unwrap();
    assert_eq!(mawu, &"test".to_string());
}

#[test]
fn test_number_conversion() {
    let mawu_int = MawuValue::Int(-123);
    let mawu_uint = MawuValue::Uint(123);
    let mawu_float = MawuValue::Float(123.123);
    let mawu_short_float = MawuValue::Float(123.0);

    // all into u64
    let mawu_int_u64 = mawu_int.to_uint();
    let mawu_uint_u64 = mawu_uint.to_uint();
    let mawu_float_u64 = mawu_float.to_uint();
    let mawu_short_float_u64 = mawu_short_float.to_uint();
    assert!(mawu_int_u64.is_none());
    assert!(mawu_uint_u64.unwrap() == 123);
    assert!(mawu_float_u64.is_none());
    assert!(mawu_short_float_u64.unwrap() == 123);

    // all into i64
    let mawu_int_i64 = mawu_int.to_int();
    let mawu_uint_i64 = mawu_uint.to_int();
    let mawu_float_i64 = mawu_float.to_int();
    let mawu_short_float_i64 = mawu_short_float.to_int();
    assert!(mawu_int_i64.unwrap() == -123);
    assert!(mawu_uint_i64.unwrap() == 123);
    assert!(mawu_float_i64.is_none());
    assert!(mawu_short_float_i64.unwrap() == 123);

    // all into f64
    let mawu_int_f64 = mawu_int.to_float();
    let mawu_uint_f64 = mawu_uint.to_float();
    let mawu_float_f64 = mawu_float.to_float();
    let mawu_short_float_f64 = mawu_short_float.to_float();
    assert!(mawu_int_f64.unwrap() == -123.0);
    assert!(mawu_uint_f64.unwrap() == 123.0);
    assert!(mawu_float_f64.unwrap() == 123.123);
    assert!(mawu_short_float_f64.unwrap() == 123.0);

}

#[test]
fn test_mawu_value_from_string() {
    let mawu_string_value = MawuValue::from("test");
    assert_eq!(mawu_string_value, MawuValue::String("test".to_string()));
    assert_eq!(mawu_string_value.is_string(), true);
    assert_eq!(mawu_string_value.as_string(), Some(&"test".to_string()));

    let mawu_int_value = MawuValue::from("123");
    assert_eq!(mawu_int_value, MawuValue::Uint(123));
    assert_eq!(mawu_int_value.is_uint(), true);
    assert_eq!(mawu_int_value.as_uint(), Some(&123));

    let mawu_int_value = MawuValue::from("-123");
    assert_eq!(mawu_int_value, MawuValue::Int(-123));
    assert_eq!(mawu_int_value.is_int(), true);
    assert_eq!(mawu_int_value.as_int(), Some(&-123));

    let mawu_float_value = MawuValue::from("123.456");
    assert_eq!(mawu_float_value, MawuValue::Float(123.456));
    assert_eq!(mawu_float_value.is_float(), true);
    assert_eq!(mawu_float_value.as_float(), Some(&123.456));

    let mawu_bool_true_value = MawuValue::from("true");
    assert_eq!(mawu_bool_true_value, MawuValue::Bool(true));
    assert_eq!(mawu_bool_true_value.is_bool(), true);
    assert_eq!(mawu_bool_true_value.as_bool(), Some(&true));

    let mawu_bool_false_value = MawuValue::from("false");
    assert_eq!(mawu_bool_false_value, MawuValue::Bool(false));
    assert_eq!(mawu_bool_false_value.is_bool(), true);
    assert_eq!(mawu_bool_false_value.as_bool(), Some(&false));

    let mawu_null_value = MawuValue::from("");
    assert_eq!(mawu_null_value, MawuValue::None);
    assert_eq!(mawu_null_value.is_none(), true);
    assert_eq!(mawu_null_value.as_none(), None);
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
    assert_eq!(
        mawu_csv_object_value.as_csv_object(),
        Some(&vec![HashMap::new()])
    );
    assert_eq!(mawu_csv_array_value.as_csv_array(), Some(&vec![vec![]]));
}
