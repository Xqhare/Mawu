# Mawu
A simple JSON and CSV parsing library written in rust

## Features

- Simple and fast
- Type aware
- Supports both CSV and JSON
    - CSV
        - With or without header
        - settable delimiter

## CSV
This parser supports CSV files, conforming to the rfc4180 standard and is itself conforming to the rfc4180 standard and nothing else. 

> [!NOTE]
> While the usage of the header is optional, if you use one, use the `headed` method of the parser, otherwise use `headless` method.
> [Learn more.](README#usage)

### Handling missing or not provided values

> [!caution]
> It is advisable to ensure there are no missing or not provided values in your data before using Mawu.

While Mawu does handle missing or not provided values, it is not 100% reliable for a variety of reasons.
Exactly how this is handled is explained in the following paragraph, however it is advisable to ensure there are no missing or not provided values in your data before using Mawu.

This parser implements missing or not provided values differently depending on if a header is present or not.
If a header is present, the missing values will be filled with a `Mawu::None` Value. 
Should a header be not present, any row ending in a `,` will append as many `Mawu::None` values as there are columns in the first row.

Because of the rfc4180 standard, a missing value in the form of `aaa, ,ccc` would still result in 3 `MawuValue`'s in the form of `[aaa][ ][ccc]` as CSV has significant whitespace, so the missing `bbb` is converted into a space.
A missing value in the form of `aaa,,ccc` would result in a `MawuValue` of `[aaa][Mawu::None][ccc]` because of the unescaped second comma.
A row where the missing value is `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb]` only in the case where there is no header, and it is in the first row.

### Return value
Mawu will return a `Result<MawuResult, MawuError>`. By using `MawuResult::headed` or `MawuResult::headless`, you access the data wrapped inside the `MawuResult`.
The `contents` field will be of type `Vec<MawuValue>` if used without a header, or `Vec<HashMap<String, MawuValue>>` if used with a header.

### Usage

## JSON
This parser supports JSON files, conforming to the rfc8259 standard and the ECMA-404 standard.
