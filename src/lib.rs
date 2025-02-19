//! # Mawu
//! A JSON and CSV serialization and deserialization library written in rust.
//! 
//! Mawu, named after the ancient creator goddess Mawu in West African mythology, offers a JSON and CSV serialization and deserialization library implementing the rfc4180, rfc8259 and the ECMA-404 standard.
//!
//! Mawu is a zero dependency library and supports 64bit systems only.
//!
//! A little technical note: While Mawu uses the same return value types for both CSV and JSON, the parsing is done by two different lexers (or implementors as the standards like to call it) bundled together into one library. If you only use the JSON functionality, this results in a bloat of almost 13kb!
//!
//! ***This is a hobbyist repo badly reinventing the wheel and not ready for production use.*** 
//!
//! The performance of Mawu does leave room for improvement.
//! Talking about the parsing alone, the JSON parser manages about 84mb in 10 to 11 seconds, while the CSV parser manages about 84mb in 14 to 15 seconds. In comparison,
//! an actual parser takes about 2sec to open the same file.
//!
//! With about 1k lines, this README contains everything you never wanted to know about Mawu, but writing good documentation is never easy.
//! So I hope everything below makes sense to you.
//!
//! ## Features
//! - Simple
//! - Type aware
//! - Supports both CSV and JSON
//! - Reading and writing
//! - Write pretty with custom spacing
//! - Supports CSV files with or without header
//! - Supports missing or not provided values
//! - Fully documented
//! - Handling of edge cases is explained in the documentation
//! - Tries to stay as close to the rfc4180, rfc8259 and ECMA-404 standard as possible for maximum interoperability
//! - Actually written by a human
//!
//! ## Naming the creation: A Legacy of the Divine
//! The name "Mawu" isn't chosen by chance, it honors the long tradition of naming things after deities, by naming it after the powerful West African goddess associated with the moon, the sun, and creation itself.
//!
//! Just as Mawu, the goddess, is linked to creation, Mawu, the library, empowers you to create new things from raw data.  JSON and CSV files are like raw materials, and Mawu provides the tools to shape them into meaningful structures, ready to be used for analysis, manipulation, and ultimately, new creations.
//!
//! ## Contents
//! - [Mawu](#mawu)
//!     - [Features](#features)
//!     - [Naming the Creation: A Legacy of the Divine](#naming-the-creation-a-legacy-of-the-divine)
//!     - [Contents](#contents)
//!     - [Using Mawu](#using-mawu)
//!     - [`MawuValue`](#mawuvalue)
//!        - [Convenience functions](#convenience-functions)
//!        - [An exhaustive list of all `MawuValue` types and functions](#an-exhaustive-list-of-all-mawuvalue-types-and-functions)
//!        - [Example of getting a `MawuValue` if its type is not known or different in the same field](#example-of-getting-a-mawuvalue-if-its-type-is-not-known-or-different-in-the-same-field)
//!        - [Constructing a `MawuValue`](#constructing-a-mawuvalue)
//!            - [A comprehensive list of all types a `MawuValue` can be constructed from](#a-comprehensive-list-of-all-types-a-mawuvalue-can-be-constructed-from)
//!     - [`MawuError`](#mawuerror)
//!        - [A comprehensive list of all `MawuError`s](#a-comprehensive-list-of-all-mawuerrors)
//!     - [Reading CSV](#reading-csv)
//!         - [Handling missing or not provided values](#handling-missing-or-not-provided-values)
//!             - [With header](#with-header)
//!             - [Without header](#without-header)
//!         - [CSV Return value](#csv-return-value)
//!         - [CSV Usage](#csv-usage)
//!     - [Reading JSON](#reading-json)
//!         - [Edge cases](#edge-cases)
//!             - [Objects](#objects)
//!             - [Arrays](#arrays)
//!             - [Numbers](#numbers)
//!             - [Strings](#strings)
//!             - [Structure](#structure)
//!         - [JSON Usage](#json-usage)
//!     - [Writing](#writing)
//!         - [Writing data to disk](#writing-data-to-disk)
//!             - [Writing examples](#writing-examples)
//!         - [Writing pretty data to disk](#writing-pretty-data-to-disk)
//!             - [Writing pretty examples](#writing-pretty-examples)
//!
//! ## Using Mawu
//! Start by adding this repository to your `Cargo.toml`.
//! ```toml
//! [dependencies]
//! mawu = { git = "https://github.com/Xqhare/mawu" }
//! ```
//! Then run a shell command: 
//! ```shell
//! cargo update
//! ```
//! Mawu is now ready to go!
//!
//! After opening your IDE of choice, I recommend importing everything in the `mawu` module, at least as you get to know it.
//! ```rust
//! use mawu::*;
//!
//! // most of the time you will ever only need
//! use mawu::read::json;
//! // or one of these two
//! use mawu::read::{csv_headed, csv_headless};
//!
//! // if you want to work with `MawuValue`'s you will need
//! use mawu::mawu_value::MawuValue;
//!
//! // You could then continue with one of the examples in the chapters on CSV and JSON,
//! // or just do the good old `println!()` on anything you see!
//!
//! // Any path to a file in the documentation points to a real file in this repo under the same path
//! let path_to_file = "data/json/json-test-data/simple-object.json";
//! let mawu_value = json(path_to_file).unwrap();
//! println!("{}", mawu_value);
//! for (key, value) in mawu_value.as_object().unwrap() {
//!     println!("{}: {}", key, value);
//! }
//!
//! // to save to a file there are only these two functions
//! use mawu::{write, write_pretty};
//!
//! // you need a MawuValue and a path to a file
//! let path_to_file = "example-file.json";
//! let mawu_value = MawuValue::from(vec![1, 2, 3]);
//! // now the writing can begin!
//! let write_result = write(path_to_file, mawu_value.clone());
//! assert!(write_result.is_ok());
//! // if you want to write pretty data, use write_pretty - It takes one additional argument to specify the number of spaces to indent
//! let write_pretty_result = write_pretty(path_to_file, mawu_value, 4);
//! assert!(write_pretty_result.is_ok());
//! assert_eq!(std::path::Path::new(path_to_file).exists(), true);
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # // this step is of course not required in the real world
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//! This would print out the following (the order of the key-value-pairs may differ):
//! ```shell
//! $ cargo run
//! {"key1": String("value1"), "key2": UInt(1), "key3": Int(-1), "key4": Bool(true), "key5": None}
//! key1: value1
//! key2: 1
//! key3: -1
//! key4: true
//! key5: None
//! ```
//!
//! In the first printed line in the code example above, is a perfect example of the `MawuValue` enum.
//! Because Mawu only returns `MawuValue`'s, and you will be interacting with them a lot, I really recommend reading the chapter on `MawuValue`'s.
//!
//! ## `MawuValue`
//! Mawu uses the `MawuValue` enum to represent the different types of values that can be found in JSON and CSV files.
//!
//! Both the CSV and the JSON modules use a different subset of this enum to represent the different types of values.
//! The difference is slight however, as only the `array` and `object` are different at all, and are represented as `MawuValue::CsvArray` and `MawuValue::CsvObject` for the CSV parser, and `Mawu::Array` and `Mawu::Object` for the JSON parser.
//! The `CsvArray` and `CsvObject` types are only ever used by the CSV parser as return values. `CsvArray` is used to return a headless CSV file, and `CsvObject` is used to return a headed CSV file.
//! Because CSV files do not implement arrays or objects in the same way as JSON files, the `CsvArray` and `CsvObject` types are never used in the JSON module and they will never contain a value of type `MawuValue::Array` or `MawuValue::Object`.
//!
//! Mawu supports only 64-bit systems, and all numbers parsed by Mawu are stored in a `_64` type, e.g. `u64`, `f64` or `i64`.
//! This also means that Mawu does not support `u128` or `i128`.
//!
//! ### Convenience Functions
//! Mawu provides convenience functions for all types in the form of `is_{MawuValue}`, `as_{MawuValue}` and `to_{MawuValue}` functions.
//!
//! Calling `is_` will return `true` if the value is the type requested, and `false` otherwise. This can be useful if you have different data-types in the same array.
//! `is_true`, `is_false`, `is_number` and `is_null` are convenience functions to check if the value is a boolean and `true`, if the value is a boolean and `false`, if the value is a number (either a float, integer or signed integer) or if the value is `None`, respectively and can be used in logic without any further processing or allocating needed.
//!
//! When you call any `as_` or `to_` function on a `MawuValue` you are returned a `Option()` wrapping the desired value, or `None` if the value is not the type requested.
//! Calling `as_null` or `to_null` will return `None` instead when the value is none, and `Some()` wrapping nothing otherwise.
//!
//! All `as_` functions return a `Option<&MawuValue>`, a pointer to the underlying data. These functions are stricter than `to_`, and will only return a value if it was parsed as such.
//!
//! All `to_` functions however return a `Option<MawuValue>`, a freshly cloned copy of the underlying data. These functions are less strict than `as_`, and will return a value if it was parsed as such OR can be converted into one. So calling `to_string` on any other type will return a String, built from the underlying data. `to_` functions only return `None` if the value could not be represented as that type.
//!
//! If you are going to clone the data anyway, you can call `to_` directly. Should you call the right `to_` function on the right type, (`to_float` on a `f64` for example) no conversion checks will be done, but you could call `to_string()` on everything and parse the values yourself if you wanted to, with the added overhead of parsing the data, re-encoding it into a String and then parsing it again. I don't think you should, but you could.
//!
//! There are some miscellaneous functions that are not `as_` or `to_`. Some, like `len` and
//! `is_empty`, work on most or all types, and some, like `clear`, `iter_array`, `iter_object` work only on `Array` and `Object` types.
//!
//! The `new_` functions are not in a place where I would call them actually useful. Instead, please construct a `MawuValue` using the `MawuValue::from()` function.
//!
//! For a better overview, take a look at the table below.
//!
//! ### An exhaustive list of all `MawuValue` types and functions
//! - Primitive types
//!     - `MawuValue::None`
//!         - can only ever be none, is wrapping nothing
//!         - `as_none` and `to_null` return `None`
//!         - `is_none` returns `true`
//!         - is returned by `MawuValue::default()` and `MawuValue::new()`
//!         - `is_empty` always returns `true`
//!         - `len` always returns 0
//!     - `MawuValue::Bool`
//!         - wrapping a `bool`
//!         - `as_bool` and `to_bool` return `Option<bool>`
//!         - `is_true` and `is_false` return `true` or `false` respectively
//!         - `is_bool` returns `true`
//!         - `is_empty` always returns `false`
//!         - `len` always returns 0
//!     - `MawuValue::Uint`
//!         - wrapping a `u64`
//!         - `as_uint` and `to_uint` return `Option<u64>`
//!         - `to_usize` returns `Option<usize>`
//!         - `is_number` and `is_uint` return `true`
//!         - `is_empty` returns `true` if the unsigned integer is 0
//!         - `is_negative` always returns `false` and `is_positive` returns `true`
//!         - `len` always returns 0
//!     - `MawuValue::Int`
//!         - wrapping a `i64`
//!         - `as_int` and `to_int` return `Option<i64>`
//!         - `to_isize` returns `Option<isize>`
//!         - `is_number` and `is_int` return `true`
//!         - `is_empty` returns `true` if the int is 0
//!         - `is_negative` and `is_positive` return `true` if the int is negative or positive
//!         - `len` always returns 0
//!     - `MawuValue::Float`
//!         - wrapping a `f64`
//!         - `as_float` and `to_float` return `Option<f64>`
//!         - `is_number` and `is_float` return `true`
//!         - `is_empty` returns `true` if the float is 0.0
//!         - `is_negative` and `is_positive` return `true` if the float is negative or positive
//!         - `len` always returns 0
//!     - `MawuValue::String`
//!         - wrapping a `String`
//!         - `as_string` and `to_string` return `Option<String>`
//!         - `as_str` returns `Option<&str>`
//!         - `is_string` returns `true`
//!         - `is_empty` returns `true` if the string has a length of 0
//!         - `len` returns the length of the string
//! - JSON exclusive types
//!     - `MawuValue::Array`
//!         - wrapping a `Vec<MawuValue>`
//!         - `as_array` and `to_array` return `Option<Vec<MawuValue>>`
//!         - `is_array` returns `true`
//!         - can be constructed by using `MawuValue::new_array`
//!         - `is_empty` returns `true` if the array is empty
//!         - `clear` removes all elements from the array
//!         - `iter_array` returns an iterator over the array
//!         - `array_insert` inserts an element into the array at the given index
//!         - `array_remove` removes an element from the array at the given index
//!         - `pop` removes and returns the last element of the array
//!         - `push` appends an element to the end of the array
//!         - `contains` returns `true` if the array contains the element
//!         - `len` returns the number of elements in the array
//!         - `clear` removes all elements from the array
//!         - `iter_array` returns an iterator over the array
//!         - `array_insert` inserts an element into the array at the given index
//!         - `array_remove` removes an element from the array at the given index
//!         - `array_peek` returns a reference to the element from the array at the given index
//!         - `pop` removes and returns the last element of the array
//!         - `push` appends an element to the end of the array
//!         - `contains` returns `true` if the array contains the element
//!     - `MawuValue::Object`
//!         - wrapping a `HashMap<String, MawuValue>`
//!         - `as_object` and `to_object` return `Option<HashMap<String, MawuValue>>`
//!         - `is_object` returns `true`
//!         - can be constructed by using `MawuValue::new_object`
//!         - `is_empty` returns `true` if the object is empty
//!         - `clear` removes all elements from the object
//!         - `iter_object` returns an iterator over the object
//!         - `get` returns a `Option<MawuValue>` if the object contains the key
//!         - `object_insert` inserts an element into the object at the given key
//!         - `object_remove` removes an element from the object at the given key
//!         - `has_key` returns `true` if the object contains the key
//!         - `len` returns the number of elements in the object
//!         - `clear` removes all elements from the object
//!         - `iter_object` returns an iterator over the object
//!         - `get` returns a `Option<MawuValue>` if the object contains the key
//!         - `object_insert` inserts an element into the object at the given key
//!         - `object_remove` removes an element from the object at the given key
//!         - `has_key` returns `true` if the object contains the key
//! - CSV exclusive types
//!     - `MawuValue::CsvArray`
//!         - wrapping a `Vec<Vec<MawuValue>>`
//!         - `as_csv_array` and `to_csv_array` return `Option<Vec<Vec<MawuValue>>>`
//!         - `is_csv_array` returns `true`
//!         - can be constructed by using `MawuValue::new_csv_array`
//!         - `is_empty` returns `true` if the array is empty
//!         - `clear` removes all elements from the array
//!         - `len` returns the number of elements in the array
//!         - `clear` removes all elements from the array
//!     - `MawuValue::CsvObject`
//!         - wrapping a `Vec<HashMap<String, MawuValue>>`
//!         - `as_csv_object` and `to_csv_object` return `Option<Vec<HashMap<String, MawuValue>>>`
//!         - `is_csv_object` returns `true`
//!         - can be constructed by using `MawuValue::new_csv_object`
//!         - `is_empty` returns `true` if the object is empty
//!         - `clear` removes all elements from the object
//!         - `len` returns the number of elements in the object
//!         - `clear` removes all elements from the object
//!
//! #### Example of getting a `MawuValue` if its type is not known or different in the same field
//! This example shows the usage of `is_`, `as_` and `to_` functions.
//! ```rust
//! use mawu::mawu_value::MawuValue;
//! use mawu::read::json;
//!
//! use std::collections::HashMap;
//!
//! let path_to_file = "data/json/json-test-data/simple-json.json";
//! // These are the primitive types
//! let mawu_value = json(path_to_file).unwrap();
//! if mawu_value.is_none() {
//!     let value: Option<()> = mawu_value.as_none();
//!     let owned_value: Option<()> = mawu_value.to_none();
//! // Do something with `value`
//!     assert_eq!(value, None);
//!     assert_eq!(owned_value, None);
//! } else if mawu_value.is_bool() {
//!     let value: &bool = mawu_value.as_bool().unwrap();
//!     let owned_value: bool = mawu_value.to_bool().unwrap();
//!     // Do something with `value`
//!     assert_eq!(value, &true);
//!     assert_eq!(owned_value, true);
//! } else if mawu_value.is_uint() {
//!     let value: &u64 = mawu_value.as_uint().unwrap();
//!     let owned_value: u64 = mawu_value.to_uint().unwrap();
//!  // Do something with `value`
//!     assert_eq!(value, &1);
//!     assert_eq!(owned_value, 1);
//! } else if mawu_value.is_int() {
//!     let value: &i64 = mawu_value.as_int().unwrap();
//!     let owned_value: i64 = mawu_value.to_int().unwrap();
//!     // Do something with `value`
//!     assert_eq!(value, &-1);
//!     assert_eq!(owned_value, -1);
//! } else if mawu_value.is_float() {
//!     let value: &f64 = mawu_value.as_float().unwrap();
//!     let owned_value: f64 = mawu_value.to_float().unwrap();
//!  // Do something with `value`
//!     assert_eq!(value, &-1.0);
//!     assert_eq!(owned_value, -1.0);
//! } else if mawu_value.is_string() {
//!     let value: &str = mawu_value.as_str().unwrap();
//!     // to_string cannot fail, as any value can be converted to a string
//!     let owned_value: String = mawu_value.to_string();
//!     let referenced_value: &String = mawu_value.as_string().unwrap();
//!     // Do something with `value`, `owned_value` or `referenced_value`
//!     assert_eq!(value, "hello");
//!     assert_eq!(owned_value, "hello".to_string());
//!     assert_eq!(referenced_value, &"hello".to_string());
//! // These are the JSON exclusive types
//! } else if mawu_value.is_array() {
//!     let array: &Vec<MawuValue> = mawu_value.as_array().unwrap();
//!     // to_array cannot fail, as any value can be converted to an array (put simply, used as the first element of a vector)
//!     let owned_array: Vec<MawuValue> = mawu_value.to_array();
//!     // Do something with `array`
//!     assert_eq!(array.len(), 1);
//!     assert_eq!(owned_array.len(), 1);
//! } else if mawu_value.is_object() {
//!     let object: &HashMap<String, MawuValue> = mawu_value.as_object().unwrap();
//!     let owned_object: HashMap<String, MawuValue> = mawu_value.to_object().unwrap();
//!     // Do something with `object`
//!     assert_eq!(object.len(), 1);
//!     assert_eq!(owned_object.len(), 1);
//! // These are the CSV exclusive return types
//! } else if mawu_value.is_csv_array() {
//!     let csv_array: &Vec<Vec<MawuValue>> = mawu_value.as_csv_array().unwrap();
//!     let owned_csv_array: Vec<Vec<MawuValue>> = mawu_value.to_csv_array().unwrap();
//!     // Do something with `csv_array`
//!     assert_eq!(csv_array.len(), 1);
//!     assert_eq!(owned_csv_array.len(), 1);
//! } else if mawu_value.is_csv_object() {
//!     let csv_object: &Vec<HashMap<String, MawuValue>> = mawu_value.as_csv_object().unwrap();
//!     let owned_csv_object: Vec<HashMap<String, MawuValue>> = mawu_value.to_csv_object().unwrap();
//!     // Do something with `csv_object`
//!     assert_eq!(csv_object.len(), 1);
//!     assert_eq!(owned_csv_object.len(), 1);
//! }
//! ```
//!
//! > Chads use `as_{MawuValue}`, just know what kind of data they are getting and know what to do with a reference.
//! >
//! > Normie Kernel devs use `to_{MawuValue}`, need to check what kind of data they are getting and have to clone it anyway.
//!
//! ### Constructing a `MawuValue`
//! `MawuValue` can be constructed from almost any type using the `MawuValue::from` function.
//! `MawuValue::new` and `MawuValue::default` will return a `MawuValue::None`.
//! There also are `MawuValue::new_array` and `MawuValue::new_object` that will return an empty `MawuValue::Array` and `MawuValue::Object`, respectively on the JSON side,
//! and `MawuValue::new_csv_array` and `MawuValue::new_csv_object` that will return an empty `MawuValue::CsvArray` and `MawuValue::CsvObject`, respectively on the CSV side.
//! With these functions, as well as `MawuValue::from(Type::default())`, you can create an empty `MawuValue` of, hopefully, any desired type.
//!
//! > I really recommend using the `MawuValue::from` function. It's simple and shockingly nice to
//! > work with. Please note the CSV section below, as CSV is a very different beast and has to be
//! > handled more explicitly.
//!
//! For example:
//! ```rust
//! use mawu::mawu_value::MawuValue;
//!
//! let mawu_value = MawuValue::from(42);
//! assert_eq!(mawu_value, MawuValue::Int(42));
//!
//! // For vectors you can just pass them into `MawuValue::from`
//! let mut mawu_value = MawuValue::from(vec![1, 2, 3]).to_array();
//! mawu_value.push(MawuValue::from(4));
//! assert_eq!(mawu_value, vec![MawuValue::Int(1), MawuValue::Int(2), MawuValue::Int(3), MawuValue::Int(4)]);
//! ```
//! One thing to note in the above example is that to mutate the array, you have to use `to_array`. This does create a new copy of the array, so if you plan to store several types inside the same array there is this approach:
//! ```rust
//! use mawu::mawu_value::MawuValue;
//!
//! let mut mawu_value = MawuValue::new_array().to_array();
//! mawu_value.push(MawuValue::from(u8::MAX));
//! mawu_value.push(MawuValue::from("hello"));
//! mawu_value.push(MawuValue::from(-3));
//! mawu_value.push(MawuValue::from(4.2));
//! mawu_value.push(MawuValue::from(vec![1, 2]));
//! mawu_value.push(MawuValue::from(true));
//! mawu_value.push(MawuValue::from(""));
//! assert_eq!(mawu_value, vec![MawuValue::Uint(255), MawuValue::String("hello".to_string()), MawuValue::Int(-3), MawuValue::Float(4.2), MawuValue::Array(vec![MawuValue::Int(1), MawuValue::Int(2)]), MawuValue::Bool(true), MawuValue::None]);
//! ```
//! However, I would recommend doing this:
//! ```rust
//! use mawu::mawu_value::MawuValue;
//!
//! let data0 = 1;
//! let data_vec = vec![1, 2];
//! let data_string = "hello";
//! let mawu_value = MawuValue::from(vec![MawuValue::from(data0), MawuValue::from(data_vec), MawuValue::from(data_string)]);
//! assert_eq!(mawu_value, MawuValue::Array(vec![MawuValue::Int(1), MawuValue::Array(vec![MawuValue::Int(1), MawuValue::Int(2)]), MawuValue::String("hello".to_string())]));
//! ```
//!
//! If you are creating an object, please take care that the keys are valid strings (or can be converted to strings, the standards require keys to be strings) and that the values are valid `MawuValue`s or can be converted to `MawuValue`s.
//!
//! You can pass in a vector of tuples of (key, value) to create an object:
//! ```rust
//! use mawu::mawu_value::MawuValue;
//!
//! let vec = vec![("key1", MawuValue::from(u8::MAX)), ("key2", MawuValue::from("hello")), ("key3", MawuValue::from(-3)), ("key4", MawuValue::from(4.2)), ("key5", MawuValue::from(vec![1,2])), ("key6", MawuValue::from(true)), ("key7", MawuValue::from(""))];
//! let object = MawuValue::from(vec);
//! assert_eq!(object.get("key1").unwrap(), &MawuValue::Uint(255));
//! assert_eq!(object.get("key2").unwrap(), &MawuValue::String("hello".to_string()));
//! assert_eq!(object.get("key3").unwrap(), &MawuValue::Int(-3));
//! assert_eq!(object.get("key4").unwrap(), &MawuValue::Float(4.2));
//! assert_eq!(object.get("key5").unwrap(), &MawuValue::Array(vec![MawuValue::Int(1), MawuValue::Int(2)]));
//! assert_eq!(object.get("key6").unwrap(), &MawuValue::Bool(true));
//! assert_eq!(object.get("key7").unwrap(), &MawuValue::None);
//!
//! let another_vec = vec![("key1", 1), ("key2", 2), ("key3", 3), ("key4", 4), ("key5", 5), ("key6", 6), ("key7", 7), ("key8", 8), ("key9", 9), ("key10", 10)];
//! let another_object = MawuValue::from(another_vec);
//! assert_eq!(another_object.get("key1").unwrap(), &MawuValue::Int(1));
//! assert_eq!(another_object.get("key2").unwrap(), &MawuValue::Int(2));
//! assert_eq!(another_object.get("key3").unwrap(), &MawuValue::Int(3));
//! assert_eq!(another_object.get("key4").unwrap(), &MawuValue::Int(4));
//! assert_eq!(another_object.get("key5").unwrap(), &MawuValue::Int(5));
//! assert_eq!(another_object.get("key6").unwrap(), &MawuValue::Int(6));
//! assert_eq!(another_object.get("key7").unwrap(), &MawuValue::Int(7));
//! assert_eq!(another_object.get("key8").unwrap(), &MawuValue::Int(8));
//! assert_eq!(another_object.get("key9").unwrap(), &MawuValue::Int(9));
//! assert_eq!(another_object.get("key10").unwrap(), &MawuValue::Int(10));
//! ```
//!
//! Or you can pass in a hashmap of (key, value) to create an object:
//! ```rust
//! use std::collections::HashMap;
//! use mawu::mawu_value::MawuValue;
//! 
//! let a_hashmap = HashMap::from([
//!     ("key1", MawuValue::from(u8::MAX)),
//!     ("key2", MawuValue::from("hello")),
//!     ("key3", MawuValue::from(-3)),
//!     ("key4", MawuValue::from(4.2)),
//!     ("key5", MawuValue::from(vec![1,2])),
//!     ("key6", MawuValue::from(true)),
//!     ("key7", MawuValue::from("")),
//!     ("key8", MawuValue::None),
//! ]);
//! let mawu_value = MawuValue::from(a_hashmap).to_object().unwrap();
//! assert_eq!(mawu_value.get("key1").unwrap(), &MawuValue::Uint(255));
//! assert_eq!(mawu_value.get("key2").unwrap(), &MawuValue::String("hello".to_string()));
//! assert_eq!(mawu_value.get("key3").unwrap(), &MawuValue::Int(-3));
//! assert_eq!(mawu_value.get("key4").unwrap(), &MawuValue::Float(4.2));
//! assert_eq!(mawu_value.get("key5").unwrap(), &MawuValue::Array(vec![MawuValue::Int(1), MawuValue::Int(2)]));
//! assert_eq!(mawu_value.get("key6").unwrap(), &MawuValue::Bool(true));
//! assert_eq!(mawu_value.get("key7").unwrap(), mawu_value.get("key8").unwrap());
//!
//! let another_hashmap = HashMap::from([
//!     ("key1", 1),
//!     ("key2", 2),
//!     ("key3", 3),
//!     ("key4", 4),
//!     ("key5", 5),
//!     ("key6", 6),
//!     ("key7", 7),
//!     ("key8", 8),
//!     ("key9", 9),
//!     ("key10", 10),
//! ]);
//! let another_mawu_value = MawuValue::from(another_hashmap).to_object().unwrap();
//! assert_eq!(another_mawu_value.get("key1").unwrap(), &MawuValue::Int(1));
//! assert_eq!(another_mawu_value.get("key2").unwrap(), &MawuValue::Int(2));
//! assert_eq!(another_mawu_value.get("key3").unwrap(), &MawuValue::Int(3));
//! assert_eq!(another_mawu_value.get("key4").unwrap(), &MawuValue::Int(4));
//! assert_eq!(another_mawu_value.get("key5").unwrap(), &MawuValue::Int(5));
//! assert_eq!(another_mawu_value.get("key6").unwrap(), &MawuValue::Int(6));
//! assert_eq!(another_mawu_value.get("key7").unwrap(), &MawuValue::Int(7));
//! assert_eq!(another_mawu_value.get("key8").unwrap(), &MawuValue::Int(8));
//! assert_eq!(another_mawu_value.get("key9").unwrap(), &MawuValue::Int(9));
//! assert_eq!(another_mawu_value.get("key10").unwrap(), &MawuValue::Int(10));
//! ```
//!
//! There are more examples in the function docs.
//! 
//! #### Creating a `MawuValue` for CSV data
//! Writing CSV data presents its own challenges, thanks to the differing implementations in use.
//! Because of this, the `MawuValue` for CSV data is a bit different from the other types and needs to be constructed with a bit more help.
//! ```rust
//! use mawu::mawu_value::MawuValue;
//! use mawu::write_pretty;
//! 
//! let path_to_file = "csv_output_pretty.csv";
//! let csv_value = MawuValue::CSVArray(vec![
//!     // each row is a vector of `MawuValue`s, with each element in the vector being a column
//!     vec![
//!         MawuValue::from("a"),
//!         MawuValue::from(1),
//!     ],
//!     vec![
//!         MawuValue::from(-1),
//!         MawuValue::from(true),
//!     ],
//! ]);
//! write_pretty(path_to_file, csv_value, 4).unwrap();
//! ```
//! 
//! To create a headed CSV file, you will need a `Vec<HashMap<String, MawuValue>>` as the value.
//! ```rust
//! use std::collections::HashMap;
//! use mawu::mawu_value::MawuValue;
//! use mawu::write_pretty;
//! 
//! let path_to_file = "csv_output_pretty2.csv";
//! 
//! let row0 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value1")),
//!   ("key2".to_string(), MawuValue::from(2))  
//! ]);
//! let row1 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value2")),
//!   ("key2".to_string(), MawuValue::from(3))  
//! ]);
//! let row2 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value3")),
//!   ("key2".to_string(), MawuValue::from(4))
//! ]);
//! 
//! let csv_value = MawuValue::CSVObject(vec![row0, row1, row2]);
//! 
//! write_pretty(path_to_file, csv_value, 4).unwrap();
//! ```
//! 
//! #### A comprehensive list of all types a `MawuValue` can be constructed from
//! Any of these values can be passed into a `Mawuvalue::from` and will be converted to a valid `MawuValue`
//! - primitives
//!     - numbers
//!          - `u8`, `u16`, `u32`, `u64`, `usize`
//!          - `i8`, `i16`, `i32`, `i64`, `isize`
//!          - `f32`, `f64`
//!     - strings
//!          - `&str`, `String`, `&String`
//!     - booleans
//!          - `true`, `false`
//!     - options
//!          - `Option<T>` where T can be any type that can be converted to `MawuValue`
//! - structures
//!     - arrays
//!          - `Vec<T>` where T can be any type that can be converted to `MawuValue`
//!     - objects
//!          - `Vec<(K, V)>` where K is a string and V is any type that can be converted to `MawuValue`
//!          - `HashMap<K, V>` where K is a string and V is any type that can be converted to `MawuValue`
//!
//! ## `MawuError`
//! `MawuError`'s are the errors that can be returned by `MawuValue`'s methods.
//! There are some `std::io::Error`s that are wrapped in `MawuError`s, they are only used for file
//! handling. E.g. file not found, file already exists, etc.
//! Internal errors, should not be encountered in normal usage, and I kindly ask you to report any
//! thrown internal errors.
//!
//! The largest part is the parsing errors section, split between `JsonError` and `CsvError`. It's
//! dealing with, as the name suggests, parsing errors.
//!
//! The way errors are implemented right now, not a lot of useful data is returned.
//! But I have really no better Idea of how to do it, it's my first real error system after all.
//!
//! For a better understanding of what a `MawuError` is, please refer to the list below.
//!
//! ### A comprehensive list of all `MawuError`s
//! - `MawuError`
//!     - `IoError`
//!         - all possible `std::io::Error`s
//!     - `CsvError`
//!         - `ParseError(CsvParseError)`
//!             - should you encounter this, your special CSV is not compatible with Mawu
//!             - `CsvParseError`
//!                 - `UnescapedDoubleQuote`
//!                 - `UnterminatedQuote`
//!                 - `UnescapedCharacter(char)`
//!                 - `ExtraValue(String)`
//!                 - `UnrecognizedHeader(String)`
//!                 - `UnexpectedNewline`
//!         - `WriteError(CsvWriteError)`
//!             - `NotCSV`
//!             - `UnallowedType(String)`
//!     - `JsonError`
//!         - `ParseError(JsonParseError)`
//!             - should you encounter this, I am certain that your file is not valid JSON
//!             - `JsonParseError`
//!                 - `UnescapedDoubleQuote`
//!                 - `UnterminatedQuote`
//!                 - `UnescapedCharacter(char)`
//!                 - `UnexpectedNewline`
//!                 - `UnexpectedEndOfFile`
//!                 - `UnexpectedCharacter(String)`
//!                 - `InvalidStructuralToken(String)`
//!                 - `InvalidCharacter(String)`
//!                 - `InvalidEscapeSequence(String)`
//!                 - `ExpectedColon`
//!                 - `ExpectedKey`
//!                 - `ExpectedValue`
//!                 - `ExpectedEndOfObject`
//!                 - `InvalidNumber(String)`
//!         - `WriteError(JsonWriteError)`
//!             - `NotJSON`
//!             - `NotJSONType(String)`
//!     - `InternalError`
//!     - should you encounter this, I am certain that there is a bug in Mawu, please report it
//!          - `UnableToLockMasterMutex`
//!          - `StringWithNoChars(String)`
//!          - `UnableToUnescapeUnicode(String)`
//!          - `NotUTF8(String)`
//!
//! ## Reading CSV
//! This library supports CSV files, conforming to the rfc4180 standard and is itself conforming to the rfc4180 standard and nothing else.
//!
//! Please note that CSV, while a standard exists, is seldom implemented as such in practice, and almost every implementation of CSV is not conforming to the rfc4180 standard in some way and thus more or less compatible with each other.
//!
//! One example would be a common shorthand for an array by using `aaa / bbb / ccc` to represent `[aaa, bbb, ccc]`.
//! This is not part of the rfc4180 standard and thus not implemented in Mawu, instead it would be treated as a single string, with the appropriate errors. Meaning that Mawu would produce a `MawuValue::String("[aaa, bbb, ccc]")` instead.
//! `aaa / "bbb" / ccc` would produce an error for example, as Mawu treats the entire thing as one string, but it encounters unescaped double-quotes.
//!
//! Another example is the way encoding is implemented. Mawu uses `utf-8` encoding exclusively for CSV, and does not recognize or produce a `BOM` or similar at the beginning of the file.
//! There are CSV files encoded in `utf-16`, `utf-32` or even some `ASCII`-variants, and there are some more esoteric implementations like the IBM one where you can define new field names in the middle of a CSV file by using `#GROUP_OBJECT_PROFILE#` [learn more](https://www.ibm.com/docs/en/sig-and-i/10.0.2?topic=schedules-example-comma-separated-value-csv-file).
//!
//! Because of this, most if not all CSV files are only supported in the ecosystem or app they were created in, and there is no guarantee that Mawu will be able to parse them correctly.
//!
//! Mawu handles CSV files with an empty or filled last row.
//!
//! > While the usage of the header is optional, you will need to use either the `read_csv_headless(path)`, or the `read_csv_headed(path)` method.
//! > [Learn more.](#csv-usage)
//!
//! ### Handling missing or not provided values
//! The rfc4180 standard allows for missing or not provided values in CSV files only implicitly. There are many different ways libraries have implemented this in the past, and Mawu goes with the closest interpretation the rfc4180 allows.
//! So while Mawu does handle missing or not provided values, it is not 100% reliable.
//! Exactly how this is handled is explained in the following paragraphs.
//!
//! Because of the rfc4180 standard, a missing value in the form of `aaa, ,ccc` would still result in 3 `MawuValue`'s in the form of `[aaa][ ][ccc]` as CSV has significant white space, so the missing `bbb` is converted into a space.
//! A row in the form of `aaa,,ccc` would result in a `MawuValue` of `[aaa][Mawu::None][ccc]` for the same reasons.
//! One last example is the handling of a value of `""` in the middle of a CSV file. This is also part of the rfc4180 standard only implicitly, and sometimes interpreted as an empty string, other times as a missing value.
//! Mawu will treat it as an empty string and uses it as the default for any empty value itself.
//!
//! This library implements missing or not provided values differently depending on if a header is present or not.
//!
//! #### With header
//! If a header is present, the missing values will be filled with a `Mawu::None` Value.
//!
//! A header of `AAA,BBB,CCC`, and the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][MMawu::None]`.
//! With a header of `AAA,BBB,CCC,DDD`, the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::None][Mawu::None]`.
//!
//! Please note that as long as a header is present Mawu will append `Mawu::None` values for as many columns as there are columns declared in the header.
//!
//! #### Without header
//! Should a header be not present, any row ending in a `,` will append as many `Mawu::None` values as there are columns in the first row.
//!
//! The row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::None]` because of the trailing comma without content.
//! A row where the missing value is `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb]` only in the case where it is in the first row.
//! However, the same row of `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb][Mawu::None]` in the case where the first row is `aaa,bbb,ccc`, or as many `Mawu::None` values as there are columns in the first row.
//!
//! ### CSV Return value
//! Mawu will return a `Result<MawuValue, MawuError>`. The wrapped `MawuValue` will have one of two types, depending on if a file with a header is parsed or not.
//!
//! If `Mawu::from_csv_headed(path)` is used, the `MawuValue` will be of type `Vec<Vec<MawuValue>>`, and if `Mawu::from_csv_headless(path)` is used, the `MawuValue` will be of type `Vec<HashMap<String, MawuValue>>`.
//!
//! To get to your data, you will need to iterate over the contents of the `MawuValue` returned. You can do this by calling `as_csv_object()` or `as_csv_array()` on the `MawuValue` returned as appropriate. If you are not sure what the returned value type is, you can check by using `is_csv_object()` or `is_csv_array()`, convenience functions for all types are provided by Mawu.  
//!
//! ### CSV Usage
//! Reading a CSV file and just printing out the values:
//!
//! ```rust
//! use mawu::mawu_value::MawuValue;
//! use std::collections::HashMap;
//! use mawu::read::{csv_headed, csv_headless};
//!
//! let path_to_file = "data/csv/csv-test-data/headed/my-own-random-data/all-types.csv";
//! // for a csv file with header
//! let mawu: Vec<HashMap<String, MawuValue>> = csv_headed(path_to_file).unwrap().to_csv_object().unwrap();
//!
//! // mawu will return a Result<MawuResult, MawuError>
//! for entry in mawu {
//!     for (key, value) in &entry {
//!         println!("{}: {}", key, value);
//!     }
//! }
//!
//! // for a csv file without header
//! let mawu_headless: Vec<Vec<MawuValue>> = csv_headless(path_to_file).unwrap().to_csv_array().unwrap();
//!
//! // mawu will return a Result<MawuResult, MawuError>
//! for entry in mawu_headless {
//!     for value in entry {
//!         println!("{}", value);
//!     }
//! }
//! ```
//!
//! ## Reading JSON
//! This library supports JSON files that conform to the rfc8259 and the ECMA-404 standard.
//! JSON is one of the most used and common file formats used for data interchange. Defined in 2001 by Douglas Crockford, JSON has gone through several editions and has been used in production for over 20 years.
//! Because of the several editions and conciseness of JSON grammar, many aspects are left undefined and the various implementations are not consistent in the way they parse JSON.
//!
//! Mawu is designed to stick as close to the standards as possible, and does not support any common JSON extension like trailing commas.
//! Most edge cases and the way they are handled are explained in the following paragraphs.
//!
//! ### Edge cases
//!
//! #### BOM
//! Mawu does not recognize or produce a `BOM` or similar at the beginning of the file at all and will error out if it encounters one.
//!
//! #### Files
//! If a file should be empty, Mawu will return a `None` value.
//!
//! #### Objects
//! In the rfc8259 standard, a JSON object is a set of key-value pairs where the keys should be unique. As this is not a hard requirement however, JSON parsers have handled this in a number of ways.
//! Mawu will parse JSON objects as a `HashMap<String, MawuValue>` and uses the same behavior for duplicate keys, in that they are replaced with the last value.
//! Because of the same behavior of `HashMap`, Mawu will return JSON objects not in the same order as the JSON file.
//!
//! #### Arrays
//! Ordering of arrays is kept the same as in the JSON file.
//!
//! #### Numbers
//! `Infinity` and `NaN` are explicitly not part of the rfc8259 standard, but are implemented in some parsers. Mawu does not support them at all, and any `NaN` or `Infinity` encountered will error. Should you pass in a string into `MawuValue::from` like "1.0e500000" instead of a float with Infinity (or NaN), you will be returned a `MawuValue::None`.
//! If you want or need to use `NaN` or `Infinity` in your code, you can always just cast them to strings.
//!
//! The rfc8259 doesn't set any limits on the range and precision of numbers, but recommends the implementation of `IEEE 754 binary64`. Because of this recommendation, Mawu supports only 64-bit systems, and all numbers parsed by Mawu are returned in a `_64` type.
//! Should Mawu encounter a number not representable in 64 bits, it will return an error.
//! As any implementor of the standards is free to set its own limits on the range and precision of numbers, Mawu chooses to use the same limits and behaviour of the rust standard library `String.parse()` function.
//! This can be the case for large numbers expressed in exponent notation. For example, `123.456e+350` is not representable in 64-bits (and will return `MawuValue::None`) while `123.456e300` is representable.
//! In the case of `123.456e-350`, the parser of the rust standard library will approximate to `0` and Mawu return `0`.
//!
//! Some numbers supplied as integers, e.g. `123456789e29`, can be converted into `f64` numbers should they be too large to be represented as `u64` but a `f64` can still hold them.
//! As a result of using the rust standard library, precision can be lost.
//!
//! > Any overflow will result in a `MawuValue::None`.
//! > Any underflow will result in a `0`.
//!
//! #### Strings
//! Mawu accepts only UTF-8 encoded files.
//! Escaped UTF-16 surrogate pairs as permitted by the standards are parsed correctly.
//!
//! #### Structure
//! Mawu accepts any amount of nested structures.
//!
//! ### JSON Usage
//! ```rust
//! use mawu::read::json;
//!
//! let path_to_file = "data/json/json-test-data/simple-json.json";
//! let json_value = json(path_to_file).unwrap();
//! for (key, value) in json_value.as_object().unwrap() {
//!     println!("{}: {}", key, value);
//! }
//!
//! ```
//!
//! Given the object:
//! ```json
//! {
//! "key1": "value1",
//! "key2": 1,
//! "key3": -1,
//! "key4": true,
//! "key5": null
//! }
//! ```
//! You can iterate over it as follows:
//! ```rust
//! use mawu::read::json;
//!
//! let path_to_file = "data/json/json-test-data/simple-object.json";
//! let binding = json(path_to_file).unwrap();
//! let json_value = binding.as_object().unwrap();
//! let key1: &str = json_value.get("key1").unwrap().as_str().unwrap();
//! let key2: &u64 = json_value.get("key2").unwrap().as_uint().unwrap();
//! let key3: &i64 = json_value.get("key3").unwrap().as_int().unwrap();
//! let key4: &bool = json_value.get("key4").unwrap().as_bool().unwrap();
//! if json_value.get("key5").unwrap().is_none() {
//!     // Do something
//! }
//! ```
//!
//! A more complex example:
//! ```json
//! {
//! "key1": {
//!     key2": {
//!         "key3": "value3"
//!     }
//! },
//! "key4": "value4",
//! "key5": null,
//! "key6": 6,
//! "key7": true,
//! "key8": -8,
//! "key9": [1, 2, 3]
//! }
//! ```
//! ```rust
//! use mawu::read::json;
//! use mawu::mawu_value::MawuValue;
//!
//! let path_to_file = "data/json/json-test-data/complex-object.json";
//! let binding = json(path_to_file).unwrap();
//! let json_value = binding.as_object().unwrap();
//! let key3: &str = json_value.get("key1").unwrap().as_object().unwrap().get("key2").unwrap().as_object().unwrap().get("key3").unwrap().as_str().unwrap();
//! let key4: &str = json_value.get("key4").unwrap().as_str().unwrap();
//! let key5: &MawuValue = json_value.get("key5").unwrap();
//! let key6: &u64 = json_value.get("key6").unwrap().as_uint().unwrap();
//! let key7: &bool = json_value.get("key7").unwrap().as_bool().unwrap();
//! let key8: &i64 = json_value.get("key8").unwrap().as_int().unwrap();
//! let key9: &Vec<MawuValue> = json_value.get("key9").unwrap().as_array().unwrap();
//! for value in key9 {
//!     println!("{}", value);
//! }
//! ```
//!
//! ## Writing
//! The writing functionality is unified, meaning that there are only two functions:
//! - `write()`
//! - `write_pretty()`
//! 
//! The `write()` function will write the `MawuValue` to a file.
//!
//! The `write_pretty()` function will write the `MawuValue` in a more human-readable formatting
//! with a custom amount of whitespace.
//!
//! Both take in a `MawuValue` and a `Path` to write to.
//! The correct encoding is decided by the `MawuValue` type you pass in.
//! Passing in either `MawuValue::CSVObject` or `MawuValue::CSVArray` will result in an CSV file
//! being written. Passing any other `MawuValue` will result in a JSON file being written.
//!
//! > Please supply the `.json` extension if you want to write a JSON file, or `.csv` if you want to write a CSV file.
//!
//! ### Writing data to disk
//! To write data to disk, you can make use of the `mawu::write()` function. It takes in a `MawuValue` and a `Path` to write to.
//! Depending on the type of `MawuValue` passed in, the correct encoding is decided. `CSVObject`
//! and `CSVArray` are written as CSV files, while any other `MawuValue` is written as JSON.
//! 
//! #### Writing examples
//! ##### JSON
//! ```rust
//! use std::collections::HashMap;
//! use mawu::mawu_value::MawuValue;
//! use mawu::write;
//!
//! let path_to_file = "json_output.json";
//! let data = vec![
//!     MawuValue::from("a"),
//!     MawuValue::from(1),
//!     vec![
//!         MawuValue::from(-1),
//!         MawuValue::from(true),
//!     ].into(),
//! ];
//! let json_value = MawuValue::from(data);
//! write(path_to_file, json_value).unwrap();
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//!
//! ##### CSV
//! ```rust
//! use mawu::write;
//! use mawu::mawu_value::MawuValue;
//!
//! let path_to_file = "csv_output.csv";
//! let csv_value = MawuValue::CSVArray(vec![
//!     vec![
//!         MawuValue::from("a"),
//!         MawuValue::from(1),
//!     ],
//!     vec![
//!         MawuValue::from(-1),
//!         MawuValue::from(true),
//!     ],
//! ]);
//! write(path_to_file, csv_value).unwrap();
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//! ```rust
//! use std::collections::HashMap;
//! use mawu::mawu_value::MawuValue;
//! use mawu::write;
//! 
//! let path_to_file = "csv_output_2.csv";
//!
//! let row0 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value1")),
//!   ("key2".to_string(), MawuValue::from(2))  
//! ]);
//! let row1 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value2")),
//!   ("key2".to_string(), MawuValue::from(3))  
//! ]);
//! let row2 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value3")),
//!   ("key2".to_string(), MawuValue::from(4))
//! ]);
//! 
//! let csv_value = MawuValue::CSVObject(vec![row0, row1, row2]);
//!
//! write(path_to_file, csv_value).unwrap();
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//!
//! ### Writing pretty data to disk
//! To write formatted data to disk, you can make use of the `mawu::write_pretty()` function. Like the `write()` function it takes in a `MawuValue` and a `Path` to write to, but it also takes in a `spaces` argument, where you define the amount of whitespace that is used for indenting the different rows.
//! 
//! Depending on the type of `MawuValue` passed in, the correct encoding is decided. `CSVObject`
//! and `CSVArray` are written as CSV files, while any other `MawuValue` is written as JSON.
//!
//! #### Writing pretty examples
//! ##### JSON
//! ```rust
//! use mawu::mawu_value::MawuValue;
//! use mawu::write_pretty;
//!
//! let path_to_file = "json_output_pretty.json";
//! let mut json_value = MawuValue::new_object().to_object().unwrap();
//! json_value.insert("key1".to_string(), MawuValue::from("value1"));
//! json_value.insert("key2".to_string(), MawuValue::from(2));
//! write_pretty(path_to_file, json_value, 4).unwrap();
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//!
//! ##### CSV
//! ```rust
//! use mawu::mawu_value::MawuValue;
//! use mawu::write_pretty;
//!
//! let path_to_file = "csv_output_pretty.csv";
//! let csv_value = MawuValue::CSVArray(vec![
//!     vec![
//!         MawuValue::from("a"),
//!         MawuValue::from(1),
//!     ],
//!     vec![
//!         MawuValue::from(-1),
//!         MawuValue::from(true),
//!     ],
//! ]);
//! write_pretty(path_to_file, csv_value, 4).unwrap();
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```
//! ```rust
//! use std::collections::HashMap;
//! use mawu::mawu_value::MawuValue;
//! use mawu::write_pretty;
//! 
//! let path_to_file = "csv_output_pretty2.csv";
//!
//! let row0 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value1")),
//!   ("key2".to_string(), MawuValue::from(2))  
//! ]);
//! let row1 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value2")),
//!   ("key2".to_string(), MawuValue::from(3))  
//! ]);
//! let row2 = HashMap::from([
//!   ("key1".to_string(), MawuValue::from("value3")),
//!   ("key2".to_string(), MawuValue::from(4))
//! ]);
//! 
//! let csv_value = MawuValue::CSVObject(vec![row0, row1, row2]);
//!
//! write_pretty(path_to_file, csv_value, 4).unwrap();
//!
//! # // Cleaning up, as `cargo test` actually creates the file on disc during testing
//! # std::fs::remove_file(path_to_file).unwrap();
//! ```

