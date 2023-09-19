#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use crate::mysql::compare_server_versions;
    use std::cmp::Ordering;

    #[test]
    fn b() {
        let a = -1_i8;
        let b = a as i64;
        let c = b as u64;
        println!("{}, {}, {}", a, b, c);
    }

    #[test]
    fn a() {
        let f = 0.1_f64;
        let i = f as u64;
        println!("{}", f);
        println!("{}", i);

        let ii = f.to_bits();
        println!("{}", ii);
        let b = f64::from_bits(ii);
        println!("{}", b);
    }

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
