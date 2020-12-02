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
    fn test_conversion(pda_path: &str, cfg_path: &str) {
        let path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), pda_path);
        let config = parser::Config { filename: path };

        let cfg = parser::run(config).unwrap();

        let path_expected = format!("{}{}", env!("CARGO_MANIFEST_DIR"), cfg_path);
        let expected_result = fs::read_to_string(path_expected);

        assert_eq!(cfg, expected_result.unwrap());
    }

    #[test]
    fn test_conversion_1() {
        test_conversion("/tests/test.pda", "/tests/test.cfg");
    }
    #[test]
    fn test_conversion_2() {
        test_conversion("/tests/test2.pda", "/tests/test2.cfg");
    }
    #[test]
    fn test_conversion_3_fail() {
        test_conversion("/tests/test3.pda", "/tests/test3.cfg");
    }
    #[test]
    fn test_conversion_4_fail() {
        test_conversion("/tests/test4.pda", "/tests/test4.cfg");
    }
    #[test]
    fn test_conversion_5() {
        test_conversion("/tests/test5.pda", "/tests/test5.cfg");
    }
}
