# Mawu
A simple JSON and CSV parsing rust library.

Mawu supports 64bit systems only.

Mawu, named after the ancient creator goddess Mawu in West African mythology, offers a simple yet robust and reliable JSON and CSV parsing library implementing the rfc4180, rfc8259 and the ECMA-404 standard. 
While not a zero dependency library, its only dependency is `unicode-segmentation`.

A little technical note: While Mawu uses the same return value types for both CSV and JSON, the parsing is done by two different lexers (or implementors as the standards like to call it) bundled together into one library. If you only use the JSON parser, this results in a bloat of almost 8kb!

> [!IMPORTANT]
> This is a hobbyist repo reinventing the wheel and probably not ready for production use.

## Features
- Simple
- Type aware
- Supports both CSV and JSON
    - CSV
        - With or without header, and data shift is more likely to occur
        - settable delimiter
- Supports missing or not provided values
- Fully documented
    - Handling of edge cases is explained in the documentation
- Tries to stay as close to the rfc4180, rfc8259 and ECMA-404 standard as possible for maximum interoperability
- Actually written by a human

## Overview
- [Mawu](#mawu)
    - [Features](#features)
    - [Overview](#overview)
    - [Naming the Creation: A Legacy of the Divine](#naming-the-creation-a-legacy-of-the-divine)
    - [`MawuValue`](#mawuvalue)
        - [An exhaustive list of all `MawuValue`'s](#an-exhaustive-list-of-all-mawuvalue's)
        - [Convenience functions](#convenience-functions)
        - [Example of getting a `MawuValue` if its type is not known or different in the same field](#example-of-getting-a-mawuvalue-if-its-type-is-not-known-or-different-in-the-same-field)
    - [CSV](#csv)
        - [Handling missing or not provided values](#handling-missing-or-not-provided-values)
            - [With header](#with-header)
            - [Without header](#without-header)
        - [CSV Return value](#csv-return-value)
        - [CSV Usage](#csv-usage)
    - [JSON](#json)
        - [Edge cases](#edge-cases)
            - [Objects](#objects)
            - [Arrays](#arrays)
            - [Numbers](#numbers)
            - [Structure](#structure)

## Naming the creation: A Legacy of the Divine
The name "Mawu" isn't chosen by chance, it honors the powerful West African goddess associated with the moon, the sun, and creation itself.
Mawu follows the long tradition of naming things after deities.

Just as Mawu, the goddess, is linked to creation, Mawu, the library, empowers you to create new things from raw data.  JSON and CSV files are like raw materials, and Mawu provides the tools to shape them into meaningful structures, ready to be used for analysis, manipulation, and ultimately, new creations.

## `MawuValue`
Mawu uses the `MawuValue` enum to represent the different types of values that can be found in JSON and CSV files.

Both the CSV parser and the JSON parser use a different subset of this enum to represent the different types of values.
The difference is slight however, as only the `array` and `object` are different at all, and are represented as `MawuValue::CsvArray` and `MawuValue::CsvObject` for the CSV parser, and `Mawu::Array` and `Mawu::Object` for the JSON parser.
The `CsvArray` and `CsvObject` types are only ever used by the CSV parser as return values. `CsvArray` is used to return a headless CSV file, and `CsvObject` is used to return a headed CSV file.

Mawu supports only 64-bit systems, and all numbers parsed by Mawu are returned in a `_64` type, e.g. `u64` or `f64`.

### Convenience Functions
Convenience functions for all types are provided by Mawu, in the form of `is_{MawuValue}`, `as_{MawuValue}` and `to_{MawuValue}` functions.

Calling `is_` will return `true` if the value is the type requested, and `false` otherwise. This can be useful if you have different data-types in the same array.
`is_true`, `is_false` and `is_null` are convenience functions to check if the value is a boolean and `true`, if the value is a boolean and `false`, or if the value is `None`, respectively and can be used in logic without any further processing or allocating needed.

When you call any `as_` or `to_` function on a `MawuValue` you are returned a `Option()` wrapping the desired value, or `None` if the value is not the type requested. 
Calling `as_null` or `to_null` will return `None` instead when the value is none, and `Some()` wrapping nothing otherwise.

All `as_` functions return a `Option<&MawuValue>`, a pointer to the underlying data. These functions are stricter than `to_`, and will only return a value if it was parsed as such.

All `to_` functions however return a `Option<MawuValue>`, a freshly cloned copy of the underlying data. These functions are less strict than `as_`, and will return a value if it was parsed as such OR can be converted into one. So calling `to_string` on any other type will return a String, built from the underlying data. `to_` functions only return `None` if the value could not be represented as that type.

If you are going to clone the data anyway, you can call `to_` directly. Should you call the right `to_` function on the right type, (`to_float` on a `f64` for example) no conversion checks will be done, but you could call `to_string()` on everything and parse the values yourself if you wanted to, with the added overhead of parsing the data, re-encoding it into a String and then parsing it again. I don't think you should, but you could.



### An exhaustive list of all `MawuValue`'s
- Primitive types
    - `MawuValue::None`
        - can only ever be none, is wrapping nothing
    - `MawuValue::Bool`
        - wrapping a `bool`
    - `MawuValue::Uint`
        - wrapping a `u64`
    - `MawuValue::Int`
        - wrapping a `i64`
    - `MawuValue::Float`
        - wrapping a `f64`
    - `MawuValue::String`
        - wrapping a `String`
- JSON exclusive types
    - `MawuValue::Array`
        - wrapping a `Vec<MawuValue>`
    - `MawuValue::Object`
        - wrapping a `HashMap<String, MawuValue>`
- CSV exclusive types
    - `MawuValue::CsvArray`
        - wrapping a `Vec<Vec<MawuValue>>`
    - `MawuValue::CsvObject`
        - wrapping a `Vec<HashMap<String, Vec<MawuValue>>>`

#### Example of getting a `MawuValue` if its type is not known or different in the same field
```rust
// These are the primitive types
if mawu_value.is_none() {
    let value: Option<()> = mawu_value.as_none().unwrap();
    // Do something with `value`
    assert_eq!(value, None);
} else if mawu_value.is_bool() {
    let value: &bool = mawu_value.as_bool().unwrap();
    // Do something with `value`
    assert_eq!(value, &true);
} else if mawu_value.is_uint() {
    let value: &u64 = mawu_value.as_uint().unwrap();
    // Do something with `value`
    assert_eq!(value, &1);
} else if mawu_value.is_int() {
    let value: &i64 = mawu_value.as_int().unwrap();
    // Do something with `value`
    assert_eq!(value, &-1);
} else if mawu_value.is_float() {
    let value: &f64 = mawu_value.as_float().unwrap();
    // Do something with `value`
    assert_eq!(value, &-1.0);
} else if mawu_value.is_string() {
    let value: &str = value.as_str().unwrap();
    let owned_value: String = mawu_value.to_string().unwrap();
    let referenced_value: &String = mawu_value.as_string().unwrap();
    // Do something with `value`, `owned_value` or `referenced_value`
    assert_eq!(value, "hello");
    assert_eq!(owned_value, "hello".to_string());
    assert_eq!(referenced_value, &"hello".to_string());
// These are the JSON exclusive types
} else if mawu_value.is_array() {
    let array: &Vec<MawuValue> = mawu_value.as_array().unwrap();
    // Do something with `array`
    assert_eq!(array.len(), 1);
} else if mawu_value.is_object() {
    let object: &HashMap<String, MawuValue> = mawu_value.as_object().unwrap();
    // Do something with `object`
    assert_eq!(object.len(), 1);
// These are the CSV exclusive return types
} else if mawu_value.is_csv_array() {
    let csv_array: &Vec<Vec<MawuValue>> = mawu_value.as_csv_array().unwrap();
    // Do something with `csv_array`
    assert_eq!(csv_array.len(), 1);
} else if mawu_value.is_csv_object() {
    let csv_object: &Vec<HashMap<String, MawuValue>> = mawu_value.as_csv_object().unwrap();
    // Do something with `csv_object`
    assert_eq!(csv_object.len(), 1);
}

```

> [!note]
> Chads use `as_{MawuValue}`, just know what kind of data they are getting and know what to do with a reference.
>
> Normie Kernel devs use `to_{MawuValue}`, need to check what kind of data they are getting and have to clone it anyway.

## CSV
This library supports CSV files, conforming to the rfc4180 standard and is itself conforming to the rfc4180 standard and nothing else.

Please note that CSV, while a standard exists, is seldom implemented as such in practice, and almost every implementation of CSV is not conforming to the rfc4180 standard in some way and thus more or less compatible with each other.

One example would be a common shorthand for an array by using `aaa / bbb / ccc` to represent `[aaa, bbb, ccc]`. 
This is not part of the rfc4180 standard and thus not implemented in Mawu, instead it would be treated as a single string, with the appropriate errors.
`aaa / "bbb" / ccc` would produce an error for example, as Mawu treats the entire thing as one string, but it encounters unescaped double-quotes.

Another example is the way encoding is implemented. Mawu uses `utf-8` encoding exclusively for CSV, and does not recognize or produce a `BOM` or similar at the beginning of the file.
There are CSV files encoded in `utf-16`, `utf-32` or even some `ASCII`-variants, and there are some more esoteric implementations like the IBM one where you can define new field names in the middle of a CSV file by using `#GROUP_OBJECT_PROFILE#` [learn more](https://www.ibm.com/docs/en/sig-and-i/10.0.2?topic=schedules-example-comma-separated-value-csv-file).

Because of this, most if not all CSV files are only supported in the ecosystem or app they were created in, and there is no guarantee that Mawu will be able to parse them correctly.

Mawu handles CSV files with an empty or filled last row.

> [!NOTE]
> While the usage of the header is optional, you will need to use either the `read_csv_headless(path)`, or the `read_csv_headed(path)` method.
> [Learn more.](#usage)

### Handling missing or not provided values
The rfc4180 standard allows for missing or not provided values in CSV files only implicitly. There are many different ways libraries have implemented this in the past, and Mawu goes with the closest interpretation the rfc4180 allows.
So while Mawu does handle missing or not provided values, it is, and cannot ever be, 100% reliable.
Exactly how this is handled is explained in the following paragraphs.

Because of the rfc4180 standard, a missing value in the form of `aaa, ,ccc` would still result in 3 `MawuValue`'s in the form of `[aaa][ ][ccc]` as CSV has significant white space, so the missing `bbb` is converted into a space.
A row in the form of `aaa,,ccc` would result in a `MawuValue` of `[aaa][Mawu::None][ccc]` for the same reasons.
One last example is the handling of a value of `""` in the middle of a CSV file. This is also part of the rfc4180 standard only implicitly, and sometimes interpreted as an empty string, other times as a missing value.
Mawu will treat it as an empty string and uses it as the default for any empty value itself.

This library implements missing or not provided values differently depending on if a header is present or not.

#### With header
If a header is present, the missing values will be filled with a `Mawu::None` Value.

A header of `AAA,BBB,CCC`, and the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][MMawu::None]`.
With a header of `AAA,BBB,CCC,DDD`, the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::None][Mawu::None]`.

Please note that as long as a header is present Mawu will append `Mawu::None` values for as many columns as there are columns declared in the header.


#### Without header
Should a header be not present, any row ending in a `,` will append as many `Mawu::None` values as there are columns in the first row.

The row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::None]` because of the trailing comma without content.
A row where the missing value is `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb]` only in the case where it is in the first row.
However, the same row of `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb][Mawu::None]` in the case where the first row is `aaa,bbb,ccc`, or as many `Mawu::None` values as there are columns in the first row.

### CSV Return value
Mawu will return a `Result<MawuValue, MawuError>`. The wrapped `MawuValue` will have one of two types, depending on if a file with a header is parsed or not.

If `Mawu::from_csv_headed(path)` is used, the `MawuValue` will be of type `Vec<Vec<MawuValue>>`, and if `Mawu::from_csv_headless(path)` is used, the `MawuValue` will be of type `Vec<HashMap<String, MawuValue>>`.

To get to your data, you will need to iterate over the contents of the `MawuValue` returned. You can do this by calling `as_csv_object()` or `as_csv_array()` on the `MawuValue` returned as appropriate. If you are not sure what the returned value type is, you can check by using `is_csv_object()` or `is_csv_array()`, convenience functions for all types are provided by Mawu.  

### CSV Usage
Reading a CSV file and just printing out the values:

```rust

fn main() {
    // for a csv file with header
    let mawu: Vec<HashMap<String, MawuValue>> = mawu::csv::read_csv_headed("/path/to/file.csv").unwrap();

    // mawu will return a Result<MawuResult, MawuError>
    for entry in mawu.as_csv_object().unwrap() {
        for (key, value) in &entry {
            println!("{}: {}", key, value);
        }
    }

    // for a csv file without header
    let mawu_headless: Vec<Vec<MawuValue>> = mawu::csv::read_csv_headless("/path/to/file.csv").unwrap();

    // mawu will return a Result<MawuResult, MawuError>
    for entry in mawu_headless.as_csv_array().unwrap() {
        for value in entry {
            println!("{}", value);
        }
    }

}
```

## JSON
This library supports JSON files that conform to the rfc8259 and the ECMA-404 standard.
JSON is one of the most used and common file formats used for data interchange. Defined in 2001 by Douglas Crockford, JSON has gone through several editions and has been used in production for over 20 years.
Because of the several editions and conciseness of JSON grammar, many aspects are left undefined and the various implementations are not consistent in the way they parse JSON.

Mawu is designed to stick as close to the standards as possible, and does not support any common JSON extension like trailing commas.
Most edge cases and the way they are handled are explained in the following paragraphs.

### Edge cases

#### Files
If a file should be empty, Mawu will return a `None` value.

#### Objects
In the rfc8259 standard, a JSON object is a set of key-value pairs where the keys should be unique. As this is not a hard requirement however, JSON parsers have handled this in a number of ways.
Mawu will parse JSON objects as a `HashMap<String, MawuValue>` and uses the same behavior for duplicate keys, in that they are replaced with the last value.
Because of the same behavior, Mawu will return JSON objects not in the same order as the JSON file.

#### Arrays
Ordering of arrays is kept.

#### Numbers
`Infinity` and `NaN` are explicitly not part of the rfc8259 standard, but are implemented in some parsers. Mawu does not support them at all.

The rfc8259 doesn't set any limits on the range and precision of numbers, but recommends the implementation of `IEEE 754 binary64`. Because of this recommendation, Mawu supports only 64-bit systems, and all numbers parsed by Mawu are returned in a `_64` type.
Should Mawu encounter a number not representable in 64 bits, it will return an error.
As any implementor of the standards is free to set its own limits on the range and precision of numbers, Mawu chooses to use the same limits and behaviour of the rust standard library `String.parse()` function.
This can be the case for large numbers expressed in exponent notation. For example, `123.456e+350` is not representable in 64-bits (and will return an error) while `123.456e300` is representable.
In the case of `123.456e-350`, the parser of the rust standard library will approximate to `0` and Mawu return `0`.

#### Strings
UTF-16 surrogate pairs are permitted by the standards but usage of them has proven unreliable in the past. So you could try your luck I guess, but Mawu makes no guarantees.

#### Structure


