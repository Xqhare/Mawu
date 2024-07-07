use crate::{mawu_value::MawuValue, utils::make_whitespace};

fn serialize_csv_value<T: Into<MawuValue>>(value: T, spaces: u8) -> String {
    let value = value.into();
    match value {
        MawuValue::String(s) => format!("{}\"{}\"", make_whitespace(spaces), s),
        MawuValue::Uint(u) => format!("{}{}", make_whitespace(spaces), u),
        MawuValue::Int(i) => format!("{}{}", make_whitespace(spaces), i),
        MawuValue::Float(f) => format!("{}{}", make_whitespace(spaces), f),
        MawuValue::Bool(b) => format!("{}{}", make_whitespace(spaces), b),
        MawuValue::Array(a) => {
            let mut out = format!("{}[", make_whitespace(spaces));
            for v in a {
                out.push_str(&serialize_csv_value(v, spaces));
                out.push(',');
            }
            out = out.trim_end_matches(',').to_string();
            out.push(']');
            out
        }
        // All other types are not allowed
        _ => String::new(),
    }
}

pub fn serialize_csv_headed(value: MawuValue, spaces: u8) -> String {
    // Headed: Vec<HashMap<String, MawuValue>>

    let mut head_created = false;
    let mut head: String = Default::default();
    let mut body: Vec<String> = Default::default();
    let mut keys: Vec<String> = Default::default();
    
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
            row.push_str(&serialize_csv_value(get_val, spaces));
            row.push(',');
        }
        row = row.trim_end_matches(',').to_string();
        body.push(row);
    }
    head = head.trim_end_matches(',').to_string();
    let mut out = format!("{}\n", head);
    out.push_str(body.join("\n").as_str());
    out
}

pub fn serialize_csv_unheaded<T: Into<MawuValue>>(value: T, spaces: u8) -> String {
    todo!()
}
