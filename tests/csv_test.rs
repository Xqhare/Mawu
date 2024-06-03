#[cfg(test)]
mod csv_tests {
    use json::*;
    use mawu::mawu_values::MawuValue::{self, *};
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn my_own_random_data_all_types() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/my-own-random-data/all-types.csv");
        assert_eq!(mawu.is_ok(), true);
        for value in mawu.as_ref().unwrap().as_csv_object().unwrap() {
            //println!("{:?}", x);
            assert_eq!(value.len(), 3);
            let id = value.get("Id").unwrap();
            let types = value.get("Type").unwrap();
            let content = value.get("Content").unwrap();
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

    #[test]
    fn random_data_no_license_customers100() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/random-data-no-license/customers-100.csv");
        assert_eq!(mawu.is_ok(), true);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 12);

        // checking a few entries against the known `true` values
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].get("Index").unwrap(), &MawuValue::String("1".to_string()));
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].get("Customer Id").unwrap(), &MawuValue::String("DD37Cf93aecA6Dc".to_string()));
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].get("Country").unwrap(), &MawuValue::String("Chile".to_string()));
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].get("Website").unwrap(), &MawuValue::String("http://www.stephenson.com/".to_string()));

        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[99].get("Index").unwrap(), &MawuValue::String("100".to_string()));
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[99].get("Customer Id").unwrap(), &MawuValue::String("2354a0E336A91A1".to_string()));
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[99].get("Country").unwrap(), &MawuValue::String("Honduras".to_string()));
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[99].get("Website").unwrap(), &MawuValue::String("http://www.hatfield-saunders.net/".to_string()));
    }

    #[test]
    fn random_data_no_license_customers100000() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/random-data-no-license/customers-100000.csv");
        assert_eq!(mawu.is_ok(), true);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100000);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 12);
    }

    #[test]
    fn random_data_no_license_organizations100() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/random-data-no-license/organizations-100.csv");
        assert_eq!(mawu.is_ok(), true);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
    }

    #[test]
    fn random_data_no_license_organizations100000() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/random-data-no-license/organizations-100000.csv");
        assert_eq!(mawu.is_ok(), true);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100000);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
    }

    #[test]
    fn random_data_no_license_people100() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/random-data-no-license/people-100.csv");
        assert_eq!(mawu.is_ok(), true);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
    }

    #[test]
    fn random_data_no_license_people100000() {
        let mawu = mawu::csv::from_csv_headed("data/csv/csv-test-data/random-data-no-license/people-100000.csv");
        assert_eq!(mawu.is_ok(), true);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100000);
        assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
    }
}
