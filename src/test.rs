#[cfg(test)]
mod tests {
    use crate::lib::parser;
    use crate::lib::pda;
    use std::fs;

    #[test]
    fn test_pda_creation() {
        let mut pda = pda::PDA::build();
        pda.set_start("foo".into());
        assert_eq!(pda.start_state, "foo".to_string());
    }

    /// Automatically tests conversion of a pda file to a cfg file
    fn test_conversion(pda_path: String, cfg_path: String) {
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), pda_path);
        let config = parser::Config { filename: path };

        let cfg = parser::run(config).unwrap();

        let path_expected = format!("{}{}", env!("CARGO_MANIFEST_DIR"), cfg_path);
        let expected_result = fs::read_to_string(path_expected);

        assert_eq!(cfg, expected_result.unwrap());
    }

    #[test]
    fn test_conversion_1() {
        test_conversion("/tests/test.pda".into(), "/tests/test.cfg".into());
    }
    #[test]
    fn test_conversion_2() {
        test_conversion("/tests/test2.pda".into(), "/tests/test2.cfg".into());
    }
    #[test]
    fn test_conversion_3() {
        test_conversion("/tests/test3.pda".into(), "/tests/test3.cfg".into());
    }
    #[test]
    fn test_conversion_4() {
        test_conversion("/tests/test4.pda".into(), "/tests/test4.cfg".into());
    }
    #[test]
    fn test_conversion_5() {
        test_conversion("/tests/test5.pda".into(), "/tests/test5.cfg".into());
    }
}