/// Contains all the errors that can be returned by Mawu
pub mod errors;
/// Contains a wrapper for all data values supported by Mawu
pub mod mawu_value;
/// Contains all the lexers for CSV and JSON files
mod lexers;
/// Contains all the serializers for CSV and JSON files
mod serializers;
/// Contains all utility functions
mod utils;

/// Reads CSV and JSON files into `MawuValue`
pub mod read {
    use std::path::Path;

    use crate::{
        errors::MawuError,
        lexers::{csv_lexer, json_lexer},
        mawu_value::MawuValue,
        utils::file_handling,
    };

    /// Reads a headed CSV file and returns a `MawuValue::CSVObject` or an error if the file could not be read or parsed.
    ///
    /// Call `as_csv_object` or `to_csv_object` on the result to get the `Vec<HashMap<String, MawuValue>>`
    ///
    /// # Arguments
    /// * `path` - The path to the CSV file, relative or absolute
    ///
    /// # Example
    /// ```rust
    /// use mawu::read::csv_headed;
    /// let path_to_file = "data/csv/csv-test-data/headed/my-own-random-data/all-types.csv";
    /// let csv_value = csv_headed(path_to_file).unwrap();
    /// ```
    ///
    /// # Errors
    /// Only returns `MawuError`'s
    pub fn csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headed(
            file_handling::read_file(path)?
        )
    }

    /// Reads a headless CSV file and returns a `MawuValue::CSVArray` or an error if the file could not be read or parsed.
    ///
    /// Call `as_csv_array` or `to_csv_array` on the result to get the `Vec<Vec<MawuValue>>`
    ///
    /// # Arguments
    /// * `path` - The path to the CSV file, relative or absolute
    ///
    /// # Example
    /// ```rust
    /// use mawu::read::csv_headless;
    /// let path_to_file = "data/csv/csv-test-data/headless/my-own-random-data/all-types.csv";
    /// let csv_value = csv_headless(path_to_file).unwrap();
    /// ```
    ///
    /// # Errors
    /// Only returns `MawuError`'s
    pub fn csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headless(
            file_handling::read_file(path)?
        )
    }

    /// Reads a JSON file and returns a `MawuValue` or an error if the file could not be read or parsed.
    ///
    /// Call the appropriate `as_` or `to_` methods on the result to get the appropriate type
    /// You can check the type with the `is_` methods
    ///
    /// # Arguments
    /// * `path` - The path to the JSON file, relative or absolute
    ///
    /// # Example
    /// ```rust
    /// use mawu::read::json;
    /// let path_to_file = "data/json/json-test-data/complex-object.json";
    /// let json_value = json(path_to_file).unwrap();
    /// ```
    ///
    /// # Errors
    /// Only returns `MawuError`'s
    pub fn json<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        json_lexer::json_lexer(
            file_handling::read_file(path)?
        )
    }
}

