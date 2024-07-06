use core::fmt;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
/// MawuValue wraps all data types supported by Mawu.
/// It can be constructed using the `MawuValue::from` function on almost any basic rust type,
/// including Option's, Vector's and HashMap's.
/// Using the `MawuValue::default` or `MawuValue::new` function will return an `MawuValue::None`.
pub enum MawuValue {
    /// Only used to hold a headed CSV file
    CSVObject(Vec<HashMap<String, MawuValue>>),
    /// Only used to hold a headless CSV file
    CSVArray(Vec<Vec<MawuValue>>),
    /// Represents a JSON Object, with string keys and values made of `MawuValue`'s
    Object(HashMap<String, MawuValue>),
    /// Represents a JSON Array, made of `MawuValue`'s
    Array(Vec<MawuValue>),
    /// Represents an unsigned integer
    Uint(u64),
    /// Represents a signed integer
    Int(i64),
    /// Represents a floating point number
    Float(f64),
    /// Represents a string
    String(String),
    /// Represents a bool
    Bool(bool),
    /// Represents an empty or null value
    None,
}

impl fmt::Display for MawuValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuValue::CSVObject(ref v) => write!(f, "{:?}", v),
            MawuValue::CSVArray(ref v) => write!(f, "{:?}", v),
            MawuValue::Object(ref v) => write!(f, "{:?}", v),
            MawuValue::Array(ref v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|v| {
                        if v.is_none() {
                            format!("\"None\"")
                        } else {
                            format!("\"{}\"", v)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" , ")
            ),
            MawuValue::Uint(ref v) => write!(f, "{}", v),
            MawuValue::Int(ref v) => write!(f, "{}", v),
            MawuValue::Float(ref v) => write!(f, "{}", v),
            MawuValue::String(ref v) => write!(f, "{}", v),
            MawuValue::Bool(ref v) => write!(f, "{}", v),
            MawuValue::None => write!(f, "None"),
        }
    }
}

#[test]
#[ignore]
fn mawu_value_display_needs_nocapture() {
    let mawu_uint = MawuValue::Uint(1);
    println!("UINT: {}", mawu_uint);

    let mawu_float = MawuValue::Float(1.0);
    println!("FLOAT: {}", mawu_float);

    let mawu_string = MawuValue::String("hello".to_string());
    println!("STRING: {}", mawu_string);

    let mawu_bool = MawuValue::Bool(true);
    println!("BOOL: {}", mawu_bool);

    let mawu_none = MawuValue::None;
    println!("NONE: {}", mawu_none);

    let mawu_array = MawuValue::Array(vec![MawuValue::None, MawuValue::Uint(1)]);
    println!("ARRAY: {}", mawu_array);

    let mawu_object = MawuValue::Object(
        vec![("hello".to_string(), MawuValue::Uint(1))]
            .into_iter()
            .collect(),
    );
    println!("OBJECT: {}", mawu_object);

    let mawu_csv_object =
        MawuValue::CSVObject(vec![vec![("hello".to_string(), MawuValue::Uint(1))]
            .into_iter()
            .collect()]);
    println!("CSV_OBJECT: {}", mawu_csv_object);

    let mawu_csv_array = MawuValue::CSVArray(vec![vec![MawuValue::Uint(1)]]);
    println!("CSV_ARRAY: {}", mawu_csv_array);

    assert!(true);
}

impl Default for MawuValue {
    fn default() -> Self {
        MawuValue::None
    }
}

impl<V> From<Option<V>> for MawuValue
where
    V: Into<MawuValue>,
{
    fn from(value: Option<V>) -> Self {
        match value {
            Some(v) => v.into(),
            None => MawuValue::None,
        }
    }
}

