use crate::{errors::{csv_error::{CsvError, CsvWriteError}, MawuError}, mawu_value::MawuValue, utils::make_whitespace};

fn serialize_csv_value<T: Into<MawuValue>>(value: T, spaces: u8) -> Result<String, MawuError> {
    let value = value.into();
    match value {
        MawuValue::String(s) => Ok(format!("{}\"{}\"", make_whitespace(spaces), s)),
        MawuValue::Uint(u) => Ok(format!("{}{}", make_whitespace(spaces), u)),
        MawuValue::Int(i) => Ok(format!("{}{}", make_whitespace(spaces), i)),
        MawuValue::Float(f) => Ok(format!("{}{}", make_whitespace(spaces), f)),
        MawuValue::Bool(b) => Ok(format!("{}{}", make_whitespace(spaces), b)),
        MawuValue::Array(a) => {
            let mut out = format!("{}[", make_whitespace(spaces));
            for v in a {
                out.push_str(&serialize_csv_value(v, spaces)?);
                out.push(',');
            }
            out = out.trim_end_matches(',').to_string();
            out.push(']');
            Ok(out)
        }
        MawuValue::None => Ok(String::new()),
        // All other types are not allowed
        MawuValue::Object(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("Object".to_string())))).unwrap(),
        MawuValue::CSVArray(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("CSV-Array inside CSV-Value".to_string())))).unwrap(),
        MawuValue::CSVObject(_) => Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType("CSV-Object inside CSV-Value".to_string())))).unwrap(),
    }
}

pub fn serialize_csv_headed(value: MawuValue, spaces: u8) -> Result<String, MawuError> {
    // Headed: Vec<HashMap<String, MawuValue>>

    let mut head_created = false;
    let mut head: String = Default::default();
    let mut body: Vec<String> = Default::default();
    let mut keys: Vec<String> = Default::default();
    if !value.is_csv_object() {
        let val_type = {
            match value {
                MawuValue::CSVArray(_) => "CSV-Array",
                MawuValue::Array(_) => "Array",
                MawuValue::Object(_) => "Object",
                MawuValue::None => "None",
                MawuValue::Bool(_) => "Bool",
                MawuValue::Uint(_) => "Uint",
                MawuValue::Int(_) => "Int",
                MawuValue::Float(_) => "Float",
                MawuValue::String(_) => "String",
                _ => "Unknown",
            }
        };
        return Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType(format!("{} is not a MawuValue::CsvObject!", val_type)))));
    }
    for map in value.to_csv_object().unwrap() {
        let mut row: String = Default::default();
        for (key, _) in &map {
            if !head_created {
                keys.push(key.clone());
                head.push_str(make_whitespace(spaces).as_str());
                head.push_str(&key);
                head.push(',');
            }
        }
        head_created = true;
        for key in keys.clone() {
            let get_val = map.get(&key).unwrap();
            row.push_str(&serialize_csv_value(get_val, spaces)?);
            row.push(',');
        }
        row = row.trim_end_matches(',').to_string();
        body.push(row);
    }
    head = head.trim_end_matches(',').to_string();
    head = head.trim_start().to_string();
    let mut out = format!("{}\n", head);
    out.push_str(body.join("\n").as_str());
    Ok(out)
}

pub fn serialize_csv_unheaded(value: MawuValue, spaces: u8) -> Result<String, MawuError> {
    // Input == Vec<Vec<MawuValue>>
    // First vec holds rows, second vec holds data in each row
    // output == String, with each row on a new line, values separated by commas
    let mut out = format!("{}", make_whitespace(spaces));
    if !value.is_csv_array() {
        let val_type = {
            match value {
                MawuValue::CSVObject(_) => "CSV-Object",
                MawuValue::Array(_) => "Array",
                MawuValue::Object(_) => "Object",
                MawuValue::None => "None",
                MawuValue::Bool(_) => "Bool",
                MawuValue::Uint(_) => "Uint",
                MawuValue::Int(_) => "Int",
                MawuValue::Float(_) => "Float",
                MawuValue::String(_) => "String",
                _ => "Unknown",
            }
        };
        return Err(MawuError::CsvError(CsvError::WriteError(CsvWriteError::UnallowedType(format!("{} is not a MawuValue::CsvArray!", val_type)))));
    }
    for v in value.to_csv_array().unwrap() {
        let mut row = String::new();
        for i in v {
            row.push_str(&serialize_csv_value(i, spaces)?);
            row.push(',');
        }
        row = row.trim_end_matches(',').to_string();
        out.push_str(&row);
        out.push('\n');
    }
    Ok(out)
}