use std::path::Path;
use crate::{errors::MawuError, mawu_value::MawuValue, serializers::{csv_serializer, json_serializer}, utils::file_handling::write_file};

/// Writes a file with the given contents.
/// Writes a CSV-file if the contents are a `MawuValue::CSVObject` our `MawuValue::CSVArray` and a JSON-file if the contents are any other `MawuValue`.
///
/// ## Arguments
/// * `path` - The path to the JSON file, relative or absolute
/// * `contents` - The contents of the JSON file, can be any `MawuValue` or value that can be converted to a `MawuValue`
///
/// ## Example
/// ### JSON
/// ```rust
/// use std::collections::HashMap;
/// use mawu::mawu_value::MawuValue;
/// use mawu::write;
///
/// let path_to_file = "json_output.json";
/// let data = vec![
///     MawuValue::from("a"),
///     MawuValue::from(1),
///     vec![
///         MawuValue::from(-1),
///         MawuValue::from(true),
///     ].into(),
/// ];
/// let json_value = MawuValue::from(data);
/// write(path_to_file, json_value).unwrap();
///
/// # // Cleaning up, as `cargo test` actually creates the file on disc during testing
/// # std::fs::remove_file(path_to_file).unwrap();
/// ```
/// ### CSV
/// ```rust
/// use mawu::write;
/// use mawu::mawu_value::MawuValue;
///
/// let path_to_file = "csv_output.csv";
/// let csv_value = MawuValue::CSVArray(vec![
///     vec![
///         MawuValue::from("a"),
///         MawuValue::from(1),
///     ],
///     vec![
///         MawuValue::from(-1),
///         MawuValue::from(true),
///     ],
/// ]);
/// write(path_to_file, csv_value).unwrap();
///
/// # // Cleaning up, as `cargo test` actually creates the file on disc during testing
/// # std::fs::remove_file(path_to_file).unwrap();
/// ```
/// ```rust
/// use std::collections::HashMap;
/// use mawu::mawu_value::MawuValue;
/// use mawu::write;
/// 
/// let path_to_file = "csv_output_2.csv";
///
/// let row0 = HashMap::from([
///   ("key1".to_string(), MawuValue::from("value1")),
///   ("key2".to_string(), MawuValue::from(2))  
/// ]);
/// let row1 = HashMap::from([
///   ("key1".to_string(), MawuValue::from("value2")),
///   ("key2".to_string(), MawuValue::from(3))  
/// ]);
/// let row2 = HashMap::from([
///   ("key1".to_string(), MawuValue::from("value3")),
///   ("key2".to_string(), MawuValue::from(4))
/// ]);
/// 
/// let csv_value = MawuValue::CSVObject(vec![row0, row1, row2]);
///
/// write(path_to_file, csv_value).unwrap();
///
/// # // Cleaning up, as `cargo test` actually creates the file on disc during testing
/// # std::fs::remove_file(path_to_file).unwrap();
/// ```
pub fn write<T: AsRef<Path>, C: Into<MawuValue>>(path: T, contents: C) -> Result<(), MawuError> {
    write_pretty(path, contents, 0)
}