impl<K, V> From<Vec<(K, V)>> for MawuValue
where
    K: Into<String>,
    V: Into<MawuValue>,
{
    fn from(value: Vec<(K, V)>) -> Self {
        MawuValue::Object(
            value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<K, V> From<HashMap<K, V>> for MawuValue
where
    K: Into<String>,
    V: Into<MawuValue>,
{
    fn from(value: HashMap<K, V>) -> Self {
        MawuValue::Object(
            value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<T> From<Vec<T>> for MawuValue
where
    T: Into<MawuValue>,
{
    fn from(value: Vec<T>) -> Self {
        MawuValue::Array(value.into_iter().map(|x| x.into()).collect())
    }
}

impl From<&MawuValue> for MawuValue {
    fn from(value: &MawuValue) -> Self {
        value.clone()
    }
}

impl From<usize> for MawuValue {
    fn from(value: usize) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<u64> for MawuValue {
    fn from(value: u64) -> Self {
        MawuValue::Uint(value)
    }
}

impl From<u32> for MawuValue {
    fn from(value: u32) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<u16> for MawuValue {
    fn from(value: u16) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<u8> for MawuValue {
    fn from(value: u8) -> Self {
        MawuValue::Uint(value as u64)
    }
}

impl From<isize> for MawuValue {
    fn from(value: isize) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<i64> for MawuValue {
    fn from(value: i64) -> Self {
        MawuValue::Int(value)
    }
}

impl From<i32> for MawuValue {
    fn from(value: i32) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<i16> for MawuValue {
    fn from(value: i16) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<i8> for MawuValue {
    fn from(value: i8) -> Self {
        MawuValue::Int(value as i64)
    }
}

impl From<f64> for MawuValue {
    fn from(value: f64) -> Self {
        MawuValue::Float(value)
    }
}

impl From<f32> for MawuValue {
    fn from(value: f32) -> Self {
        MawuValue::Float(value as f64)
    }
}

impl From<bool> for MawuValue {
    fn from(value: bool) -> Self {
        MawuValue::Bool(value)
    }
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
            let test_bind = value.parse::<f64>().unwrap();
            if test_bind.is_nan() || test_bind.is_infinite() {
                MawuValue::None
            } else {
                MawuValue::Float(value.parse().unwrap())
            }
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
            let test_bind = value.parse::<f64>().unwrap();
            if test_bind.is_nan() || test_bind.is_infinite() {
                MawuValue::None
            } else {
                MawuValue::Float(value.parse().unwrap())
            }
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
            let test_bind = value.parse::<f64>().unwrap();
            if test_bind.is_nan() || test_bind.is_infinite() {
                MawuValue::None
            } else {
                MawuValue::Float(value.parse().unwrap())
            }
        } else if value.parse::<bool>().is_ok() {
            MawuValue::Bool(value.parse().unwrap())
        } else {
            MawuValue::String(value.to_string())
        }
    }
}

impl MawuValue {
    /// To create a new `MawuValue`, please use the `MawuValue::from` function. It works on almost any basic rust type,
    /// including Option's, Vector's and HashMap's.
    /// Using the `MawuValue::default` or `MawuValue::new` function will return an `MawuValue::None`.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::default();
    /// assert_eq!(mawu_value, MawuValue::None);
    /// ```
    pub fn new() -> Self {
        MawuValue::None
    }

    /// Used only to create a new `MawuValue::CSVObject` you want to fill yourself
    ///
    /// Creates a new `MawuValue::CSVObject` with the first vector and hashmap inside initialized and
    /// empty.
    ///
    /// To unwrap, use `.to_csv_object()`
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_csv_object();
    /// let mut csv_object = mawu_value.to_csv_object().unwrap();
    /// csv_object[0].insert("hello".to_string(), MawuValue::Uint(1));
    /// assert_eq!(
    ///     csv_object[0].get("hello").unwrap(),
    ///     &MawuValue::Uint(1)
    /// );
    /// ```
    pub fn new_csv_object() -> MawuValue {
        MawuValue::CSVObject(vec![HashMap::new()])
    }

    /// Used only to create a new `MawuValue::CSVArray` you want to fill yourself
    ///
    /// Creates a new `MawuValue::CSVArray` with the first vector and vector inside initialized and empty.
    ///
    /// To unwrap, use `.to_csv_array()`
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_csv_array();
    /// let mut csv_array = mawu_value.to_csv_array().unwrap();
    /// csv_array[0].push(MawuValue::Uint(1));
    /// assert_eq!(
    ///     csv_array[0][0],
    ///     MawuValue::Uint(1)
    /// );
    /// ```
    pub fn new_csv_array() -> MawuValue {
        MawuValue::CSVArray(vec![Vec::new()])
    }

    /// Used only to create a new object you want to fill yourself
    ///
    /// Creates a new `MawuValue::Object` with an empty hashmap
    ///
    /// To unwrap, use `.to_object()`
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_object();
    /// let mut object = mawu_value.to_object().unwrap();
    /// object.insert("hello".to_string(), MawuValue::Uint(1));
    /// assert_eq!(
    ///     object.get("hello").unwrap(),
    ///     &MawuValue::Uint(1)
    /// );
    /// ```
    pub fn new_object() -> MawuValue {
        MawuValue::Object(HashMap::new())
    }

    /// Used only to create a new array you want to fill yourself
    ///
    /// Creates a new `MawuValue::Array` with an empty vector
    ///
    /// To unwrap, use `.to_array()`
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_array();
    /// let mut array = mawu_value.to_array();
    /// array.push(MawuValue::Uint(1));
    /// assert_eq!(
    ///     array[0],
    ///     MawuValue::Uint(1)
    /// );
    /// ```
    pub fn new_array() -> MawuValue {
        MawuValue::Array(Vec::new())
    }
}

#[test]
fn new_array_object() {
    let array = MawuValue::new_array();
    let object = MawuValue::new_object();
    let csv_array = MawuValue::new_csv_array();
    let csv_object = MawuValue::new_csv_object();
    assert_eq!(array, MawuValue::Array(vec![]));
    assert_eq!(object, MawuValue::Object(HashMap::new()));
    assert_eq!(csv_array, MawuValue::CSVArray(vec![vec![]]));
    assert_eq!(csv_object, MawuValue::CSVObject(vec![HashMap::new()]));
}

#[test]
fn from_hashmap() {
    let mawu_value = MawuValue::Object(HashMap::from([(
        "key".to_string(),
        MawuValue::from(u8::MAX),
    )]));
    println!("{:?}", mawu_value);
    assert!(mawu_value.is_object());
}

#[test]
fn creating_csv_object() {
    use std::collections::HashMap;

    let a_hashmap = HashMap::from([("key1".to_string(), MawuValue::from(u8::MAX))]);
    let mawu_value = MawuValue::CSVObject(vec![a_hashmap]);
    println!("{:?}", mawu_value);
    assert!(mawu_value.is_csv_object());
}

#[test]
fn creating_csv_array() {
    let mawu_value = MawuValue::CSVArray(vec![vec![MawuValue::from(u8::MAX)]]);
    println!("{:?}", mawu_value);
    assert!(mawu_value.is_csv_array());
}

impl MawuValue {
    /// Check if the value is an `CSV-Object`
    ///
    /// ## Returns
    /// `true` if the value is an `CSV-Object`, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_csv_object();
    /// assert!(mawu_value.is_csv_object());
    /// ```
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
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_csv_array();
    /// assert!(mawu_value.is_csv_array());
    /// ```
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
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_object();
    /// assert!(mawu_value.is_object());
    /// ```
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
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::new_array();
    /// assert!(mawu_value.is_array());
    /// ```
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
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::from("test");
    /// assert!(mawu_value.is_string());
    pub fn is_string(&self) -> bool {
        match self {
            MawuValue::String(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an unsigned integer
    /// To check if the value is any kind of number, use `is_number`
    ///
    /// ## Returns
    /// `true` if the value is an unsigned integer, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Uint(1);
    /// assert!(mawu_value.is_uint());
    /// ```
    pub fn is_uint(&self) -> bool {
        match self {
            MawuValue::Uint(_) => true,
            _ => false,
        }
    }

    /// Check if the value is an integer
    /// To check if the value is any kind of number, use `is_number`
    ///
    /// ## Returns
    /// `true` if the value is an integer, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Int(-1);
    /// assert!(mawu_value.is_int());
    /// ```
    pub fn is_int(&self) -> bool {
        match self {
            MawuValue::Int(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a float
    /// To check if the value is any kind of number, use `is_number`
    ///
    /// ## Returns
    /// `true` if the value is a float, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Float(1.0);
    /// assert!(mawu_value.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        match self {
            MawuValue::Float(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a number
    /// To check if the value is a specific kind of number, use `is_uint`, `is_int`, or `is_float` respectively
    ///
    /// ## Returns
    /// `true` if the value is a number, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_values = vec![MawuValue::Uint(1), MawuValue::Int(-1), MawuValue::Float(1.0)];
    /// for mawu_value in mawu_values {
    ///     assert!(mawu_value.is_number());
    /// }
    /// ```
    pub fn is_number(&self) -> bool {
        match self {
            MawuValue::Uint(_) => true,
            MawuValue::Int(_) => true,
            MawuValue::Float(_) => true,
            _ => false,
        }
    }

    /// Check if the value is a boolean
    ///
    /// ## Returns
    /// `true` if the value is a boolean, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Bool(true);
    /// assert!(mawu_value.is_bool());
    /// ```
    pub fn is_bool(&self) -> bool {
        match self {
            MawuValue::Bool(_) => true,
            _ => false,
        }
    }

    /// Convenience method to check if the value is a boolean and `true`.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Bool(true);
    /// assert!(mawu_value.is_true());
    /// ```
    pub fn is_true(&self) -> bool {
        match self {
            MawuValue::Bool(v) => match v {
                true => true,
                false => false,
            },
            _ => false,
        }
    }

    /// Convenience method to check if the value is a boolean and `false`.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Bool(false);
    /// assert!(mawu_value.is_false());
    /// ```
    pub fn is_false(&self) -> bool {
        match self {
            MawuValue::Bool(v) => match v {
                true => false,
                false => true,
            },
            _ => false,
        }
    }

    /// Convenience method to check if the value is `None`.
    ///
    /// ## Returns
    /// `true` if the value is `None`, `false` otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::None;
    /// assert!(mawu_value.is_none());
    /// let any_mawu_value = MawuValue::Int(1);
    /// assert!(!any_mawu_value.is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        match self {
            MawuValue::None => true,
            _ => false,
        }
    }

    /// Convenience method to check if the value is empty.
    /// For arrays and objects, this will return `true` if the array or object has no elements.
    /// For Strings, this will return `true` if the string has a length of zero.
    /// For numbers, this will return `true` if the number is zero.
    /// For booleans, this will always return `false`, as booleans cannot be empty.
    /// For `None`, this will always return `true`, as `None` cannot be something.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::None;
    /// assert!(mawu_value.is_empty());
    /// let any_mawu_value = MawuValue::Int(1);
    /// assert!(!any_mawu_value.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match self {
            MawuValue::CSVObject(v) => v.is_empty(),
            MawuValue::CSVArray(v) => v.is_empty(),
            MawuValue::Object(v) => v.is_empty(),
            MawuValue::Array(v) => v.is_empty(),
            MawuValue::String(v) => v.is_empty(),
            MawuValue::Uint(v) => *v == 0,
            MawuValue::Int(v) => *v == 0,
            MawuValue::Float(v) => *v == 0.0,
            MawuValue::Bool(_) => false,
            MawuValue::None => true,
        }
    }

    /// Convenience method to check if the value is negative.
    ///
    /// ## Returns
    /// `Some(true)` if the value is negative, `Some(false)` if the value is positive, and `None` if the value is not a number.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Int(-1);
    /// assert!(mawu_value.is_negative().unwrap());
    /// ```
    pub fn is_negative(&self) -> Option<bool> {
        match self {
            // unsigned cannot be negative
            MawuValue::Uint(_) => Some(false),
            MawuValue::Int(v) => {
                if *v < 0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }
            MawuValue::Float(v) => {
                if *v < 0.0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }
            _ => None,
        }
    }

    /// Convenience method to check if the value is positive.
    ///
    /// ## Returns
    /// `Some(true)` if the value is positive, `Some(false)` if the value is negative, and `None` if the value is not a number.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mawu_value = MawuValue::Int(1);
    /// assert!(mawu_value.is_positive().unwrap());
    /// ```
    pub fn is_positive(&self) -> Option<bool> {
        Some(!self.is_negative()?)
    }

    /// Returns `Some(&Vec<HashMap<String, MawuValue>>)` if the value is an `CSV-Object`, `None` otherwise.
    ///
    /// Consider using `to_csv_object` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let csv_object = MawuValue::CSVObject(vec![HashMap::from([("a".to_string(), MawuValue::Int(-1))])]);
    /// let mawu_value = csv_object.as_csv_object().unwrap();
    /// assert_eq!(mawu_value[0].get("a").unwrap(), &MawuValue::Int(-1));
    /// ```
    pub fn as_csv_object(&self) -> Option<&Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<Vec<MawuValue>>)` if the value is an `CSV-Array`, `None` otherwise.
    ///
    /// Consider using `to_csv_array` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let csv_array = MawuValue::CSVArray(vec![vec![MawuValue::Int(-1)]]);
    /// let mawu_value = csv_array.as_csv_array().unwrap();
    /// assert_eq!(mawu_value[0][0], MawuValue::Int(-1));
    /// ```
    pub fn as_csv_array(&self) -> Option<&Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&HashMap<String, MawuValue>)` if the value is an object, `None` otherwise.
    ///
    /// Consider using `to_object` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let object = MawuValue::Object(HashMap::from([("a".to_string(), MawuValue::Int(-1))]));
    /// let mawu_value = object.as_object().unwrap();
    /// assert_eq!(mawu_value.get("a").unwrap(), &MawuValue::Int(-1));
    /// ```
    pub fn as_object(&self) -> Option<&HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&Vec<MawuValue>)` if the value is an array, `None` otherwise.
    ///
    /// Consider using `to_array` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let array = MawuValue::Array(vec![MawuValue::Int(-1)]);
    /// let mawu_value = array.as_array().unwrap();
    /// assert_eq!(mawu_value[0], MawuValue::Int(-1));
    /// ```
    pub fn as_array(&self) -> Option<&Vec<MawuValue>> {
        match self {
            MawuValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&String)` if the value is a String, `None` otherwise.
    /// Please pay attention to the string type of `&String`
    ///
    /// Consider using `to_string` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let string = MawuValue::String("test".to_string());
    /// let mawu_value = string.as_string().unwrap();
    /// assert_eq!(mawu_value, &"test".to_string());
    /// ```
    pub fn as_string(&self) -> Option<&String> {
        match self {
            MawuValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&str)` if the value is a String, `None` otherwise.
    /// Please pay attention to the string type of `&str`
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let string = MawuValue::String("test".to_string());
    /// let mawu_value = string.as_str().unwrap();
    /// assert_eq!(mawu_value, "test");
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MawuValue::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    /// Returns `Some(&u64)` if the value is an integer, `None` otherwise.
    ///
    /// Consider using `to_uint` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let unsigned_integer = MawuValue::Uint(1);
    /// let mawu_value = unsigned_integer.as_uint().unwrap();
    /// assert_eq!(mawu_value, &1);
    /// ```
    pub fn as_uint(&self) -> Option<&u64> {
        match self {
            MawuValue::Uint(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&i64)` if the value is an integer, `None` otherwise.
    ///
    /// Consider using `to_int` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let integer = MawuValue::Int(-1);
    /// let mawu_value = integer.as_int().unwrap();
    /// assert_eq!(mawu_value, &-1);
    /// ```
    pub fn as_int(&self) -> Option<&i64> {
        match self {
            MawuValue::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&f64)` if the value is a float, `None` otherwise.
    ///
    /// Consider using `to_float` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let float = MawuValue::Float(1.0);
    /// let mawu_value = float.as_float().unwrap();
    /// assert_eq!(mawu_value, &1.0);
    /// ```
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            MawuValue::Float(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `Some(&bool)` if the value is a boolean, `None` otherwise.
    ///
    /// Consider using `to_bool` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let boolean = MawuValue::Bool(true);
    /// let mawu_value = boolean.as_bool().unwrap();
    /// assert_eq!(mawu_value, &true);
    /// ```
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            MawuValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `None` if the value is `None` and `Some(())` otherwise.
    ///
    /// Consider using `to_none` instead if you prefer to get an owned value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let none = MawuValue::None;
    /// let mawu_value = none.as_none();
    /// assert!(mawu_value.is_none());
    /// ```
    pub fn as_none(&self) -> Option<()> {
        match self {
            MawuValue::None => None,
            _ => Some(()),
        }
    }

    /// Returns a owned copy of the value as an `Vec<HashMap<String, MawuValue>>`.
    /// Returns `None` if the value is not an `CSV-Object`.
    /// In contrast to the rest of the `to_*` methods, this method does not cast any non
    /// `MawuValue::CSVObject` values to `MawuValue::CSVObject`.
    ///
    /// Consider using `as_csv_array` instead if you prefer to get a borrowed value
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let csv_object = MawuValue::CSVObject(vec![HashMap::from([("key".to_string(), MawuValue::from("value"))])]);
    /// let mawu_value = csv_object.to_csv_object().unwrap();
    /// assert_eq!(mawu_value[0].get("key").unwrap(), &MawuValue::String("value".to_string()));
    pub fn to_csv_object(&self) -> Option<Vec<HashMap<String, MawuValue>>> {
        match self {
            MawuValue::CSVObject(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<Vec<MawuValue>>`.
    /// Returns `None` if the value is not a `CSV-Array`.
    /// In contrast to the rest of the `to_*` methods, this method does not cast any non
    /// `MawuValue::CSVArray` values to `MawuValue::CSVArray`.
    ///
    /// Consider using `as_csv_array` instead if you prefer to get a borrowed value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let csv_array = MawuValue::CSVArray(vec![vec![MawuValue::from("value")]]);
    /// let mawu_value = csv_array.to_csv_array().unwrap();
    /// assert_eq!(mawu_value[0][0], MawuValue::String("value".to_string()));
    /// ```
    pub fn to_csv_array(&self) -> Option<Vec<Vec<MawuValue>>> {
        match self {
            MawuValue::CSVArray(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `HashMap<String, MawuValue>`.
    /// Returns `None` if the value is not an `Object`.
    /// In contrast to the rest of the `to_*` methods, this method does not cast any non
    /// `MawuValue::Object` values to `MawuValue::Object`.
    ///
    /// Consider using `as_object` instead if you prefer to get a borrowed value
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let object = MawuValue::Object(HashMap::from([("key".to_string(), MawuValue::from("value"))]));
    /// let mawu_value = object.to_object().unwrap();
    /// assert_eq!(mawu_value.get("key").unwrap(), &MawuValue::String("value".to_string()));
    /// ```
    pub fn to_object(&self) -> Option<HashMap<String, MawuValue>> {
        match self {
            MawuValue::Object(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `Vec<MawuValue>`.
    /// Also casts any other `MawuValue` to an `Vec<MawuValue>`, with the first element being the `MawuValue` you called this function on itself.
    /// This function and `to_string` are the only `to_*` functions that cannot fail.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let array = MawuValue::Array(vec![MawuValue::from("value")]);
    /// let mawu_value = array.to_array();
    /// assert_eq!(mawu_value[0], MawuValue::String("value".to_string()));
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let string = MawuValue::from("value");
    /// let mawu_value = string.to_array();
    /// assert_eq!(mawu_value[0], MawuValue::String("value".to_string()));
    /// ```
    pub fn to_array(&self) -> Vec<MawuValue> {
        match self {
            MawuValue::Array(v) => v.clone(),
            MawuValue::String(v) => vec![MawuValue::String(v.clone())],
            MawuValue::None => vec![MawuValue::None],
            MawuValue::Int(v) => vec![MawuValue::Int(*v)],
            MawuValue::Uint(v) => vec![MawuValue::Uint(*v)],
            MawuValue::Float(v) => vec![MawuValue::Float(*v)],
            MawuValue::Bool(v) => vec![MawuValue::Bool(*v)],
            MawuValue::CSVObject(v) => vec![MawuValue::CSVObject(v.clone())],
            MawuValue::CSVArray(v) => vec![MawuValue::CSVArray(v.clone())],
            MawuValue::Object(v) => vec![MawuValue::Object(v.clone())],
        }
    }

    /// Returns a owned copy of the value as a `String`.
    /// Also casts any other `MawuValue` to a `String`
    /// This function and `to_array` are the only `to_*` functions that cannot fail.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let string = MawuValue::String("value".to_string());
    /// let mawu_value = string.to_string();
    /// assert_eq!(mawu_value, "value".to_string());
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let none = MawuValue::None;
    /// let mawu_value = none.to_string();
    /// assert_eq!(mawu_value, "".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        // I implemented display so I'll use display!
        match self {
            MawuValue::String(_) => {
                format!("{}", self)
            }
            MawuValue::Int(_) => {
                format!("{}", self)
            }
            MawuValue::Uint(_) => {
                format!("{}", self)
            }
            MawuValue::Float(_) => {
                format!("{}", self)
            }
            MawuValue::Bool(_) => {
                format!("{}", self)
            }
            MawuValue::CSVObject(_) => {
                format!("{}", self)
            }
            MawuValue::CSVArray(_) => {
                format!("{}", self)
            }
            MawuValue::Object(_) => {
                format!("{}", self)
            }
            MawuValue::Array(_) => {
                format!("{}", self)
            }
            MawuValue::None => {
                format!("")
            }
        }
    }

    /// Returns a owned copy of the value as a `u64`
    /// Also casts any other `MawuValue` containing a number to a `u64`, however only some
    /// `MawuValue::Int` and `MawuValue::Float` can be represented as a `u64`
    /// a failure will be returned as `None`
    /// Please note that converting a float to a `u64` will lose the decimal part.
    /// Returns `None` if the value is not a number.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let uint = MawuValue::Uint(42);
    /// let mawu_value = uint.to_uint().unwrap();
    /// assert_eq!(mawu_value, 42);
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let int = MawuValue::Int(42);
    /// let mawu_value = int.to_uint();
    /// assert_eq!(mawu_value.unwrap(), 42);
    ///
    /// let float = MawuValue::Float(42.0);
    /// let mawu_value = float.to_uint();
    /// assert_eq!(mawu_value.unwrap(), 42);
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let neg_int = MawuValue::Int(-42);
    /// let mawu_value = neg_int.to_uint();
    /// assert!(mawu_value.is_none());
    /// ```
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
            }
            MawuValue::Float(v) => {
                // INF and NaN check
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
            }
            _ => None,
        }
    }

    /// Returns a owned copy of the value as an `usize`.
    /// Also casts any other `MawuValue` containing a number to an `usize`, however only some
    /// `MawuValue::Int` and `MawuValue::Float` can be represented as an `usize`
    /// a failure will be returned as `None`.
    /// Returns `None` if the value is not a number.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let uint = MawuValue::Uint(42);
    /// let mawu_value = uint.to_usize().unwrap();
    /// assert_eq!(mawu_value, 42);
    /// ```
    pub fn to_usize(&self) -> Option<usize> {
        let tmp = self.to_uint();
        if tmp.is_some() {
            let tmp2 = tmp.unwrap();
            if tmp2 > usize::MAX as u64 {
                None
            } else {
                Some(tmp2 as usize)
            }
        } else {
            None
        }
    }

    /// Returns a owned copy of the value as an `isize`.
    /// Also casts any other `MawuValue` containing a number to an `isize`, however only some
    /// `MawuValue::Uint` and `MawuValue::Float` can be represented as an `isize`
    /// a failure will be returned as `None`.
    /// Returns `None` if the value is not a number.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let int = MawuValue::Int(-42);
    /// let mawu_value = int.to_isize().unwrap();
    /// assert_eq!(mawu_value, -42);
    /// ```
    pub fn to_isize(&self) -> Option<isize> {
        let tmp = self.to_int();
        if tmp.is_some() {
            let tmp2 = tmp.unwrap();
            if tmp2 > isize::MAX as i64 || tmp2 < isize::MIN as i64 {
                None
            } else {
                Some(tmp2 as isize)
            }
        } else {
            None
        }
    }

    /// Returns a owned copy of the value as an `i64`.
    /// Also casts any other `MawuValue` containing a number to an `i64`, however only some
    /// `MawuValue::Uint` and `MawuValue::Float` can be represented as an `i64`
    /// a failure will be returned as `None`.
    /// Please note that converting a float to an `i64` will lose the decimal part.
    /// Returns `None` if the value is not a number.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let int = MawuValue::Int(-42);
    /// let mawu_value = int.to_int().unwrap();
    /// assert_eq!(mawu_value, -42);
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let uint = MawuValue::Uint(42);
    /// let mawu_value = uint.to_int();
    /// assert_eq!(mawu_value.unwrap(), 42);
    ///
    /// let float = MawuValue::Float(42.0);
    /// let mawu_value = float.to_int();
    /// assert_eq!(mawu_value.unwrap(), 42);
    /// ```
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
            }
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
            }
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `f64`.
    /// Also casts any other `MawuValue` containing a number to a `f64`, however only some
    /// `MawuValue::Uint` and `MawuValue::Float` can be represented as a `f64`
    /// a failure will be returned as `None`.
    /// Returns `None` if the value is not a number.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let float = MawuValue::Float(4.2);
    /// let mawu_value = float.to_float().unwrap();
    /// assert_eq!(mawu_value, 4.2);
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let int = MawuValue::Int(-42);
    /// let mawu_value = int.to_float();
    /// assert_eq!(mawu_value.unwrap(), -42.0);
    ///
    /// let uint = MawuValue::Uint(42);
    /// let mawu_value = uint.to_float();
    /// assert_eq!(mawu_value.unwrap(), 42.0);
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let string = MawuValue::String("Value".to_string());
    /// let mawu_value = string.to_float();
    /// assert!(mawu_value.is_none());
    /// ```
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
            }
            MawuValue::Uint(v) => {
                let tmp = v.to_string().parse::<f64>();
                if tmp.is_ok() {
                    Some(tmp.unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns a owned copy of the value as a `bool`.
    /// Also tries to cast any other `MawuValue` to a `bool`.
    /// Returns `None` if the value is not a boolean and could not be represented as one.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let bool = MawuValue::Bool(true);
    /// let mawu_value = bool.to_bool().unwrap();
    /// assert_eq!(mawu_value, true);
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let int = MawuValue::Int(-42);
    /// let mawu_value = int.to_bool();
    /// assert!(mawu_value.is_none());
    /// ```
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            MawuValue::Bool(v) => Some(*v),
            // I don't think that this code will ever actually return anything besides `None`
            // I have tried to pass in a lot of data and it always returns `None`, maybe remove it
            // for performance reasons?
            // I'll leave it here for now and completeness sake
            _ => {
                let tmp = self.as_string();
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
            }
        }
    }

    /// Returns `None` if the value is `None` and `Some(())` otherwise.
    /// Consider using `is_none` instead.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let none = MawuValue::None;
    /// let mawu_value = none.to_none();
    /// assert!(mawu_value.is_none());
    /// ```
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let int = MawuValue::Int(-42);
    /// let mawu_value = int.to_none();
    /// assert!(mawu_value.is_some());
    /// ```
    pub fn to_none(&self) -> Option<()> {
        match self {
            MawuValue::None => None,
            _ => Some(()),
        }
    }

    /// Clears the value
    /// For arrays and objects, it removes all values, the allocated size is not changed.
    /// For each other type, it sets the value to `MawuValue::None`.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut int = MawuValue::Int(-42);
    /// int.clear();
    /// assert!(int.is_none());
    /// ```
    pub fn clear(&mut self) {
        match self {
            MawuValue::CSVObject(v) => v.clear(),
            MawuValue::CSVArray(v) => v.clear(),
            MawuValue::Array(v) => v.clear(),
            MawuValue::Object(v) => v.clear(),
            _ => *self = MawuValue::None,
        }
    }

    /// Returns an iterator over the values of an array
    /// The values are borrowed (`&MawuValue`'s).
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut array = MawuValue::Array(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// let mut iterator = array.iter_array();
    /// assert_eq!(iterator.next(), Some(&MawuValue::from(1)));
    /// assert_eq!(iterator.next(), Some(&MawuValue::from(2)));
    /// assert_eq!(iterator.next(), Some(&MawuValue::from(3)));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter_array(&self) -> impl Iterator<Item = &MawuValue> {
        self.as_array().unwrap().iter()
    }

    /// Returns an iterator over the key-value-pairs of an object
    /// The values are borrowed (`&MawuValue`'s).
    /// The keys are borrowed (`&String`'s).
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut object = MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key2".to_string(), MawuValue::from(2)), ("key3".to_string(), MawuValue::from(3))]);
    /// let mut iterator = object.iter_object();
    /// for (key, value) in iterator {
    ///     if key == "key1" {
    ///         assert_eq!(value, &MawuValue::from(1));
    ///     } else if key == "key2" {
    ///         assert_eq!(value, &MawuValue::from(2));
    ///     } else if key == "key3" {
    ///         assert_eq!(value, &MawuValue::from(3));
    ///     }
    /// }
    /// ```
    pub fn iter_object(&self) -> impl Iterator<Item = (&String, &MawuValue)> {
        self.as_object().unwrap().iter()
    }

    /// Works on objects only.
    /// Returns a reference to the value with the given key.
    ///
    /// The key is may be any type that can be converted to a `String`.
    ///
    /// ## Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut object = MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key2".to_string(), MawuValue::from(2)), ("key3".to_string(), MawuValue::from(3))]);
    /// assert_eq!(object.get("key1").unwrap(), &MawuValue::from(1));
    /// assert_eq!(object.get("key2").unwrap(), &MawuValue::from(2));
    /// assert_eq!(object.get("key3").unwrap(), &MawuValue::from(3));
    /// assert_eq!(object.get("key4"), None);
    /// ```
    ///
    pub fn get<S>(&self, key: S) -> Option<&MawuValue>
    where
        S: Into<String>,
    {
        match self {
            MawuValue::Object(v) => v.get(key.into().as_str()),
            _ => None,
        }
    }

    /// Works on arrays only.
    /// Inserts the given value at the given index.
    ///
    /// ## Examples
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut array = MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// array.array_insert(0, MawuValue::from(0));
    /// assert_eq!(array, MawuValue::from(vec![MawuValue::from(0), MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]));
    /// ```
    pub fn array_insert(&mut self, index: usize, value: MawuValue) {
        match self {
            MawuValue::Array(v) => v.insert(index, value),
            _ => {}
        }
    }

    /// Works on objects only.
    /// Inserts the given value with the given key.
    ///
    /// ## Returns
    /// Returns `Some(MawuValue)` if the key already existed. The value was replaced and returned.
    /// Returns `None` if the key did not exist.
    /// Returns `Some(MawuValue)` if the `MawuValue` was not an `MawuValue::Object`. The `MawuValue` passed into the function was returned.
    ///
    /// ## Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut object = MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key2".to_string(), MawuValue::from(2)), ("key3".to_string(), MawuValue::from(3))]);
    /// object.object_insert("key4", MawuValue::from(10));
    /// assert_eq!(object.get("key4").unwrap(), &MawuValue::from(10));
    /// ```
    pub fn object_insert<S: Into<String>, M: Into<MawuValue>>(
        &mut self,
        key: S,
        value: M,
    ) -> Option<MawuValue> {
        match self {
            MawuValue::Object(v) => {
                let tmp = v.insert(key.into(), value.into());
                if tmp.is_none() {
                    None
                } else {
                    Some(tmp.unwrap())
                }
            }
            _ => Some(value.into()),
        }
    }

    /// Works on arrays only.
    /// Removes the value at the given index and returns it.
    /// The same restricitions as `Vec::remove` apply, as this is just a convenience function
    /// calling it.
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut array = MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// assert_eq!(array.array_remove(1), Some(MawuValue::from(2)));
    /// assert_eq!(array, MawuValue::from(vec![MawuValue::from(1), MawuValue::from(3)]));
    /// ```
    pub fn array_remove(&mut self, index: usize) -> Option<MawuValue> {
        match self {
            MawuValue::Array(v) => Some(v.remove(index)),
            _ => None,
        }
    }

    /// Works on objects only.
    /// Removes the value with the given key and returns it.
    /// The same restricitions as `HashMap::remove` apply, as this is just a convenience function
    /// calling it.
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut object = MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key2".to_string(), MawuValue::from(2)), ("key3".to_string(), MawuValue::from(3))]);
    /// assert_eq!(object.object_remove("key2"), Some(MawuValue::from(2)));
    /// assert_eq!(object, MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key3".to_string(), MawuValue::from(3))]));
    /// ```
    pub fn object_remove<S: Into<String>>(&mut self, key: S) -> Option<MawuValue> {
        match self {
            MawuValue::Object(v) => v.remove(key.into().as_str()),
            _ => None,
        }
    }

    /// Works on objects only.
    /// Checks if the object contains the given key
    ///
    /// ## Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let object = MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key2".to_string(), MawuValue::from(2)), ("key3".to_string(), MawuValue::from(3))]);
    /// assert!(object.has_key("key1"));
    /// assert!(!object.has_key("key4"));
    /// ```
    pub fn has_key<S: Into<String>>(&self, key: S) -> bool {
        match self {
            MawuValue::Object(v) => v.contains_key(key.into().as_str()),
            _ => false,
        }
    }

    /// Works on arrays only.
    /// Removes and returns the last element of the array
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut array = MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// assert_eq!(array.pop(), Some(MawuValue::from(3)));
    /// assert_eq!(array, MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2)]));
    /// ```
    pub fn pop(&mut self) -> Option<MawuValue> {
        match self {
            MawuValue::Array(v) => v.pop(),
            _ => None,
        }
    }

    /// Works on arrays only.
    /// Appends the given value to the array
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let mut array = MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// array.push(MawuValue::from(4));
    /// assert_eq!(array, MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3), MawuValue::from(4)]));
    /// ```
    pub fn push<M: Into<MawuValue>>(&mut self, value: M) {
        match self {
            MawuValue::Array(v) => v.push(value.into()),
            _ => {}
        }
    }

    /// Works on arrays only.
    /// Checks if the array contains the given value
    ///
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let array = MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// assert!(array.contains(&MawuValue::from(2)));
    /// assert!(!array.contains(&MawuValue::from(4)));
    /// ```
    pub fn contains<M: Into<MawuValue>>(&self, value: M) -> bool {
        match self {
            MawuValue::Array(v) => v.contains(&value.into()),
            _ => false,
        }
    }

    /// Returns the length of the value
    ///
    /// Returns 0 if the value is `None`, `Bool`, `Uint`, `Int` or `Float`
    /// ## Example
    /// ```rust
    /// use mawu::mawu_value::MawuValue;
    ///
    /// let array = MawuValue::from(vec![MawuValue::from(1), MawuValue::from(2), MawuValue::from(3)]);
    /// assert_eq!(array.len(), 3);
    /// let object = MawuValue::from(vec![("key1".to_string(), MawuValue::from(1)), ("key2".to_string(), MawuValue::from(2)), ("key3".to_string(), MawuValue::from(3))]);
    /// assert_eq!(object.len(), 3);
    /// let none = MawuValue::None;
    /// assert_eq!(none.len(), 0);
    /// let bool = MawuValue::from(true);
    /// assert_eq!(bool.len(), 0);
    /// let uint = MawuValue::from(123);
    /// assert_eq!(uint.len(), 0);
    /// let string = MawuValue::from("string");
    /// assert_eq!(string.len(), 6);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            MawuValue::CSVObject(v) => v.len(),
            MawuValue::CSVArray(v) => v.len(),
            MawuValue::Array(v) => v.len(),
            MawuValue::Object(v) => v.len(),
            MawuValue::None => 0,
            MawuValue::Bool(_) => 0,
            MawuValue::Uint(_) => 0,
            MawuValue::Int(_) => 0,
            MawuValue::Float(_) => 0,
            MawuValue::String(v) => v.len(),
        }
    }
}

// While not 100% test coverage, it's a decent sanity check

#[test]
fn general_as_all_types() {
    let num_uint = MawuValue::from(u8::MAX);
    assert_eq!(num_uint.as_uint().unwrap(), &255);
    let num_int = MawuValue::from(-123);
    assert_eq!(num_int.as_int().unwrap(), &-123);
    let num_float = MawuValue::from(123.2);
    assert_eq!(num_float.as_float().unwrap(), &123.2);
    let bool = MawuValue::from(true);
    assert_eq!(bool.as_bool().unwrap(), &true);
    let none = MawuValue::from("");
    assert!(none.as_none().is_none());

    let array = MawuValue::from(vec!["test", "test2", "test3"]);
    assert_eq!(array.as_array().unwrap()[2], MawuValue::from("test3"));
    let mut hashmap = HashMap::new();
    hashmap.insert("test".to_string(), MawuValue::from(123));
    let object = MawuValue::Object(hashmap);
    assert_eq!(
        object.as_object().unwrap().get("test").unwrap(),
        &MawuValue::from(123)
    );

    let string = MawuValue::from("test");
    assert_eq!(string.as_string().unwrap(), &"test");
    let str_ing = MawuValue::from(String::from("test"));
    assert_eq!(str_ing.as_str().unwrap(), "test");
}

#[test]
fn general_convenience_functions() {
    let num = MawuValue::from(123);
    assert!(num.is_number());
}

#[test]
fn convenience_boolean_methods() {
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
fn from_vec_and_hashmap() {
    let vec = vec!["test", "test2", "test3"];
    let mawu_vec = MawuValue::from(vec);
    assert_eq!(
        mawu_vec,
        MawuValue::Array(vec!["test".into(), "test2".into(), "test3".into()])
    );

    let hashmap = std::collections::HashMap::from([("test", "test2")]);
    let mawu_hashmap = MawuValue::from(hashmap);
    assert_eq!(
        mawu_hashmap,
        MawuValue::Object(HashMap::from([("test".into(), "test2".into())]))
    );
}

#[test]
fn to_primitive() {
    let mawu = MawuValue::from("test").to_string();
    assert_eq!(mawu, "test".to_string());
    let bool_true = MawuValue::from("true").to_bool().unwrap();
    assert_eq!(bool_true, true);
    let bool_false = MawuValue::from("false").to_bool().unwrap();
    assert_eq!(bool_false, false);
}

#[test]
fn as_primitive() {
    let tmp = MawuValue::from("test");
    let mawu_str = tmp.as_str().unwrap();
    assert_eq!(mawu_str, "test");
    let mawu = tmp.as_string().unwrap();
    assert_eq!(mawu, &"test".to_string());
}

#[test]
fn float_inf() {
    let float_inf = MawuValue::from("1.0e500000");
    assert!(float_inf.is_none());
}

#[test]
fn number_conversion() {
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
fn mawu_value_from_string() {
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
fn mawu_value_constructed() {
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
