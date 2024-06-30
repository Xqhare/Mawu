#[cfg(test)]
mod json_tests {
    use mawu::read::read_json;
    use pretty_assertions::{assert_eq, assert_ne};

    // This is implicitly testing a lot of stuff!
    // But mainly testing the optimisations of the JSON parser
    #[test]
    #[ignore]
    fn large_json_file_26mb() {
        let large_result = read_json("data/json/json-test-data/large-file-json/large-file.json").unwrap();
        assert_eq!(large_result.is_array(), true);
        assert_eq!(large_result.as_array().unwrap().len(), 11351);
    }

    #[test]
    fn simple_valid_json() {
        let simple_result = read_json("data/json/json-test-data/simple-json.json").unwrap();
        let tmp_simple_bind = simple_result.as_object().unwrap();
        assert_eq!(tmp_simple_bind.len(), 1);
        let tmp_quiz = tmp_simple_bind.get("quiz").unwrap().as_object().unwrap();
        // Sports
        let sports = tmp_quiz.get("sport").unwrap().as_object().unwrap();
        let sport_q1 = sports.get("q1").unwrap().as_object().unwrap();
        let s_q1_question = sport_q1.get("question").unwrap().as_str().unwrap();
        assert_eq!(s_q1_question, "Which one is correct team name in 1. Bundesliga?");
        let s_q1_options = sport_q1.get("options").unwrap();
        assert_eq!(s_q1_options.as_array().unwrap().len(), 4);
        assert_eq!(s_q1_options.as_array().unwrap()[0].as_str().unwrap(), "2. Fc Bayern");
        let s_q1_answer = sport_q1.get("answer").unwrap();
        assert_eq!(s_q1_answer.as_str().unwrap(), "VfB Stuttgart");
        // Maths
        let maths = tmp_quiz.get("maths").unwrap().as_object().unwrap();
        let m_q1 = maths.get("q1").unwrap().as_object().unwrap();
        let m_q1_question = m_q1.get("question").unwrap().as_str().unwrap();
        assert_eq!(m_q1_question, "5 + 8 = ?");
        let m_q1_options = m_q1.get("options").unwrap().as_array().unwrap();
        assert_eq!(m_q1_options.len(), 4);
        assert_eq!(m_q1_options[0].as_uint().unwrap(), &10);
        let m_q1_answer = m_q1.get("answer").unwrap();
        assert_eq!(m_q1_answer.as_uint().unwrap(), &13);
        let m_q2 = maths.get("q2").unwrap().as_object().unwrap();
        let m_q2_question = m_q2.get("question").unwrap().as_str().unwrap();
        assert_eq!(m_q2_question, "12 - 10 = ?");
        let m_q2_options = m_q2.get("options").unwrap().as_array().unwrap();
        assert_eq!(m_q2_options.len(), 4);
        assert_eq!(m_q2_options[0].as_uint().unwrap(), &1);
        let m_q2_answer = m_q2.get("answer").unwrap();
        assert_eq!(m_q2_answer.as_uint().unwrap(), &2);

        let very_simple_result = read_json("data/json/json-test-data/very-simple-json.json").unwrap();
        let vs_obj = very_simple_result.as_object().unwrap();
        assert_eq!(vs_obj.len(), 3);
        let vs_key1 = vs_obj.get("fruit").unwrap();
        assert_eq!(vs_key1.as_str().unwrap(), "Banana");
        let vs_key2 = vs_obj.get("size").unwrap();
        assert_eq!(vs_key2.as_str().unwrap(), "Medium");
        let vs_key3 = vs_obj.get("colour").unwrap();
        assert_eq!(vs_key3.as_str().unwrap(), "Blue");
    }