/// Writes a pretty printed file with the given contents.
/// Writes a CSV-file if the contents are a `MawuValue::CSVObject` our `MawuValue::CSVArray` and a JSON-file if the contents are any other `MawuValue`.
///
/// ## Arguments
/// * `path` - The path to the file, relative or absolute
/// * `contents` - The contents of the file, can be any `MawuValue` or value that can be converted to a `MawuValue`
/// * `space` - The number of spaces to use for indentation
///
/// ## Examples
/// ### JSON
/// ```rust
/// use mawu::mawu_value::MawuValue;
/// use mawu::write_pretty;
///
/// let path_to_file = "json_output_pretty.json";
/// let mut json_value = MawuValue::new_object().to_object().unwrap();
/// json_value.insert("key1".to_string(), MawuValue::from("value1"));
/// json_value.insert("key2".to_string(), MawuValue::from(2));
/// write_pretty(path_to_file, json_value, 4).unwrap();
///
/// # // Cleaning up, as `cargo test` actually creates the file on disc during testing
/// # std::fs::remove_file(path_to_file).unwrap();
/// ```
/// ### CSV
/// ```rust
/// use mawu::mawu_value::MawuValue;
/// use mawu::write_pretty;
///
/// let path_to_file = "csv_output_pretty.csv";
/// let csv_value = MawuValue::CSVArray(vec![
///     vec![
///         MawuValue::from("a"),
///         MawuValue::from(1),
///     ],
///     vec![
///         MawuValue::from(-1),
///         MawuValue::from(true),
///     ],
/// ]);
/// write_pretty(path_to_file, csv_value, 4).unwrap();
///
/// # // Cleaning up, as `cargo test` actually creates the file on disc during testing
/// # std::fs::remove_file(path_to_file).unwrap();
/// ```
/// ```rust
/// use std::collections::HashMap;
/// use mawu::mawu_value::MawuValue;
/// use mawu::write_pretty;
/// 
/// let path_to_file = "csv_output_pretty2.csv";
///
/// let row0 = HashMap::from([
///   ("key1".to_string(), MawuValue::from("value1")),
///   ("key2".to_string(), MawuValue::from(2))  
/// ]);
/// let row1 = HashMap::from([
///   ("key1".to_string(), MawuValue::from("value2")),
///   ("key2".to_string(), MawuValue::from(3))  
/// ]);
/// let row2 = HashMap::from([
///   ("key1".to_string(), MawuValue::from("value3")),
///   ("key2".to_string(), MawuValue::from(4))
/// ]);
/// 
/// let csv_value = MawuValue::CSVObject(vec![row0, row1, row2]);
///
/// write_pretty(path_to_file, csv_value, 4).unwrap();
///
/// # // Cleaning up, as `cargo test` actually creates the file on disc during testing
/// # std::fs::remove_file(path_to_file).unwrap();
/// ```
pub fn write_pretty<T: AsRef<Path>, C: Into<MawuValue>>(path: T, contents: C, spaces: u8) -> Result<(), MawuError> {
    let contents = contents.into();
    match contents {
        MawuValue::CSVObject(v) => write_file(path, csv_serializer::serialize_csv_headed(MawuValue::CSVObject(v.clone()), spaces)?),
        MawuValue::CSVArray(v) => write_file(path, csv_serializer::serialize_csv_unheaded(MawuValue::CSVArray(v.clone()), spaces)?),
        _ => write_file(path, json_serializer::serialize_json(contents, spaces, 0)?),
    }
}

