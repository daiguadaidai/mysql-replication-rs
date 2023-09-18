#[cfg(test)]
mod tests {
    use crate::error::{MyError, ReplicationError};
    use crate::mysql::{
        length_encoded_int, GTIDSet, Interval, IntervalSlice, MysqlGTIDSet, ParseBinary, UUIDSet,
    };
    use uuid::Uuid;

    #[test]
    fn test_mysql_gtid_interval() -> Result<(), ReplicationError> {
        let i = Interval::parse_interval("1-2")?;
        assert_eq!(i, Interval { start: 1, stop: 3 });

        let i = Interval::parse_interval("1")?;
        assert_eq!(i, Interval { start: 1, stop: 2 });

        let i = Interval::parse_interval("1-1")?;
        assert_eq!(i, Interval { start: 1, stop: 2 });

        Ok(())
    }

    #[test]
    fn test_mysql_gtid_interval_slice() -> Result<(), ReplicationError> {
        let mut i = IntervalSlice {
            s: vec![
                Interval { start: 1, stop: 2 },
                Interval { start: 2, stop: 4 },
                Interval { start: 2, stop: 3 },
            ],
        };
        i.sort();
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval { start: 1, stop: 2 },
                    Interval { start: 2, stop: 3 },
                    Interval { start: 2, stop: 4 },
                ],
            }
        );
        let n = i.normalize();
        assert_eq!(
            n,
            IntervalSlice {
                s: vec![Interval { start: 1, stop: 4 }]
            }
        );

        let mut i = IntervalSlice {
            s: vec![
                Interval { start: 1, stop: 2 },
                Interval { start: 3, stop: 5 },
                Interval { start: 1, stop: 3 },
            ],
        };
        i.sort();
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval { start: 1, stop: 2 },
                    Interval { start: 1, stop: 3 },
                    Interval { start: 3, stop: 5 },
                ],
            }
        );
        let n = i.normalize();
        assert_eq!(
            n,
            IntervalSlice {
                s: vec![Interval { start: 1, stop: 5 }]
            }
        );

        let mut i = IntervalSlice {
            s: vec![
                Interval { start: 1, stop: 2 },
                Interval { start: 4, stop: 5 },
                Interval { start: 1, stop: 3 },
            ],
        };
        i.sort();
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval { start: 1, stop: 2 },
                    Interval { start: 1, stop: 3 },
                    Interval { start: 4, stop: 5 },
                ],
            }
        );
        let n = i.normalize();
        assert_eq!(
            n,
            IntervalSlice {
                s: vec![
                    Interval { start: 1, stop: 3 },
                    Interval { start: 4, stop: 5 },
                ]
            }
        );

        let mut i = IntervalSlice {
            s: vec![
                Interval { start: 1, stop: 4 },
                Interval { start: 2, stop: 3 },
            ],
        };
        i.sort();
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval { start: 1, stop: 4 },
                    Interval { start: 2, stop: 3 },
                ],
            }
        );
        let n = i.normalize();
        assert_eq!(
            n,
            IntervalSlice {
                s: vec![Interval { start: 1, stop: 4 }]
            }
        );

        let n1 = IntervalSlice {
            s: vec![
                Interval { start: 1, stop: 3 },
                Interval { start: 4, stop: 5 },
            ],
        };
        let n2 = IntervalSlice {
            s: vec![Interval { start: 1, stop: 2 }],
        };
        assert_eq!(true, n1.contain(&n2));
        assert_eq!(false, n2.contain(&n1));

        let n1 = IntervalSlice {
            s: vec![
                Interval { start: 1, stop: 3 },
                Interval { start: 4, stop: 5 },
            ],
        };
        let n2 = IntervalSlice {
            s: vec![Interval { start: 1, stop: 6 }],
        };
        assert_eq!(false, n1.contain(&n2));
        assert_eq!(true, n2.contain(&n1));

        Ok(())
    }

    #[test]
    fn test_mysql_gtid_insert_interval() -> Result<(), ReplicationError> {
        let mut i = IntervalSlice {
            s: vec![Interval {
                start: 100,
                stop: 200,
            }],
        };
        i.insert_interval(Interval {
            start: 300,
            stop: 400,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 100,
                        stop: 200,
                    },
                    Interval {
                        start: 300,
                        stop: 400,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 50,
            stop: 70,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 70,
                    },
                    Interval {
                        start: 100,
                        stop: 200,
                    },
                    Interval {
                        start: 300,
                        stop: 400,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 101,
            stop: 201,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 70,
                    },
                    Interval {
                        start: 100,
                        stop: 201,
                    },
                    Interval {
                        start: 300,
                        stop: 400,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 99,
            stop: 202,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 70,
                    },
                    Interval {
                        start: 99,
                        stop: 202,
                    },
                    Interval {
                        start: 300,
                        stop: 400,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 102,
            stop: 302,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 70,
                    },
                    Interval {
                        start: 99,
                        stop: 400,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 500,
            stop: 600,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 70,
                    },
                    Interval {
                        start: 99,
                        stop: 400,
                    },
                    Interval {
                        start: 500,
                        stop: 600,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 50,
            stop: 100,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 400,
                    },
                    Interval {
                        start: 500,
                        stop: 600,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 900,
            stop: 1000,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 400,
                    },
                    Interval {
                        start: 500,
                        stop: 600,
                    },
                    Interval {
                        start: 900,
                        stop: 1000,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 1010,
            stop: 1020,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 50,
                        stop: 400,
                    },
                    Interval {
                        start: 500,
                        stop: 600,
                    },
                    Interval {
                        start: 900,
                        stop: 1000,
                    },
                    Interval {
                        start: 1010,
                        stop: 1020,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 49,
            stop: 1000,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![
                    Interval {
                        start: 49,
                        stop: 1000,
                    },
                    Interval {
                        start: 1010,
                        stop: 1020,
                    }
                ],
            }
        );

        i.insert_interval(Interval {
            start: 1,
            stop: 1012,
        });
        assert_eq!(
            i,
            IntervalSlice {
                s: vec![Interval {
                    start: 1,
                    stop: 1020,
                }],
            }
        );

        Ok(())
    }

    #[test]
    fn test_mysql_gtid_codec() -> Result<(), ReplicationError> {
        let mut us = UUIDSet::parse_uuid_set("de278ad0-2106-11e4-9f8e-6edd0ca20947:1-2")?;
        assert_eq!("de278ad0-2106-11e4-9f8e-6edd0ca20947:1-2", &us.to_string());

        let buf = us.encode();
        us.decode(&buf)?;

        let gs = MysqlGTIDSet::parse_gtid_set(
            "de278ad0-2106-11e4-9f8e-6edd0ca20947:1-2,de278ad0-2106-11e4-9f8e-6edd0ca20948:1-2",
        )?;

        let buf = gs.encode()?;
        let o = MysqlGTIDSet::decode(&buf)?;
        assert_eq!(&gs, &o);
        Ok(())
    }

    #[test]
    fn test_mysql_update() -> Result<(), ReplicationError> {
        let mut g1 = MysqlGTIDSet::parse_gtid_set("3E11FA47-71CA-11E1-9E33-C80AA9429562:21-57")?;
        g1.update("3E11FA47-71CA-11E1-9E33-C80AA9429562:21-58")?;
        assert_eq!(
            "3E11FA47-71CA-11E1-9E33-C80AA9429562:21-58",
            g1.to_string().to_uppercase()
        );

        let mut g1 = MysqlGTIDSet::parse_gtid_set(
            r#"
            519CE70F-A893-11E9-A95A-B32DC65A7026:1-1154661,
            5C9CA52B-9F11-11E9-8EAF-3381EC1CC790:1-244,
            802D69FD-A3B6-11E9-B1EA-50BAB55BA838:1-1221371,
            F2B50559-A891-11E9-B646-884FF0CA2043:1-479261
        "#,
        )?;
        g1.update(
            r#"
            802D69FD-A3B6-11E9-B1EA-50BAB55BA838:1221110-1221371,
            F2B50559-A891-11E9-B646-884FF0CA2043:478509-479266
        "#,
        )?;
        let g2 = MysqlGTIDSet::parse_gtid_set(
            r#"
            519CE70F-A893-11E9-A95A-B32DC65A7026:1-1154661,
            5C9CA52B-9F11-11E9-8EAF-3381EC1CC790:1-244,
            802D69FD-A3B6-11E9-B1EA-50BAB55BA838:1-1221371,
            F2B50559-A891-11E9-B646-884FF0CA2043:1-479266
        "#,
        )?;
        assert_eq!(&g1, &g2);

        Ok(())
    }

    #[test]
    fn test_mysql_add_gtid() -> Result<(), ReplicationError> {
        let mut g1 = MysqlGTIDSet::parse_gtid_set("3E11FA47-71CA-11E1-9E33-C80AA9429562:21-57")?;
        let u = Uuid::parse_str("3E11FA47-71CA-11E1-9E33-C80AA9429562")?;
        g1.add_gtid(&u, 58);
        assert_eq!(
            "3E11FA47-71CA-11E1-9E33-C80AA9429562:21-58",
            g1.to_string().to_uppercase()
        );

        g1.add_gtid(&u, 60);
        assert_eq!(
            "3E11FA47-71CA-11E1-9E33-C80AA9429562:21-58:60",
            g1.to_string().to_uppercase()
        );

        g1.add_gtid(&u, 59);
        assert_eq!(
            "3E11FA47-71CA-11E1-9E33-C80AA9429562:21-60",
            g1.to_string().to_uppercase()
        );

        let u2 = Uuid::parse_str("519CE70F-A893-11E9-A95A-B32DC65A7026")?;
        g1.add_gtid(&u2, 58);
        let g2 = MysqlGTIDSet::parse_gtid_set(
            r#"
            3E11FA47-71CA-11E1-9E33-C80AA9429562:21-60,
            519CE70F-A893-11E9-A95A-B32DC65A7026:58
        "#,
        )?;

        assert_eq!(g1, g2);

        Ok(())
    }

    #[test]
    fn test_mysql_gtid_contain() -> Result<(), ReplicationError> {
        let g1 = MysqlGTIDSet::parse_gtid_set("3E11FA47-71CA-11E1-9E33-C80AA9429562:23")?;
        let g2 = MysqlGTIDSet::parse_gtid_set("3E11FA47-71CA-11E1-9E33-C80AA9429562:21-57")?;

        assert_eq!(true, g2.contain(&g1));
        assert_eq!(false, g1.contain(&g2));

        Ok(())
    }

    #[test]
    fn test_mysql_gtid_add() -> Result<(), ReplicationError> {
        struct Case {
            pub left: String,
            pub right: String,
            pub expected: String,
        }
        let cases = vec![
            Case {
                left: "3E11FA47-71CA-11E1-9E33-C80AA9429562:23".to_string(),
                right: "3E11FA47-71CA-11E1-9E33-C80AA9429562:28-57".to_string(),
                expected: "3E11FA47-71CA-11E1-9E33-C80AA9429562:23:28-57".to_string(),
            },
            Case {
                left: "3E11FA47-71CA-11E1-9E33-C80AA9429562:28-57".to_string(),
                right: "3E11FA47-71CA-11E1-9E33-C80AA9429562:23".to_string(),
                expected: "3E11FA47-71CA-11E1-9E33-C80AA9429562:23:28-57".to_string(),
            },
            Case {
                left: "3E11FA47-71CA-11E1-9E33-C80AA9429562:23-27".to_string(),
                right: "3E11FA47-71CA-11E1-9E33-C80AA9429562:28-57".to_string(),
                expected: "3E11FA47-71CA-11E1-9E33-C80AA9429562:23-57".to_string(),
            },
        ];

        for tc in &cases {
            let mut m1 = MysqlGTIDSet::parse_gtid_set(&tc.left)?;
            let m2 = MysqlGTIDSet::parse_gtid_set(&tc.right)?;
            m1.add(&m2)?;
            let one = format!(
                "{left} + {right} = {m1}",
                left = &tc.left,
                right = &tc.right,
                m1 = m1.to_string().to_uppercase()
            );
            let other = format!(
                "{left} + {right} = {expected}",
                left = &tc.left,
                right = &tc.right,
                expected = &tc.expected
            );
            assert_eq!(other, one)
        }

        Ok(())
    }

    #[test]
    fn test_mysql_parse_binary() -> Result<(), ReplicationError> {
        let int8 = ParseBinary::i8_little_endian(&vec![128]);
        assert_eq!(-128 as i8, int8);
        let uint8 = ParseBinary::u8_little_endian(&vec![128]);
        assert_eq!(128 as u8, uint8);

        let int16 = ParseBinary::i16_little_endian(&vec![1, 128])?;
        assert_eq!(-128 * 256 + 1 as i16, int16);
        let uint16 = ParseBinary::u16_little_endian(&vec![1, 128])?;
        assert_eq!(128 * 256 + 1 as u16, uint16);

        let int32 = ParseBinary::i24_little_endian(&vec![1, 2, 128]);
        assert_eq!((-128 * 65536 + 2 * 256 + 1) as i32, int32);
        let uint32 = ParseBinary::u24_little_endian(&vec![1, 2, 128]);
        assert_eq!((128 * 65536 + 2 * 256 + 1) as u32, uint32);

        let int32 = ParseBinary::i32_little_endian(&vec![1, 2, 3, 128])?;
        assert_eq!((-128 * 16777216 + 3 * 65536 + 2 * 256 + 1) as i32, int32);
        let uint32 = ParseBinary::u32_little_endian(&vec![1, 2, 3, 128])?;
        assert_eq!(128_u32 * 16777216 + 3 * 65536 + 2 * 256 + 1, uint32);

        let int64 = ParseBinary::i64_little_endian(&vec![1, 2, 3, 4, 5, 6, 7, 128])?;
        assert_eq!(
            -128_i64 * 72057594037927936
                + 7 * 281474976710656
                + 6 * 1099511627776
                + 5 * 4294967296
                + 4 * 16777216
                + 3 * 65536
                + 2 * 256
                + 1,
            int64
        );
        let uint64 = ParseBinary::u64_little_endian(&vec![1, 2, 3, 4, 5, 6, 7, 128])?;
        assert_eq!(
            128_u64 * 72057594037927936
                + 7 * 281474976710656
                + 6 * 1099511627776
                + 5 * 4294967296
                + 4 * 16777216
                + 3 * 65536
                + 2 * 256
                + 1,
            uint64
        );

        Ok(())
    }

    #[test]
    fn test_error_code() -> Result<(), ReplicationError> {
        struct Tbl {
            pub msg: String,
            pub code: isize,
        }
        let tbls = vec![
            Tbl {
                msg: "ERROR 1094 (HY000): Unknown thread id: 1094".to_string(),
                code: 1094,
            },
            Tbl {
                msg: "error string".to_string(),
                code: 0,
            },
            Tbl {
                msg: "123455 ks094".to_string(),
                code: 0,
            },
            Tbl {
                msg: "ERROR 1046 (3D000): Unknown error 1046".to_string(),
                code: 1046,
            },
        ];

        for v in &tbls {
            assert_eq!(v.code, MyError::error_code(&v.msg))
        }

        Ok(())
    }

    #[test]
    fn test_mysql_null_decode() {
        let bytes: [u8; 1] = [0xfb];
        let (_, is_null, n) = length_encoded_int(&bytes);

        assert_eq!(is_null, true);
        assert_eq!(1, n)
    }

    #[test]
    fn test_mysql_uuid_clone() -> Result<(), ReplicationError> {
        let us = UUIDSet::parse_uuid_set("de278ad0-2106-11e4-9f8e-6edd0ca20947:1-2")?;
        assert_eq!("de278ad0-2106-11e4-9f8e-6edd0ca20947:1-2", us.to_string());

        let c = us.clone();
        assert_eq!("de278ad0-2106-11e4-9f8e-6edd0ca20947:1-2", c.to_string());

        Ok(())
    }

    #[test]
    fn test_mysql_empty_decode() {
        let (_, is_null, n) = length_encoded_int(&vec![]);
        assert_eq!(true, is_null);
        assert_eq!(0, n);
    }
}
