#[cfg(test)]
mod csv_tests {

    mod headed {
        use mawu::mawu_values::MawuValue;
        use pretty_assertions::assert_eq;

        #[test]
        fn my_own_random_data_all_types() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/my-own-random-data/all-types.csv",
            );
            assert!(mawu_result.is_ok());
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 50);
            assert_eq!(mawu.as_csv_object().unwrap()[0].len(), 3);
            for value in mawu.as_csv_object().unwrap() {
                //println!("{:?}", x);
                assert_eq!(value.len(), 3);
                let id = value.get("Id").unwrap().as_uint().unwrap();
                let types = value.get("Type").unwrap();
                let content = value.get("Content").unwrap();
                // test for all possible `MawuValue`s for CSV's
                if *id == 1 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(0));
                } else if *id == 2 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(100));
                } else if *id == 10 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-42));
                } else if *id == 20 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(-185911.8114191311414));
                } else if *id == 25 {
                    assert_eq!(types, &MawuValue::String("bool".to_string()));
                    assert_eq!(content, &MawuValue::Bool(true));
                } else if *id == 26 {
                    assert_eq!(types, &MawuValue::String("bool".to_string()));
                    assert_eq!(content, &MawuValue::Bool(false));
                } else if *id == 27 {
                    assert_eq!(types, &MawuValue::String("none".to_string()));
                    assert_eq!(content, &MawuValue::None);
                } else if *id == 28 {
                    assert_eq!(types, &MawuValue::None);
                    assert_eq!(content, &MawuValue::None);
                } else if *id == 31 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(
                        content,
                        &MawuValue::String("a string, with a comma".to_string())
                    );
                } else if *id == 50 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(
                        content,
                        &MawuValue::String("Los Disturbados - \"Stuparena\"".to_string())
                    );
                }
            }
        }

        #[test]
        fn random_data_no_license_customers100() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/customers-100.csv",
            );
            assert_eq!(mawu_result.is_ok(), true);
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 100);
            assert_eq!(mawu.as_csv_object().unwrap()[0].len(), 12);

            // checking a few entries against the known `true` values
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Index").unwrap(),
                &MawuValue::Uint(1)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Customer Id").unwrap(),
                &MawuValue::String("DD37Cf93aecA6Dc".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Country").unwrap(),
                &MawuValue::String("Chile".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Website").unwrap(),
                &MawuValue::String("http://www.stephenson.com/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[34].get("Index").unwrap(),
                &MawuValue::Uint(35)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[34]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("aA9BYFfBc3710fe".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[34].get("Country").unwrap(),
                &MawuValue::String("Bahamas".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[34].get("Website").unwrap(),
                &MawuValue::String("https://spencer-charles.info/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[54].get("Index").unwrap(),
                &MawuValue::Uint(55)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[54]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("f3BEf3Be028166f".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[54].get("Country").unwrap(),
                &MawuValue::String("Ecuador".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[54].get("Website").unwrap(),
                &MawuValue::String("https://www.mora.com/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[99].get("Index").unwrap(),
                &MawuValue::Uint(100)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[99]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("2354a0E336A91A1".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[99].get("Country").unwrap(),
                &MawuValue::String("Honduras".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[99].get("Website").unwrap(),
                &MawuValue::String("http://www.hatfield-saunders.net/".to_string())
            );
        }

        #[test]
        #[ignore]
        fn random_data_no_license_customers100000() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/customers-100000.csv",
            );
            assert_eq!(mawu_result.is_ok(), true);
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 100000);
            assert_eq!(mawu.as_csv_object().unwrap()[0].len(), 12);

            // checking a few entries against the known `true` values
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Index").unwrap(),
                &MawuValue::Uint(1)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Customer Id").unwrap(),
                &MawuValue::String("ffeCAb7AbcB0f07".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Country").unwrap(),
                &MawuValue::String("Eritrea".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[0].get("Website").unwrap(),
                &MawuValue::String("https://www.mccarthy.info/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[39].get("Index").unwrap(),
                &MawuValue::Uint(40)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[39]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("06bDd08a9FdC9bF".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[39].get("Country").unwrap(),
                &MawuValue::String("Samoa".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[39].get("Website").unwrap(),
                &MawuValue::String("http://gould-parrish.com/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[345].get("Index").unwrap(),
                &MawuValue::Uint(346)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[345]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("0C609822F49dCF7".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[345].get("Country").unwrap(),
                &MawuValue::String("Somalia".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[345].get("Website").unwrap(),
                &MawuValue::String("https://york-fernandez.net/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[1034].get("Index").unwrap(),
                &MawuValue::Uint(1035)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[1034]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("16fD8A6c8D76a19".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[1034].get("Country").unwrap(),
                &MawuValue::String("Saint Kitts and Nevis".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[1034].get("Website").unwrap(),
                &MawuValue::String("https://www.jenkins-meyers.info/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[2823].get("Index").unwrap(),
                &MawuValue::Uint(2824)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[2823]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("2F786b83ce3CCdA".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[2823].get("Country").unwrap(),
                &MawuValue::String("Korea".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[2823].get("Website").unwrap(),
                &MawuValue::String("http://www.fritz-pineda.com/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[44412].get("Index").unwrap(),
                &MawuValue::Uint(44413)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[44412]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("1bF524EFaaABb3D".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[44412].get("Country").unwrap(),
                &MawuValue::String("Grenada".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[44412].get("Website").unwrap(),
                &MawuValue::String("http://www.mullen.org/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[53644].get("Index").unwrap(),
                &MawuValue::Uint(53645)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[53644]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("5f817DAE1A4c715".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[53644].get("Country").unwrap(),
                &MawuValue::String("Oman".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[53644].get("Website").unwrap(),
                &MawuValue::String("http://www.dawson.net/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[61768].get("Index").unwrap(),
                &MawuValue::Uint(61769)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[61768]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("E0228Edge53b28E2".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[61768].get("Country").unwrap(),
                &MawuValue::String("Korea".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[61768].get("Website").unwrap(),
                &MawuValue::String("https://www.barton.biz/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[82751].get("Index").unwrap(),
                &MawuValue::Uint(82752)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[82751]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("D62Ae4fD64Fdae5".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[82751].get("Country").unwrap(),
                &MawuValue::String("Sudan".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[82751].get("Website").unwrap(),
                &MawuValue::String("https://hendrix.com/".to_string())
            );

            assert_eq!(
                mawu.as_csv_object().unwrap()[99999].get("Index").unwrap(),
                &MawuValue::Uint(100000)
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[99999]
                    .get("Customer Id")
                    .unwrap(),
                &MawuValue::String("FaE5E3c1Ea0dEc2".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[99999].get("Country").unwrap(),
                &MawuValue::String("Estonia".to_string())
            );
            assert_eq!(
                mawu.as_csv_object().unwrap()[99999].get("Website").unwrap(),
                &MawuValue::String("https://www.walter.com/".to_string())
            );
        }

        #[test]
        fn random_data_no_license_organizations100() {
            let mawu = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/organizations-100.csv",
            );
            assert_eq!(mawu.is_ok(), true);
            assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100);
            assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
        }

        #[test]
        #[ignore]
        fn random_data_no_license_organizations100000() {
            let mawu = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/organizations-100000.csv",
            );
            assert_eq!(mawu.is_ok(), true);
            assert_eq!(
                mawu.as_ref().unwrap().as_csv_object().unwrap().len(),
                100000
            );
            assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
        }

        #[test]
        fn random_data_no_license_people100() {
            let mawu = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/people-100.csv",
            );
            assert_eq!(mawu.is_ok(), true);
            assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap().len(), 100);
            assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
        }

        #[test]
        #[ignore]
        fn random_data_no_license_people100000() {
            let mawu = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/people-100000.csv",
            );
            assert_eq!(mawu.is_ok(), true);
            assert_eq!(
                mawu.as_ref().unwrap().as_csv_object().unwrap().len(),
                100000
            );
            assert_eq!(mawu.as_ref().unwrap().as_csv_object().unwrap()[0].len(), 9);
        }
    }
    mod headless {
        use mawu::mawu_values::MawuValue;
        use pretty_assertions::assert_eq;

        #[test]
        fn my_own_random_data_all_types() {
            let mawu_result = mawu::read::csv_headless(
                "data/csv/csv-test-data/headless/my-own-random-data/all-types.csv",
            );
            assert_eq!(mawu_result.is_ok(), true);
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_array().unwrap().len(), 50);

            for (index, value) in mawu.as_csv_array().unwrap().iter().enumerate() {
                assert_eq!(value.len(), 3);
                for entry in value {
                    match index {
                        0 => {
                            assert_eq!(value[0], MawuValue::Uint(1));
                            assert_eq!(value[1], MawuValue::String("uint".to_string()));
                            assert_eq!(value[2], MawuValue::Uint(0));
                        }
                        8 => {
                            assert_eq!(value[0], MawuValue::Uint(9));
                            assert_eq!(value[1], MawuValue::String("sint".to_string()));
                            assert_eq!(value[2], MawuValue::Int(-188));
                        }
                        18 => {
                            assert_eq!(value[0], MawuValue::Uint(19));
                            assert_eq!(value[1], MawuValue::String("float".to_string()));
                            assert_eq!(value[2], MawuValue::Float(8114191311414.185911));
                        }

                        26 => {
                            assert_eq!(value[0], MawuValue::Uint(27));
                            assert_eq!(value[1], MawuValue::String("none".to_string()));
                            assert_eq!(value[2], MawuValue::None);
                        }
                        27 => {
                            assert_eq!(value[0], MawuValue::Uint(28));
                            assert_eq!(value[1], MawuValue::None);
                            assert_eq!(value[2], MawuValue::None);
                        }
                        35 => {
                            assert_eq!(value[0], MawuValue::Uint(36));
                            assert_eq!(value[1], MawuValue::String("string".to_string()));
                            assert_eq!(value[2], MawuValue::String("discworld".to_string()));
                        }
                        _ => match entry {
                            MawuValue::Int(_) => assert!(true),
                            MawuValue::Uint(_) => assert!(true),
                            MawuValue::Float(_) => assert!(true),
                            MawuValue::String(_) => assert!(true),
                            MawuValue::Bool(_) => assert!(true),
                            MawuValue::None => assert!(true),
                            _ => assert!(false),
                        },
                    }
                }
            }
        }
    }
}