#[test]
fn write_json_doc_files() {
    let path_to_file1 = "json_output_pretty.json";
    let path_to_file2 = "json_output.json";
    if std::fs::File::open(path_to_file1).is_err() {
        let path_to_file = "json_output_pretty.json";
        let mut json_value = mawu_value::MawuValue::new_object().to_object().unwrap();
        json_value.insert("key1".to_string(), mawu_value::MawuValue::from("value1"));
        json_value.insert("key2".to_string(), mawu_value::MawuValue::from(2));
        crate::write_pretty(path_to_file, json_value, 4).expect("Failed to write JSON file");
    }
    if std::fs::File::open(path_to_file2).is_err() {
        let path_to_file = "json_output.json";
        let data = vec![
            mawu_value::MawuValue::from("a"),
            mawu_value::MawuValue::from(1),
            vec![
                mawu_value::MawuValue::from(-1),
                mawu_value::MawuValue::from(true),
            ].into(),
        ];
        let json_value = mawu_value::MawuValue::from(data);
        crate::write(path_to_file, json_value).expect("Failed to write JSON file");
    }
    let json_value1 = read::json(path_to_file1).unwrap();
    let json_value2 = read::json(path_to_file2).unwrap();

    assert!(json_value1.is_object());
    assert_eq!(json_value1.get("key1").unwrap().as_str().unwrap(), "value1");
    assert_eq!(json_value1.get("key2").unwrap().as_uint().unwrap(), &2);

    assert!(json_value2.is_array());
    assert_eq!(json_value2.len(), 3);
    assert_eq!(json_value2.array_peek(0).unwrap().as_str().unwrap(), "a");
    assert_eq!(json_value2.array_peek(1).unwrap().as_uint().unwrap(), &1);
    assert!(json_value2.array_peek(2).unwrap().is_array());
    assert_eq!(json_value2.array_peek(2).unwrap().len(), 2);
    assert_eq!(json_value2.array_peek(2).unwrap().array_peek(0).unwrap().as_int().unwrap(), &-1);
    assert_eq!(json_value2.array_peek(2).unwrap().array_peek(1).unwrap().as_bool().unwrap(), &true);

    // remove the test files
    std::fs::remove_file(path_to_file1).unwrap();
    std::fs::remove_file(path_to_file2).unwrap();
}

