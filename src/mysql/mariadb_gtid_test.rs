#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use crate::init_log;
    use crate::mysql::{GTIDSet, MariadbGTID, MariadbGTIDSet};
    use std::collections::HashMap;

    #[test]
    fn test_parse_mariadb_gtid() -> Result<(), ReplicationError> {
        struct Case {
            pub gtid_str: String,
            pub hash_error: bool,
        }

        let cases = vec![
            Case {
                gtid_str: "0-1-1".to_string(),
                hash_error: false,
            },
            Case {
                gtid_str: "".to_string(),
                hash_error: false,
            },
            Case {
                gtid_str: "0-1-1-1".to_string(),
                hash_error: true,
            },
            Case {
                gtid_str: "1".to_string(),
                hash_error: true,
            },
            Case {
                gtid_str: "0-1-seq".to_string(),
                hash_error: true,
            },
        ];

        for cs in &cases {
            let gtid = MariadbGTID::parse_gtid(&cs.gtid_str);
            if cs.hash_error {
                assert_eq!(gtid.is_err(), true)
            } else {
                assert_eq!(&cs.gtid_str, &gtid.unwrap().to_string())
            }
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_conatin() -> Result<(), ReplicationError> {
        struct Case {
            pub origin_gtid_str: String,
            pub other_gtid_str: String,
            pub contain: bool,
        }
        let cases = vec![
            Case {
                origin_gtid_str: "0-1-1".to_string(),
                other_gtid_str: "0-1-2".to_string(),
                contain: false,
            },
            Case {
                origin_gtid_str: "0-1-1".to_string(),
                other_gtid_str: "".to_string(),
                contain: true,
            },
            Case {
                origin_gtid_str: "2-1-1".to_string(),
                other_gtid_str: "1-1-1".to_string(),
                contain: false,
            },
            Case {
                origin_gtid_str: "1-2-1".to_string(),
                other_gtid_str: "1-1-1".to_string(),
                contain: true,
            },
            Case {
                origin_gtid_str: "1-2-2".to_string(),
                other_gtid_str: "1-1-1".to_string(),
                contain: true,
            },
        ];

        for cs in &cases {
            let origin_grid = MariadbGTID::parse_gtid(&cs.origin_gtid_str)?;
            let other_grid = MariadbGTID::parse_gtid(&cs.other_gtid_str)?;
            assert_eq!(cs.contain, origin_grid.contain(&other_grid))
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_clone() -> Result<(), ReplicationError> {
        let gtid = MariadbGTID::parse_gtid("1-1-1")?;

        let gtid_clone = gtid.clone();

        assert_eq!(&gtid, &gtid_clone);

        Ok(())
    }

    #[test]
    fn test_mariadb_forward() -> Result<(), ReplicationError> {
        init_log()?;

        struct Case {
            pub current_gtid_str: String,
            pub newer_gtid_str: String,
            pub hash_error: bool,
        }
        let cases = vec![
            Case {
                current_gtid_str: "0-1-1".to_string(),
                newer_gtid_str: "0-1-2".to_string(),
                hash_error: false,
            },
            Case {
                current_gtid_str: "0-1-1".to_string(),
                newer_gtid_str: "".to_string(),
                hash_error: false,
            },
            Case {
                current_gtid_str: "2-1-1".to_string(),
                newer_gtid_str: "1-1-1".to_string(),
                hash_error: true,
            },
            Case {
                current_gtid_str: "1-2-1".to_string(),
                newer_gtid_str: "1-1-1".to_string(),
                hash_error: false,
            },
            Case {
                current_gtid_str: "1-2-2".to_string(),
                newer_gtid_str: "1-1-1".to_string(),
                hash_error: false,
            },
        ];

        for cs in &cases {
            let mut current_gtid = MariadbGTID::parse_gtid(&cs.current_gtid_str)?;
            let newer_gtid = MariadbGTID::parse_gtid(&cs.newer_gtid_str)?;

            let _ = current_gtid.forward(&newer_gtid);
            if cs.hash_error {
                assert_eq!(&cs.current_gtid_str, &current_gtid.to_string())
            } else {
                assert_eq!(&cs.newer_gtid_str, &current_gtid.to_string())
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_mariadb_gtid_set() -> Result<(), ReplicationError> {
        struct Case {
            pub gtid_str: String,
            pub sub_gtids: HashMap<u32, String>, //domain ID => gtid string
            pub expected_str: Vec<String>,       // test String()
            pub has_error: bool,
        }
        let cases = vec![
            Case {
                gtid_str: "0-1-1".to_string(),
                sub_gtids: {
                    let keys = vec![0_u32];
                    let values = vec!["0-1-1".to_string()];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
                expected_str: vec!["0-1-1".to_string()],
                has_error: false,
            },
            Case {
                gtid_str: "".to_string(),
                sub_gtids: Default::default(),
                expected_str: vec!["".to_string()],
                has_error: false,
            },
            Case {
                gtid_str: "0-1-1,1-2-3".to_string(),
                sub_gtids: {
                    let keys = vec![0_u32, 1];
                    let values = vec!["0-1-1".to_string(), "1-2-3".to_string()];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
                expected_str: vec!["0-1-1,1-2-3".to_string(), "1-2-3,0-1-1".to_string()],
                has_error: false,
            },
            Case {
                gtid_str: "0-1--1".to_string(),
                sub_gtids: Default::default(),
                expected_str: vec![],
                has_error: true,
            },
        ];

        for cs in &cases {
            let gtid_set = MariadbGTIDSet::parse_gtid_set(&cs.gtid_str);
            if cs.has_error {
                assert_eq!(gtid_set.is_err(), true);
            } else {
                assert_eq!(gtid_set.is_ok(), true);
                let mariadb_gtid_set = gtid_set.unwrap();

                // check sub gtid
                assert_eq!(mariadb_gtid_set.sets.len(), cs.sub_gtids.len());
                for (domain_id, gtid) in &mariadb_gtid_set.sets {
                    assert_eq!(mariadb_gtid_set.sets.get(domain_id).is_some(), true);
                    assert_eq!(cs.sub_gtids.get(domain_id).is_some(), true);
                    assert_eq!(cs.sub_gtids.get(domain_id).unwrap(), &gtid.to_string())
                }

                // check String() function
                let mut in_expected_result = false;
                let actual_str = mariadb_gtid_set.to_string();
                for s in &cs.expected_str {
                    if s.eq(&actual_str) {
                        in_expected_result = true;
                        break;
                    }
                }
                assert_eq!(in_expected_result, true);
            }
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_set_update() -> Result<(), ReplicationError> {
        struct Case {
            pub is_nil_gtid: bool,
            pub gtid_str: String,
            pub sub_gtids: HashMap<u32, String>,
        }
        let cases = vec![
            Case {
                is_nil_gtid: true,
                gtid_str: "".to_string(),
                sub_gtids: {
                    let keys = vec![1_u32, 2];
                    let values = vec!["1-1-1".to_string(), "2-2-2".to_string()];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
            },
            Case {
                is_nil_gtid: false,
                gtid_str: "1-2-2".to_string(),
                sub_gtids: {
                    let keys = vec![1_u32, 2];
                    let values = vec!["1-2-2".to_string(), "2-2-2".to_string()];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
            },
            Case {
                is_nil_gtid: false,
                gtid_str: "1-2-1".to_string(),
                sub_gtids: {
                    let keys = vec![1_u32, 2];
                    let values = vec!["1-2-1".to_string(), "2-2-2".to_string()];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
            },
            Case {
                is_nil_gtid: false,
                gtid_str: "3-2-1".to_string(),
                sub_gtids: {
                    let keys = vec![1_u32, 2, 3];
                    let values = vec![
                        "1-1-1".to_string(),
                        "2-2-2".to_string(),
                        "3-2-1".to_string(),
                    ];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
            },
            Case {
                is_nil_gtid: false,
                gtid_str: "3-2-1,4-2-1".to_string(),
                sub_gtids: {
                    let keys = vec![1_u32, 2, 3, 4];
                    let values = vec![
                        "1-1-1".to_string(),
                        "2-2-2".to_string(),
                        "3-2-1".to_string(),
                        "4-2-1".to_string(),
                    ];
                    keys.into_iter()
                        .zip(values.into_iter())
                        .collect::<HashMap<u32, String>>()
                },
            },
        ];

        for cs in &cases {
            let gtid_set = MariadbGTIDSet::parse_gtid_set("1-1-1,2-2-2");
            assert_eq!(gtid_set.is_ok(), true);
            let mut mariadb_gtid_set = gtid_set.unwrap();

            if cs.is_nil_gtid {
            } else {
                let _ = mariadb_gtid_set.update(&cs.gtid_str);
            }

            // check sub gtid
            assert_eq!(mariadb_gtid_set.sets.len(), cs.sub_gtids.len());
            for (domain_id, gtid) in &mariadb_gtid_set.sets {
                assert_eq!(mariadb_gtid_set.sets.get(&domain_id).is_some(), true);
                let sub_set = cs.sub_gtids.get(&domain_id);
                assert_eq!(sub_set.is_some(), true);
                assert_eq!(sub_set.unwrap(), &gtid.to_string());
            }
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_set_equal() -> Result<(), ReplicationError> {
        struct Case {
            pub origin_gtid_str: String,
            pub other_gtid_str: String,
            pub equals: bool,
        }
        let cases = vec![
            Case {
                origin_gtid_str: "".to_string(),
                other_gtid_str: "".to_string(),
                equals: true,
            },
            Case {
                origin_gtid_str: "1-1-1".to_string(),
                other_gtid_str: "1-1-1,2-2-2".to_string(),
                equals: false,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1".to_string(),
                equals: false,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1,2-2-2".to_string(),
                equals: true,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1,2-2-3".to_string(),
                equals: false,
            },
        ];

        for cs in &cases {
            let origin_gtid = MariadbGTIDSet::parse_gtid_set(&cs.origin_gtid_str)?;
            let other_gtid = MariadbGTIDSet::parse_gtid_set(&cs.other_gtid_str)?;
            assert_eq!(origin_gtid.equal(&other_gtid), cs.equals);
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_set_contain() -> Result<(), ReplicationError> {
        struct Case {
            pub origin_gtid_str: String,
            pub other_gtid_str: String,
            pub contain: bool,
        }
        let cases = vec![
            Case {
                origin_gtid_str: "".to_string(),
                other_gtid_str: "".to_string(),
                contain: true,
            },
            Case {
                origin_gtid_str: "1-1-1".to_string(),
                other_gtid_str: "1-1-1,2-2-2".to_string(),
                contain: false,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1".to_string(),
                contain: true,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1,2-2-2".to_string(),
                contain: true,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1,2-2-1".to_string(),
                contain: true,
            },
            Case {
                origin_gtid_str: "1-1-1,2-2-2".to_string(),
                other_gtid_str: "1-1-1,2-2-3".to_string(),
                contain: false,
            },
        ];

        for cs in &cases {
            let origin_gtid_set = MariadbGTIDSet::parse_gtid_set(&cs.origin_gtid_str)?;
            let other_gtid_set = MariadbGTIDSet::parse_gtid_set(&cs.other_gtid_str)?;

            assert_eq!(cs.contain, origin_gtid_set.contain(&other_gtid_set))
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_set_clone() -> Result<(), ReplicationError> {
        let cases = vec!["", "1-1-1", "1-1-1,2-2-2"];

        for s in cases {
            let gtid_set = MariadbGTIDSet::parse_gtid_set(s)?;

            assert_eq!(gtid_set.to_string(), gtid_set.clone().to_string());
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_set_sorted_string() -> Result<(), ReplicationError> {
        let cases = vec![
            vec!["", ""],
            vec!["1-1-1", "1-1-1"],
            vec!["2-2-2,1-1-1,3-2-1", "1-1-1,2-2-2,3-2-1"],
        ];

        for strs in &cases {
            let gtid_set = MariadbGTIDSet::parse_gtid_set(strs[0])?;

            assert_eq!(strs[1], gtid_set.to_string())
        }

        Ok(())
    }
}
