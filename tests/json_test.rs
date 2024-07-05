#[cfg(test)]
mod json_tests {
    use mawu::read::json;
    use pretty_assertions::assert_eq;

    // This is implicitly testing a lot of stuff!
    // But mainly testing the optimisations of the JSON parser
    #[test]
    #[ignore]
    fn large_json_file_26mb() {
        let large_result =
            json("data/json/json-test-data/large-file-json/large-file.json").unwrap();
        assert_eq!(large_result.is_array(), true);
        assert_eq!(large_result.as_array().unwrap().len(), 11351);
    }

    #[test]
    #[ignore]
    fn my_own_random_data_26mb() {
        let mawu_result = json("data/json/json-test-data/test_data_160k.json").unwrap();
        assert_eq!(mawu_result.is_array(), true);
        assert_eq!(mawu_result.as_array().unwrap().len(), 160000);
    }

    #[test]
    #[ignore]
    fn speed_test() {
        let mawu_result = json("data/json/json-test-data/test_data_517k.json").unwrap();
        assert_eq!(mawu_result.is_array(), true);
        assert_eq!(mawu_result.as_array().unwrap().len(), 517000);
    }

    #[test]
    #[ignore]
    fn my_own_random_data_84mb() {
        let mawu_result = json("data/json/json-test-data/test_data_517k.json").unwrap();
        assert_eq!(mawu_result.is_array(), true);
        assert_eq!(mawu_result.as_array().unwrap().len(), 517000);
    }

    #[test]
    fn simple_valid_json() {
        let simple_result = json("data/json/json-test-data/simple-json.json").unwrap();
        let tmp_simple_bind = simple_result.as_object().unwrap();
        assert_eq!(tmp_simple_bind.len(), 1);
        let tmp_quiz = tmp_simple_bind.get("quiz").unwrap().as_object().unwrap();
        // Sports
        let sports = tmp_quiz.get("sport").unwrap().as_object().unwrap();
        let sport_q1 = sports.get("q1").unwrap().as_object().unwrap();
        let s_q1_question = sport_q1.get("question").unwrap().as_str().unwrap();
        assert_eq!(
            s_q1_question,
            "Which one is correct team name in 1. Bundesliga?"
        );
        let s_q1_options = sport_q1.get("options").unwrap();
        assert_eq!(s_q1_options.as_array().unwrap().len(), 4);
        assert_eq!(
            s_q1_options.as_array().unwrap()[0].as_str().unwrap(),
            "2. Fc Bayern"
        );
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

        let very_simple_result = json("data/json/json-test-data/very-simple-json.json").unwrap();
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
        let rfc8259_array = json("data/json/json-test-data/rfc8259-test-data/array.json").unwrap();
        let rfc8259_object =
            json("data/json/json-test-data/rfc8259-test-data/object.json").unwrap();
        let rfc8259_string =
            json("data/json/json-test-data/rfc8259-test-data/small-text1.json").unwrap();
        let rfc8259_num =
            json("data/json/json-test-data/rfc8259-test-data/small-text2.json").unwrap();
        let rfc8259_bool =
            json("data/json/json-test-data/rfc8259-test-data/small-text3.json").unwrap();
        assert_eq!(rfc8259_array.is_array(), true);
        assert_eq!(rfc8259_array.as_array().unwrap().len(), 2);
        assert_eq!(rfc8259_object.is_object(), true);
        assert_eq!(rfc8259_object.as_object().unwrap().len(), 1);
        assert_eq!(
            rfc8259_object
                .as_object()
                .unwrap()
                .get("Image")
                .unwrap()
                .as_object()
                .unwrap()
                .len(),
            6
        );
        assert_eq!(rfc8259_string.is_string(), true);
        assert_eq!(
            rfc8259_string.as_string().unwrap(),
            &"Hello world!".to_string()
        );
        assert_eq!(rfc8259_num.is_uint(), true);
        assert_eq!(rfc8259_num.as_uint().unwrap(), &42);
        assert_eq!(rfc8259_bool.is_bool(), true);
        assert_eq!(rfc8259_bool.as_bool().unwrap(), &true);
    }

    #[test]
    fn json_org_valid_json() {
        let small_weird_json =
            json("data/json/json-test-data/jsonOrg-json-examples/small-weird-json.json").unwrap();
        let small_simple_json =
            json("data/json/json-test-data/jsonOrg-json-examples/small-simple-json.json").unwrap();
        let small_complex_json =
            json("data/json/json-test-data/jsonOrg-json-examples/small-complex-json.json").unwrap();
        let medium_complex_json =
            json("data/json/json-test-data/jsonOrg-json-examples/medium-complex-json.json")
                .unwrap();
        let large_complex_json =
            json("data/json/json-test-data/jsonOrg-json-examples/large-complex-json.json").unwrap();
        assert_eq!(small_weird_json.is_object(), true);
        assert_eq!(small_simple_json.is_object(), true);
        assert_eq!(small_complex_json.is_object(), true);
        assert_eq!(medium_complex_json.is_object(), true);
        assert_eq!(large_complex_json.is_object(), true);
    }