#[test]
fn write_json() {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use crate::mawu_value::MawuValue;

    let path_to_file = "json_output_pretty2.json";
    let mut json_value = MawuValue::new_object().to_object().unwrap();
    json_value.insert("key1".to_string(), MawuValue::from("value1"));
    json_value.insert("key2".to_string(), MawuValue::from(u8::from(2)));
    crate::write_pretty(path_to_file, json_value, 4).unwrap();
    // parse output
    let pretty_output = read::json(path_to_file).unwrap();
    assert!(pretty_output.is_object());
    assert_eq!(pretty_output.get("key1").unwrap().as_str().unwrap(), "value1");
    assert_eq!(pretty_output.get("key2").unwrap().as_uint().unwrap(), &2);

    let filepath = "json_output2.json";
    let json_value = MawuValue::from(vec![
        MawuValue::from("hello"),
        MawuValue::from("world"),
        MawuValue::from(42),
        MawuValue::from(3.14),
        MawuValue::from(HashMap::from([
            ("key1".to_string(), MawuValue::from("value1")),
            ("key2".to_string(), MawuValue::from(u8::from(2))),
        ])),
        MawuValue::from(vec![
            MawuValue::from("hello"),
            MawuValue::from("world"),
            MawuValue::from(42),
            MawuValue::from(3.14),
            MawuValue::from(HashMap::from([
                ("key1".to_string(), MawuValue::from("value1")),
                ("key2".to_string(), MawuValue::from(u8::from(2))),
            ])),
        ]),
    ]);
    crate::write_pretty(filepath, json_value, 4).unwrap();
    // again parse output
    let large_output: MawuValue = read::json(filepath).unwrap();

    assert_eq!(large_output.is_array(), true);
    assert_eq!(large_output.len(), 6);
    assert_eq!(large_output.as_array().unwrap().len(), 6);

    assert_eq!(large_output.as_array().unwrap()[0].as_str().unwrap(), "hello");
    assert_eq!(large_output.as_array().unwrap()[1].as_str().unwrap(), "world");
    assert_eq!(large_output.as_array().unwrap()[2].as_uint().unwrap(), &42);
    assert_eq!(large_output.as_array().unwrap()[3].as_float().unwrap(), &3.14);

    assert!(large_output.as_array().unwrap()[4].is_object());
    assert_eq!(large_output.as_array().unwrap()[5].len(), 5);

    // remove the test files
    std::fs::remove_file(path_to_file).unwrap();
    std::fs::remove_file(filepath).unwrap();
}

