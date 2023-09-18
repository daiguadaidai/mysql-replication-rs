#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use crate::mysql::compare_server_versions;
    use std::cmp::Ordering;

    #[test]
    fn test_compare_server_versions() -> Result<(), ReplicationError> {
        struct Case {
            pub a: String,
            pub b: String,
            pub expect: Ordering,
        }
        let tests = vec![
            Case {
                a: "1.2.3".to_string(),
                b: "1.2.3".to_string(),
                expect: Ordering::Equal,
            },
            Case {
                a: "5.6-999".to_string(),
                b: "8.0".to_string(),
                expect: Ordering::Less,
            },
            Case {
                a: "8.0.32-0ubuntu0.20.04.2".to_string(),
                b: "8.0.28".to_string(),
                expect: Ordering::Greater,
            },
        ];

        for test in &tests {
            let got = compare_server_versions(&test.a, &test.b)?;
            assert_eq!(test.expect, got);
        }

        Ok(())
    }
}
