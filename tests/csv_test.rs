#[cfg(test)]
mod csv_tests {
    use json::*;
    use mawu::mawu_values::MawuValue::{self, *};
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn my_own_random_data_all_types() {
        let t= mawu::csv::from_csv_headed("data/csv/csv-test-data/my-own-random-data/all-types.csv");
        //assert_eq!(t.is_ok(), true);
        for x in t.as_ref().unwrap().as_csv_object().unwrap() {
            //println!("{:?}", x);
            assert_eq!(x.len(), 3);
            let id = x.get("Id").unwrap();
            let types = x.get("Type").unwrap();
            let content = x.get("Content").unwrap();
            if id == &MawuValue::String(1.to_string()) {
                assert_eq!(types, &MawuValue::String("uint".to_string()));
                assert_eq!(content, &MawuValue::String("0".to_string()));
            }
            if id == &MawuValue::String(2.to_string()) {
                assert_eq!(types, &MawuValue::String("uint".to_string()));
                assert_eq!(content, &MawuValue::String("100".to_string()));
            }
            if id == &MawuValue::String(10.to_string()) {
                assert_eq!(types, &MawuValue::String("sint".to_string()));
                assert_eq!(content, &MawuValue::String("-42".to_string()));
            }
            if id == &MawuValue::String(20.to_string()) {
                assert_eq!(types, &MawuValue::String("float".to_string()));
                assert_eq!(content, &MawuValue::String("-185911.8114191311414".to_string()));
            }
            if id == &MawuValue::String(25.to_string()) {
                assert_eq!(types, &MawuValue::String("bool".to_string()));
                assert_eq!(content, &MawuValue::String("true".to_string()));
            }
            if id == &MawuValue::String(26.to_string()) {
                assert_eq!(types, &MawuValue::String("bool".to_string()));
                assert_eq!(content, &MawuValue::String("false".to_string()));
            }
            if id == &MawuValue::String(27.to_string()) {
                assert_eq!(types, &MawuValue::String("none".to_string()));
                assert_eq!(content, &MawuValue::Null);
            }
            if id == &MawuValue::String(28.to_string()) {
                assert_eq!(types, &MawuValue::String("null".to_string()));
                assert_eq!(content, &MawuValue::Null);
            }
            if id == &MawuValue::String(31.to_string()) {
                assert_eq!(types, &MawuValue::String("string".to_string()));
                assert_eq!(content, &MawuValue::String("a string, with a comma".to_string()));
            }
            if id == &MawuValue::String(50.to_string()) {
                assert_eq!(types, &MawuValue::String("string".to_string()));
                assert_eq!(content, &MawuValue::String("Los Disturbados - \"Stuparena\"".to_string()));
            }
        }
    }
}