#[test]
fn write_csv() {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    use crate::mawu_value::MawuValue;
    use crate::write;
    use crate::write_pretty;
    
    // test headed
    let path_to_file = "csv_output_pretty2.csv";

    let row0 = HashMap::from([
      ("key1".to_string(), MawuValue::from("value1")),
      ("key2".to_string(), MawuValue::from(u8::from(2)))  
    ]);
    let row1 = HashMap::from([
      ("key1".to_string(), MawuValue::from("value2")),
      ("key2".to_string(), MawuValue::from(u8::from(3)))  
    ]);
    let row2 = HashMap::from([
      ("key1".to_string(), MawuValue::from("value3")),
      ("key2".to_string(), MawuValue::from(u8::from(4)))
    ]);
    
    let csv_value = MawuValue::CSVObject(vec![row0, row1, row2]);

    write_pretty(path_to_file, csv_value.clone(), 4).unwrap();
    // lets parse the output and make sure it is correct
    let pretty_output = read::csv_headed(path_to_file);
    assert!(pretty_output.is_ok());
    let pretty_output_bind = pretty_output.unwrap();
    assert_eq!(pretty_output_bind.as_csv_object().unwrap()[0].get("key1").unwrap(), &MawuValue::from("value1"));
    assert_eq!(pretty_output_bind.as_csv_object().unwrap()[0].get("key2").unwrap(), &MawuValue::from(u8::from(2)));
    assert_eq!(pretty_output_bind.as_csv_object().unwrap()[1].get("key1").unwrap(), &MawuValue::from("value2"));
    assert_eq!(pretty_output_bind.as_csv_object().unwrap()[1].get("key2").unwrap(), &MawuValue::from(u8::from(3)));
    assert_eq!(pretty_output_bind.as_csv_object().unwrap()[2].get("key1").unwrap(), &MawuValue::from("value3"));
    assert_eq!(pretty_output_bind.as_csv_object().unwrap()[2].get("key2").unwrap(), &MawuValue::from(u8::from(4)));
    // test headless
    let filepath = "csv_output2.csv";
    let csv_value_headless = MawuValue::CSVArray(vec![
        vec![
            MawuValue::from("a"),
            MawuValue::from(u8::from(1)),
        ],
        vec![
            MawuValue::from(-1),
            MawuValue::from(true),
        ],
    ]);
    write(filepath, csv_value_headless.clone()).unwrap();
    // again parse output
    let output = read::csv_headless(filepath);
    assert!(output.is_ok());
    let output_bind = output.unwrap();
    assert_eq!(output_bind.as_csv_array().unwrap()[0][0], MawuValue::from("a"));
    assert_eq!(output_bind.as_csv_array().unwrap()[0][1], MawuValue::from(u8::from(1)));
    assert_eq!(output_bind.as_csv_array().unwrap()[1][0], MawuValue::from(-1));
    assert_eq!(output_bind.as_csv_array().unwrap()[1][1], MawuValue::from(true));

    // Cleaning up the filesystem
    std::fs::remove_file(path_to_file).unwrap();
    std::fs::remove_file(filepath).unwrap();
}