    #[test]
    fn rfc8259_valid_json() {
        let rfc8259_array = read_json("data/json/json-test-data/rfc8259-test-data/array.json").unwrap();
        let rfc8259_object = read_json("data/json/json-test-data/rfc8259-test-data/object.json").unwrap();
        let rfc8259_string = read_json("data/json/json-test-data/rfc8259-test-data/small-text1.json").unwrap();
        let rfc8259_num = read_json("data/json/json-test-data/rfc8259-test-data/small-text2.json").unwrap();
        let rfc8259_bool = read_json("data/json/json-test-data/rfc8259-test-data/small-text3.json").unwrap();
        assert_eq!(rfc8259_array.is_array(), true);
        assert_eq!(rfc8259_array.as_array().unwrap().len(), 2);
        assert_eq!(rfc8259_object.is_object(), true);
        assert_eq!(rfc8259_object.as_object().unwrap().len(), 1);
        assert_eq!(rfc8259_object.as_object().unwrap().get("Image").unwrap().as_object().unwrap().len(), 6);
        assert_eq!(rfc8259_string.is_string(), true);
        assert_eq!(rfc8259_string.as_string().unwrap(), &"Hello world!".to_string());
        assert_eq!(rfc8259_num.is_uint(), true);
        assert_eq!(rfc8259_num.as_uint().unwrap(), &42);
        assert_eq!(rfc8259_bool.is_bool(), true);
        assert_eq!(rfc8259_bool.as_bool().unwrap(), &true);
    }

    #[test]
    fn json_org_valid_json() {
        let small_weird_json = read_json("data/json/json-test-data/jsonOrg-json-examples/small-weird-json.json").unwrap();
        let small_simple_json = read_json("data/json/json-test-data/jsonOrg-json-examples/small-simple-json.json").unwrap();
        let small_complex_json = read_json("data/json/json-test-data/jsonOrg-json-examples/small-complex-json.json").unwrap();
        let medium_complex_json = read_json("data/json/json-test-data/jsonOrg-json-examples/medium-complex-json.json").unwrap();
        let large_complex_json = read_json("data/json/json-test-data/jsonOrg-json-examples/large-complex-json.json").unwrap();
        assert_eq!(small_weird_json.is_object(), true);
        assert_eq!(small_simple_json.is_object(), true);
        assert_eq!(small_complex_json.is_object(), true);
        assert_eq!(medium_complex_json.is_object(), true);
        assert_eq!(large_complex_json.is_object(), true);
    }

    #[test]
    fn microsoft_edge_valid_dummy_json_small() {
        let micro_64kb = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/64KB.json").unwrap();
        assert_eq!(micro_64kb.is_array(), true);
        let micro_64kb_mini = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/64KB-min.json").unwrap();
        assert_eq!(micro_64kb_mini.is_array(), true);

        let micro_128kb = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/128KB.json").unwrap();
        assert_eq!(micro_128kb.is_array(), true);
        let micro_128kb_mini = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/128KB-min.json").unwrap();
        assert_eq!(micro_128kb_mini.is_array(), true);

        let micro_256kb = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/256KB.json").unwrap();
        assert_eq!(micro_256kb.is_array(), true);
        let micro_256kb_mini = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/256KB-min.json").unwrap();
        assert_eq!(micro_256kb_mini.is_array(), true);

        let micro_512kb = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/512KB.json").unwrap();
        assert_eq!(micro_512kb.is_array(), true);
        let micro_512kb_mini = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/512KB-min.json").unwrap();
        assert_eq!(micro_512kb_mini.is_array(), true);
    }

    #[test]
    #[ignore]
    fn microsoft_edge_valid_dummy_json_large() {
        let micro_1mb = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/1MB.json").unwrap();
        assert_eq!(micro_1mb.is_array(), true);
        let micro_1mb_mini = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/1MB-min.json").unwrap();
        assert_eq!(micro_1mb_mini.is_array(), true);

        let micro_5mb = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/5MB.json").unwrap();
        assert_eq!(micro_5mb.is_array(), true);
        let micro_5mb_mini = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/5MB-min.json").unwrap();
        assert_eq!(micro_5mb_mini.is_array(), true);
    }

    #[test]
    /// Remember all of this errors!
    fn microsoft_edge_invalid_dummy_json() {
        let invalid_binary = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/binary-data.json");
        assert_eq!(invalid_binary.is_err(), true);

        let invalid_missing_colon = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/missing-colon.json");
        assert_eq!(invalid_missing_colon.is_err(), true);

        let unterminated = read_json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/unterminated.json");
        assert_eq!(unterminated.is_err(), true);
    }

