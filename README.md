# Mawu
A simple JSON and CSV parsing library written in rust.

Mawu, named after the ancient creator goddess Mawu in west-african mythology, offers a simple yet robust and reliable JSON and CSV parsing library implementing the rfc4180, rfc8259 standards and the ECMA-404 standard.

> [!WARNING]
> WIP CODE. DO NOT USE.

## Features

- Simple and fast
- Type aware
- Supports both CSV and JSON
    - CSV
        - With or without header, and data shift is more likely to occur
        - settable delimiter
- Supports missing or not provided values

## Chapters and subchapters

- [Features](#features)
- [Naming the Creation: A Legacy of the Divine](#naming-the-creation-a-legacy-of-the-divine)
- [CSV](#csv)
    - [Handling missing or not provided values](#handling-missing-or-not-provided-values)
        - [With header](#with-header)
        - [Without header](#without-header)

- [JSON](#json)

## Naming the Creation: A Legacy of the Divine

The name "Mawu" isn't chosen by chance, it honors the powerful West African goddess associated with the moon, the sun, and creation itself.
There's a long and rich human tradition of naming significant things after deities. Mawu embodies this tradition perfectly.

Just as Mawu, the goddess, is linked to creation, Mawu, the library, empowers you to create new things from raw data.  JSON and CSV files are like raw materials, and Mawu provides the tools to shape them into meaningful structures, ready to be used for analysis, manipulation, and ultimately, new creations.

## CSV
This library supports CSV files, conforming to the rfc4180 standard and is itself conforming to the rfc4180 standard and nothing else.

Please note that CSV, while a standard exists, is seldom implemented as such in practice.

One example would be a common shorthand for an array by using `aaa / bbb / ccc` to represent `[aaa, bbb, ccc]`. 
This is not part of the rfc4180 standard and thus not implemented in Mawu, instead it would be treated as a single string, with the appropriate errors.
`aaa / "bbb" / ccc` would produce an error for example, as Mewa treats the entire thing as one string, but it encounters unescaped double-quotes.

Another example is the way encoding is implemented. Mewa uses `utf-8` encoding exclusively for CSV, and does not recognize or produce a `BOM` or similar at the beginning of the file.
There are CSV files encoded in `utf-16` and `utf-32` as well as some more esoteric implementations like the IBM one where you can define new field names in the middle of a CSV file by using `#GROUP_OBJECT_PROFILE#` [learn more](https://www.ibm.com/docs/en/sig-and-i/10.0.2?topic=schedules-example-comma-separated-value-csv-file) and many others.

One last example is the handling of a value of `""` in the middle of a CSV file. This is part of the rfc4180 standard only implicitly, and sometimes interpreted as an empty string, other times as a missing value.
Mawu will treat it as an empty string and uses it as the default for an empty value.

Because of this, most if not all CSV files are only supported in the ecosystem or app they were created in, and there is no guarantee that Mawu will be able to parse them correctly.

> [!NOTE]
> While the usage of the header is optional, you need to use the `from_csv_headless(path)` method of the library, otherwise use the `from_csv_headed(path)` method.
> [Learn more.](#usage)

### Handling missing or not provided values
> [!caution]
> It is advisable to ensure there are no missing or not provided values in your data before using Mawu.
> To cut a long story short: You risk shifting your data, which is somewhat undesirable for most use-cases.

The rfc4180 standard allows for missing or not provided values in CSV files only implicitly. There are several ways libraries have implemented this in the past, and Mawu goes with the closest interpretation the rfc4180 allows.

While Mawu does handle missing or not provided values, it is not 100% reliable.
Exactly how this is handled is explained in the following paragraphs, however it is advisable to ensure there are no missing or not provided values in your data before using Mawu.

This library implements missing or not provided values differently depending on if a header is present or not.

Because of the rfc4180 standard, a missing value in the form of `aaa, ,ccc` would still result in 3 `MawuValue`'s in the form of `[aaa][ ][ccc]` as CSV has significant whitespace, so the missing `bbb` is converted into a space.

A row in the form of `aaa,,ccc` would result in a `MawuValue` of `[aaa][Mawu::Empty][ccc]` because of the unescaped second comma.

#### With header
If a header is present, the missing values will be filled with a `Mawu::None` Value. 

A header of `AAA,BBB,CCC`, and the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::Empty]`.
With a header of `AAA,BBB,CCC,DDD`, the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::Empty][Mawu::Empty]`.

Please note that as long as a header is present Mawu will append `Mawu::Empty` values for as many columns as there are columns declared in the header.


#### Without header

Should a header be not present, any row ending in a `,` will append as many `Mawu::None` values as there are columns in the first row.

The row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::Empty]` because of the trailing comma without content.
A row where the missing value is `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb]` only in the case where it is in the first row.
However, the row `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb]`.




### Return value
Mawu will return a `Result<MawuResult, MawuError>`. By using `MawuResult::headed` or `MawuResult::headless`, you access the data wrapped inside the `MawuResult`.
The `contents` field will be of type `Vec<Vec<MawuValue>>` if used without a header, or `Vec<HashMap<String, MawuValue>>` if used with a header.

### Usage

> [!WARNING]
> WIP CODE. DO NOT USE. DO NOT PUBLISH.

#### Example code

Reading a CSV file and just printing out the values:

```rust
use mawu::Mawu::*;

fn main() {
    // for a csv file with header
    let mawu: Vec<HashMap<String, MawuValue>> = Mawu::from_csv_headed("/path/to/file.csv");

    // mawu will return a Result<MawuResult, MawuError>
    for entry in mawu.unwrap().contents {
        for (key, value) in &entry {
            println!("{}: {}", key, value);
        }
    }

    // for a csv file without header
    let mawu_headless: Vec<Vec<MawuValue>> = Mawu::from_csv_headless("/path/to/file.csv");

    // mawu will return a Result<MawuResult, MawuError>
    for entry in mawu_headless.unwrap().contents {
        for value in entry {
            println!("{}", value);
        }
    }
}
```

## JSON
This library supports JSON files, conforming to the rfc8259 standard and the ECMA-404 standard.
