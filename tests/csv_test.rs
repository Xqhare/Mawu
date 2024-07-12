#[cfg(test)]
mod csv_tests {

    mod headed {
        use std::{fs::File, io::Read};

        use json::parse;
        use mawu::mawu_value::MawuValue;
        use pretty_assertions::assert_eq;

        #[test]
        #[ignore]
        fn speed_external() {
            let mut inp = File::open("data/json/json-test-data/test_data_517k.json").unwrap();
            let mut buf: String = Default::default();
            let _ = inp.read_to_string(&mut buf);
            let out = parse(&buf);
            assert!(out.is_ok());
            assert!(out.as_ref().unwrap().is_array());
            assert_eq!(out.unwrap().len(), 517000);
        }

        #[test]
        #[ignore]
        fn my_own_random_large_data_set_84mb_1mil_rows() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/my-own-random-data/test_data_1mil.csv",
            );
            assert!(mawu_result.is_ok());
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 1_000_000);
        }

        #[test]
        #[ignore]
        fn my_own_random_large_data_set_26mb() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/my-own-random-data/test_data_310k.csv",
            );
            assert!(mawu_result.is_ok());
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 310_000);
        }

        #[test]
        fn write_and_read_data() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/my-own-random-data/all-types.csv",
            ).unwrap();
            assert!(mawu_result.is_csv_object());
            let write_succ = mawu::write::csv("test_file_delete_me.csv", mawu_result.clone());
            let write_read = mawu::read::csv_headed("test_file_delete_me.csv");
            assert!(write_succ.is_ok());
            assert!(write_read.is_ok());
            // actual equality check
            for row in mawu_result.as_csv_object().unwrap() {
                assert_eq!(row.len(), 3);
                let id = row.get("Id").unwrap().as_uint().unwrap();
                let types = row.get("Type").unwrap();
                let content = row.get("Content").unwrap();
                // test for all possible `MawuValue`s for CSV's up to id 50
                if *id == 1 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(0));
                } else if *id == 2 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(100));
                } else if *id == 3 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(1000));
                } else if *id == 4 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(8114191311414));
                } else if *id == 5 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(69));
                } else if *id == 6 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(420));
                } else if *id == 7 {
                    assert_eq!(types, &MawuValue::String("uint".to_string()));
                    assert_eq!(content, &MawuValue::Uint(185911));
                } else if *id == 8 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-1));
                } else if *id == 9 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-188));
                } else if *id == 10 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-42));
                } else if *id == 11 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-185911));
                } else if *id == 12 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-8114191311414));
                } else if *id == 13 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-1000));
                } else if *id == 14 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-10000));
                } else if *id == 15 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-69));
                } else if *id == 16 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-420));
                } else if *id == 17 {
                    assert_eq!(types, &MawuValue::String("sint".to_string()));
                    assert_eq!(content, &MawuValue::Int(-1337));
                } else if *id == 18 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(0.0));
                } else if *id == 19 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(8114191311414.185911));
                } else if *id == 20 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(-185911.8114191311414));
                } else if *id == 21 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(18.8));
                } else if *id == 22 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(-18.8));
                } else if *id == 23 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(185911.8114191311414));
                } else if *id == 24 {
                    assert_eq!(types, &MawuValue::String("float".to_string()));
                    assert_eq!(content, &MawuValue::Float(-8114191311414.185911));
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
                } else if *id == 29 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("let's support all special characters I can find on my keyboard üöä +*~ #' -_ .: ,; µ@€ <|> ^′!\"§$%&/{([])}=ß\\?`´ /*-+,".to_string()));
                } else if *id == 30 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Some String".to_string()));
                } else if *id == 31 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("a string, with a comma".to_string()));
                } else if *id == 32 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("lorem ipsum".to_string()));
                } else if *id == 33 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("test string".to_string()));
                } else if *id == 34 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("hitchhikers guide to the galaxy".to_string()));
                } else if *id == 35 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("lord of the rings".to_string()));
                } else if *id == 36 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("discworld".to_string()));
                } else if *id == 37 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("das geheimniss von askir".to_string()));
                } else if *id == 38 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Götterkrieg".to_string()));
                } else if *id == 39 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("the wanderer".to_string()));
                } else if *id == 40 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("hurt".to_string()));
                } else if *id == 41 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("rasputin".to_string()));
                } else if *id == 42 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("YMCA".to_string()));
                } else if *id == 43 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Ding-dong the witch is dead".to_string()));
                } else if *id == 44 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("It's time to take a shit\n on the Company's Dime".to_string()));
                } else if *id == 45 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("You're An Ace, Kid".to_string()));
                } else if *id == 46 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Schälläfäscht 3000".to_string()));
                } else if *id == 47 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Wild Wild West".to_string()));
                } else if *id == 48 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Chaos Theory".to_string()));
                } else if *id == 49 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("CPR".to_string()));
                } else if *id == 50 {
                    assert_eq!(types, &MawuValue::String("string".to_string()));
                    assert_eq!(content, &MawuValue::String("Los Disturbados - \"Stuparena\"".to_string()));
                }
            }

            std::fs::remove_file("test_file_delete_me.csv").unwrap();
        }

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
                //println!("{:?}", value);
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
            //println!("{:?}", mawu.as_csv_object().unwrap());
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
        use mawu::mawu_value::MawuValue;
        use pretty_assertions::assert_eq;

        #[test]
        fn read_and_write_data() {
            let mawu_result = mawu::read::csv_headless(
                "data/csv/csv-test-data/headless/my-own-random-data/all-types.csv",
            );
            assert_eq!(mawu_result.is_ok(), true);
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_array().unwrap().len(), 50);
            let write_succ = mawu::write::csv("test_file_delete_me2.csv", mawu.clone());
            assert!(write_succ.is_ok());
            let read_write = mawu::read::csv_headless("test_file_delete_me2.csv");
            assert!(read_write.is_ok());
            assert_eq!(read_write.as_ref().unwrap(), &mawu);
            // lets check the 50 rows
            let mut id = 1;
            for row in read_write.unwrap().as_csv_array().unwrap() {
                assert_eq!(row.len(), 3);
                if id == 1 {
                    assert_eq!(row[0], MawuValue::Uint(1));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(0));
                } else if id == 2 {
                    assert_eq!(row[0], MawuValue::Uint(2));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(100));
                } else if id == 3 {
                    assert_eq!(row[0], MawuValue::Uint(3));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(1000));
                } else if id == 4 {
                    assert_eq!(row[0], MawuValue::Uint(4));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(8114191311414));
                } else if id == 5 {
                    assert_eq!(row[0], MawuValue::Uint(5));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(69));
                } else if id == 6 {
                    assert_eq!(row[0], MawuValue::Uint(6));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(420));
                } else if id == 7 {
                    assert_eq!(row[0], MawuValue::Uint(7));
                    assert_eq!(row[1], MawuValue::String("uint".to_string()));
                    assert_eq!(row[2], MawuValue::Uint(185911));
                } else if id == 8 {
                    assert_eq!(row[0], MawuValue::Uint(8));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-1));
                } else if id == 9 {
                    assert_eq!(row[0], MawuValue::Uint(9));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-188));
                } else if id == 10 {
                    assert_eq!(row[0], MawuValue::Uint(10));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-42));
                } else if id == 11 {
                    assert_eq!(row[0], MawuValue::Uint(11));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-185911));
                } else if id == 12 {
                    assert_eq!(row[0], MawuValue::Uint(12));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-8114191311414));
                } else if id == 13 {
                    assert_eq!(row[0], MawuValue::Uint(13));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-1000));
                } else if id == 14 {
                    assert_eq!(row[0], MawuValue::Uint(14));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-10000));
                } else if id == 15 {
                    assert_eq!(row[0], MawuValue::Uint(15));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-69));
                } else if id == 16 {
                    assert_eq!(row[0], MawuValue::Uint(16));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-420));
                } else if id == 17 {
                    assert_eq!(row[0], MawuValue::Uint(17));
                    assert_eq!(row[1], MawuValue::String("sint".to_string()));
                    assert_eq!(row[2], MawuValue::Int(-1337));
                } else if id == 18 {
                    assert_eq!(row[0], MawuValue::Uint(18));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(0.0));
                } else if id == 19 {
                    assert_eq!(row[0], MawuValue::Uint(19));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(8114191311414.186));
                } else if id == 20 {
                    assert_eq!(row[0], MawuValue::Uint(20));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(-185911.8114191311414));
                } else if id == 21 {
                    assert_eq!(row[0], MawuValue::Uint(21));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(18.8));
                } else if id == 22 {
                    assert_eq!(row[0], MawuValue::Uint(22));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(-18.8));
                } else if id == 23 {
                    assert_eq!(row[0], MawuValue::Uint(23));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(185911.8114191311414));
                } else if id == 24 {
                    assert_eq!(row[0], MawuValue::Uint(24));
                    assert_eq!(row[1], MawuValue::String("float".to_string()));
                    assert_eq!(row[2], MawuValue::Float(-8114191311414.186));
                } else if id == 25 {
                    assert_eq!(row[0], MawuValue::Uint(25));
                    assert_eq!(row[1], MawuValue::String("bool".to_string()));
                    assert_eq!(row[2], MawuValue::Bool(true));
                } else if id == 26 {
                    assert_eq!(row[0], MawuValue::Uint(26));
                    assert_eq!(row[1], MawuValue::String("bool".to_string()));
                    assert_eq!(row[2], MawuValue::Bool(false));
                } else if id == 27 {
                    assert_eq!(row[0], MawuValue::Uint(27));
                    assert_eq!(row[1], MawuValue::String("none".to_string()));
                    assert_eq!(row[2], MawuValue::None);
                } else if id == 28 {
                    assert_eq!(row[0], MawuValue::Uint(28));
                    assert_eq!(row[1], MawuValue::None);
                    assert_eq!(row[2], MawuValue::None);
                } else if id == 29 {
                    assert_eq!(row[0], MawuValue::Uint(29));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("let's support all special characters I can find on my keyboard üöä +*~ #' -_ .: ,; µ@€ <|> ^′!\"§$%&/{([])}=ß\\?`´ /*-+,".to_string()));
                } else if id == 30 {
                    assert_eq!(row[0], MawuValue::Uint(30));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Some String".to_string()));
                } else if id == 31 {
                    assert_eq!(row[0], MawuValue::Uint(31));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("a string, with a comma".to_string()));
                } else if id == 32 {
                    assert_eq!(row[0], MawuValue::Uint(32));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("lorem ipsum".to_string()));
                } else if id == 33 {
                    assert_eq!(row[0], MawuValue::Uint(33));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("test string".to_string()));
                } else if id == 34 {
                    assert_eq!(row[0], MawuValue::Uint(34));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("hitchhikers guide to the galaxy".to_string()));
                } else if id == 35 {
                    assert_eq!(row[0], MawuValue::Uint(35));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("lord of the rings".to_string()));
                } else if id == 36 {
                    assert_eq!(row[0], MawuValue::Uint(36));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("discworld".to_string()));
                } else if id == 37 {
                    assert_eq!(row[0], MawuValue::Uint(37));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("das geheimniss von askir".to_string()));
                } else if id == 38 {
                    assert_eq!(row[0], MawuValue::Uint(38));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Götterkrieg".to_string()));
                } else if id == 39 {
                    assert_eq!(row[0], MawuValue::Uint(39));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("the wanderer".to_string()));
                } else if id == 40 {
                    assert_eq!(row[0], MawuValue::Uint(40));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("hurt".to_string()));
                } else if id == 41 {
                    assert_eq!(row[0], MawuValue::Uint(41));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("rasputin".to_string()));
                } else if id == 42 {
                    assert_eq!(row[0], MawuValue::Uint(42));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("YMCA".to_string()));
                } else if id == 43 {
                    assert_eq!(row[0], MawuValue::Uint(43));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Ding-dong the witch is dead".to_string()));
                } else if id == 44 {
                    assert_eq!(row[0], MawuValue::Uint(44));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("It's time to take a shit\n on the Company's Dime".to_string()));
                } else if id == 45 {
                    assert_eq!(row[0], MawuValue::Uint(45));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("You're An Ace, Kid".to_string()));
                } else if id == 46 {
                    assert_eq!(row[0], MawuValue::Uint(46));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Schälläfäscht 3000".to_string()));
                } else if id == 47 {
                    assert_eq!(row[0], MawuValue::Uint(47));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Wild Wild West".to_string()));
                } else if id == 48 {
                    assert_eq!(row[0], MawuValue::Uint(48));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Chaos Theory".to_string()));
                } else if id == 49 {
                    assert_eq!(row[0], MawuValue::Uint(49));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("CPR".to_string()));
                } else if id == 50 {
                    assert_eq!(row[0], MawuValue::Uint(50));
                    assert_eq!(row[1], MawuValue::String("string".to_string()));
                    assert_eq!(row[2], MawuValue::String("Los Disturbados - \"Stuparena\"".to_string()));
                } else {
                    assert!(false);
                }

                id += 1;
            }
            std::fs::remove_file("test_file_delete_me2.csv").unwrap();
        }

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