    #[test]
    fn microsoft_edge_valid_dummy_json_small() {
        let micro_64kb =
            json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/64KB.json")
                .unwrap();
        assert_eq!(micro_64kb.is_array(), true);
        let micro_64kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/64KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_64kb_mini.is_array(), true);

        let micro_128kb = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/128KB.json",
        )
        .unwrap();
        assert_eq!(micro_128kb.is_array(), true);
        let micro_128kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/128KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_128kb_mini.is_array(), true);

        let micro_256kb = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/256KB.json",
        )
        .unwrap();
        assert_eq!(micro_256kb.is_array(), true);
        let micro_256kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/256KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_256kb_mini.is_array(), true);

        let micro_512kb = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/512KB.json",
        )
        .unwrap();
        assert_eq!(micro_512kb.is_array(), true);
        let micro_512kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/512KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_512kb_mini.is_array(), true);
    }

    #[test]
    #[ignore]
    fn microsoft_edge_valid_dummy_json_large() {
        let micro_1mb =
            json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/1MB.json")
                .unwrap();
        assert_eq!(micro_1mb.is_array(), true);
        let micro_1mb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/1MB-min.json",
        )
        .unwrap();
        assert_eq!(micro_1mb_mini.is_array(), true);

        let micro_5mb =
            json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/5MB.json")
                .unwrap();
        assert_eq!(micro_5mb.is_array(), true);
        let micro_5mb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/5MB-min.json",
        )
        .unwrap();
        assert_eq!(micro_5mb_mini.is_array(), true);
    }

    #[test]
    /// Remember all of this errors!
    fn microsoft_edge_invalid_dummy_json() {
        let invalid_binary = json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/binary-data.json");
        assert_eq!(invalid_binary.is_err(), true);

        let invalid_missing_colon = json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/missing-colon.json");
        assert_eq!(invalid_missing_colon.is_err(), true);

        let unterminated = json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/unterminated.json");
        assert_eq!(unterminated.is_err(), true);
    }
    #[cfg(test)]
    mod json_test_suite {
        use mawu::read::json;
        use pretty_assertions::{assert_eq, assert_ne};

        #[test]
        fn transformation_numbers() {
            let number_10 = json(
                "data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1.0.json",
            )
            .unwrap();
            assert_eq!(number_10.as_array().unwrap().len(), 1);
            assert!(number_10.as_array().unwrap()[0].as_float().unwrap() == &1.0);
            let number_1000000000000000005 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1.000000000000000005.json").unwrap();
            assert_eq!(
                number_1000000000000000005.as_array().unwrap()[0]
                    .as_float()
                    .unwrap(),
                &1.0
            );
            let number_1e6 = json(
                "data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1e6.json",
            )
            .unwrap();
            assert_eq!(
                number_1e6.as_array().unwrap()[0].as_float().unwrap(),
                &1000000.0
            );
            let number_1e_999 = json(
                "data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1e-999.json",
            )
            .unwrap();
            assert_eq!(
                number_1e_999.as_array().unwrap()[0].as_float().unwrap(),
                &0.0
            );
            let number_1000000000000000 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1000000000000000.json").unwrap();
            assert_eq!(
                number_1000000000000000.as_array().unwrap()[0]
                    .as_uint()
                    .unwrap(),
                &1000000000000000
            );
            let number_9223372036854775807 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_9223372036854775807.json").unwrap();
            assert_eq!(
                number_9223372036854775807.as_array().unwrap()[0]
                    .as_uint()
                    .unwrap(),
                &9223372036854775807
            );
            let number_9223372036854775808 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_9223372036854775808.json").unwrap();
            assert_eq!(
                number_9223372036854775808.as_array().unwrap()[0]
                    .as_uint()
                    .unwrap(),
                &9223372036854775808
            );
            let number_10000000000000000999 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_10000000000000000999.json").unwrap();
            assert_eq!(
                number_10000000000000000999.as_array().unwrap()[0]
                    .as_uint()
                    .unwrap(),
                &10000000000000000999
            );
            let number_min_9223372036854775808 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_-9223372036854775808.json").unwrap();
            assert_eq!(
                number_min_9223372036854775808.as_array().unwrap()[0]
                    .as_int()
                    .unwrap(),
                &-9223372036854775808
            );
            let number_min_9223372036854775809 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_-9223372036854775809.json").unwrap();
            // below should be false? its e18, not e3
            assert_eq!(
                number_min_9223372036854775809.as_array().unwrap()[0]
                    .as_float()
                    .unwrap(),
                &-9223372036854776e3
            );
        }

        #[test]
        fn transformation_objects() {
            let object_key_nfc_nfd = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_key_nfc_nfd.json").unwrap();
            assert_eq!(object_key_nfc_nfd.as_object().unwrap().len(), 2);
            let object_key_nfd_nfc = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_key_nfd_nfc.json").unwrap();
            assert_eq!(object_key_nfd_nfc.as_object().unwrap().len(), 2);
            // overwrites as expected
            let object_same_key_different_values = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_same_key_different_values.json").unwrap();
            assert_eq!(
                object_same_key_different_values.as_object().unwrap().len(),
                1
            );
            assert!(
                object_same_key_different_values
                    .as_object()
                    .unwrap()
                    .get("a")
                    .unwrap()
                    .as_uint()
                    .unwrap()
                    == &2
            );
            let object_same_key_same_value = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_same_key_same_value.json").unwrap();
            assert_eq!(object_same_key_same_value.as_object().unwrap().len(), 1);
            assert!(
                object_same_key_same_value
                    .as_object()
                    .unwrap()
                    .get("a")
                    .unwrap()
                    .as_uint()
                    .unwrap()
                    == &1
            );
            let object_same_key_unclear_value = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_same_key_unclear_values.json").unwrap();
            assert_eq!(object_same_key_unclear_value.as_object().unwrap().len(), 1);
            assert!(
                object_same_key_unclear_value
                    .as_object()
                    .unwrap()
                    .get("a")
                    .unwrap()
                    .as_int()
                    .unwrap()
                    == &0
            );
        }

        #[test]
        fn transformation_strings() {
            let string_1_escaped_invalid_codepoint = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_1_escaped_invalid_codepoint.json");
            assert!(string_1_escaped_invalid_codepoint.is_err());
            let string_1_invalid_codepoint = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_1_invalid_codepoint.json");
            assert!(string_1_invalid_codepoint.is_err());
            let string_2_escaped_invalid_codepoint = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_2_escaped_invalid_codepoints.json");
            assert!(string_2_escaped_invalid_codepoint.is_err());
            let string_2_invalid_codepoints = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_2_invalid_codepoints.json");
            assert!(string_2_invalid_codepoints.is_err());
            let string_3_escaped_invalid_codepoint = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_3_escaped_invalid_codepoints.json");
            assert!(string_3_escaped_invalid_codepoint.is_err());
            let string_3_invalid_codepoints = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_3_invalid_codepoints.json");
            assert!(string_3_invalid_codepoints.is_err());
            let string_with_escaped_null = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/string_with_escaped_NULL.json");
            assert!(string_with_escaped_null.is_ok());
        }

        #[test]
        fn implementor_dependent_numbers() {
            // I accept underflow to 0.0 - documented
            let number_double_huge_neg_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_double_huge_neg_exp.json").unwrap();
            assert_eq!(
                number_double_huge_neg_exp.as_array().unwrap()[0].is_number(),
                true
            );
            assert!(
                number_double_huge_neg_exp.as_array().unwrap()[0]
                    .as_float()
                    .unwrap()
                    == &0.0
            );
            let number_real_underflow = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_real_underflow.json").unwrap();
            assert_eq!(number_real_underflow.as_array().unwrap().len(), 1);
            assert!(
                number_real_underflow.as_array().unwrap()[0]
                    .as_float()
                    .unwrap()
                    == &0.0
            );
            // I don't accept overflow to infinity - documented
            let number_huge_exp = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_huge_exp.json",
            )
            .unwrap();
            assert_eq!(number_huge_exp.as_array().unwrap().len(), 1);
            assert!(number_huge_exp.as_array().unwrap()[0].is_none());
            let number_neg_int_huge_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_neg_int_huge_exp.json").unwrap();
            assert_eq!(number_neg_int_huge_exp.as_array().unwrap().len(), 1);
            assert!(number_neg_int_huge_exp.as_array().unwrap()[0].is_none());
            let number_pos_double_huge_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_pos_double_huge_exp.json").unwrap();
            assert_eq!(number_pos_double_huge_exp.as_array().unwrap().len(), 1);
            assert!(number_pos_double_huge_exp.as_array().unwrap()[0].is_none());
            let number_real_pos_overflow = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_real_pos_overflow.json").unwrap();
            assert_eq!(number_real_pos_overflow.as_array().unwrap().len(), 1);
            assert!(number_real_pos_overflow.as_array().unwrap()[0].is_none());
            // I accept numbers that may be converted to fit, eg int to float
            let number_too_big_neg_int = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_too_big_neg_int.json").unwrap();
            assert_eq!(number_too_big_neg_int.as_array().unwrap().len(), 1);
            assert!(
                number_too_big_neg_int.as_array().unwrap()[0]
                    .as_float()
                    .unwrap()
                    == &-1.2312312312312312e29
            );
            let number_too_big_pos_int = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_too_big_pos_int.json").unwrap();
            assert_eq!(number_too_big_pos_int.as_array().unwrap().len(), 1);
            assert!(
                number_too_big_pos_int.as_array().unwrap()[0]
                    .as_float()
                    .unwrap()
                    == &1e20
            );
            let number_very_big_negative_int = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_very_big_negative_int.json").unwrap();
            assert_eq!(number_very_big_negative_int.as_array().unwrap().len(), 1);
            assert!(
                number_very_big_negative_int.as_array().unwrap()[0]
                    .as_float()
                    .unwrap()
                    == &-2.374623746732769e47
            );
        }

        #[test]
        fn implementor_dependent_strings() {
            // I don't accept missing or invalid surrogate pairs
            let string_1st_surrogate_but_2nd_missing = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_1st_surrogate_but_2nd_missing.json");
            assert!(string_1st_surrogate_but_2nd_missing.is_err());
            let string_1st_valid_surrogate_2nd_invalid = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_1st_valid_surrogate_2nd_invalid.json");
            assert!(string_1st_valid_surrogate_2nd_invalid.is_err());
            let string_incomplete_surrogate_and_escape_valid = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_incomplete_surrogate_and_escape_valid.json");
            assert!(string_incomplete_surrogate_and_escape_valid.is_err());
            let string_incomplete_surrogate_pair = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_incomplete_surrogate_pair.json");
            assert!(string_incomplete_surrogate_pair.is_err());
            let string_incomplete_surrogates_escape_valid = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_incomplete_surrogates_escape_valid.json");
            assert!(string_incomplete_surrogates_escape_valid.is_err());
            let string_invalid_lonely_surrogate = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_invalid_lonely_surrogate.json");
            assert!(string_invalid_lonely_surrogate.is_err());
            let string_invalid_surrogate = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_invalid_surrogate.json");
            assert!(string_invalid_surrogate.is_err());
            let string_invalid_utf8 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_invalid_utf-8.json");
            assert!(string_invalid_utf8.is_err());
            let string_inverted_surrogates_uplus1d11e = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_inverted_surrogates_U+1D11E.json");
            assert!(string_inverted_surrogates_uplus1d11e.is_err());
            let string_lone_second_surrogate = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_lone_second_surrogate.json");
            assert!(string_lone_second_surrogate.is_err());
            // I also don't accept Invalid utf8
            let string_iso_latin_1 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_iso_latin_1.json");
            assert!(string_iso_latin_1.is_err());
            let string_lone_utf8_continuation_byte = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_lone_utf8_continuation_byte.json");
            assert!(string_lone_utf8_continuation_byte.is_err());
            let string_truncated_utf8 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_truncated-utf-8.json");
            assert!(string_truncated_utf8.is_err());
            let string_utf16be_no_bom = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_utf16BE_no_BOM.json");
            assert!(string_utf16be_no_bom.is_err());
            let string_utf16le_no_bom = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_utf16LE_no_BOM.json");
            assert!(string_utf16le_no_bom.is_err());
            let string_utf8_surrogate_uplusd800 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_UTF8_surrogate_U+D800.json");
            assert!(string_utf8_surrogate_uplusd800.is_err());
            let string_utf_8_invalid_sequence = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_UTF-8_invalid_sequence.json");
            assert!(string_utf_8_invalid_sequence.is_err());
            let string_utf_16le_with_bom = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_UTF-16LE_with_BOM.json");
            assert!(string_utf_16le_with_bom.is_err());
            // I don't accept invalid unicode
            let string_not_in_unicode_range = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_not_in_unicode_range.json");
            assert!(string_not_in_unicode_range.is_err());
            let string_overlong_sequence_2_bytes = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_overlong_sequence_2_bytes.json");
            assert!(string_overlong_sequence_2_bytes.is_err());
            let string_overlong_sequence_6_bytes = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_overlong_sequence_6_bytes.json");
            assert!(string_overlong_sequence_6_bytes.is_err());
            let string_overlong_sequence_6_bytes_null = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_string_overlong_sequence_6_bytes_null.json");
            assert!(string_overlong_sequence_6_bytes_null.is_err());
        }

        #[test]
        fn implementor_dependent_structures() {
            // Nest as much as you want!
            let structure_500_nested_arrays = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_structure_500_nested_arrays.json");
            assert!(structure_500_nested_arrays.is_ok());
            let mut bind = structure_500_nested_arrays.unwrap().clone();
            let mut count = 1;
            while bind.is_array() && count < 500 {
                bind = bind.as_array().unwrap()[0].clone();
                assert!(bind.is_array());
                count += 1;
            }
            // No BOM support!
            let structure_utf_8_bom_empty_object = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_structure_UTF-8_BOM_empty_object.json");
            assert!(structure_utf_8_bom_empty_object.is_err());
        }

        #[test]
        fn implementor_dependent_objects() {
            let object_key_lone_2nd_surrogate = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_object_key_lone_2nd_surrogate.json");
            assert!(object_key_lone_2nd_surrogate.is_err());
        }

        #[test]
        fn valid_arrays() {
            let arrays_with_spaces = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_arraysWithSpaces.json").unwrap();
            assert_eq!(arrays_with_spaces.is_array(), true);
            let array_empty =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_empty.json")
                    .unwrap();
            assert_eq!(array_empty.is_array(), true);
            let array_empty_string = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_empty-string.json").unwrap();
            assert_eq!(array_empty_string.is_array(), true);
            let array_ending_with_newline = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_ending_with_newline.json").unwrap();
            assert_eq!(array_ending_with_newline.is_array(), true);
            let array_false =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_false.json")
                    .unwrap();
            assert_eq!(array_false.is_array(), true);
            let array_heterogeneous = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_heterogeneous.json").unwrap();
            assert_eq!(array_heterogeneous.is_array(), true);
            let array_null =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_null.json")
                    .unwrap();
            assert_eq!(array_null.is_array(), true);
            assert_eq!(array_null.as_array().unwrap()[0].is_none(), true);
            let array_with_1_and_newline = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_1_and_newline.json").unwrap();
            assert_eq!(array_with_1_and_newline.is_array(), true);
            let array_with_leading_space = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_leading_space.json").unwrap();
            assert_eq!(array_with_leading_space.is_array(), true);
            let array_with_several_null = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_several_null.json").unwrap();
            assert_eq!(array_with_several_null.is_array(), true);
            assert_eq!(
                array_with_several_null.as_array().unwrap()[0].is_number(),
                true
            );
            assert_eq!(
                array_with_several_null.as_array().unwrap()[1].is_none(),
                true
            );
            assert_eq!(
                array_with_several_null.as_array().unwrap()[2].is_none(),
                true
            );
            assert_eq!(
                array_with_several_null.as_array().unwrap()[3].is_none(),
                true
            );
            assert_eq!(
                array_with_several_null.as_array().unwrap()[4].is_number(),
                true
            );
            let array_with_trailing_space = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_with_trailing_space.json").unwrap();
            assert_eq!(array_with_trailing_space.is_array(), true);
        }

        #[test]
        fn valid_numbers() {
            let number =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number.json")
                    .unwrap();
            assert_eq!(number.as_array().unwrap()[0].is_number(), true);
            let number_0e1 =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_0e1.json")
                    .unwrap();
            assert_eq!(number_0e1.as_array().unwrap()[0].is_number(), true);
            let number_0eplusone =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_0e+1.json")
                    .unwrap();
            assert_eq!(number_0eplusone.as_array().unwrap()[0].is_number(), true);
            let number_after_space = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_after_space.json").unwrap();
            assert_eq!(number_after_space.as_array().unwrap()[0].is_number(), true);
            let number_double_close_to_zero = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_double_close_to_zero.json").unwrap();
            assert_eq!(
                number_double_close_to_zero.as_array().unwrap()[0].is_number(),
                true
            );
            assert_ne!(
                number_double_close_to_zero.as_array().unwrap()[0]
                    .to_float()
                    .unwrap(),
                0.0
            );
            let number_int_with_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_int_with_exp.json").unwrap();
            assert_eq!(number_int_with_exp.as_array().unwrap()[0].is_number(), true);
            assert_eq!(
                number_int_with_exp.as_array().unwrap()[0]
                    .to_float()
                    .unwrap(),
                200.0
            );
            let number_minus_zero = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_minus_zero.json",
            )
            .unwrap();
            assert_eq!(number_minus_zero.as_array().unwrap()[0].is_number(), true);
            assert_eq!(
                number_minus_zero.as_array().unwrap()[0].to_float().unwrap(),
                -0.0
            );
            assert_eq!(
                number_minus_zero.as_array().unwrap()[0].to_float().unwrap(),
                0.0
            );
            let number_negative_int = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_negative_int.json").unwrap();
            assert_eq!(number_negative_int.as_array().unwrap()[0].is_number(), true);
            let number_negative_one = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_negative_one.json").unwrap();
            assert_eq!(number_negative_one.as_array().unwrap()[0].is_number(), true);
            let number_negative_zero = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_negative_zero.json").unwrap();
            assert_eq!(
                number_negative_zero.as_array().unwrap()[0].is_number(),
                true
            );
            let number_real_capital_e_neg_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_capital_e_neg_exp.json").unwrap();
            assert_eq!(
                number_real_capital_e_neg_exp.as_array().unwrap()[0].is_number(),
                true
            );
            let number_real_capital_e_pos_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_capital_e_pos_exp.json").unwrap();
            assert_eq!(
                number_real_capital_e_pos_exp.as_array().unwrap()[0].is_number(),
                true
            );
            let number_real_exponent = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_exponent.json").unwrap();
            assert_eq!(
                number_real_exponent.as_array().unwrap()[0].is_number(),
                true
            );
            let number_real_fraction_exponent = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_fraction_exponent.json").unwrap();
            assert_eq!(
                number_real_fraction_exponent.as_array().unwrap()[0].is_number(),
                true
            );
            let number_real_neg_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_neg_exp.json").unwrap();
            assert_eq!(number_real_neg_exp.as_array().unwrap()[0].is_number(), true);
            let number_real_pos_exponent = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_real_pos_exponent.json").unwrap();
            assert_eq!(
                number_real_pos_exponent.as_array().unwrap()[0].is_number(),
                true
            );
            let number_simple_int = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_simple_int.json",
            )
            .unwrap();
            assert_eq!(number_simple_int.as_array().unwrap()[0].is_number(), true);
            assert_eq!(
                number_simple_int.as_array().unwrap()[0].to_int().unwrap(),
                123
            );
            let number_simple_real = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_number_simple_real.json").unwrap();
            assert_eq!(number_simple_real.as_array().unwrap()[0].is_number(), true);
            assert_eq!(
                number_simple_real.as_array().unwrap()[0]
                    .to_float()
                    .unwrap(),
                123.456789
            );
        }

        #[test]
        fn valid_objects() {
            let object =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object.json")
                    .unwrap();
            assert_eq!(object.is_object(), true);
            assert_eq!(
                object
                    .as_object()
                    .unwrap()
                    .get("asd")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "sdf"
            );
            assert_eq!(
                object
                    .as_object()
                    .unwrap()
                    .get("dfg")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "fgh"
            );
            let object_basic = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_basic.json",
            )
            .unwrap();
            assert!(object_basic.is_object());
            let object_duplicated_key = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_duplicated_key.json").unwrap();
            assert!(object_duplicated_key.is_object());
            assert!(object_duplicated_key.as_object().unwrap().len() == 1);
            let object_duplicated_key_and_value = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_duplicated_key_and_value.json").unwrap();
            assert!(object_duplicated_key_and_value.is_object());
            assert!(object_duplicated_key_and_value.as_object().unwrap().len() == 1);
            let object_empty = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_empty.json",
            )
            .unwrap();
            assert!(object_empty.is_object());
            let object_empty_key = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_empty_key.json",
            )
            .unwrap();
            assert!(object_empty_key.is_object());
            let object_escaped_null_in_key = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_escaped_null_in_key.json").unwrap();
            assert!(object_escaped_null_in_key.is_object());
            let object_extreme_numbers = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_extreme_numbers.json").unwrap();
            assert!(object_extreme_numbers.is_object());
            let object_long_strings = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_long_strings.json").unwrap();
            assert!(object_long_strings.is_object());
            let object_simple = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_simple.json",
            )
            .unwrap();
            assert!(object_simple.is_object());
            let object_string_unicode = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_string_unicode.json").unwrap();
            assert!(object_string_unicode.is_object());
            let object_with_newlines = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object_with_newlines.json").unwrap();
            assert!(object_with_newlines.is_object());
        }

        #[test]
        fn valid_strings() {
            let string_1_2_3_bytges_utf_8_sequences = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_1_2_3_bytes_UTF-8_sequences.json").unwrap();
            assert!(string_1_2_3_bytges_utf_8_sequences.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_1_2_3_bytges_utf_8_sequences.as_array().unwrap()[0]
                    .as_str()
                    .unwrap(),
                "\u{0060}\u{012a}\u{12AB}"
            );
            let string_accepted_surrogate_pair = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_accepted_surrogate_pair.json").unwrap();
            assert!(string_accepted_surrogate_pair.as_array().unwrap()[0].is_string());
            let string_accepted_surrogate_pairs = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_accepted_surrogate_pairs.json").unwrap();
            assert!(string_accepted_surrogate_pairs.as_array().unwrap()[0].is_string());
            let string_allowed_escapes = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_allowed_escapes.json").unwrap();
            assert!(string_allowed_escapes.as_array().unwrap()[0].is_string());
            let string_backslash_and_u_escaped_zero = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_backslash_and_u_escaped_zero.json").unwrap();
            assert!(string_backslash_and_u_escaped_zero.as_array().unwrap()[0].is_string());
            let string_backslash_doublequotes = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_backslash_doublequotes.json").unwrap();
            assert!(string_backslash_doublequotes.as_array().unwrap()[0].is_string());
            let string_comments = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_comments.json",
            )
            .unwrap();
            assert!(string_comments.as_array().unwrap()[0].is_string());
            let string_double_escape_a = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_double_escape_a.json").unwrap();
            assert!(string_double_escape_a.as_array().unwrap()[0].is_string());
            let string_double_escape_n = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_double_escape_n.json").unwrap();
            assert!(string_double_escape_n.as_array().unwrap()[0].is_string());
            let string_escaped_control_character = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_escaped_control_character.json").unwrap();
            assert!(string_escaped_control_character.as_array().unwrap()[0].is_string());
            let string_escaped_noncharacter = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_escaped_noncharacter.json").unwrap();
            assert!(string_escaped_noncharacter.as_array().unwrap()[0].is_string());
            let string_in_array = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_in_array.json",
            )
            .unwrap();
            assert!(string_in_array.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_in_array.as_array().unwrap()[0].as_str().unwrap(),
                "asd"
            );
            let string_in_array_with_leading_space = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_in_array_with_leading_space.json").unwrap();
            assert!(string_in_array_with_leading_space.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_in_array_with_leading_space.as_array().unwrap()[0]
                    .as_str()
                    .unwrap(),
                "asd"
            );
            let string_last_surrogates_1_and_2 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_last_surrogates_1_and_2.json").unwrap();
            assert!(string_last_surrogates_1_and_2.as_array().unwrap()[0].is_string());
            let string_nbsp_uescaped = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_nbsp_uescaped.json").unwrap();
            assert!(string_nbsp_uescaped.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_nbsp_uescaped.as_array().unwrap()[0]
                    .as_str()
                    .unwrap(),
                "new\u{00A0}line"
            );
            let string_noncharacterinutf8_uplus10ffff = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_nonCharacterInUTF-8_U+10FFFF.json").unwrap();
            assert!(string_noncharacterinutf8_uplus10ffff.as_array().unwrap()[0].is_string());
            let string_null_escape = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_null_escape.json").unwrap();
            assert!(string_null_escape.as_array().unwrap()[0].is_string());
            let string_one_byte_utf8 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_one-byte-utf-8.json").unwrap();
            assert!(string_one_byte_utf8.as_array().unwrap()[0].is_string());
            let string_pi =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_pi.json")
                    .unwrap();
            assert!(string_pi.as_array().unwrap()[0].is_string());
            let string_reservedcharacterinutf_8_uplus1bfff = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_reservedCharacterInUTF-8_U+1BFFF.json").unwrap();
            assert!(string_reservedcharacterinutf_8_uplus1bfff
                .as_array()
                .unwrap()[0]
                .is_string());
            let string_simple_ascii = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_simple_ascii.json").unwrap();
            assert!(string_simple_ascii.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_simple_ascii.as_array().unwrap()[0].as_str().unwrap(),
                "asd "
            );
            let string_space = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_space.json",
            )
            .unwrap();
            assert!(string_space.is_string());
            assert_eq!(string_space.as_str().unwrap(), " ");
            let string_surrogates_uplus1d11e_cleef = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_surrogates_U+1D11E_MUSICAL_SYMBOL_G_CLEF.json").unwrap();
            assert!(string_surrogates_uplus1d11e_cleef.as_array().unwrap()[0].is_string());
            let string_three_byte_utf8 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_three-byte-utf-8.json").unwrap();
            assert!(string_three_byte_utf8.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_three_byte_utf8.as_array().unwrap()[0]
                    .as_str()
                    .unwrap(),
                "\u{0821}"
            );
            let string_two_byte_utf8 = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_two-byte-utf-8.json").unwrap();
            assert!(string_two_byte_utf8.as_array().unwrap()[0].is_string());
            assert_eq!(
                string_two_byte_utf8.as_array().unwrap()[0]
                    .as_str()
                    .unwrap(),
                "\u{0123}"
            );
            let string_uplus2028_line_sep = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_u+2028_line_sep.json").unwrap();
            assert!(string_uplus2028_line_sep.as_array().unwrap()[0].is_string());
            let string_uplus2029_para_sep = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_u+2029_par_sep.json").unwrap();
            assert!(string_uplus2029_para_sep.as_array().unwrap()[0].is_string());
            let string_uescape = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_uEscape.json",
            )
            .unwrap();
            assert!(string_uescape.as_array().unwrap()[0].is_string());
            let string_uescaped_newline = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_uescaped_newline.json").unwrap();
            assert!(string_uescaped_newline.as_array().unwrap()[0].is_string());
            let string_unescaped_char_delete = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unescaped_char_delete.json").unwrap();
            assert!(string_unescaped_char_delete.as_array().unwrap()[0].is_string());
            let string_unicode = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode.json",
            )
            .unwrap();
            assert!(string_unicode.as_array().unwrap()[0].is_string());
            let string_unicode2 = json(
                "data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_2.json",
            )
            .unwrap();
            assert!(string_unicode2.as_array().unwrap()[0].is_string());
            let string_unicodeescapedbackslash = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicodeEscapedBackslash.json").unwrap();
            assert!(string_unicodeescapedbackslash.as_array().unwrap()[0].is_string());
            let string_unicode_escaped_double_quote = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_escaped_double_quote.json").unwrap();
            assert!(string_unicode_escaped_double_quote.as_array().unwrap()[0].is_string());
            let string_unicode_uplus1fffe_nonchar = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_U+1FFFE_nonchar.json").unwrap();
            assert!(string_unicode_uplus1fffe_nonchar.as_array().unwrap()[0].is_string());
            let string_unicode_uplus10fffe_nonchar = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_U+10FFFE_nonchar.json").unwrap();
            assert!(string_unicode_uplus10fffe_nonchar.as_array().unwrap()[0].is_string());
            let string_unicode_uplus200b_zero_width_space = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_U+200B_ZERO_WIDTH_SPACE.json").unwrap();
            assert!(string_unicode_uplus200b_zero_width_space
                .as_array()
                .unwrap()[0]
                .is_string());
            let string_unicode_uplus2064_invisible_plus = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_U+2064_invisible_plus.json").unwrap();
            assert!(string_unicode_uplus2064_invisible_plus.as_array().unwrap()[0].is_string());
            let string_unicode_uplusfddo_nonchar = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_U+FDD0_nonchar.json").unwrap();
            assert!(string_unicode_uplusfddo_nonchar.as_array().unwrap()[0].is_string());
            let string_unicode_uplusfffe_nonchar = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_unicode_U+FFFE_nonchar.json").unwrap();
            assert!(string_unicode_uplusfffe_nonchar.as_array().unwrap()[0].is_string());
            let string_utf8 =
                json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_utf8.json")
                    .unwrap();
            assert!(string_utf8.as_array().unwrap()[0].is_string());
            let string_with_del_character = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_string_with_del_character.json").unwrap();
            assert!(string_with_del_character.as_array().unwrap()[0].is_string());
        }

        #[test]
        fn valid_structures() {
            let structure_lonely_false = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_lonely_false.json").unwrap();
            assert!(structure_lonely_false.as_bool().unwrap() == &false);
            let structure_lonely_int = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_lonely_int.json").unwrap();
            assert!(structure_lonely_int.as_uint().unwrap() == &42);
            let structure_lonely_negative_real = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_lonely_negative_real.json").unwrap();
            assert!(structure_lonely_negative_real.as_float().unwrap() == &-0.1);
            let structure_lonely_null = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_lonely_null.json").unwrap();
            assert!(structure_lonely_null.is_none());
            let structure_lonely_string = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_lonely_string.json").unwrap();
            assert!(structure_lonely_string.as_str().unwrap() == "asd");
            let structure_lonely_true = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_lonely_true.json").unwrap();
            assert!(structure_lonely_true.as_bool().unwrap() == &true);
            let structure_string_empty = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_string_empty.json").unwrap();
            assert!(structure_string_empty.as_str().unwrap() == "");
            let structure_trailing_newline = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_trailing_newline.json").unwrap();
            assert!(structure_trailing_newline.as_array().unwrap()[0].is_string());
            let structure_true_in_array = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_true_in_array.json").unwrap();
            assert!(structure_true_in_array.as_array().unwrap()[0].is_true());
            let structure_whitespace_array = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_structure_whitespace_array.json").unwrap();
            assert!(structure_whitespace_array.is_array());
        }
    }
}