    #[test]
    fn json_test_suite_valid_json() {
        let arrays_with_spaces = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_arraysWithSpaces.json").unwrap();
        assert_eq!(arrays_with_spaces.is_array(), true);
        let array_empty = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_empty.json").unwrap();
        assert_eq!(array_empty.is_array(), true);
        let array_empty_string = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_empty-string.json").unwrap();
        assert_eq!(array_empty_string.is_array(), true);
        let array_ending_with_newline = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_ending_with_newline.json").unwrap();
        assert_eq!(array_ending_with_newline.is_array(), true);
        let array_false = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_false.json").unwrap();
        assert_eq!(array_false.is_array(), true);
        let array_heterogeneous = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_heterogeneous.json").unwrap();
        assert_eq!(array_heterogeneous.is_array(), true);
        let array_null = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_null.json").unwrap();
        assert_eq!(array_null.is_array(), true);
        assert_eq!(array_null.as_array().unwrap()[0].is_none(), true);
        let array_with_1_and_newline = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_1_and_newline.json").unwrap();
        assert_eq!(array_with_1_and_newline.is_array(), true);
        let array_with_leading_space = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_leading_space.json").unwrap();
        assert_eq!(array_with_leading_space.is_array(), true);
        let array_with_several_null = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_several_null.json").unwrap();
        assert_eq!(array_with_several_null.is_array(), true);
        assert_eq!(array_with_several_null.as_array().unwrap()[0].is_number(), true);
        assert_eq!(array_with_several_null.as_array().unwrap()[1].is_none(), true);
        assert_eq!(array_with_several_null.as_array().unwrap()[2].is_none(), true);
        assert_eq!(array_with_several_null.as_array().unwrap()[3].is_none(), true);
        assert_eq!(array_with_several_null.as_array().unwrap()[4].is_number(), true);
        let array_with_trailing_space = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_trailing_space.json").unwrap();
        assert_eq!(array_with_trailing_space.is_array(), true);

        let number = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number.json").unwrap();
        assert_eq!(number.as_array().unwrap()[0].is_number(), true);
        let number_0e1 = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_0e1.json").unwrap();
        assert_eq!(number_0e1.as_array().unwrap()[0].is_number(), true);
        let number_0eplusone = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_0e+1.json").unwrap();
        assert_eq!(number_0eplusone.as_array().unwrap()[0].is_number(), true);
        let number_after_space = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_after_space.json").unwrap();
        assert_eq!(number_after_space.as_array().unwrap()[0].is_number(), true);
        let number_double_close_to_zero = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_double_close_to_zero.json").unwrap();
        assert_eq!(number_double_close_to_zero.as_array().unwrap()[0].is_number(), true);
        assert_ne!(number_double_close_to_zero.as_array().unwrap()[0].to_float().unwrap(), 0.0);
        let number_int_with_exp = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_int_with_exp.json").unwrap();
        assert_eq!(number_int_with_exp.as_array().unwrap()[0].is_number(), true);
        assert_eq!(number_int_with_exp.as_array().unwrap()[0].to_float().unwrap(), 200.0);
        let number_minus_zero = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_minus_zero.json").unwrap();
        assert_eq!(number_minus_zero.as_array().unwrap()[0].is_number(), true);
        assert_eq!(number_minus_zero.as_array().unwrap()[0].to_float().unwrap(), -0.0);
        assert_eq!(number_minus_zero.as_array().unwrap()[0].to_float().unwrap(), 0.0);
        let number_negative_int = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_negative_int.json").unwrap();
        assert_eq!(number_negative_int.as_array().unwrap()[0].is_number(), true);
        let number_negative_one = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_negative_one.json").unwrap();
        assert_eq!(number_negative_one.as_array().unwrap()[0].is_number(), true);
        let number_negative_zero = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_negative_zero.json").unwrap();
        assert_eq!(number_negative_zero.as_array().unwrap()[0].is_number(), true);
        let number_real_capital_e_neg_exp = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_capital_e_neg_exp.json").unwrap();
        assert_eq!(number_real_capital_e_neg_exp.as_array().unwrap()[0].is_number(), true);
        let number_real_capital_e_pos_exp = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_capital_e_pos_exp.json").unwrap();
        assert_eq!(number_real_capital_e_pos_exp.as_array().unwrap()[0].is_number(), true);
        let number_real_exponent = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_exponent.json").unwrap();
        assert_eq!(number_real_exponent.as_array().unwrap()[0].is_number(), true);
        let number_real_fraction_exponent = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_fraction_exponent.json").unwrap();
        assert_eq!(number_real_fraction_exponent.as_array().unwrap()[0].is_number(), true);
        let number_real_neg_exp = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_neg_exp.json").unwrap();
        assert_eq!(number_real_neg_exp.as_array().unwrap()[0].is_number(), true);
        let number_real_pos_exponent = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_pos_exponent.json").unwrap();
        assert_eq!(number_real_pos_exponent.as_array().unwrap()[0].is_number(), true);
        let number_simple_int = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_simple_int.json").unwrap();
        assert_eq!(number_simple_int.as_array().unwrap()[0].is_number(), true);
        assert_eq!(number_simple_int.as_array().unwrap()[0].to_int().unwrap(), 123);
        let number_simple_real = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_simple_real.json").unwrap();
        assert_eq!(number_simple_real.as_array().unwrap()[0].is_number(), true);
        assert_eq!(number_simple_real.as_array().unwrap()[0].to_float().unwrap(), 123.456789);

        let object = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object.json").unwrap();
        assert_eq!(object.is_object(), true);
        assert_eq!(object.as_object().unwrap().get("asd").unwrap().as_str().unwrap(), "sdf");
        assert_eq!(object.as_object().unwrap().get("dfg").unwrap().as_str().unwrap(), "fgh");
        let object_basic = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_basic.json").unwrap();
        assert!(object_basic.is_object());
        let object_duplicated_key = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_duplicated_key.json").unwrap();
        assert!(object_duplicated_key.is_object());
        assert!(object_duplicated_key.as_object().unwrap().len() == 1);
        let object_duplicated_key_and_value = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_duplicated_key_and_value.json").unwrap();
        assert!(object_duplicated_key_and_value.is_object());
        assert!(object_duplicated_key_and_value.as_object().unwrap().len() == 1);
        let object_empty = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_empty.json").unwrap();
        assert!(object_empty.is_object());
        let object_empty_key = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_empty_key.json").unwrap();
        assert!(object_empty_key.is_object());
        let object_escaped_null_in_key = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_escaped_null_in_key.json").unwrap();
        assert!(object_escaped_null_in_key.is_object());
        let object_extreme_numbers = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_extreme_numbers.json").unwrap();
        assert!(object_extreme_numbers.is_object());
        let object_long_strings = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_long_strings.json").unwrap();
        assert!(object_long_strings.is_object());
        let object_simple = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_simple.json").unwrap();
        assert!(object_simple.is_object());
        let object_string_unicode = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_string_unicode.json").unwrap();
        assert!(object_string_unicode.is_object());
        let object_with_newlines = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_with_newlines.json").unwrap();
        assert!(object_with_newlines.is_object());

        let string_1_2_3_bytges_utf_8_sequences = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_1_2_3_bytes_UTF-8_sequences.json").unwrap();
        assert!(string_1_2_3_bytges_utf_8_sequences.as_array().unwrap()[0].is_string());
        assert_eq!(string_1_2_3_bytges_utf_8_sequences.as_array().unwrap()[0].as_str().unwrap(), "\u{0060}\u{012a}\u{12AB}");
        let string_accepted_surrogate_pair = read_json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_accepted_surrogate_pair.json").unwrap();
        assert!(string_accepted_surrogate_pair.as_array().unwrap()[0].is_string());
        println!("{}", string_accepted_surrogate_pair.as_array().unwrap()[0].as_str().unwrap());

    }
}
