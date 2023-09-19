#[cfg(test)]
mod tests {
    use crate::common::row_fields::{DecodeDatetime, DecodeDecimal, DecodeFieldData, DecodeJson};
    use crate::error::ReplicationError;
    use crate::mysql;
    use crate::replication::{decode_helper, Event, RowsEvent, TableMapEvent};
    use bigdecimal::BigDecimal;
    use std::collections::HashMap;
    use std::str::FromStr;

    // These are cases from the mysql test cases
    /*
        DROP TABLE IF EXISTS decodedecimal;
        CREATE TABLE decodedecimal (
            id     int(11) not null auto_increment,
            v4_2 decimal(4,2),
            v5_0 decimal(5,0),
            v7_3 decimal(7,3),
            v10_2 decimal(10,2),
            v10_3 decimal(10,3),
            v13_2 decimal(13,2),
            v15_14 decimal(15,14),
            v20_10 decimal(20,10),
            v30_5 decimal(30,5),
            v30_20 decimal(30,20),
            v30_25 decimal(30,25),
            prec   int(11),
            scale  int(11),
            PRIMARY KEY(id)
        ) engine=InnoDB;
        INSERT INTO decodedecimal (v4_2,v5_0,v7_3,v10_2,v10_3,v13_2,v15_14,v20_10,v30_5,v30_20,v30_25,prec,scale) VALUES
        ("-10.55","-10.55","-10.55","-10.55","-10.55","-10.55","-10.55","-10.55","-10.55","-10.55","-10.55",4,2),
        ("0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345","0.0123456789012345678912345",30,25),
        ("12345","12345","12345","12345","12345","12345","12345","12345","12345","12345","12345",5,0),
        ("12345","12345","12345","12345","12345","12345","12345","12345","12345","12345","12345",10,3),
        ("123.45","123.45","123.45","123.45","123.45","123.45","123.45","123.45","123.45","123.45","123.45",10,3),
        ("-123.45","-123.45","-123.45","-123.45","-123.45","-123.45","-123.45","-123.45","-123.45","-123.45","-123.45",20,10),
        (".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",15,14),
        (".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",".00012345000098765",22,20),
        (".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",".12345000098765",30,20),
        ("-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765","-.000000012345000098765",30,20),
        ("1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5","1234500009876.5",30,5),
        ("111111111.11","111111111.11","111111111.11","111111111.11","111111111.11","111111111.11","111111111.11","111111111.11","111111111.11","111111111.11","111111111.11",10,2),
        ("000000000.01","000000000.01","000000000.01","000000000.01","000000000.01","000000000.01","000000000.01","000000000.01","000000000.01","000000000.01","000000000.01",7,3),
        ("123.4","123.4","123.4","123.4","123.4","123.4","123.4","123.4","123.4","123.4","123.4",10,2),
        ("-562.58","-562.58","-562.58","-562.58","-562.58","-562.58","-562.58","-562.58","-562.58","-562.58","-562.58",13,2),
        ("-3699.01","-3699.01","-3699.01","-3699.01","-3699.01","-3699.01","-3699.01","-3699.01","-3699.01","-3699.01","-3699.01",13,2),
        ("-1948.14","-1948.14","-1948.14","-1948.14","-1948.14","-1948.14","-1948.14","-1948.14","-1948.14","-1948.14","-1948.14",13,2)
        ;
        select * from decodedecimal;
        +----+--------+-------+-----------+-------------+-------------+----------------+-------------------+-----------------------+---------------------+---------------------------------+---------------------------------+------+-------+
        | id | v4_2   | v5_0  | v7_3      | v10_2       | v10_3       | v13_2          | v15_14            | v20_10                | v30_5               | v30_20                          | v30_25                          | prec | scale |
        +----+--------+-------+-----------+-------------+-------------+----------------+-------------------+-----------------------+---------------------+---------------------------------+---------------------------------+------+-------+
        |  1 | -10.55 |   -11 |   -10.550 |      -10.55 |     -10.550 |         -10.55 | -9.99999999999999 |        -10.5500000000 |           -10.55000 |        -10.55000000000000000000 |   -10.5500000000000000000000000 |    4 |     2 |
        |  2 |   0.01 |     0 |     0.012 |        0.01 |       0.012 |           0.01 |  0.01234567890123 |          0.0123456789 |             0.01235 |          0.01234567890123456789 |     0.0123456789012345678912345 |   30 |    25 |
        |  3 |  99.99 | 12345 |  9999.999 |    12345.00 |   12345.000 |       12345.00 |  9.99999999999999 |      12345.0000000000 |         12345.00000 |      12345.00000000000000000000 | 12345.0000000000000000000000000 |    5 |     0 |
        |  4 |  99.99 | 12345 |  9999.999 |    12345.00 |   12345.000 |       12345.00 |  9.99999999999999 |      12345.0000000000 |         12345.00000 |      12345.00000000000000000000 | 12345.0000000000000000000000000 |   10 |     3 |
        |  5 |  99.99 |   123 |   123.450 |      123.45 |     123.450 |         123.45 |  9.99999999999999 |        123.4500000000 |           123.45000 |        123.45000000000000000000 |   123.4500000000000000000000000 |   10 |     3 |
        |  6 | -99.99 |  -123 |  -123.450 |     -123.45 |    -123.450 |        -123.45 | -9.99999999999999 |       -123.4500000000 |          -123.45000 |       -123.45000000000000000000 |  -123.4500000000000000000000000 |   20 |    10 |
        |  7 |   0.00 |     0 |     0.000 |        0.00 |       0.000 |           0.00 |  0.00012345000099 |          0.0001234500 |             0.00012 |          0.00012345000098765000 |     0.0001234500009876500000000 |   15 |    14 |
        |  8 |   0.00 |     0 |     0.000 |        0.00 |       0.000 |           0.00 |  0.00012345000099 |          0.0001234500 |             0.00012 |          0.00012345000098765000 |     0.0001234500009876500000000 |   22 |    20 |
        |  9 |   0.12 |     0 |     0.123 |        0.12 |       0.123 |           0.12 |  0.12345000098765 |          0.1234500010 |             0.12345 |          0.12345000098765000000 |     0.1234500009876500000000000 |   30 |    20 |
        | 10 |   0.00 |     0 |     0.000 |        0.00 |       0.000 |           0.00 | -0.00000001234500 |         -0.0000000123 |             0.00000 |         -0.00000001234500009877 |    -0.0000000123450000987650000 |   30 |    20 |
        | 11 |  99.99 | 99999 |  9999.999 | 99999999.99 | 9999999.999 | 99999999999.99 |  9.99999999999999 | 9999999999.9999999999 | 1234500009876.50000 | 9999999999.99999999999999999999 | 99999.9999999999999999999999999 |   30 |     5 |
        | 12 |  99.99 | 99999 |  9999.999 | 99999999.99 | 9999999.999 |   111111111.11 |  9.99999999999999 |  111111111.1100000000 |     111111111.11000 |  111111111.11000000000000000000 | 99999.9999999999999999999999999 |   10 |     2 |
        | 13 |   0.01 |     0 |     0.010 |        0.01 |       0.010 |           0.01 |  0.01000000000000 |          0.0100000000 |             0.01000 |          0.01000000000000000000 |     0.0100000000000000000000000 |    7 |     3 |
        | 14 |  99.99 |   123 |   123.400 |      123.40 |     123.400 |         123.40 |  9.99999999999999 |        123.4000000000 |           123.40000 |        123.40000000000000000000 |   123.4000000000000000000000000 |   10 |     2 |
        | 15 | -99.99 |  -563 |  -562.580 |     -562.58 |    -562.580 |        -562.58 | -9.99999999999999 |       -562.5800000000 |          -562.58000 |       -562.58000000000000000000 |  -562.5800000000000000000000000 |   13 |     2 |
        | 16 | -99.99 | -3699 | -3699.010 |    -3699.01 |   -3699.010 |       -3699.01 | -9.99999999999999 |      -3699.0100000000 |         -3699.01000 |      -3699.01000000000000000000 | -3699.0100000000000000000000000 |   13 |     2 |
        | 17 | -99.99 | -1948 | -1948.140 |    -1948.14 |   -1948.140 |       -1948.14 | -9.99999999999999 |      -1948.1400000000 |         -1948.14000 |      -1948.14000000000000000000 | -1948.1400000000000000000000000 |   13 |     2 |
        +----+--------+-------+-----------+-------------+-------------+----------------+-------------------+-----------------------+---------------------+---------------------------------+---------------------------------+------+-------+
    */
    #[test]
    fn test_decode_decimal() -> Result<(), ReplicationError> {
        // _PLACEHOLDER_ := 0
        struct Case {
            pub data: Vec<u8>,
            pub precision: isize,
            pub decimals: isize,
            pub expected: String,
            pub expected_pos: isize,
            pub expected_err: Result<(), ReplicationError>,
        }
        let test_cases = vec![
            Case {
                data: vec![117_u8, 200, 127, 255],
                precision: 4,
                decimals: 2,
                expected: "-10.55".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 244, 127, 245],
                precision: 5,
                decimals: 0,
                expected: "-11".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 245, 253, 217, 127, 255],
                precision: 7,
                decimals: 3,
                expected: "-10.550".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 245, 253, 217, 127, 255],
                precision: 10,
                decimals: 3,
                expected: "-10.550".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 255, 245, 200, 118, 196],
                precision: 13,
                decimals: 2,
                expected: "-10.55".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![118, 196, 101, 54, 0, 254, 121, 96, 127, 255],
                precision: 15,
                decimals: 14,
                expected: "-9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 255, 245, 223, 55, 170, 127, 255, 127, 255],
                precision: 20,
                decimals: 10,
                expected: "-10.5500000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 245, 255, 41, 39, 127,
                    255,
                ],
                precision: 30,
                decimals: 5,
                expected: "-10.55000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 245, 223, 55, 170, 127, 255, 255, 255, 255, 255, 127, 255,
                ],
                precision: 30,
                decimals: 20,
                expected: "-10.55000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 245, 223, 55, 170, 127, 255, 255, 255, 255, 255, 255, 255, 255, 4, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "-10.5500000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 1, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "0.01".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 1, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "0.01".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 12, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "0.012".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 1, 128, 0],
                precision: 13,
                decimals: 2,
                expected: "0.01".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 188, 97, 78, 1, 96, 11, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "0.01234567890123".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 188, 97, 78, 9, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "0.0123456789".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 211, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "0.01235".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 0, 0, 0, 0, 188, 97, 78, 53, 183, 191, 135, 89, 128, 0,
                ],
                precision: 30,
                decimals: 20,
                expected: "0.01234567890123456789".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 0, 0, 188, 97, 78, 53, 183, 191, 135, 0, 135, 253, 217, 30, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "0.0123456789012345678912345".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 128, 48],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 48, 57, 167, 15],
                precision: 5,
                decimals: 0,
                expected: "12345".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![167, 15, 3, 231, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "9999.999".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 48, 57, 0, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "12345.00".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 48, 57, 0, 0, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "12345.000".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "12345.00".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 0, 0, 0, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "12345.0000000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 57, 0, 0, 0, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "12345.00000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 48],
                precision: 30,
                decimals: 20,
                expected: "12345.00000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 48, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0],
                precision: 30,
                decimals: 25,
                expected: "12345.0000000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 128, 48],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 48, 57, 167, 15],
                precision: 5,
                decimals: 0,
                expected: "12345".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![167, 15, 3, 231, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "9999.999".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 48, 57, 0, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "12345.00".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 48, 57, 0, 0, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "12345.000".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "12345.00".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 0, 0, 0, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "12345.0000000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 57, 0, 0, 0, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "12345.00000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 48],
                precision: 30,
                decimals: 20,
                expected: "12345.00000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 48, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0],
                precision: 30,
                decimals: 25,
                expected: "12345.0000000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 128, 48],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 48, 57, 167, 15],
                precision: 5,
                decimals: 0,
                expected: "12345".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![167, 15, 3, 231, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "9999.999".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 48, 57, 0, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "12345.00".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 48, 57, 0, 0, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "12345.000".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "12345.00".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 0, 0, 0, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "12345.0000000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 57, 0, 0, 0, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "12345.00000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 48, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 48],
                precision: 30,
                decimals: 20,
                expected: "12345.00000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 48, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0],
                precision: 30,
                decimals: 25,
                expected: "12345.0000000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 123, 128, 123],
                precision: 5,
                decimals: 0,
                expected: "123".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 123, 1, 194, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "123.450".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 123, 45, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "123.45".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 123, 1, 194, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "123.450".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 123, 45, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "123.45".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 123, 26, 210, 116, 128, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "123.4500000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 175, 200, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "123.45000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 123, 26, 210, 116, 128, 0, 0, 0, 0, 0, 128, 0],
                precision: 30,
                decimals: 20,
                expected: "123.45000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 123, 26, 210, 116, 128, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "123.4500000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![28, 156, 127, 255],
                precision: 4,
                decimals: 2,
                expected: "-99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 132, 127, 132],
                precision: 5,
                decimals: 0,
                expected: "-123".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 132, 254, 61, 127, 255],
                precision: 7,
                decimals: 3,
                expected: "-123.450".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 132, 210, 127, 255],
                precision: 10,
                decimals: 2,
                expected: "-123.45".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 132, 254, 61, 127, 255],
                precision: 10,
                decimals: 3,
                expected: "-123.450".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 255, 132, 210, 118, 196],
                precision: 13,
                decimals: 2,
                expected: "-123.45".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![118, 196, 101, 54, 0, 254, 121, 96, 127, 255],
                precision: 15,
                decimals: 14,
                expected: "-9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 255, 132, 229, 45, 139, 127, 255, 127, 255],
                precision: 20,
                decimals: 10,
                expected: "-123.4500000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 132, 255, 80, 55, 127,
                    255,
                ],
                precision: 30,
                decimals: 5,
                expected: "-123.45000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 132, 229, 45, 139, 127, 255, 255, 255, 255, 255, 127, 255,
                ],
                precision: 30,
                decimals: 20,
                expected: "-123.45000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 132, 229, 45, 139, 127, 255, 255, 255, 255, 255, 255, 255, 255, 20, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "-123.4500000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 128, 0],
                precision: 5,
                decimals: 0,
                expected: "0".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "0.000".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "0.000".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 128, 0],
                precision: 13,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 1, 226, 58, 0, 0, 99, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "0.00012345000099".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 1, 226, 58, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "0.0001234500".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "0.00012".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 1, 226, 58, 0, 15, 18, 2, 0, 128, 0],
                precision: 30,
                decimals: 20,
                expected: "0.00012345000098765000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 1, 226, 58, 0, 15, 18, 2, 0, 0, 0, 0, 15, 0],
                precision: 30,
                decimals: 25,
                expected: "0.0001234500009876500000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 128, 0],
                precision: 5,
                decimals: 0,
                expected: "0".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "0.000".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "0.000".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 128, 0],
                precision: 13,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 1, 226, 58, 0, 0, 99, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "0.00012345000099".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 1, 226, 58, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "0.0001234500".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "0.00012".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 1, 226, 58, 0, 15, 18, 2, 0, 128, 0],
                precision: 30,
                decimals: 20,
                expected: "0.00012345000098765000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 1, 226, 58, 0, 15, 18, 2, 0, 0, 0, 0, 22, 0],
                precision: 30,
                decimals: 25,
                expected: "0.0001234500009876500000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 12, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "0.12".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 128, 0],
                precision: 5,
                decimals: 0,
                expected: "0".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 123, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "0.123".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 12, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "0.12".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 123, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "0.123".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 12, 128, 7],
                precision: 13,
                decimals: 2,
                expected: "0.12".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 7, 91, 178, 144, 1, 129, 205, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "0.12345000098765".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 7, 91, 178, 145, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "0.1234500010".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 57, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "0.12345".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 0, 0, 0, 7, 91, 178, 144, 58, 222, 87, 208, 0, 128, 0,
                ],
                precision: 30,
                decimals: 20,
                expected: "0.12345000098765000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 0, 7, 91, 178, 144, 58, 222, 87, 208, 0, 0, 0, 0, 30, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "0.1234500009876500000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 128, 0],
                precision: 5,
                decimals: 0,
                expected: "0".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "0.000".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "0.000".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 127, 255],
                precision: 13,
                decimals: 2,
                expected: "0.00".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 255, 243, 255, 121, 59, 127, 255],
                precision: 15,
                decimals: 14,
                expected: "-0.00000001234500".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 255, 255, 255, 255, 255, 243, 252, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "-0.0000000123".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 255],
                precision: 30,
                decimals: 5,
                expected: "0.00000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 255, 255, 243, 235, 111, 183, 93, 178, 127, 255,
                ],
                precision: 30,
                decimals: 20,
                expected: "-0.00000001234500009877".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 243, 235, 111, 183, 93, 255, 139, 69, 47, 30, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "-0.0000000123450000987650000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 129, 134],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![129, 134, 159, 167, 15],
                precision: 5,
                decimals: 0,
                expected: "99999".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![167, 15, 3, 231, 133, 245],
                precision: 7,
                decimals: 3,
                expected: "9999.999".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![133, 245, 224, 255, 99, 128, 152],
                precision: 10,
                decimals: 2,
                expected: "99999999.99".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 152, 150, 127, 3, 231, 227, 59],
                precision: 10,
                decimals: 3,
                expected: "9999999.999".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 59, 154, 201, 255, 99, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "99999999999.99".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 137, 59],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 59, 154, 201, 255, 9, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "9999999999.9999999999".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 0, 0, 0, 0, 4, 210, 29, 205, 139, 148, 0, 195, 80, 137, 59,
                ],
                precision: 30,
                decimals: 5,
                expected: "1234500009876.50000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    137, 59, 154, 201, 255, 59, 154, 201, 255, 59, 154, 201, 255, 99, 129, 134,
                ],
                precision: 30,
                decimals: 20,
                expected: "9999999999.99999999999999999999".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    129, 134, 159, 59, 154, 201, 255, 59, 154, 201, 255, 0, 152, 150, 127, 30, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "99999.9999999999999999999999999".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 129, 134],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![129, 134, 159, 167, 15],
                precision: 5,
                decimals: 0,
                expected: "99999".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![167, 15, 3, 231, 133, 245],
                precision: 7,
                decimals: 3,
                expected: "9999.999".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![133, 245, 224, 255, 99, 128, 152],
                precision: 10,
                decimals: 2,
                expected: "99999999.99".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 152, 150, 127, 3, 231, 128, 6],
                precision: 10,
                decimals: 3,
                expected: "9999999.999".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 6, 159, 107, 199, 11, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "111111111.11".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 128, 6],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 6, 159, 107, 199, 6, 142, 119, 128, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "111111111.1100000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 0, 0, 0, 0, 0, 0, 0, 6, 159, 107, 199, 0, 42, 248, 128, 6,
                ],
                precision: 30,
                decimals: 5,
                expected: "111111111.11000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    128, 6, 159, 107, 199, 6, 142, 119, 128, 0, 0, 0, 0, 0, 129, 134,
                ],
                precision: 30,
                decimals: 20,
                expected: "111111111.11000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    129, 134, 159, 59, 154, 201, 255, 59, 154, 201, 255, 0, 152, 150, 127, 10, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "99999.9999999999999999999999999".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 1, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "0.01".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 128, 0],
                precision: 5,
                decimals: 0,
                expected: "0".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 10, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "0.010".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 1, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "0.01".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 10, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "0.010".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 1, 128, 0],
                precision: 13,
                decimals: 2,
                expected: "0.01".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 152, 150, 128, 0, 0, 0, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "0.01000000000000".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 152, 150, 128, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "0.0100000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "0.01000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 152, 150, 128, 0, 0, 0, 0, 0, 128, 0],
                precision: 30,
                decimals: 20,
                expected: "0.01000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 152, 150, 128, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0],
                precision: 30,
                decimals: 25,
                expected: "0.0100000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![227, 99, 128, 0],
                precision: 4,
                decimals: 2,
                expected: "99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 123, 128, 123],
                precision: 5,
                decimals: 0,
                expected: "123".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 123, 1, 144, 128, 0],
                precision: 7,
                decimals: 3,
                expected: "123.400".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 123, 40, 128, 0],
                precision: 10,
                decimals: 2,
                expected: "123.40".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 123, 1, 144, 128, 0],
                precision: 10,
                decimals: 3,
                expected: "123.400".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 123, 40, 137, 59],
                precision: 13,
                decimals: 2,
                expected: "123.40".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![137, 59, 154, 201, 255, 1, 134, 159, 128, 0],
                precision: 15,
                decimals: 14,
                expected: "9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 123, 23, 215, 132, 0, 0, 128, 0],
                precision: 20,
                decimals: 10,
                expected: "123.4000000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 156, 64, 128, 0],
                precision: 30,
                decimals: 5,
                expected: "123.40000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 0, 0, 123, 23, 215, 132, 0, 0, 0, 0, 0, 0, 128, 0],
                precision: 30,
                decimals: 20,
                expected: "123.40000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![128, 0, 123, 23, 215, 132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0],
                precision: 30,
                decimals: 25,
                expected: "123.4000000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![28, 156, 127, 253],
                precision: 4,
                decimals: 2,
                expected: "-99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 253, 204, 125, 205],
                precision: 5,
                decimals: 0,
                expected: "-563".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![125, 205, 253, 187, 127, 255],
                precision: 7,
                decimals: 3,
                expected: "-562.580".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 253, 205, 197, 127, 255],
                precision: 10,
                decimals: 2,
                expected: "-562.58".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 253, 205, 253, 187, 127, 255],
                precision: 10,
                decimals: 3,
                expected: "-562.580".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 253, 205, 197, 118, 196],
                precision: 13,
                decimals: 2,
                expected: "-562.58".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![118, 196, 101, 54, 0, 254, 121, 96, 127, 255],
                precision: 15,
                decimals: 14,
                expected: "-9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 253, 205, 221, 109, 230, 255, 255, 127, 255],
                precision: 20,
                decimals: 10,
                expected: "-562.5800000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 253, 205, 255, 29, 111, 127,
                    255,
                ],
                precision: 30,
                decimals: 5,
                expected: "-562.58000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 253, 205, 221, 109, 230, 255, 255, 255, 255, 255, 255, 127, 253,
                ],
                precision: 30,
                decimals: 20,
                expected: "-562.58000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 253, 205, 221, 109, 230, 255, 255, 255, 255, 255, 255, 255, 255, 255, 13,
                    0,
                ],
                precision: 30,
                decimals: 25,
                expected: "-562.5800000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![28, 156, 127, 241],
                precision: 4,
                decimals: 2,
                expected: "-99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 241, 140, 113, 140],
                precision: 5,
                decimals: 0,
                expected: "-3699".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![113, 140, 255, 245, 127, 255],
                precision: 7,
                decimals: 3,
                expected: "-3699.010".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 241, 140, 254, 127, 255],
                precision: 10,
                decimals: 2,
                expected: "-3699.01".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 241, 140, 255, 245, 127, 255],
                precision: 10,
                decimals: 3,
                expected: "-3699.010".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 241, 140, 254, 118, 196],
                precision: 13,
                decimals: 2,
                expected: "-3699.01".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![118, 196, 101, 54, 0, 254, 121, 96, 127, 255],
                precision: 15,
                decimals: 14,
                expected: "-9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 241, 140, 255, 103, 105, 127, 255, 127, 255],
                precision: 20,
                decimals: 10,
                expected: "-3699.0100000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 241, 140, 255, 252, 23, 127,
                    255,
                ],
                precision: 30,
                decimals: 5,
                expected: "-3699.01000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 241, 140, 255, 103, 105, 127, 255, 255, 255, 255, 255, 127, 241,
                ],
                precision: 30,
                decimals: 20,
                expected: "-3699.01000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 241, 140, 255, 103, 105, 127, 255, 255, 255, 255, 255, 255, 255, 255, 13,
                    0,
                ],
                precision: 30,
                decimals: 25,
                expected: "-3699.0100000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![28, 156, 127, 248],
                precision: 4,
                decimals: 2,
                expected: "-99.99".to_string(),
                expected_pos: 2,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 248, 99, 120, 99],
                precision: 5,
                decimals: 0,
                expected: "-1948".to_string(),
                expected_pos: 3,
                expected_err: Ok(()),
            },
            Case {
                data: vec![120, 99, 255, 115, 127, 255],
                precision: 7,
                decimals: 3,
                expected: "-1948.140".to_string(),
                expected_pos: 4,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 248, 99, 241, 127, 255],
                precision: 10,
                decimals: 2,
                expected: "-1948.14".to_string(),
                expected_pos: 5,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 248, 99, 255, 115, 127, 255],
                precision: 10,
                decimals: 3,
                expected: "-1948.140".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 248, 99, 241, 118, 196],
                precision: 13,
                decimals: 2,
                expected: "-1948.14".to_string(),
                expected_pos: 6,
                expected_err: Ok(()),
            },
            Case {
                data: vec![118, 196, 101, 54, 0, 254, 121, 96, 127, 255],
                precision: 15,
                decimals: 14,
                expected: "-9.99999999999999".to_string(),
                expected_pos: 8,
                expected_err: Ok(()),
            },
            Case {
                data: vec![127, 255, 255, 248, 99, 247, 167, 196, 255, 255, 127, 255],
                precision: 20,
                decimals: 10,
                expected: "-1948.1400000000".to_string(),
                expected_pos: 10,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 248, 99, 255, 201, 79, 127,
                    255,
                ],
                precision: 30,
                decimals: 5,
                expected: "-1948.14000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 255, 255, 248, 99, 247, 167, 196, 255, 255, 255, 255, 255, 255, 127, 248,
                ],
                precision: 30,
                decimals: 20,
                expected: "-1948.14000000000000000000".to_string(),
                expected_pos: 14,
                expected_err: Ok(()),
            },
            Case {
                data: vec![
                    127, 248, 99, 247, 167, 196, 255, 255, 255, 255, 255, 255, 255, 255, 255, 13, 0,
                ],
                precision: 30,
                decimals: 25,
                expected: "-1948.1400000000000000000000000".to_string(),
                expected_pos: 15,
                expected_err: Ok(()),
            },
        ];

        for tc in &test_cases {
            match decode_helper::decode_decimal(&tc.data, tc.precision, tc.decimals, false) {
                Ok((ref value, pos)) => {
                    assert_eq!(tc.expected, value.to_string());
                    assert_eq!(tc.expected_pos, pos);
                }
                Err(_) => {
                    assert_eq!(true, tc.expected_err.is_err());
                }
            }

            match decode_helper::decode_decimal(&tc.data, tc.precision, tc.decimals, true) {
                Ok((ref value, pos)) => {
                    let expected_decimal = BigDecimal::from_str(&tc.expected)?;
                    assert_eq!(value, &DecodeDecimal::Decimal(expected_decimal));
                    assert_eq!(tc.expected_pos, pos);
                }
                Err(_) => {
                    assert_eq!(true, tc.expected_err.is_err());
                }
            }
        }

        Ok(())
    }

    // Table format:
    // desc funnytable;
    // +-------+------------+------+-----+---------+-------+
    // | Field | Type       | Null | Key | Default | Extra |
    // +-------+------------+------+-----+---------+-------+
    // | value | tinyint(4) | YES  |     | NULL    |       |
    // +-------+------------+------+-----+---------+-------+

    // insert into funnytable values (1), (2), (null);
    // insert into funnytable values (1), (null), (2);
    // all must get 3 rows
    #[test]
    fn test_last_null() -> Result<(), ReplicationError> {
        let table_map_event_data =
            b"\xd3\x01\x00\x00\x00\x00\x01\x00\x04test\x00\nfunnytable\x00\x01\x01\x00\x01";
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let tbls = vec![
            b"\xd3\x01\x00\x00\x00\x00\x01\x00\x02\x00\x01\xff\xfe\x01\xff\xfe\x02",
            b"\xd3\x01\x00\x00\x00\x00\x01\x00\x02\x00\x01\xff\xfe\x01\xfe\x02\xff",
        ];
        for tbl in &tbls {
            rows.rows = vec![];
            rows.decode(*tbl)?;
            assert_eq!(rows.rows.len(), 3)
        }

        Ok(())
    }

    #[test]
    fn test_parse_row_panic() -> Result<(), ReplicationError> {
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.table_id = 1810;
        table_map_event.column_type = vec![
            3, 15, 15, 15, 9, 15, 15, 252, 3, 3, 3, 15, 3, 3, 3, 15, 3, 15, 1, 15, 3, 1, 252, 15,
            15, 15,
        ];
        table_map_event.column_meta = vec![
            0, 108, 60, 765, 0, 765, 765, 4, 0, 0, 0, 765, 0, 0, 0, 3, 0, 3, 0, 765, 0, 0, 2, 108,
            108, 108,
        ];

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let data = vec![
            18_u8, 7, 0, 0, 0, 0, 1, 0, 2, 0, 26, 1, 1, 16, 252, 248, 142, 63, 0, 0, 13, 0, 0, 0,
            13, 0, 0, 0,
        ];
        rows.decode(&data)?;

        assert_eq!(DecodeFieldData::Isize(16270), rows.rows[0][0]);

        Ok(())
    }

    // Table format:
    // mysql> desc t10;
    // +-------+---------------+------+-----+---------+-------+
    // | Field | Type          | Null | Key | Default | Extra |
    // +-------+---------------+------+-----+---------+-------+
    // | c1    | json          | YES  |     | NULL    |       |
    // | c2    | decimal(10,0) | YES  |     | NULL    |       |
    // +-------+---------------+------+-----+---------+-------+

    // CREATE TABLE `t10` (
    //   `c1` json DEFAULT NULL,
    //   `c2` decimal(10,0)
    // ) ENGINE=InnoDB DEFAULT CHARSET=utf8;

    //nolint:misspell
    // INSERT INTO `t10` (`c2`) VALUES (1);
    // INSERT INTO `t10` (`c1`, `c2`) VALUES ('{"key1": "value1", "key2": "value2"}', 1);
    // test json deserialization
    // INSERT INTO `t10`(`c1`,`c2`) VALUES ('{"text":"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget bibendum sodales, augue velit cursus nunc, quis gravida magna mi a libero. Fusce vulputate eleifend sapien. Vestibulum purus quam, scelerisque ut, mollis sed, nonummy id, metus. Nullam accumsan lorem in dui. Cras ultricies mi eu turpis hendrerit fringilla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; In ac dui quis mi consectetuer lacinia. Nam pretium turpis et arcu. Duis arcu tortor, suscipit eget, imperdiet nec, imperdiet iaculis, ipsum. Sed aliquam ultrices mauris. Integer ante arcu, accumsan a, consectetuer eget, posuere ut, mauris. Praesent adipiscing. Phasellus ullamcorper ipsum rutrum nunc. Nunc nonummy metus. Vestibulum volutpat pretium libero. Cras id dui. Aenean ut eros et nisl sagittis vestibulum. Nullam nulla eros, ultricies sit amet, nonummy id, imperdiet feugiat, pede. Sed lectus. Donec mollis hendrerit risus. Phasellus nec sem in justo pellentesque facilisis. Etiam imperdiet imperdiet orci. Nunc nec neque. Phasellus leo dolor, tempus non, auctor et, hendrerit quis, nisi. Curabitur ligula sapien, tincidunt non, euismod vitae, posuere imperdiet, leo. Maecenas malesuada. Praesent congue erat at massa. Sed cursus turpis vitae tortor. Donec posuere vulputate arcu. Phasellus accumsan cursus velit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed aliquam, nisi quis porttitor congue, elit erat euismod orci, ac"}',101);
    #[test]
    fn test_parse_json() -> Result<(), ReplicationError> {
        let table_map_event_data = vec![
            109, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 3, 116, 49, 48, 0, 2, 245, 246, 3,
            4, 10, 0, 3,
        ];
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let tbls = vec![
            vec![
                109_u8, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 255, 253, 128, 0, 0, 0, 1,
            ],
            vec![
                109, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 255, 252, 41, 0, 0, 0, 0, 2, 0, 40, 0, 18, 0, 4,
                0, 22, 0, 4, 0, 12, 26, 0, 12, 33, 0, 107, 101, 121, 49, 107, 101, 121, 50, 6, 118,
                97, 108, 117, 101, 49, 6, 118, 97, 108, 117, 101, 50, 128, 0, 0, 0, 1,
            ],
        ];

        for tbl in &tbls {
            rows.rows = vec![];
            rows.decode(&tbl)?;
            assert_eq!(
                DecodeFieldData::Decimal(DecodeDecimal::String("1".to_string())),
                rows.rows[0][1]
            );
        }

        let long_tbls = vec![vec![
            109, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 255, 252, 208, 10, 0, 0, 0, 1, 0, 207, 10, 11, 0, 4,
            0, 12, 15, 0, 116, 101, 120, 116, 190, 21, 76, 111, 114, 101, 109, 32, 105, 112, 115,
            117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44,
            32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 101, 114, 32, 97, 100, 105, 112,
            105, 115, 99, 105, 110, 103, 32, 101, 108, 105, 116, 46, 32, 65, 101, 110, 101, 97,
            110, 32, 99, 111, 109, 109, 111, 100, 111, 32, 108, 105, 103, 117, 108, 97, 32, 101,
            103, 101, 116, 32, 100, 111, 108, 111, 114, 46, 32, 65, 101, 110, 101, 97, 110, 32,
            109, 97, 115, 115, 97, 46, 32, 67, 117, 109, 32, 115, 111, 99, 105, 105, 115, 32, 110,
            97, 116, 111, 113, 117, 101, 32, 112, 101, 110, 97, 116, 105, 98, 117, 115, 32, 101,
            116, 32, 109, 97, 103, 110, 105, 115, 32, 100, 105, 115, 32, 112, 97, 114, 116, 117,
            114, 105, 101, 110, 116, 32, 109, 111, 110, 116, 101, 115, 44, 32, 110, 97, 115, 99,
            101, 116, 117, 114, 32, 114, 105, 100, 105, 99, 117, 108, 117, 115, 32, 109, 117, 115,
            46, 32, 68, 111, 110, 101, 99, 32, 113, 117, 97, 109, 32, 102, 101, 108, 105, 115, 44,
            32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 110, 101, 99, 44, 32, 112, 101,
            108, 108, 101, 110, 116, 101, 115, 113, 117, 101, 32, 101, 117, 44, 32, 112, 114, 101,
            116, 105, 117, 109, 32, 113, 117, 105, 115, 44, 32, 115, 101, 109, 46, 32, 78, 117,
            108, 108, 97, 32, 99, 111, 110, 115, 101, 113, 117, 97, 116, 32, 109, 97, 115, 115, 97,
            32, 113, 117, 105, 115, 32, 101, 110, 105, 109, 46, 32, 68, 111, 110, 101, 99, 32, 112,
            101, 100, 101, 32, 106, 117, 115, 116, 111, 44, 32, 102, 114, 105, 110, 103, 105, 108,
            108, 97, 32, 118, 101, 108, 44, 32, 97, 108, 105, 113, 117, 101, 116, 32, 110, 101, 99,
            44, 32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 101, 103, 101, 116, 44, 32, 97,
            114, 99, 117, 46, 32, 73, 110, 32, 101, 110, 105, 109, 32, 106, 117, 115, 116, 111, 44,
            32, 114, 104, 111, 110, 99, 117, 115, 32, 117, 116, 44, 32, 105, 109, 112, 101, 114,
            100, 105, 101, 116, 32, 97, 44, 32, 118, 101, 110, 101, 110, 97, 116, 105, 115, 32,
            118, 105, 116, 97, 101, 44, 32, 106, 117, 115, 116, 111, 46, 32, 78, 117, 108, 108, 97,
            109, 32, 100, 105, 99, 116, 117, 109, 32, 102, 101, 108, 105, 115, 32, 101, 117, 32,
            112, 101, 100, 101, 32, 109, 111, 108, 108, 105, 115, 32, 112, 114, 101, 116, 105, 117,
            109, 46, 32, 73, 110, 116, 101, 103, 101, 114, 32, 116, 105, 110, 99, 105, 100, 117,
            110, 116, 46, 32, 67, 114, 97, 115, 32, 100, 97, 112, 105, 98, 117, 115, 46, 32, 86,
            105, 118, 97, 109, 117, 115, 32, 101, 108, 101, 109, 101, 110, 116, 117, 109, 32, 115,
            101, 109, 112, 101, 114, 32, 110, 105, 115, 105, 46, 32, 65, 101, 110, 101, 97, 110,
            32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 101, 108, 101, 105, 102, 101, 110,
            100, 32, 116, 101, 108, 108, 117, 115, 46, 32, 65, 101, 110, 101, 97, 110, 32, 108,
            101, 111, 32, 108, 105, 103, 117, 108, 97, 44, 32, 112, 111, 114, 116, 116, 105, 116,
            111, 114, 32, 101, 117, 44, 32, 99, 111, 110, 115, 101, 113, 117, 97, 116, 32, 118,
            105, 116, 97, 101, 44, 32, 101, 108, 101, 105, 102, 101, 110, 100, 32, 97, 99, 44, 32,
            101, 110, 105, 109, 46, 32, 65, 108, 105, 113, 117, 97, 109, 32, 108, 111, 114, 101,
            109, 32, 97, 110, 116, 101, 44, 32, 100, 97, 112, 105, 98, 117, 115, 32, 105, 110, 44,
            32, 118, 105, 118, 101, 114, 114, 97, 32, 113, 117, 105, 115, 44, 32, 102, 101, 117,
            103, 105, 97, 116, 32, 97, 44, 32, 116, 101, 108, 108, 117, 115, 46, 32, 80, 104, 97,
            115, 101, 108, 108, 117, 115, 32, 118, 105, 118, 101, 114, 114, 97, 32, 110, 117, 108,
            108, 97, 32, 117, 116, 32, 109, 101, 116, 117, 115, 32, 118, 97, 114, 105, 117, 115,
            32, 108, 97, 111, 114, 101, 101, 116, 46, 32, 81, 117, 105, 115, 113, 117, 101, 32,
            114, 117, 116, 114, 117, 109, 46, 32, 65, 101, 110, 101, 97, 110, 32, 105, 109, 112,
            101, 114, 100, 105, 101, 116, 46, 32, 69, 116, 105, 97, 109, 32, 117, 108, 116, 114,
            105, 99, 105, 101, 115, 32, 110, 105, 115, 105, 32, 118, 101, 108, 32, 97, 117, 103,
            117, 101, 46, 32, 67, 117, 114, 97, 98, 105, 116, 117, 114, 32, 117, 108, 108, 97, 109,
            99, 111, 114, 112, 101, 114, 32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 110,
            105, 115, 105, 46, 32, 78, 97, 109, 32, 101, 103, 101, 116, 32, 100, 117, 105, 46, 32,
            69, 116, 105, 97, 109, 32, 114, 104, 111, 110, 99, 117, 115, 46, 32, 77, 97, 101, 99,
            101, 110, 97, 115, 32, 116, 101, 109, 112, 117, 115, 44, 32, 116, 101, 108, 108, 117,
            115, 32, 101, 103, 101, 116, 32, 99, 111, 110, 100, 105, 109, 101, 110, 116, 117, 109,
            32, 114, 104, 111, 110, 99, 117, 115, 44, 32, 115, 101, 109, 32, 113, 117, 97, 109, 32,
            115, 101, 109, 112, 101, 114, 32, 108, 105, 98, 101, 114, 111, 44, 32, 115, 105, 116,
            32, 97, 109, 101, 116, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 115,
            101, 109, 32, 110, 101, 113, 117, 101, 32, 115, 101, 100, 32, 105, 112, 115, 117, 109,
            46, 32, 78, 97, 109, 32, 113, 117, 97, 109, 32, 110, 117, 110, 99, 44, 32, 98, 108, 97,
            110, 100, 105, 116, 32, 118, 101, 108, 44, 32, 108, 117, 99, 116, 117, 115, 32, 112,
            117, 108, 118, 105, 110, 97, 114, 44, 32, 104, 101, 110, 100, 114, 101, 114, 105, 116,
            32, 105, 100, 44, 32, 108, 111, 114, 101, 109, 46, 32, 77, 97, 101, 99, 101, 110, 97,
            115, 32, 110, 101, 99, 32, 111, 100, 105, 111, 32, 101, 116, 32, 97, 110, 116, 101, 32,
            116, 105, 110, 99, 105, 100, 117, 110, 116, 32, 116, 101, 109, 112, 117, 115, 46, 32,
            68, 111, 110, 101, 99, 32, 118, 105, 116, 97, 101, 32, 115, 97, 112, 105, 101, 110, 32,
            117, 116, 32, 108, 105, 98, 101, 114, 111, 32, 118, 101, 110, 101, 110, 97, 116, 105,
            115, 32, 102, 97, 117, 99, 105, 98, 117, 115, 46, 32, 78, 117, 108, 108, 97, 109, 32,
            113, 117, 105, 115, 32, 97, 110, 116, 101, 46, 32, 69, 116, 105, 97, 109, 32, 115, 105,
            116, 32, 97, 109, 101, 116, 32, 111, 114, 99, 105, 32, 101, 103, 101, 116, 32, 101,
            114, 111, 115, 32, 102, 97, 117, 99, 105, 98, 117, 115, 32, 116, 105, 110, 99, 105,
            100, 117, 110, 116, 46, 32, 68, 117, 105, 115, 32, 108, 101, 111, 46, 32, 83, 101, 100,
            32, 102, 114, 105, 110, 103, 105, 108, 108, 97, 32, 109, 97, 117, 114, 105, 115, 32,
            115, 105, 116, 32, 97, 109, 101, 116, 32, 110, 105, 98, 104, 46, 32, 68, 111, 110, 101,
            99, 32, 115, 111, 100, 97, 108, 101, 115, 32, 115, 97, 103, 105, 116, 116, 105, 115,
            32, 109, 97, 103, 110, 97, 46, 32, 83, 101, 100, 32, 99, 111, 110, 115, 101, 113, 117,
            97, 116, 44, 32, 108, 101, 111, 32, 101, 103, 101, 116, 32, 98, 105, 98, 101, 110, 100,
            117, 109, 32, 115, 111, 100, 97, 108, 101, 115, 44, 32, 97, 117, 103, 117, 101, 32,
            118, 101, 108, 105, 116, 32, 99, 117, 114, 115, 117, 115, 32, 110, 117, 110, 99, 44,
            32, 113, 117, 105, 115, 32, 103, 114, 97, 118, 105, 100, 97, 32, 109, 97, 103, 110, 97,
            32, 109, 105, 32, 97, 32, 108, 105, 98, 101, 114, 111, 46, 32, 70, 117, 115, 99, 101,
            32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 101, 108, 101, 105, 102, 101, 110,
            100, 32, 115, 97, 112, 105, 101, 110, 46, 32, 86, 101, 115, 116, 105, 98, 117, 108,
            117, 109, 32, 112, 117, 114, 117, 115, 32, 113, 117, 97, 109, 44, 32, 115, 99, 101,
            108, 101, 114, 105, 115, 113, 117, 101, 32, 117, 116, 44, 32, 109, 111, 108, 108, 105,
            115, 32, 115, 101, 100, 44, 32, 110, 111, 110, 117, 109, 109, 121, 32, 105, 100, 44,
            32, 109, 101, 116, 117, 115, 46, 32, 78, 117, 108, 108, 97, 109, 32, 97, 99, 99, 117,
            109, 115, 97, 110, 32, 108, 111, 114, 101, 109, 32, 105, 110, 32, 100, 117, 105, 46,
            32, 67, 114, 97, 115, 32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 109, 105, 32,
            101, 117, 32, 116, 117, 114, 112, 105, 115, 32, 104, 101, 110, 100, 114, 101, 114, 105,
            116, 32, 102, 114, 105, 110, 103, 105, 108, 108, 97, 46, 32, 86, 101, 115, 116, 105,
            98, 117, 108, 117, 109, 32, 97, 110, 116, 101, 32, 105, 112, 115, 117, 109, 32, 112,
            114, 105, 109, 105, 115, 32, 105, 110, 32, 102, 97, 117, 99, 105, 98, 117, 115, 32,
            111, 114, 99, 105, 32, 108, 117, 99, 116, 117, 115, 32, 101, 116, 32, 117, 108, 116,
            114, 105, 99, 101, 115, 32, 112, 111, 115, 117, 101, 114, 101, 32, 99, 117, 98, 105,
            108, 105, 97, 32, 67, 117, 114, 97, 101, 59, 32, 73, 110, 32, 97, 99, 32, 100, 117,
            105, 32, 113, 117, 105, 115, 32, 109, 105, 32, 99, 111, 110, 115, 101, 99, 116, 101,
            116, 117, 101, 114, 32, 108, 97, 99, 105, 110, 105, 97, 46, 32, 78, 97, 109, 32, 112,
            114, 101, 116, 105, 117, 109, 32, 116, 117, 114, 112, 105, 115, 32, 101, 116, 32, 97,
            114, 99, 117, 46, 32, 68, 117, 105, 115, 32, 97, 114, 99, 117, 32, 116, 111, 114, 116,
            111, 114, 44, 32, 115, 117, 115, 99, 105, 112, 105, 116, 32, 101, 103, 101, 116, 44,
            32, 105, 109, 112, 101, 114, 100, 105, 101, 116, 32, 110, 101, 99, 44, 32, 105, 109,
            112, 101, 114, 100, 105, 101, 116, 32, 105, 97, 99, 117, 108, 105, 115, 44, 32, 105,
            112, 115, 117, 109, 46, 32, 83, 101, 100, 32, 97, 108, 105, 113, 117, 97, 109, 32, 117,
            108, 116, 114, 105, 99, 101, 115, 32, 109, 97, 117, 114, 105, 115, 46, 32, 73, 110,
            116, 101, 103, 101, 114, 32, 97, 110, 116, 101, 32, 97, 114, 99, 117, 44, 32, 97, 99,
            99, 117, 109, 115, 97, 110, 32, 97, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116,
            117, 101, 114, 32, 101, 103, 101, 116, 44, 32, 112, 111, 115, 117, 101, 114, 101, 32,
            117, 116, 44, 32, 109, 97, 117, 114, 105, 115, 46, 32, 80, 114, 97, 101, 115, 101, 110,
            116, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 46, 32, 80, 104, 97, 115, 101,
            108, 108, 117, 115, 32, 117, 108, 108, 97, 109, 99, 111, 114, 112, 101, 114, 32, 105,
            112, 115, 117, 109, 32, 114, 117, 116, 114, 117, 109, 32, 110, 117, 110, 99, 46, 32,
            78, 117, 110, 99, 32, 110, 111, 110, 117, 109, 109, 121, 32, 109, 101, 116, 117, 115,
            46, 32, 86, 101, 115, 116, 105, 98, 117, 108, 117, 109, 32, 118, 111, 108, 117, 116,
            112, 97, 116, 32, 112, 114, 101, 116, 105, 117, 109, 32, 108, 105, 98, 101, 114, 111,
            46, 32, 67, 114, 97, 115, 32, 105, 100, 32, 100, 117, 105, 46, 32, 65, 101, 110, 101,
            97, 110, 32, 117, 116, 32, 101, 114, 111, 115, 32, 101, 116, 32, 110, 105, 115, 108,
            32, 115, 97, 103, 105, 116, 116, 105, 115, 32, 118, 101, 115, 116, 105, 98, 117, 108,
            117, 109, 46, 32, 78, 117, 108, 108, 97, 109, 32, 110, 117, 108, 108, 97, 32, 101, 114,
            111, 115, 44, 32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 115, 105, 116, 32,
            97, 109, 101, 116, 44, 32, 110, 111, 110, 117, 109, 109, 121, 32, 105, 100, 44, 32,
            105, 109, 112, 101, 114, 100, 105, 101, 116, 32, 102, 101, 117, 103, 105, 97, 116, 44,
            32, 112, 101, 100, 101, 46, 32, 83, 101, 100, 32, 108, 101, 99, 116, 117, 115, 46, 32,
            68, 111, 110, 101, 99, 32, 109, 111, 108, 108, 105, 115, 32, 104, 101, 110, 100, 114,
            101, 114, 105, 116, 32, 114, 105, 115, 117, 115, 46, 32, 80, 104, 97, 115, 101, 108,
            108, 117, 115, 32, 110, 101, 99, 32, 115, 101, 109, 32, 105, 110, 32, 106, 117, 115,
            116, 111, 32, 112, 101, 108, 108, 101, 110, 116, 101, 115, 113, 117, 101, 32, 102, 97,
            99, 105, 108, 105, 115, 105, 115, 46, 32, 69, 116, 105, 97, 109, 32, 105, 109, 112,
            101, 114, 100, 105, 101, 116, 32, 105, 109, 112, 101, 114, 100, 105, 101, 116, 32, 111,
            114, 99, 105, 46, 32, 78, 117, 110, 99, 32, 110, 101, 99, 32, 110, 101, 113, 117, 101,
            46, 32, 80, 104, 97, 115, 101, 108, 108, 117, 115, 32, 108, 101, 111, 32, 100, 111,
            108, 111, 114, 44, 32, 116, 101, 109, 112, 117, 115, 32, 110, 111, 110, 44, 32, 97,
            117, 99, 116, 111, 114, 32, 101, 116, 44, 32, 104, 101, 110, 100, 114, 101, 114, 105,
            116, 32, 113, 117, 105, 115, 44, 32, 110, 105, 115, 105, 46, 32, 67, 117, 114, 97, 98,
            105, 116, 117, 114, 32, 108, 105, 103, 117, 108, 97, 32, 115, 97, 112, 105, 101, 110,
            44, 32, 116, 105, 110, 99, 105, 100, 117, 110, 116, 32, 110, 111, 110, 44, 32, 101,
            117, 105, 115, 109, 111, 100, 32, 118, 105, 116, 97, 101, 44, 32, 112, 111, 115, 117,
            101, 114, 101, 32, 105, 109, 112, 101, 114, 100, 105, 101, 116, 44, 32, 108, 101, 111,
            46, 32, 77, 97, 101, 99, 101, 110, 97, 115, 32, 109, 97, 108, 101, 115, 117, 97, 100,
            97, 46, 32, 80, 114, 97, 101, 115, 101, 110, 116, 32, 99, 111, 110, 103, 117, 101, 32,
            101, 114, 97, 116, 32, 97, 116, 32, 109, 97, 115, 115, 97, 46, 32, 83, 101, 100, 32,
            99, 117, 114, 115, 117, 115, 32, 116, 117, 114, 112, 105, 115, 32, 118, 105, 116, 97,
            101, 32, 116, 111, 114, 116, 111, 114, 46, 32, 68, 111, 110, 101, 99, 32, 112, 111,
            115, 117, 101, 114, 101, 32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 97, 114,
            99, 117, 46, 32, 80, 104, 97, 115, 101, 108, 108, 117, 115, 32, 97, 99, 99, 117, 109,
            115, 97, 110, 32, 99, 117, 114, 115, 117, 115, 32, 118, 101, 108, 105, 116, 46, 32, 86,
            101, 115, 116, 105, 98, 117, 108, 117, 109, 32, 97, 110, 116, 101, 32, 105, 112, 115,
            117, 109, 32, 112, 114, 105, 109, 105, 115, 32, 105, 110, 32, 102, 97, 117, 99, 105,
            98, 117, 115, 32, 111, 114, 99, 105, 32, 108, 117, 99, 116, 117, 115, 32, 101, 116, 32,
            117, 108, 116, 114, 105, 99, 101, 115, 32, 112, 111, 115, 117, 101, 114, 101, 32, 99,
            117, 98, 105, 108, 105, 97, 32, 67, 117, 114, 97, 101, 59, 32, 83, 101, 100, 32, 97,
            108, 105, 113, 117, 97, 109, 44, 32, 110, 105, 115, 105, 32, 113, 117, 105, 115, 32,
            112, 111, 114, 116, 116, 105, 116, 111, 114, 32, 99, 111, 110, 103, 117, 101, 44, 32,
            101, 108, 105, 116, 32, 101, 114, 97, 116, 32, 101, 117, 105, 115, 109, 111, 100, 32,
            111, 114, 99, 105, 44, 32, 97, 99, 128, 0, 0, 0, 101,
        ]];

        //nolint:misspell
        for ltbl in &long_tbls {
            rows.rows = vec![];
            rows.decode(ltbl)?;
            assert_eq!(
                DecodeFieldData::Decimal(DecodeDecimal::String("101".to_string())),
                rows.rows[0][1]
            )
        }
        Ok(())
    }

    // Table format:
    // mysql> desc t10;
    // +-------+---------------+------+-----+---------+-------+
    // | Field | Type          | Null | Key | Default | Extra |
    // +-------+---------------+------+-----+---------+-------+
    // | c1    | json          | YES  |     | NULL    |       |
    // | c2    | decimal(10,0) | YES  |     | NULL    |       |
    // +-------+---------------+------+-----+---------+-------+

    // CREATE TABLE `t10` (
    //   `c1` json DEFAULT NULL,
    //   `c2` decimal(10,0)
    // ) ENGINE=InnoDB DEFAULT CHARSET=utf8;

    //nolint:misspell
    // INSERT INTO `t10` (`c2`) VALUES (1);
    // INSERT INTO `t10` (`c1`, `c2`) VALUES ('{"key1": "value1", "key2": "value2"}', 1);
    // test json deserialization
    // INSERT INTO `t10`(`c1`,`c2`) VALUES ('{"text":"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget bibendum sodales, augue velit cursus nunc, quis gravida magna mi a libero. Fusce vulputate eleifend sapien. Vestibulum purus quam, scelerisque ut, mollis sed, nonummy id, metus. Nullam accumsan lorem in dui. Cras ultricies mi eu turpis hendrerit fringilla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; In ac dui quis mi consectetuer lacinia. Nam pretium turpis et arcu. Duis arcu tortor, suscipit eget, imperdiet nec, imperdiet iaculis, ipsum. Sed aliquam ultrices mauris. Integer ante arcu, accumsan a, consectetuer eget, posuere ut, mauris. Praesent adipiscing. Phasellus ullamcorper ipsum rutrum nunc. Nunc nonummy metus. Vestibulum volutpat pretium libero. Cras id dui. Aenean ut eros et nisl sagittis vestibulum. Nullam nulla eros, ultricies sit amet, nonummy id, imperdiet feugiat, pede. Sed lectus. Donec mollis hendrerit risus. Phasellus nec sem in justo pellentesque facilisis. Etiam imperdiet imperdiet orci. Nunc nec neque. Phasellus leo dolor, tempus non, auctor et, hendrerit quis, nisi. Curabitur ligula sapien, tincidunt non, euismod vitae, posuere imperdiet, leo. Maecenas malesuada. Praesent congue erat at massa. Sed cursus turpis vitae tortor. Donec posuere vulputate arcu. Phasellus accumsan cursus velit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed aliquam, nisi quis porttitor congue, elit erat euismod orci, ac"}',101);
    #[test]
    fn test_parse_json_decimal() -> Result<(), ReplicationError> {
        let table_map_event_data =
            b"m\x00\x00\x00\x00\x00\x01\x00\x04test\x00\x03t10\x00\x02\xf5\xf6\x03\x04\n\x00\x03"
                .to_vec();
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.use_decimal = true;
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let tbls = vec![
            vec![109, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 255, 253, 128, 0, 0, 0, 1],
            vec![
                109, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 255, 252, 41, 0, 0, 0, 0, 2, 0, 40, 0, 18, 0, 4,
                0, 22, 0, 4, 0, 12, 26, 0, 12, 33, 0, 107, 101, 121, 49, 107, 101, 121, 50, 6, 118,
                97, 108, 117, 101, 49, 6, 118, 97, 108, 117, 101, 50, 128, 0, 0, 0, 1,
            ],
        ];

        for tbl in &tbls {
            rows.rows = vec![];
            rows.decode(&tbl)?;
            assert_eq!(
                DecodeFieldData::Decimal(DecodeDecimal::Decimal("1".parse().unwrap())),
                rows.rows[0][1]
            );
        }

        //nolint:misspell
        let long_tbls = vec![vec![
            109, 0, 0, 0, 0, 0, 1, 0, 2, 0, 2, 255, 252, 208, 10, 0, 0, 0, 1, 0, 207, 10, 11, 0, 4,
            0, 12, 15, 0, 116, 101, 120, 116, 190, 21, 76, 111, 114, 101, 109, 32, 105, 112, 115,
            117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44,
            32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 101, 114, 32, 97, 100, 105, 112,
            105, 115, 99, 105, 110, 103, 32, 101, 108, 105, 116, 46, 32, 65, 101, 110, 101, 97,
            110, 32, 99, 111, 109, 109, 111, 100, 111, 32, 108, 105, 103, 117, 108, 97, 32, 101,
            103, 101, 116, 32, 100, 111, 108, 111, 114, 46, 32, 65, 101, 110, 101, 97, 110, 32,
            109, 97, 115, 115, 97, 46, 32, 67, 117, 109, 32, 115, 111, 99, 105, 105, 115, 32, 110,
            97, 116, 111, 113, 117, 101, 32, 112, 101, 110, 97, 116, 105, 98, 117, 115, 32, 101,
            116, 32, 109, 97, 103, 110, 105, 115, 32, 100, 105, 115, 32, 112, 97, 114, 116, 117,
            114, 105, 101, 110, 116, 32, 109, 111, 110, 116, 101, 115, 44, 32, 110, 97, 115, 99,
            101, 116, 117, 114, 32, 114, 105, 100, 105, 99, 117, 108, 117, 115, 32, 109, 117, 115,
            46, 32, 68, 111, 110, 101, 99, 32, 113, 117, 97, 109, 32, 102, 101, 108, 105, 115, 44,
            32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 110, 101, 99, 44, 32, 112, 101,
            108, 108, 101, 110, 116, 101, 115, 113, 117, 101, 32, 101, 117, 44, 32, 112, 114, 101,
            116, 105, 117, 109, 32, 113, 117, 105, 115, 44, 32, 115, 101, 109, 46, 32, 78, 117,
            108, 108, 97, 32, 99, 111, 110, 115, 101, 113, 117, 97, 116, 32, 109, 97, 115, 115, 97,
            32, 113, 117, 105, 115, 32, 101, 110, 105, 109, 46, 32, 68, 111, 110, 101, 99, 32, 112,
            101, 100, 101, 32, 106, 117, 115, 116, 111, 44, 32, 102, 114, 105, 110, 103, 105, 108,
            108, 97, 32, 118, 101, 108, 44, 32, 97, 108, 105, 113, 117, 101, 116, 32, 110, 101, 99,
            44, 32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 101, 103, 101, 116, 44, 32, 97,
            114, 99, 117, 46, 32, 73, 110, 32, 101, 110, 105, 109, 32, 106, 117, 115, 116, 111, 44,
            32, 114, 104, 111, 110, 99, 117, 115, 32, 117, 116, 44, 32, 105, 109, 112, 101, 114,
            100, 105, 101, 116, 32, 97, 44, 32, 118, 101, 110, 101, 110, 97, 116, 105, 115, 32,
            118, 105, 116, 97, 101, 44, 32, 106, 117, 115, 116, 111, 46, 32, 78, 117, 108, 108, 97,
            109, 32, 100, 105, 99, 116, 117, 109, 32, 102, 101, 108, 105, 115, 32, 101, 117, 32,
            112, 101, 100, 101, 32, 109, 111, 108, 108, 105, 115, 32, 112, 114, 101, 116, 105, 117,
            109, 46, 32, 73, 110, 116, 101, 103, 101, 114, 32, 116, 105, 110, 99, 105, 100, 117,
            110, 116, 46, 32, 67, 114, 97, 115, 32, 100, 97, 112, 105, 98, 117, 115, 46, 32, 86,
            105, 118, 97, 109, 117, 115, 32, 101, 108, 101, 109, 101, 110, 116, 117, 109, 32, 115,
            101, 109, 112, 101, 114, 32, 110, 105, 115, 105, 46, 32, 65, 101, 110, 101, 97, 110,
            32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 101, 108, 101, 105, 102, 101, 110,
            100, 32, 116, 101, 108, 108, 117, 115, 46, 32, 65, 101, 110, 101, 97, 110, 32, 108,
            101, 111, 32, 108, 105, 103, 117, 108, 97, 44, 32, 112, 111, 114, 116, 116, 105, 116,
            111, 114, 32, 101, 117, 44, 32, 99, 111, 110, 115, 101, 113, 117, 97, 116, 32, 118,
            105, 116, 97, 101, 44, 32, 101, 108, 101, 105, 102, 101, 110, 100, 32, 97, 99, 44, 32,
            101, 110, 105, 109, 46, 32, 65, 108, 105, 113, 117, 97, 109, 32, 108, 111, 114, 101,
            109, 32, 97, 110, 116, 101, 44, 32, 100, 97, 112, 105, 98, 117, 115, 32, 105, 110, 44,
            32, 118, 105, 118, 101, 114, 114, 97, 32, 113, 117, 105, 115, 44, 32, 102, 101, 117,
            103, 105, 97, 116, 32, 97, 44, 32, 116, 101, 108, 108, 117, 115, 46, 32, 80, 104, 97,
            115, 101, 108, 108, 117, 115, 32, 118, 105, 118, 101, 114, 114, 97, 32, 110, 117, 108,
            108, 97, 32, 117, 116, 32, 109, 101, 116, 117, 115, 32, 118, 97, 114, 105, 117, 115,
            32, 108, 97, 111, 114, 101, 101, 116, 46, 32, 81, 117, 105, 115, 113, 117, 101, 32,
            114, 117, 116, 114, 117, 109, 46, 32, 65, 101, 110, 101, 97, 110, 32, 105, 109, 112,
            101, 114, 100, 105, 101, 116, 46, 32, 69, 116, 105, 97, 109, 32, 117, 108, 116, 114,
            105, 99, 105, 101, 115, 32, 110, 105, 115, 105, 32, 118, 101, 108, 32, 97, 117, 103,
            117, 101, 46, 32, 67, 117, 114, 97, 98, 105, 116, 117, 114, 32, 117, 108, 108, 97, 109,
            99, 111, 114, 112, 101, 114, 32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 110,
            105, 115, 105, 46, 32, 78, 97, 109, 32, 101, 103, 101, 116, 32, 100, 117, 105, 46, 32,
            69, 116, 105, 97, 109, 32, 114, 104, 111, 110, 99, 117, 115, 46, 32, 77, 97, 101, 99,
            101, 110, 97, 115, 32, 116, 101, 109, 112, 117, 115, 44, 32, 116, 101, 108, 108, 117,
            115, 32, 101, 103, 101, 116, 32, 99, 111, 110, 100, 105, 109, 101, 110, 116, 117, 109,
            32, 114, 104, 111, 110, 99, 117, 115, 44, 32, 115, 101, 109, 32, 113, 117, 97, 109, 32,
            115, 101, 109, 112, 101, 114, 32, 108, 105, 98, 101, 114, 111, 44, 32, 115, 105, 116,
            32, 97, 109, 101, 116, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 115,
            101, 109, 32, 110, 101, 113, 117, 101, 32, 115, 101, 100, 32, 105, 112, 115, 117, 109,
            46, 32, 78, 97, 109, 32, 113, 117, 97, 109, 32, 110, 117, 110, 99, 44, 32, 98, 108, 97,
            110, 100, 105, 116, 32, 118, 101, 108, 44, 32, 108, 117, 99, 116, 117, 115, 32, 112,
            117, 108, 118, 105, 110, 97, 114, 44, 32, 104, 101, 110, 100, 114, 101, 114, 105, 116,
            32, 105, 100, 44, 32, 108, 111, 114, 101, 109, 46, 32, 77, 97, 101, 99, 101, 110, 97,
            115, 32, 110, 101, 99, 32, 111, 100, 105, 111, 32, 101, 116, 32, 97, 110, 116, 101, 32,
            116, 105, 110, 99, 105, 100, 117, 110, 116, 32, 116, 101, 109, 112, 117, 115, 46, 32,
            68, 111, 110, 101, 99, 32, 118, 105, 116, 97, 101, 32, 115, 97, 112, 105, 101, 110, 32,
            117, 116, 32, 108, 105, 98, 101, 114, 111, 32, 118, 101, 110, 101, 110, 97, 116, 105,
            115, 32, 102, 97, 117, 99, 105, 98, 117, 115, 46, 32, 78, 117, 108, 108, 97, 109, 32,
            113, 117, 105, 115, 32, 97, 110, 116, 101, 46, 32, 69, 116, 105, 97, 109, 32, 115, 105,
            116, 32, 97, 109, 101, 116, 32, 111, 114, 99, 105, 32, 101, 103, 101, 116, 32, 101,
            114, 111, 115, 32, 102, 97, 117, 99, 105, 98, 117, 115, 32, 116, 105, 110, 99, 105,
            100, 117, 110, 116, 46, 32, 68, 117, 105, 115, 32, 108, 101, 111, 46, 32, 83, 101, 100,
            32, 102, 114, 105, 110, 103, 105, 108, 108, 97, 32, 109, 97, 117, 114, 105, 115, 32,
            115, 105, 116, 32, 97, 109, 101, 116, 32, 110, 105, 98, 104, 46, 32, 68, 111, 110, 101,
            99, 32, 115, 111, 100, 97, 108, 101, 115, 32, 115, 97, 103, 105, 116, 116, 105, 115,
            32, 109, 97, 103, 110, 97, 46, 32, 83, 101, 100, 32, 99, 111, 110, 115, 101, 113, 117,
            97, 116, 44, 32, 108, 101, 111, 32, 101, 103, 101, 116, 32, 98, 105, 98, 101, 110, 100,
            117, 109, 32, 115, 111, 100, 97, 108, 101, 115, 44, 32, 97, 117, 103, 117, 101, 32,
            118, 101, 108, 105, 116, 32, 99, 117, 114, 115, 117, 115, 32, 110, 117, 110, 99, 44,
            32, 113, 117, 105, 115, 32, 103, 114, 97, 118, 105, 100, 97, 32, 109, 97, 103, 110, 97,
            32, 109, 105, 32, 97, 32, 108, 105, 98, 101, 114, 111, 46, 32, 70, 117, 115, 99, 101,
            32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 101, 108, 101, 105, 102, 101, 110,
            100, 32, 115, 97, 112, 105, 101, 110, 46, 32, 86, 101, 115, 116, 105, 98, 117, 108,
            117, 109, 32, 112, 117, 114, 117, 115, 32, 113, 117, 97, 109, 44, 32, 115, 99, 101,
            108, 101, 114, 105, 115, 113, 117, 101, 32, 117, 116, 44, 32, 109, 111, 108, 108, 105,
            115, 32, 115, 101, 100, 44, 32, 110, 111, 110, 117, 109, 109, 121, 32, 105, 100, 44,
            32, 109, 101, 116, 117, 115, 46, 32, 78, 117, 108, 108, 97, 109, 32, 97, 99, 99, 117,
            109, 115, 97, 110, 32, 108, 111, 114, 101, 109, 32, 105, 110, 32, 100, 117, 105, 46,
            32, 67, 114, 97, 115, 32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 109, 105, 32,
            101, 117, 32, 116, 117, 114, 112, 105, 115, 32, 104, 101, 110, 100, 114, 101, 114, 105,
            116, 32, 102, 114, 105, 110, 103, 105, 108, 108, 97, 46, 32, 86, 101, 115, 116, 105,
            98, 117, 108, 117, 109, 32, 97, 110, 116, 101, 32, 105, 112, 115, 117, 109, 32, 112,
            114, 105, 109, 105, 115, 32, 105, 110, 32, 102, 97, 117, 99, 105, 98, 117, 115, 32,
            111, 114, 99, 105, 32, 108, 117, 99, 116, 117, 115, 32, 101, 116, 32, 117, 108, 116,
            114, 105, 99, 101, 115, 32, 112, 111, 115, 117, 101, 114, 101, 32, 99, 117, 98, 105,
            108, 105, 97, 32, 67, 117, 114, 97, 101, 59, 32, 73, 110, 32, 97, 99, 32, 100, 117,
            105, 32, 113, 117, 105, 115, 32, 109, 105, 32, 99, 111, 110, 115, 101, 99, 116, 101,
            116, 117, 101, 114, 32, 108, 97, 99, 105, 110, 105, 97, 46, 32, 78, 97, 109, 32, 112,
            114, 101, 116, 105, 117, 109, 32, 116, 117, 114, 112, 105, 115, 32, 101, 116, 32, 97,
            114, 99, 117, 46, 32, 68, 117, 105, 115, 32, 97, 114, 99, 117, 32, 116, 111, 114, 116,
            111, 114, 44, 32, 115, 117, 115, 99, 105, 112, 105, 116, 32, 101, 103, 101, 116, 44,
            32, 105, 109, 112, 101, 114, 100, 105, 101, 116, 32, 110, 101, 99, 44, 32, 105, 109,
            112, 101, 114, 100, 105, 101, 116, 32, 105, 97, 99, 117, 108, 105, 115, 44, 32, 105,
            112, 115, 117, 109, 46, 32, 83, 101, 100, 32, 97, 108, 105, 113, 117, 97, 109, 32, 117,
            108, 116, 114, 105, 99, 101, 115, 32, 109, 97, 117, 114, 105, 115, 46, 32, 73, 110,
            116, 101, 103, 101, 114, 32, 97, 110, 116, 101, 32, 97, 114, 99, 117, 44, 32, 97, 99,
            99, 117, 109, 115, 97, 110, 32, 97, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116,
            117, 101, 114, 32, 101, 103, 101, 116, 44, 32, 112, 111, 115, 117, 101, 114, 101, 32,
            117, 116, 44, 32, 109, 97, 117, 114, 105, 115, 46, 32, 80, 114, 97, 101, 115, 101, 110,
            116, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 46, 32, 80, 104, 97, 115, 101,
            108, 108, 117, 115, 32, 117, 108, 108, 97, 109, 99, 111, 114, 112, 101, 114, 32, 105,
            112, 115, 117, 109, 32, 114, 117, 116, 114, 117, 109, 32, 110, 117, 110, 99, 46, 32,
            78, 117, 110, 99, 32, 110, 111, 110, 117, 109, 109, 121, 32, 109, 101, 116, 117, 115,
            46, 32, 86, 101, 115, 116, 105, 98, 117, 108, 117, 109, 32, 118, 111, 108, 117, 116,
            112, 97, 116, 32, 112, 114, 101, 116, 105, 117, 109, 32, 108, 105, 98, 101, 114, 111,
            46, 32, 67, 114, 97, 115, 32, 105, 100, 32, 100, 117, 105, 46, 32, 65, 101, 110, 101,
            97, 110, 32, 117, 116, 32, 101, 114, 111, 115, 32, 101, 116, 32, 110, 105, 115, 108,
            32, 115, 97, 103, 105, 116, 116, 105, 115, 32, 118, 101, 115, 116, 105, 98, 117, 108,
            117, 109, 46, 32, 78, 117, 108, 108, 97, 109, 32, 110, 117, 108, 108, 97, 32, 101, 114,
            111, 115, 44, 32, 117, 108, 116, 114, 105, 99, 105, 101, 115, 32, 115, 105, 116, 32,
            97, 109, 101, 116, 44, 32, 110, 111, 110, 117, 109, 109, 121, 32, 105, 100, 44, 32,
            105, 109, 112, 101, 114, 100, 105, 101, 116, 32, 102, 101, 117, 103, 105, 97, 116, 44,
            32, 112, 101, 100, 101, 46, 32, 83, 101, 100, 32, 108, 101, 99, 116, 117, 115, 46, 32,
            68, 111, 110, 101, 99, 32, 109, 111, 108, 108, 105, 115, 32, 104, 101, 110, 100, 114,
            101, 114, 105, 116, 32, 114, 105, 115, 117, 115, 46, 32, 80, 104, 97, 115, 101, 108,
            108, 117, 115, 32, 110, 101, 99, 32, 115, 101, 109, 32, 105, 110, 32, 106, 117, 115,
            116, 111, 32, 112, 101, 108, 108, 101, 110, 116, 101, 115, 113, 117, 101, 32, 102, 97,
            99, 105, 108, 105, 115, 105, 115, 46, 32, 69, 116, 105, 97, 109, 32, 105, 109, 112,
            101, 114, 100, 105, 101, 116, 32, 105, 109, 112, 101, 114, 100, 105, 101, 116, 32, 111,
            114, 99, 105, 46, 32, 78, 117, 110, 99, 32, 110, 101, 99, 32, 110, 101, 113, 117, 101,
            46, 32, 80, 104, 97, 115, 101, 108, 108, 117, 115, 32, 108, 101, 111, 32, 100, 111,
            108, 111, 114, 44, 32, 116, 101, 109, 112, 117, 115, 32, 110, 111, 110, 44, 32, 97,
            117, 99, 116, 111, 114, 32, 101, 116, 44, 32, 104, 101, 110, 100, 114, 101, 114, 105,
            116, 32, 113, 117, 105, 115, 44, 32, 110, 105, 115, 105, 46, 32, 67, 117, 114, 97, 98,
            105, 116, 117, 114, 32, 108, 105, 103, 117, 108, 97, 32, 115, 97, 112, 105, 101, 110,
            44, 32, 116, 105, 110, 99, 105, 100, 117, 110, 116, 32, 110, 111, 110, 44, 32, 101,
            117, 105, 115, 109, 111, 100, 32, 118, 105, 116, 97, 101, 44, 32, 112, 111, 115, 117,
            101, 114, 101, 32, 105, 109, 112, 101, 114, 100, 105, 101, 116, 44, 32, 108, 101, 111,
            46, 32, 77, 97, 101, 99, 101, 110, 97, 115, 32, 109, 97, 108, 101, 115, 117, 97, 100,
            97, 46, 32, 80, 114, 97, 101, 115, 101, 110, 116, 32, 99, 111, 110, 103, 117, 101, 32,
            101, 114, 97, 116, 32, 97, 116, 32, 109, 97, 115, 115, 97, 46, 32, 83, 101, 100, 32,
            99, 117, 114, 115, 117, 115, 32, 116, 117, 114, 112, 105, 115, 32, 118, 105, 116, 97,
            101, 32, 116, 111, 114, 116, 111, 114, 46, 32, 68, 111, 110, 101, 99, 32, 112, 111,
            115, 117, 101, 114, 101, 32, 118, 117, 108, 112, 117, 116, 97, 116, 101, 32, 97, 114,
            99, 117, 46, 32, 80, 104, 97, 115, 101, 108, 108, 117, 115, 32, 97, 99, 99, 117, 109,
            115, 97, 110, 32, 99, 117, 114, 115, 117, 115, 32, 118, 101, 108, 105, 116, 46, 32, 86,
            101, 115, 116, 105, 98, 117, 108, 117, 109, 32, 97, 110, 116, 101, 32, 105, 112, 115,
            117, 109, 32, 112, 114, 105, 109, 105, 115, 32, 105, 110, 32, 102, 97, 117, 99, 105,
            98, 117, 115, 32, 111, 114, 99, 105, 32, 108, 117, 99, 116, 117, 115, 32, 101, 116, 32,
            117, 108, 116, 114, 105, 99, 101, 115, 32, 112, 111, 115, 117, 101, 114, 101, 32, 99,
            117, 98, 105, 108, 105, 97, 32, 67, 117, 114, 97, 101, 59, 32, 83, 101, 100, 32, 97,
            108, 105, 113, 117, 97, 109, 44, 32, 110, 105, 115, 105, 32, 113, 117, 105, 115, 32,
            112, 111, 114, 116, 116, 105, 116, 111, 114, 32, 99, 111, 110, 103, 117, 101, 44, 32,
            101, 108, 105, 116, 32, 101, 114, 97, 116, 32, 101, 117, 105, 115, 109, 111, 100, 32,
            111, 114, 99, 105, 44, 32, 97, 99, 128, 0, 0, 0, 101,
        ]];

        for ltbl in &long_tbls {
            rows.rows = vec![];
            rows.decode(ltbl)?;
            assert_eq!(
                DecodeFieldData::Decimal(DecodeDecimal::Decimal("101".parse().unwrap())),
                rows.rows[0][1]
            )
        }

        Ok(())
    }

    // mysql> desc aenum;
    // +-------+-------------------------------------------+------+-----+---------+-------+
    // | Field | Type                                      | Null | Key | Default | Extra |
    // +-------+-------------------------------------------+------+-----+---------+-------+
    // | id    | int(11)                                   | YES  |     | NULL    |       |
    // | aset  | enum('0','1','2','3','4','5','6','7','8') | YES  |     | NULL    |       |
    // +-------+-------------------------------------------+------+-----+---------+-------+
    // 2 rows in set (0.00 sec)
    //
    // insert into aenum(id, aset) values(1, '0');
    #[test]
    fn test_enum() -> Result<(), ReplicationError> {
        let mut table_map_event_data =
            b"\x42\x0f\x00\x00\x00\x00\x01\x00\x05\x74\x74\x65\x73\x74\x00\x05".to_vec();
        table_map_event_data
            .extend(b"\x61\x65\x6e\x75\x6d\x00\x02\x03\xfe\x02\xf7\x01\x03".to_vec());
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let data =
            b"\x42\x0f\x00\x00\x00\x00\x01\x00\x02\x00\x02\xff\xfc\x01\x00\x00\x00\x01".to_vec();

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(DecodeFieldData::Isize(1), rows.rows[0][1]);

        Ok(())
    }

    // CREATE TABLE numbers (
    // 	id int auto_increment,
    // 	num ENUM( '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '10', '11', '12', '13', '14', '15', '16', '17', '18', '19', '20', '21', '22', '23', '24', '25', '26', '27', '28', '29', '30', '31', '32', '33', '34', '35', '36', '37', '38', '39', '40', '41', '42', '43', '44', '45', '46', '47', '48', '49', '50', '51', '52', '53', '54', '55', '56', '57', '58', '59', '60', '61', '62', '63', '64', '65', '66', '67', '68', '69', '70', '71', '72', '73', '74', '75', '76', '77', '78', '79', '80', '81', '82', '83', '84', '85', '86', '87', '88', '89', '90', '91', '92', '93', '94', '95', '96', '97', '98', '99', '100', '101', '102', '103', '104', '105', '106', '107', '108', '109', '110', '111', '112', '113', '114', '115', '116', '117', '118', '119', '120', '121', '122', '123', '124', '125', '126', '127', '128', '129', '130', '131', '132', '133', '134', '135', '136', '137', '138', '139', '140', '141', '142', '143', '144', '145', '146', '147', '148', '149', '150', '151', '152', '153', '154', '155', '156', '157', '158', '159', '160', '161', '162', '163', '164', '165', '166', '167', '168', '169', '170', '171', '172', '173', '174', '175', '176', '177', '178', '179', '180', '181', '182', '183', '184', '185', '186', '187', '188', '189', '190', '191', '192', '193', '194', '195', '196', '197', '198', '199', '200', '201', '202', '203', '204', '205', '206', '207', '208', '209', '210', '211', '212', '213', '214', '215', '216', '217', '218', '219', '220', '221', '222', '223', '224', '225', '226', '227', '228', '229', '230', '231', '232', '233', '234', '235', '236', '237', '238', '239', '240', '241', '242', '243', '244', '245', '246', '247', '248', '249', '250', '251', '252', '253', '254', '255','256','257'

    // ),
    // primary key(id)
    // );

    //
    // insert into numbers(num) values ('0'), ('256');
    #[test]
    fn test_multi_bytes_enum() -> Result<(), ReplicationError> {
        let mut table_map_event_data =
            b"\x84\x0f\x00\x00\x00\x00\x01\x00\x05\x74\x74\x65\x73\x74\x00\x07".to_vec();
        table_map_event_data
            .extend(b"\x6e\x75\x6d\x62\x65\x72\x73\x00\x02\x03\xfe\x02\xf7\x02\x02".to_vec());
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let data =
        b"\x84\x0f\x00\x00\x00\x00\x01\x00\x02\x00\x02\xff\xfc\x01\x00\x00\x00\x01\x00\xfc\x02\x00\x00\x00\x01\x01".to_vec();

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(DecodeFieldData::Isize(1), rows.rows[0][1]);
        assert_eq!(DecodeFieldData::Isize(257), rows.rows[1][1]);

        Ok(())
    }

    // mysql> desc aset;
    // +--------+---------------------------------------------------------------------------------------+------+-----+---------+-------+
    // | Field  | Type                                                                                  | Null | Key | Default | Extra |
    // +--------+---------------------------------------------------------------------------------------+------+-----+---------+-------+
    // | id     | int(11)                                                                               | YES  |     | NULL    |       |
    // | region | set('1','2','3','4','5','6','7','8','9','10','11','12','13','14','15','16','17','18') | YES  |     | NULL    |       |
    // +--------+---------------------------------------------------------------------------------------+------+-----+---------+-------+
    // 2 rows in set (0.00 sec)
    //
    // insert into aset(id, region) values(1, '1,3');
    #[test]
    fn test_set() -> Result<(), ReplicationError> {
        let mut table_map_event_data =
            b"\xe7\x0e\x00\x00\x00\x00\x01\x00\x05\x74\x74\x65\x73\x74\x00\x04".to_vec();
        table_map_event_data.extend(b"\x61\x73\x65\x74\x00\x02\x03\xfe\x02\xf8\x03\x03".to_vec());
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let data =
            b"\xe7\x0e\x00\x00\x00\x00\x01\x00\x02\x00\x02\xff\xfc\x01\x00\x00\x00\x05\x00\x00"
                .to_vec();

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(DecodeFieldData::Isize(5), rows.rows[0][1]);

        Ok(())
    }

    // Table:
    // desc hj_order_preview
    // +------------------+------------+------+-----+-------------------+----------------+
    // | Field            | Type       | Null | Key | Default           | Extra          |
    // +------------------+------------+------+-----+-------------------+----------------+
    // | id               | int(13)    | NO   | PRI | <null>            | auto_increment |
    // | buyer_id         | bigint(13) | NO   |     | <null>            |                |
    // | order_sn         | bigint(13) | NO   |     | <null>            |                |
    // | order_detail     | json       | NO   |     | <null>            |                |
    // | is_del           | tinyint(1) | NO   |     | 0                 |                |
    // | add_time         | int(13)    | NO   |     | <null>            |                |
    // | last_update_time | timestamp  | NO   |     | CURRENT_TIMESTAMP |                |
    // +------------------+------------+------+-----+-------------------+----------------+
    // insert into hj_order_preview
    // (id, buyer_id, order_sn, is_del, add_time, last_update_time)
    // values (1, 95891865464386, 13376222192996417, 0, 1479983995, 1479983995)
    #[test]
    fn test_json_null() -> Result<(), ReplicationError> {
        let table_map_event_data = vec![
            114, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 16, 104, 106, 95, 111, 114, 100,
            101, 114, 95, 112, 114, 101, 118, 105, 101, 119, 0, 7, 3, 8, 8, 245, 1, 3, 17, 2, 4, 0,
            0,
        ];

        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let data = vec![
            114, 0, 0, 0, 0, 0, 1, 0, 2, 0, 7, 255, 128, 1, 0, 0, 0, 66, 238, 147, 144, 54, 87, 0,
            0, 65, 16, 64, 108, 154, 133, 47, 0, 0, 0, 0, 0, 0, 123, 195, 54, 88, 0, 0, 0, 0,
        ];

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::Bytes(vec![])),
            rows.rows[0][3]
        );

        Ok(())
    }

    // Table:
    // mysql> desc t11;
    // +----------+--------------+------+-----+---------+-------------------+
    // | Field    | Type         | Null | Key | Default | Extra             |
    // +----------+--------------+------+-----+---------+-------------------+
    // | id       | int(11)      | YES  |     | NULL    |                   |
    // | cfg      | varchar(100) | YES  |     | NULL    |                   |
    // | cfg_json | json         | YES  |     | NULL    | VIRTUAL GENERATED |
    // | age      | int(11)      | YES  |     | NULL    |                   |
    // +----------+--------------+------+-----+---------+-------------------+
    // mysql> insert into t11(id, cfg) values (1, '{}');

    // test json deserialization
    // mysql> update t11 set cfg = '{"a":1234}' where id = 1;
    // mysql> update test set cfg = '{}' where id = 1;
    #[test]
    fn test_json_compatibility() -> Result<(), ReplicationError> {
        let table_map_event_data =
            b"l\x00\x00\x00\x00\x00\x01\x00\x04test\x00\x03t11\x00\x04\x03\x0f\xf5\x03\x03d\x00\x04\x0f".to_vec();
        let mut table_map_event = TableMapEvent::default();
        table_map_event.table_id_size = 6;
        table_map_event.decode(&table_map_event_data)?;

        let mut rows = RowsEvent::default();
        rows.table_id_size = 6;
        rows.tables = HashMap::default();
        rows.tables
            .insert(table_map_event.table_id, table_map_event);
        rows.version = 2;

        let data =
            b"l\x00\x00\x00\x00\x00\x01\x00\x02\x00\x04\xff\xf8\x01\x00\x00\x00\x02{}\x05\x00\x00\x00\x00\x00\x00\x04\x00"
                .to_vec();

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{}".to_string())),
            rows.rows[0][2]
        );

        // after MySQL 5.7.22
        let data =
            b"l\x00\x00\x00\x00\x00\x01\x00\x02\x00\x04\xff\xff\xf8\x01\x00\x00\x00\x02{}\x05\x00\x00\x00\x00\x00\x00\x04\x00\xf8\x01\x00\x00\x00\n{\"a\":1234}\r\x00\x00\x00\x00\x01\x00\x0c\x00\x0b\x00\x01\x00\x05\xd2\x04a"
                .to_vec();

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{\"a\":1234}".to_string())),
            rows.rows[2][2]
        );

        let data =
            b"l\x00\x00\x00\x00\x00\x01\x00\x02\x00\x04\xff\xff\xf8\x01\x00\x00\x00\n{\"a\":1234}\r\x00\x00\x00\x00\x01\x00\x0c\x00\x0b\x00\x01\x00\x05\xd2\x04a\xf8\x01\x00\x00\x00\x02{}\x05\x00\x00\x00\x00\x00\x00\x04\x00"
                .to_vec();

        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{\"a\":1234}".to_string())),
            rows.rows[1][2]
        );
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{}".to_string())),
            rows.rows[2][2]
        );

        // before MySQL 5.7.22
        let data =
            b"l\x00\x00\x00\x00\x00\x01\x00\x02\x00\x04\xff\xff\xf8\x01\x00\x00\x00\x02{}\x05\x00\x00\x00\x00\x01\x00\x0c\x00\xf8\x01\x00\x00\x00\n{\"a\":1234}\r\x00\x00\x00\x00\x01\x00\x0c\x00\x0b\x00\x01\x00\x05\xd2\x04a"
                .to_vec();

        rows.ignore_json_decode_err = true;
        rows.rows = vec![];
        rows.decode(&data)?;
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("null".to_string())),
            rows.rows[1][2]
        );
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{\"a\":1234}".to_string())),
            rows.rows[2][2]
        );

        let data =
            b"l\x00\x00\x00\x00\x00\x01\x00\x02\x00\x04\xff\xff\xf8\x01\x00\x00\x00\n{\"a\":1234}\r\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x05\xd2\x04a\xf8\x01\x00\x00\x00\x02{}\x05\x00\x00\x00\x00\x00\x00\x04\x00"
                .to_vec();

        rows.ignore_json_decode_err = false;
        rows.rows = vec![];
        rows.decode(&data)?;
        // this value is wrong in binlog, but can be parsed without error
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{}".to_string())),
            rows.rows[1][2]
        );
        assert_eq!(
            DecodeFieldData::Json(DecodeJson::String("{}".to_string())),
            rows.rows[2][2]
        );

        Ok(())
    }

    #[test]
    fn test_decode_datetime2() -> Result<(), ReplicationError> {
        struct Case {
            pub data: Vec<u8>,
            pub dec: u16,
            pub get_frac_time: bool,
            pub expected: String,
        }
        let testcases = vec![
            Case {
                data: b"\xfe\xf3\xff\x7e\xfb".to_vec(),
                dec: 0,
                get_frac_time: true,
                expected: "9999-12-31 23:59:59".to_string(),
            },
            Case {
                data: b"\x99\x9a\xb8\xf7\xaa".to_vec(),
                dec: 0,
                get_frac_time: true,
                expected: "2016-10-28 15:30:42".to_string(),
            },
            Case {
                data: b"\x99\x02\xc2\x00\x00".to_vec(),
                dec: 0,
                get_frac_time: true,
                expected: "1970-01-01 00:00:00".to_string(),
            },
            Case {
                data: b"\x80\x00\x00\x00\x00".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "0000-00-00 00:00:00".to_string(),
            },
            Case {
                data: b"\x80\x00\x02\xf1\x05".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "0000-00-01 15:04:05".to_string(),
            },
            Case {
                data: b"\x80\x03\x82\x00\x00".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "0001-01-01 00:00:00".to_string(),
            },
            Case {
                data: b"\x80\x03\x82\x00\x00\x0c".to_vec(),
                dec: 2,
                get_frac_time: false,
                expected: "0001-01-01 00:00:00.12".to_string(),
            },
            Case {
                data: b"\x80\x03\x82\x00\x00\x04\xd3".to_vec(),
                dec: 4,
                get_frac_time: false,
                expected: "0001-01-01 00:00:00.1235".to_string(),
            },
            Case {
                data: b"\x80\x03\x82\x00\x00\x01\xe2\x40".to_vec(),
                dec: 6,
                get_frac_time: false,
                expected: "0001-01-01 00:00:00.123456".to_string(),
            },
        ];

        for tc in &testcases {
            let (value, _) = decode_helper::decode_datetime2(&tc.data, tc.dec)?;
            match value {
                DecodeDatetime::FracTime(ref v) => {
                    assert_eq!(true, tc.get_frac_time);
                    assert_eq!(tc.expected, v.to_string());
                }
                DecodeDatetime::String(ref v) => {
                    assert_eq!(false, tc.get_frac_time);
                    assert_eq!(&tc.expected, v);
                }
            }
        }
        Ok(())
    }

    /*
        create table _null (c1 int null, c2 int not null default '2', c3 timestamp default now(), c4 text);
    */
    #[test]
    fn test_table_map_nullable() -> Result<(), ReplicationError> {
        let nullables = vec![true, false, false, true];
        let testcases = vec![
            vec![
                122, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 110, 117, 108, 108, 0,
                4, 3, 3, 17, 252, 2, 0, 2, 9,
            ],
            vec![
                122, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 110, 117, 108, 108, 0,
                4, 3, 3, 17, 252, 2, 0, 2, 9, 1, 1, 0, 2, 1, 224, 4, 12, 2, 99, 49, 2, 99, 50, 2,
                99, 51, 2, 99, 52,
            ],
            vec![
                30, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 110, 117, 108, 108, 0, 4,
                3, 3, 17, 252, 2, 0, 2, 9,
            ],
            vec![
                29, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 110, 117, 108, 108, 0, 4,
                3, 3, 17, 252, 2, 0, 2, 9, 1, 1, 0, 2, 1, 224, 4, 12, 2, 99, 49, 2, 99, 50, 2, 99,
                51, 2, 99, 52,
            ],
        ];

        for tc in &testcases {
            let mut table_map_event = TableMapEvent::default();
            table_map_event.table_id_size = 6;
            table_map_event.decode(tc)?;
            assert_eq!(table_map_event.column_count as usize, nullables.len());

            for i in 0..table_map_event.column_count as usize {
                let (available, nullable) = table_map_event.nullable(i);
                assert_eq!(true, available);
                assert_eq!(nullables[i], nullable);
            }
        }
        Ok(())
    }

    /*
        CREATE TABLE `_types` (
            `b_bit` bit(64) NOT NULL DEFAULT b'0',

            `n_boolean` boolean not null default '0',
            `n_tinyint` tinyint not null default '0',
            `n_smallint` smallint not null default '0',
            `n_mediumint` mediumint not null default '0',
            `n_int` int not null default '0',
            `n_bigint` bigint not null default '0',
            `n_decimal` decimal(65,30) not null default '0',
            `n_float` float not null default '0',
            `n_double` double not null default '0',

            `nu_tinyint` tinyint unsigned not null default '0',
            `nu_smallint` smallint unsigned not null default '0',
            `nu_mediumint` mediumint unsigned not null default '0',
            `nu_int` int unsigned not null default '0',
            `nu_bigint` bigint unsigned not null default '0',
            `nu_decimal` decimal(65,30) unsigned not null default '0',
            `nu_float` float unsigned not null default '0',
            `nu_double` double unsigned not null default '0',

            `t_year` year default null,
            `t_date` date default null,
            `t_time` time default null,
            `t_ftime` time(6) default null,
            `t_datetime` datetime default null,
            `t_fdatetime` datetime(6) default null,
            `t_timestamp` timestamp default current_timestamp,
            `t_ftimestamp` timestamp(6) default current_timestamp(6),

            `c_char` char(255) not null default '',
            `c_varchar` varchar(255) not null default '',
            `c_binary` binary(64) not null default '',
            `c_varbinary` varbinary(64) not null default '',
            `c_tinyblob` tinyblob,
            `c_blob` blob,
            `c_mediumblob` mediumblob,
            `c_longblob` longblob,
            `c_tinytext` tinytext,
            `c_text` text,
            `c_mediumtext` mediumtext,
            `c_longtext` longtext,

            `e_enum` enum('a','b') default 'a',
            `s_set` set('1','2') default '1',
            `g_geometry` geometry DEFAULT NULL,
            `j_json` json DEFAULT NULL
        );
        insert into _types values ();
    */
    #[test]
    fn test_table_map_opt_meta_names() -> Result<(), ReplicationError> {
        let col_names = vec![
            "b_bit",
            "n_boolean",
            "n_tinyint",
            "n_smallint",
            "n_mediumint",
            "n_int",
            "n_bigint",
            "n_decimal",
            "n_float",
            "n_double",
            "nu_tinyint",
            "nu_smallint",
            "nu_mediumint",
            "nu_int",
            "nu_bigint",
            "nu_decimal",
            "nu_float",
            "nu_double",
            "t_year",
            "t_date",
            "t_time",
            "t_ftime",
            "t_datetime",
            "t_fdatetime",
            "t_timestamp",
            "t_ftimestamp",
            "c_char",
            "c_varchar",
            "c_binary",
            "c_varbinary",
            "c_tinyblob",
            "c_blob",
            "c_mediumblob",
            "c_longblob",
            "c_tinytext",
            "c_text",
            "c_mediumtext",
            "c_longtext",
            "e_enum",
            "s_set",
            "g_geometry",
            "j_json",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let enum_vals = vec![vec!["a".to_string(), "b".to_string()]];
        let set_vals = vec![vec!["1".to_string(), "2".to_string()]];

        struct Case {
            data: Vec<u8>,
            has_names: bool,
        }
        let testcases = vec![
            // mysql 5.7
            Case {
                data: vec![
                    117, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 42, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 245, 38, 0, 8, 65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0,
                    6, 206, 252, 252, 3, 254, 64, 64, 0, 1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4,
                    4, 0, 0, 252, 192, 255, 3,
                ],
                has_names: false,
            },
            // mysql 8.0
            Case {
                data: vec![
                    106, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 42, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 245, 38, 0, 8, 65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0,
                    6, 206, 252, 252, 3, 254, 64, 64, 0, 1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4,
                    4, 0, 0, 252, 195, 255, 3, 1, 3, 0, 127, 128, 3, 12, 224, 224, 63, 63, 63, 63,
                    63, 63, 224, 224, 224, 224, 7, 1, 0, 4, 252, 148, 1, 5, 98, 95, 98, 105, 116,
                    9, 110, 95, 98, 111, 111, 108, 101, 97, 110, 9, 110, 95, 116, 105, 110, 121,
                    105, 110, 116, 10, 110, 95, 115, 109, 97, 108, 108, 105, 110, 116, 11, 110, 95,
                    109, 101, 100, 105, 117, 109, 105, 110, 116, 5, 110, 95, 105, 110, 116, 8, 110,
                    95, 98, 105, 103, 105, 110, 116, 9, 110, 95, 100, 101, 99, 105, 109, 97, 108,
                    7, 110, 95, 102, 108, 111, 97, 116, 8, 110, 95, 100, 111, 117, 98, 108, 101,
                    10, 110, 117, 95, 116, 105, 110, 121, 105, 110, 116, 11, 110, 117, 95, 115,
                    109, 97, 108, 108, 105, 110, 116, 12, 110, 117, 95, 109, 101, 100, 105, 117,
                    109, 105, 110, 116, 6, 110, 117, 95, 105, 110, 116, 9, 110, 117, 95, 98, 105,
                    103, 105, 110, 116, 10, 110, 117, 95, 100, 101, 99, 105, 109, 97, 108, 8, 110,
                    117, 95, 102, 108, 111, 97, 116, 9, 110, 117, 95, 100, 111, 117, 98, 108, 101,
                    6, 116, 95, 121, 101, 97, 114, 6, 116, 95, 100, 97, 116, 101, 6, 116, 95, 116,
                    105, 109, 101, 7, 116, 95, 102, 116, 105, 109, 101, 10, 116, 95, 100, 97, 116,
                    101, 116, 105, 109, 101, 11, 116, 95, 102, 100, 97, 116, 101, 116, 105, 109,
                    101, 11, 116, 95, 116, 105, 109, 101, 115, 116, 97, 109, 112, 12, 116, 95, 102,
                    116, 105, 109, 101, 115, 116, 97, 109, 112, 6, 99, 95, 99, 104, 97, 114, 9, 99,
                    95, 118, 97, 114, 99, 104, 97, 114, 8, 99, 95, 98, 105, 110, 97, 114, 121, 11,
                    99, 95, 118, 97, 114, 98, 105, 110, 97, 114, 121, 10, 99, 95, 116, 105, 110,
                    121, 98, 108, 111, 98, 6, 99, 95, 98, 108, 111, 98, 12, 99, 95, 109, 101, 100,
                    105, 117, 109, 98, 108, 111, 98, 10, 99, 95, 108, 111, 110, 103, 98, 108, 111,
                    98, 10, 99, 95, 116, 105, 110, 121, 116, 101, 120, 116, 6, 99, 95, 116, 101,
                    120, 116, 12, 99, 95, 109, 101, 100, 105, 117, 109, 116, 101, 120, 116, 10, 99,
                    95, 108, 111, 110, 103, 116, 101, 120, 116, 6, 101, 95, 101, 110, 117, 109, 5,
                    115, 95, 115, 101, 116, 10, 103, 95, 103, 101, 111, 109, 101, 116, 114, 121, 6,
                    106, 95, 106, 115, 111, 110, 10, 1, 224, 5, 5, 2, 1, 49, 1, 50, 6, 5, 2, 1, 97,
                    1, 98,
                ],
                has_names: true,
            },
            // mariadb 10.4
            Case {
                data: vec![
                    27, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 42, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 252, 38, 0, 8, 65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0,
                    6, 206, 252, 252, 3, 254, 64, 64, 0, 1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4,
                    4, 0, 0, 252, 192, 255, 3,
                ],
                has_names: false,
            },
            // mariadb 10.5
            Case {
                data: vec![
                    26, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 42, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 252, 38, 0, 8, 65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0,
                    6, 206, 252, 252, 3, 254, 64, 64, 0, 1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4,
                    4, 0, 0, 252, 192, 255, 3, 1, 3, 0, 127, 192, 3, 14, 224, 224, 63, 63, 63, 63,
                    63, 63, 224, 224, 224, 224, 63, 46, 7, 1, 0, 4, 252, 148, 1, 5, 98, 95, 98,
                    105, 116, 9, 110, 95, 98, 111, 111, 108, 101, 97, 110, 9, 110, 95, 116, 105,
                    110, 121, 105, 110, 116, 10, 110, 95, 115, 109, 97, 108, 108, 105, 110, 116,
                    11, 110, 95, 109, 101, 100, 105, 117, 109, 105, 110, 116, 5, 110, 95, 105, 110,
                    116, 8, 110, 95, 98, 105, 103, 105, 110, 116, 9, 110, 95, 100, 101, 99, 105,
                    109, 97, 108, 7, 110, 95, 102, 108, 111, 97, 116, 8, 110, 95, 100, 111, 117,
                    98, 108, 101, 10, 110, 117, 95, 116, 105, 110, 121, 105, 110, 116, 11, 110,
                    117, 95, 115, 109, 97, 108, 108, 105, 110, 116, 12, 110, 117, 95, 109, 101,
                    100, 105, 117, 109, 105, 110, 116, 6, 110, 117, 95, 105, 110, 116, 9, 110, 117,
                    95, 98, 105, 103, 105, 110, 116, 10, 110, 117, 95, 100, 101, 99, 105, 109, 97,
                    108, 8, 110, 117, 95, 102, 108, 111, 97, 116, 9, 110, 117, 95, 100, 111, 117,
                    98, 108, 101, 6, 116, 95, 121, 101, 97, 114, 6, 116, 95, 100, 97, 116, 101, 6,
                    116, 95, 116, 105, 109, 101, 7, 116, 95, 102, 116, 105, 109, 101, 10, 116, 95,
                    100, 97, 116, 101, 116, 105, 109, 101, 11, 116, 95, 102, 100, 97, 116, 101,
                    116, 105, 109, 101, 11, 116, 95, 116, 105, 109, 101, 115, 116, 97, 109, 112,
                    12, 116, 95, 102, 116, 105, 109, 101, 115, 116, 97, 109, 112, 6, 99, 95, 99,
                    104, 97, 114, 9, 99, 95, 118, 97, 114, 99, 104, 97, 114, 8, 99, 95, 98, 105,
                    110, 97, 114, 121, 11, 99, 95, 118, 97, 114, 98, 105, 110, 97, 114, 121, 10,
                    99, 95, 116, 105, 110, 121, 98, 108, 111, 98, 6, 99, 95, 98, 108, 111, 98, 12,
                    99, 95, 109, 101, 100, 105, 117, 109, 98, 108, 111, 98, 10, 99, 95, 108, 111,
                    110, 103, 98, 108, 111, 98, 10, 99, 95, 116, 105, 110, 121, 116, 101, 120, 116,
                    6, 99, 95, 116, 101, 120, 116, 12, 99, 95, 109, 101, 100, 105, 117, 109, 116,
                    101, 120, 116, 10, 99, 95, 108, 111, 110, 103, 116, 101, 120, 116, 6, 101, 95,
                    101, 110, 117, 109, 5, 115, 95, 115, 101, 116, 10, 103, 95, 103, 101, 111, 109,
                    101, 116, 114, 121, 6, 106, 95, 106, 115, 111, 110, 10, 1, 224, 5, 5, 2, 1, 49,
                    1, 50, 6, 5, 2, 1, 97, 1, 98,
                ],
                has_names: true,
            },
        ];

        for tc in &testcases {
            let mut table_map_event = TableMapEvent::default();
            table_map_event.table_id_size = 6;
            table_map_event.decode(&tc.data)?;

            if tc.has_names {
                assert_eq!(col_names, table_map_event.column_name_string());
                assert_eq!(set_vals, table_map_event.set_str_value_string());
                assert_eq!(enum_vals, table_map_event.enum_str_value_string());
            } else {
                let a: Vec<String> = vec![];
                assert_eq!(a, table_map_event.column_name_string());
                let b: Vec<Vec<String>> = vec![];
                assert_eq!(b, table_map_event.set_str_value_string());
                let c: Vec<Vec<String>> = vec![];
                assert_eq!(c, table_map_event.enum_str_value_string());
            }
        }

        Ok(())
    }

    /*
        create table _prim (id2 int, col varchar(30), id1 bigint, primary key (id1, id2));
    */
    #[test]
    fn test_table_map_opt_meta_primary_key() -> Result<(), ReplicationError> {
        let case1_primary_key = vec![2_u64, 0];
        let case1_primary_key_prefix = vec![0_u64, 0];

        /*
            create table _prim2 (col1 int, id1 char(10), col2 int, id2 varchar(20), primary key (id1, id2(10)));
        */
        let case2_primary_key = vec![1_u64, 3];
        let case2_primary_key_prefix = vec![0_u64, 10];

        struct Case {
            data: Vec<u8>,
            expected_primary_key: Vec<u64>,
            expected_primary_key_prefix: Vec<u64>,
        }

        let testcases = vec![
            Case {
                // mysql 5.7, case1
                data: vec![
                    119, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 112, 114, 105, 109,
                    0, 3, 3, 15, 8, 2, 120, 0, 2,
                ],
                expected_primary_key: vec![],
                expected_primary_key_prefix: vec![],
            },
            Case {
                // mysql 8.0, case1
                data: vec![
                    108, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 112, 114, 105, 109,
                    0, 3, 3, 15, 8, 2, 120, 0, 2, 1, 1, 0, 2, 1, 224, 4, 12, 3, 105, 100, 50, 3,
                    99, 111, 108, 3, 105, 100, 49, 8, 2, 2, 0,
                ],
                expected_primary_key: case1_primary_key.clone(),
                expected_primary_key_prefix: case1_primary_key_prefix.clone(),
            },
            Case {
                // mariadb 10.4, case1
                data: vec![
                    28, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 112, 114, 105, 109,
                    0, 3, 3, 15, 8, 2, 120, 0, 2,
                ],
                expected_primary_key: vec![],
                expected_primary_key_prefix: vec![],
            },
            Case {
                // mariadb 10.5, case1
                data: vec![
                    27, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 5, 95, 112, 114, 105, 109,
                    0, 3, 3, 15, 8, 2, 120, 0, 2, 1, 1, 0, 2, 1, 224, 4, 12, 3, 105, 100, 50, 3,
                    99, 111, 108, 3, 105, 100, 49, 8, 2, 2, 0,
                ],
                expected_primary_key: case1_primary_key.clone(),
                expected_primary_key_prefix: case1_primary_key_prefix.clone(),
            },
            Case {
                // mysql 5.7, case2
                data: vec![
                    121, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 112, 114, 105, 109,
                    50, 0, 4, 3, 254, 3, 15, 4, 254, 40, 80, 0, 5,
                ],
                expected_primary_key: vec![],
                expected_primary_key_prefix: vec![],
            },
            Case {
                // mysql 8.0, case2
                data: vec![
                    109, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 112, 114, 105, 109,
                    50, 0, 4, 3, 254, 3, 15, 4, 254, 40, 80, 0, 5, 1, 1, 0, 2, 1, 224, 4, 18, 4,
                    99, 111, 108, 49, 3, 105, 100, 49, 4, 99, 111, 108, 50, 3, 105, 100, 50, 9, 4,
                    1, 0, 3, 10,
                ],
                expected_primary_key: case2_primary_key.clone(),
                expected_primary_key_prefix: case2_primary_key_prefix.clone(),
            },
            Case {
                // mariadb 10.4, case2
                data: vec![
                    29, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 112, 114, 105, 109,
                    50, 0, 4, 3, 254, 3, 15, 4, 254, 40, 80, 0, 5,
                ],
                expected_primary_key: vec![],
                expected_primary_key_prefix: vec![],
            },
            Case {
                // mariadb 10.5, case2
                data: vec![
                    28, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 112, 114, 105, 109,
                    50, 0, 4, 3, 254, 3, 15, 4, 254, 40, 80, 0, 5, 1, 1, 0, 2, 1, 224, 4, 18, 4,
                    99, 111, 108, 49, 3, 105, 100, 49, 4, 99, 111, 108, 50, 3, 105, 100, 50, 9, 4,
                    1, 0, 3, 10,
                ],
                expected_primary_key: case2_primary_key.clone(),
                expected_primary_key_prefix: case2_primary_key_prefix.clone(),
            },
        ];

        for tc in &testcases {
            let mut table_map_event = TableMapEvent::default();
            table_map_event.table_id_size = 6;
            table_map_event.decode(&tc.data)?;
            assert_eq!(&tc.expected_primary_key, &table_map_event.primary_key);
            assert_eq!(
                &tc.expected_primary_key_prefix,
                &table_map_event.primary_key_prefix
            );
        }

        Ok(())
    }

    /*
        CREATE TABLE `_types` (
            `b_bit` bit(64) NOT NULL DEFAULT b'0',

            `n_boolean` boolean not null default '0',
            `n_tinyint` tinyint not null default '0',
            `n_smallint` smallint not null default '0',
            `n_mediumint` mediumint not null default '0',
            `n_int` int not null default '0',
            `n_bigint` bigint not null default '0',
            `n_decimal` decimal(65,30) not null default '0',
            `n_float` float not null default '0',
            `n_double` double not null default '0',

            `nu_tinyint` tinyint unsigned not null default '0',
            `nu_smallint` smallint unsigned not null default '0',
            `nu_mediumint` mediumint unsigned not null default '0',
            `nu_int` int unsigned not null default '0',
            `nu_bigint` bigint unsigned not null default '0',
            `nu_decimal` decimal(65,30) unsigned not null default '0',
            `nu_float` float unsigned not null default '0',
            `nu_double` double unsigned not null default '0',

            `t_year` year default null,
            `t_date` date default null,
            `t_time` time default null,
            `t_ftime` time(6) default null,
            `t_datetime` datetime default null,
            `t_fdatetime` datetime(6) default null,
            `t_timestamp` timestamp default current_timestamp,
            `t_ftimestamp` timestamp(6) default current_timestamp(6),

            `c_char` char(255) collate gbk_chinese_ci not null default '',  -- collate id: 28
            `c_varchar` varchar(255) not null default '',
            `c_binary` binary(64) not null default '',
            `c_varbinary` varbinary(64) not null default '',
            `c_tinyblob` tinyblob,
            `c_blob` blob,
            `c_mediumblob` mediumblob,
            `c_longblob` longblob,
            `c_tinytext` tinytext,
            `c_text` text,
            `c_mediumtext` mediumtext,
            `c_longtext` longtext,

            `e_enum` enum('a','b') default 'a',
            `s_set` set('1','2') default '1',
            `g_geometry` geometry default null,
            `j_json` json default null,

            `s_set2` set('3','4') collate gbk_chinese_ci default '4',
            `e_enum2` enum('c','d') collate gbk_chinese_ci default 'd',
            `g_geometrycollection` geometrycollection default null,
            `g_multipolygon` multipolygon default null,
            `g_multilinestring` multilinestring default null,
            `g_multipoint` multipoint default null,
            `g_polygon` polygon default null,
            `g_linestring` linestring default null,
            `g_point` point default null
        );
    */
    #[test]
    fn test_table_map_helper_maps() -> Result<(), ReplicationError> {
        let mut unsigned_map = HashMap::<isize, bool>::new();
        for i in 1..=9 {
            unsigned_map.insert(i, false);
        }
        for i in 10..=17 {
            unsigned_map.insert(i, true);
        }

        // collation id | collatation
        //     28       | gbk_chinese_ci
        //     46       | utf8mb4_bin
        //     63       | binary
        //     224      | utf8mb4_unicode_ci
        let mut mysql_collation_map = HashMap::<isize, u64>::new();
        // (var)char/(var)binary
        mysql_collation_map.insert(26, 28);
        mysql_collation_map.insert(27, 224);
        mysql_collation_map.insert(28, 63);
        mysql_collation_map.insert(29, 63);
        // blobs
        mysql_collation_map.insert(30, 63);
        mysql_collation_map.insert(31, 63);
        mysql_collation_map.insert(32, 63);
        mysql_collation_map.insert(33, 63);
        // texts
        mysql_collation_map.insert(34, 224);
        mysql_collation_map.insert(35, 224);
        mysql_collation_map.insert(36, 224);
        mysql_collation_map.insert(37, 224);

        // NOTE: mariadb treat json/geometry as character fields
        let mut mariadb_collation_map = HashMap::<isize, u64>::new();
        // (var)char/(var)binary
        mariadb_collation_map.insert(26, 28);
        mariadb_collation_map.insert(27, 224);
        mariadb_collation_map.insert(28, 63);
        mariadb_collation_map.insert(29, 63);
        // blobs
        mariadb_collation_map.insert(30, 63);
        mariadb_collation_map.insert(31, 63);
        mariadb_collation_map.insert(32, 63);
        mariadb_collation_map.insert(33, 63);
        // texts
        mariadb_collation_map.insert(34, 224);
        mariadb_collation_map.insert(35, 224);
        mariadb_collation_map.insert(36, 224);
        mariadb_collation_map.insert(37, 224);
        // geometry
        mariadb_collation_map.insert(40, 63);
        // json
        mariadb_collation_map.insert(41, 46);
        // geometry
        mariadb_collation_map.insert(44, 63);
        mariadb_collation_map.insert(45, 63);
        mariadb_collation_map.insert(46, 63);
        mariadb_collation_map.insert(47, 63);
        mariadb_collation_map.insert(48, 63);
        mariadb_collation_map.insert(49, 63);
        mariadb_collation_map.insert(50, 63);

        let mut enum_set_collation_map = HashMap::<isize, u64>::new();
        enum_set_collation_map.insert(38, 224);
        enum_set_collation_map.insert(39, 224);
        enum_set_collation_map.insert(42, 28);
        enum_set_collation_map.insert(43, 28);

        let mut enum_str_value_map = HashMap::<isize, Vec<String>>::new();
        enum_str_value_map.insert(38, vec!["a".to_string(), "b".to_string()]);
        enum_str_value_map.insert(43, vec!["c".to_string(), "d".to_string()]);

        let mut set_str_value_map = HashMap::<isize, Vec<String>>::new();
        set_str_value_map.insert(39, vec!["1".to_string(), "2".to_string()]);
        set_str_value_map.insert(42, vec!["3".to_string(), "4".to_string()]);

        let mut geometry_type_map = HashMap::<isize, u64>::new();
        geometry_type_map.insert(40, 0);
        geometry_type_map.insert(44, 7);
        geometry_type_map.insert(45, 6);
        geometry_type_map.insert(46, 5);
        geometry_type_map.insert(47, 4);
        geometry_type_map.insert(48, 3);
        geometry_type_map.insert(49, 2);
        geometry_type_map.insert(50, 1);

        struct Case {
            flavor: String,
            data: Vec<u8>,
            unsigned_map: HashMap<isize, bool>,
            collation_map: HashMap<isize, u64>,
            enum_set_collation_map: HashMap<isize, u64>,
            enum_str_value_map: HashMap<isize, Vec<String>>,
            set_str_value_map: HashMap<isize, Vec<String>>,
            geometry_type_map: HashMap<isize, u64>,
        }

        let testcases = vec![
            Case {
                flavor: "mysql".to_string(), // mysql 8.0,
                data: vec![
                    101, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 51, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 245, 254, 254, 255, 255, 255, 255, 255, 255, 255, 49, 0, 8,
                    65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0, 6, 238, 254, 252, 3, 254, 64, 64, 0,
                    1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4, 4, 248, 1, 247, 1, 4, 4, 4, 4, 4, 4,
                    4, 0, 0, 252, 195, 255, 255, 7, 1, 3, 0, 127, 128, 3, 12, 28, 224, 63, 63, 63,
                    63, 63, 63, 224, 224, 224, 224, 7, 8, 0, 7, 6, 5, 4, 3, 2, 1, 4, 252, 5, 2, 5,
                    98, 95, 98, 105, 116, 9, 110, 95, 98, 111, 111, 108, 101, 97, 110, 9, 110, 95,
                    116, 105, 110, 121, 105, 110, 116, 10, 110, 95, 115, 109, 97, 108, 108, 105,
                    110, 116, 11, 110, 95, 109, 101, 100, 105, 117, 109, 105, 110, 116, 5, 110, 95,
                    105, 110, 116, 8, 110, 95, 98, 105, 103, 105, 110, 116, 9, 110, 95, 100, 101,
                    99, 105, 109, 97, 108, 7, 110, 95, 102, 108, 111, 97, 116, 8, 110, 95, 100,
                    111, 117, 98, 108, 101, 10, 110, 117, 95, 116, 105, 110, 121, 105, 110, 116,
                    11, 110, 117, 95, 115, 109, 97, 108, 108, 105, 110, 116, 12, 110, 117, 95, 109,
                    101, 100, 105, 117, 109, 105, 110, 116, 6, 110, 117, 95, 105, 110, 116, 9, 110,
                    117, 95, 98, 105, 103, 105, 110, 116, 10, 110, 117, 95, 100, 101, 99, 105, 109,
                    97, 108, 8, 110, 117, 95, 102, 108, 111, 97, 116, 9, 110, 117, 95, 100, 111,
                    117, 98, 108, 101, 6, 116, 95, 121, 101, 97, 114, 6, 116, 95, 100, 97, 116,
                    101, 6, 116, 95, 116, 105, 109, 101, 7, 116, 95, 102, 116, 105, 109, 101, 10,
                    116, 95, 100, 97, 116, 101, 116, 105, 109, 101, 11, 116, 95, 102, 100, 97, 116,
                    101, 116, 105, 109, 101, 11, 116, 95, 116, 105, 109, 101, 115, 116, 97, 109,
                    112, 12, 116, 95, 102, 116, 105, 109, 101, 115, 116, 97, 109, 112, 6, 99, 95,
                    99, 104, 97, 114, 9, 99, 95, 118, 97, 114, 99, 104, 97, 114, 8, 99, 95, 98,
                    105, 110, 97, 114, 121, 11, 99, 95, 118, 97, 114, 98, 105, 110, 97, 114, 121,
                    10, 99, 95, 116, 105, 110, 121, 98, 108, 111, 98, 6, 99, 95, 98, 108, 111, 98,
                    12, 99, 95, 109, 101, 100, 105, 117, 109, 98, 108, 111, 98, 10, 99, 95, 108,
                    111, 110, 103, 98, 108, 111, 98, 10, 99, 95, 116, 105, 110, 121, 116, 101, 120,
                    116, 6, 99, 95, 116, 101, 120, 116, 12, 99, 95, 109, 101, 100, 105, 117, 109,
                    116, 101, 120, 116, 10, 99, 95, 108, 111, 110, 103, 116, 101, 120, 116, 6, 101,
                    95, 101, 110, 117, 109, 5, 115, 95, 115, 101, 116, 10, 103, 95, 103, 101, 111,
                    109, 101, 116, 114, 121, 6, 106, 95, 106, 115, 111, 110, 6, 115, 95, 115, 101,
                    116, 50, 7, 101, 95, 101, 110, 117, 109, 50, 20, 103, 95, 103, 101, 111, 109,
                    101, 116, 114, 121, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 14, 103,
                    95, 109, 117, 108, 116, 105, 112, 111, 108, 121, 103, 111, 110, 17, 103, 95,
                    109, 117, 108, 116, 105, 108, 105, 110, 101, 115, 116, 114, 105, 110, 103, 12,
                    103, 95, 109, 117, 108, 116, 105, 112, 111, 105, 110, 116, 9, 103, 95, 112,
                    111, 108, 121, 103, 111, 110, 12, 103, 95, 108, 105, 110, 101, 115, 116, 114,
                    105, 110, 103, 7, 103, 95, 112, 111, 105, 110, 116, 11, 4, 224, 224, 28, 28, 5,
                    10, 2, 1, 49, 1, 50, 2, 1, 51, 1, 52, 6, 10, 2, 1, 97, 1, 98, 2, 1, 99, 1, 100,
                ],
                unsigned_map: unsigned_map.clone(),
                collation_map: mysql_collation_map.clone(),
                enum_set_collation_map: enum_set_collation_map.clone(),
                enum_str_value_map: enum_str_value_map.clone(),
                set_str_value_map: set_str_value_map.clone(),
                geometry_type_map: geometry_type_map.clone(),
            },
            Case {
                flavor: "mariadb".to_string(), // mariadb 10.5
                data: vec![
                    30, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 51, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 252, 254, 254, 255, 255, 255, 255, 255, 255, 255, 49, 0, 8,
                    65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0, 6, 238, 254, 252, 3, 254, 64, 64, 0,
                    1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4, 4, 248, 1, 247, 1, 4, 4, 4, 4, 4, 4,
                    4, 0, 0, 252, 192, 255, 255, 7, 1, 3, 0, 127, 192, 2, 15, 63, 0, 28, 1, 224, 8,
                    224, 9, 224, 10, 224, 11, 224, 13, 46, 7, 8, 0, 7, 6, 5, 4, 3, 2, 1, 4, 252, 5,
                    2, 5, 98, 95, 98, 105, 116, 9, 110, 95, 98, 111, 111, 108, 101, 97, 110, 9,
                    110, 95, 116, 105, 110, 121, 105, 110, 116, 10, 110, 95, 115, 109, 97, 108,
                    108, 105, 110, 116, 11, 110, 95, 109, 101, 100, 105, 117, 109, 105, 110, 116,
                    5, 110, 95, 105, 110, 116, 8, 110, 95, 98, 105, 103, 105, 110, 116, 9, 110, 95,
                    100, 101, 99, 105, 109, 97, 108, 7, 110, 95, 102, 108, 111, 97, 116, 8, 110,
                    95, 100, 111, 117, 98, 108, 101, 10, 110, 117, 95, 116, 105, 110, 121, 105,
                    110, 116, 11, 110, 117, 95, 115, 109, 97, 108, 108, 105, 110, 116, 12, 110,
                    117, 95, 109, 101, 100, 105, 117, 109, 105, 110, 116, 6, 110, 117, 95, 105,
                    110, 116, 9, 110, 117, 95, 98, 105, 103, 105, 110, 116, 10, 110, 117, 95, 100,
                    101, 99, 105, 109, 97, 108, 8, 110, 117, 95, 102, 108, 111, 97, 116, 9, 110,
                    117, 95, 100, 111, 117, 98, 108, 101, 6, 116, 95, 121, 101, 97, 114, 6, 116,
                    95, 100, 97, 116, 101, 6, 116, 95, 116, 105, 109, 101, 7, 116, 95, 102, 116,
                    105, 109, 101, 10, 116, 95, 100, 97, 116, 101, 116, 105, 109, 101, 11, 116, 95,
                    102, 100, 97, 116, 101, 116, 105, 109, 101, 11, 116, 95, 116, 105, 109, 101,
                    115, 116, 97, 109, 112, 12, 116, 95, 102, 116, 105, 109, 101, 115, 116, 97,
                    109, 112, 6, 99, 95, 99, 104, 97, 114, 9, 99, 95, 118, 97, 114, 99, 104, 97,
                    114, 8, 99, 95, 98, 105, 110, 97, 114, 121, 11, 99, 95, 118, 97, 114, 98, 105,
                    110, 97, 114, 121, 10, 99, 95, 116, 105, 110, 121, 98, 108, 111, 98, 6, 99, 95,
                    98, 108, 111, 98, 12, 99, 95, 109, 101, 100, 105, 117, 109, 98, 108, 111, 98,
                    10, 99, 95, 108, 111, 110, 103, 98, 108, 111, 98, 10, 99, 95, 116, 105, 110,
                    121, 116, 101, 120, 116, 6, 99, 95, 116, 101, 120, 116, 12, 99, 95, 109, 101,
                    100, 105, 117, 109, 116, 101, 120, 116, 10, 99, 95, 108, 111, 110, 103, 116,
                    101, 120, 116, 6, 101, 95, 101, 110, 117, 109, 5, 115, 95, 115, 101, 116, 10,
                    103, 95, 103, 101, 111, 109, 101, 116, 114, 121, 6, 106, 95, 106, 115, 111,
                    110, 6, 115, 95, 115, 101, 116, 50, 7, 101, 95, 101, 110, 117, 109, 50, 20,
                    103, 95, 103, 101, 111, 109, 101, 116, 114, 121, 99, 111, 108, 108, 101, 99,
                    116, 105, 111, 110, 14, 103, 95, 109, 117, 108, 116, 105, 112, 111, 108, 121,
                    103, 111, 110, 17, 103, 95, 109, 117, 108, 116, 105, 108, 105, 110, 101, 115,
                    116, 114, 105, 110, 103, 12, 103, 95, 109, 117, 108, 116, 105, 112, 111, 105,
                    110, 116, 9, 103, 95, 112, 111, 108, 121, 103, 111, 110, 12, 103, 95, 108, 105,
                    110, 101, 115, 116, 114, 105, 110, 103, 7, 103, 95, 112, 111, 105, 110, 116,
                    11, 4, 224, 224, 28, 28, 5, 10, 2, 1, 49, 1, 50, 2, 1, 51, 1, 52, 6, 10, 2, 1,
                    97, 1, 98, 2, 1, 99, 1, 100,
                ],
                unsigned_map: unsigned_map.clone(),
                collation_map: mariadb_collation_map.clone(),
                enum_set_collation_map: enum_set_collation_map.clone(),
                enum_str_value_map: enum_str_value_map.clone(),
                set_str_value_map: set_str_value_map.clone(),
                geometry_type_map: geometry_type_map.clone(),
            },
            Case {
                flavor: "mariadb".to_string(), // mysql 5.7
                data: vec![
                    113, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 51, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 245, 254, 254, 255, 255, 255, 255, 255, 255, 255, 49, 0, 8,
                    65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0, 6, 238, 254, 252, 3, 254, 64, 64, 0,
                    1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4, 4, 248, 1, 247, 1, 4, 4, 4, 4, 4, 4,
                    4, 0, 0, 252, 192, 255, 255, 7,
                ],
                unsigned_map: HashMap::default(),
                collation_map: HashMap::default(),
                enum_set_collation_map: HashMap::default(),
                enum_str_value_map: HashMap::default(),
                set_str_value_map: HashMap::default(),
                geometry_type_map: HashMap::default(),
            },
            Case {
                flavor: "mariadb".to_string(), // mariadb 10.4
                data: vec![
                    26, 0, 0, 0, 0, 0, 1, 0, 4, 116, 101, 115, 116, 0, 6, 95, 116, 121, 112, 101,
                    115, 0, 51, 16, 1, 1, 2, 9, 3, 8, 246, 4, 5, 1, 2, 9, 3, 8, 246, 4, 5, 13, 10,
                    19, 19, 18, 18, 17, 17, 254, 15, 254, 15, 252, 252, 252, 252, 252, 252, 252,
                    252, 254, 254, 255, 252, 254, 254, 255, 255, 255, 255, 255, 255, 255, 49, 0, 8,
                    65, 30, 4, 8, 65, 30, 4, 8, 0, 6, 0, 6, 0, 6, 238, 254, 252, 3, 254, 64, 64, 0,
                    1, 2, 3, 4, 1, 2, 3, 4, 247, 1, 248, 1, 4, 4, 248, 1, 247, 1, 4, 4, 4, 4, 4, 4,
                    4, 0, 0, 252, 192, 255, 255, 7,
                ],
                unsigned_map: HashMap::default(),
                collation_map: HashMap::default(),
                enum_set_collation_map: HashMap::default(),
                enum_str_value_map: HashMap::default(),
                set_str_value_map: HashMap::default(),
                geometry_type_map: HashMap::default(),
            },
        ];

        for tc in &testcases {
            let mut table_map_event = TableMapEvent::default();
            table_map_event.flavor = tc.flavor.clone();
            table_map_event.table_id_size = 6;
            table_map_event.decode(&tc.data)?;

            assert_eq!(tc.unsigned_map, table_map_event.unsigned_map());
            assert_eq!(tc.collation_map, table_map_event.collation_map());
            assert_eq!(
                tc.enum_set_collation_map,
                table_map_event.enum_set_collation_map()
            );
            assert_eq!(tc.enum_str_value_map, table_map_event.enum_str_value_map());
            assert_eq!(tc.set_str_value_map, table_map_event.set_str_value_map());
            assert_eq!(tc.geometry_type_map, table_map_event.geometry_type_map());
        }

        Ok(())
    }

    #[test]
    fn test_invalid_event() -> Result<(), ReplicationError> {
        let data = b"@\x01\x00\x00\x00\x00\x01\x00\x02\xff\xfc\x01\x00\x00\x00\x00B\x14U\x16\x8ew"
            .to_vec();
        let mut table = TableMapEvent::default();
        table.table_id_size = 6;
        table.table_id = 0x140;
        table.flags = 0x1;
        table.schema = vec![0x74, 0x65, 0x73, 0x74];
        table.table = vec![0x74];
        table.column_count = 0x2;
        table.column_type = vec![0x3, 0xc];
        table.column_meta = vec![0x0, 0x0];
        table.null_bitmap = vec![0x2];

        let mut e2 = RowsEvent::default();
        e2.version = 1;
        e2.table_id_size = 6;
        e2.tables = HashMap::default();
        e2.tables.insert(0x140, table);
        let have_err = e2.decode(&data).is_err();
        assert_eq!(true, have_err);

        Ok(())
    }

    #[allow(dead_code)]
    #[test]
    fn test_decode_time2() -> Result<(), ReplicationError> {
        struct Case {
            data: Vec<u8>,
            dec: u16,
            get_frac_time: bool,
            expected: String,
        }
        let testcases = vec![
            Case {
                data: b"\xb4\x6e\xfb".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "838:59:59".to_string(),
            },
            Case {
                data: b"\x80\xf1\x05".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "15:04:05".to_string(),
            },
            Case {
                data: b"\x80\x00\x00".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "00:00:00".to_string(),
            },
            Case {
                data: b"\x7f\xff\xff".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "-00:00:01".to_string(),
            },
            Case {
                data: b"\x7f\x0e\xfb".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "-15:04:05".to_string(),
            },
            Case {
                data: b"\x4b\x91\x05".to_vec(),
                dec: 0,
                get_frac_time: false,
                expected: "-838:59:59".to_string(),
            },
            Case {
                data: b"\x7f\xff\xff\xff".to_vec(),
                dec: 2,
                get_frac_time: true,
                expected: "-00:00:00.01".to_string(),
            },
            Case {
                data: b"\x7f\x0e\xfa\xf4".to_vec(),
                dec: 2,
                get_frac_time: true,
                expected: "-15:04:05.12".to_string(),
            },
            Case {
                data: b"\x4b\x91\x05\xf4".to_vec(),
                dec: 2,
                get_frac_time: true,
                expected: "-838:59:58.12".to_string(),
            },
            Case {
                data: b"\x7f\xff\xff\xff\xff".to_vec(),
                dec: 4,
                get_frac_time: true,
                expected: "-00:00:00.0001".to_string(),
            },
            Case {
                data: b"\x7f\x0e\xfa\xfb\x2d".to_vec(),
                dec: 4,
                get_frac_time: true,
                expected: "-15:04:05.1235".to_string(),
            },
            Case {
                data: b"\x4b\x91\x05\xfb\x2d".to_vec(),
                dec: 4,
                get_frac_time: true,
                expected: "-838:59:58.1235".to_string(),
            },
            Case {
                data: b"\x7f\x0e\xfa\xfe\x1d\xc0".to_vec(),
                dec: 6,
                get_frac_time: true,
                expected: "-15:04:05.123456".to_string(),
            },
            Case {
                data: b"\x4b\x91\x05\xfe\x1d\xc0".to_vec(),
                dec: 6,
                get_frac_time: true,
                expected: "-838:59:58.123456".to_string(),
            },
        ];

        for tc in &testcases {
            let (value, _) = decode_helper::decode_time2(&tc.data, tc.dec)?;
            assert_eq!(tc.expected, value);
        }

        Ok(())
    }

    struct DecimalTest {
        pub num: String,
        pub dump_data: Vec<u8>,
        pub meta: u16,
    }

    fn get_decimal_data() -> Vec<DecimalTest> {
        // DECIMAL(40, 16)
        vec![
            DecimalTest {
                num: "123.4560000000000000".to_string(),
                dump_data: vec![
                    128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 27, 46, 2, 0, 0, 0, 0, 0,
                ],
                meta: 10256,
            },
            DecimalTest {
                num: "0.0000010000000000".to_string(),
                dump_data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232, 0, 0, 0, 0],
                meta: 10256,
            },
            DecimalTest {
                num: "100000000.0000000000000000".to_string(),
                dump_data: vec![
                    128, 0, 0, 0, 0, 0, 0, 5, 245, 225, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                meta: 10256,
            },
            DecimalTest {
                num: "100000000.0000000200000000".to_string(),
                dump_data: vec![
                    128, 0, 0, 0, 0, 0, 0, 5, 245, 225, 0, 0, 0, 0, 20, 0, 0, 0, 0,
                ],
                meta: 10256,
            },
            DecimalTest {
                num: "123456.1234567890000000".to_string(),
                dump_data: vec![
                    128, 0, 0, 0, 0, 0, 0, 0, 1, 226, 64, 7, 91, 205, 21, 0, 0, 0, 0,
                ],
                meta: 10256,
            },
            DecimalTest {
                num: "123456234234234757655.1234567890123456".to_string(),
                dump_data: vec![
                    128, 0, 123, 27, 49, 148, 250, 13, 254, 30, 23, 7, 91, 205, 21, 0, 1, 226, 64,
                ],
                meta: 10256,
            },
            DecimalTest {
                num: "-123456234234234757655.1234567890123456".to_string(),
                dump_data: vec![
                    127, 255, 132, 228, 206, 107, 5, 242, 1, 225, 232, 248, 164, 50, 234, 255, 254,
                    29, 191,
                ],
                meta: 10256,
            },
            DecimalTest {
                num: "0.0000000000000000".to_string(),
                dump_data: vec![128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                meta: 10256,
            },
            // DECIMAL(60, 0)
            DecimalTest {
                num: "1000000000000000000000000000000".to_string(),
                dump_data: vec![
                    128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0,
                ],
                meta: 15360,
            },
            DecimalTest {
                num: "1".to_string(),
                dump_data: vec![
                    128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    1,
                ],
                meta: 15360,
            },
            // DECIMAL(30, 30)
            DecimalTest {
                num: "0.100000000000000000000000000000".to_string(),
                dump_data: vec![133, 245, 225, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                meta: 7710,
            },
            DecimalTest {
                num: "0.000000000000001000000000000000".to_string(),
                dump_data: vec![128, 0, 0, 0, 0, 0, 3, 232, 0, 0, 0, 0, 0, 0],
                meta: 7710,
            },
        ]
    }

    #[test]
    fn test_decimal() -> Result<(), ReplicationError> {
        let mut e = RowsEvent::default();
        e.use_decimal = true;

        let mut e2 = RowsEvent::default();
        e2.use_decimal = false;

        let decimal_datas = get_decimal_data();
        for d in &decimal_datas {
            let (v, _) =
                e.decode_value(&d.dump_data, mysql::MYSQL_TYPE_NEWDECIMAL, d.meta, false)?;
            // no trailing zero
            let dec = d.num.parse::<BigDecimal>()?;
            assert_eq!(
                DecodeFieldData::Decimal(DecodeDecimal::Decimal(dec.clone())),
                v
            );

            let (v, _) =
                e.decode_value(&d.dump_data, mysql::MYSQL_TYPE_NEWDECIMAL, d.meta, true)?;
            assert_eq!(
                DecodeFieldData::Decimal(DecodeDecimal::Decimal(dec.clone())),
                v
            );
        }

        Ok(())
    }

    #[bench]
    fn benchmark_use_decimal(b: &mut test::Bencher) {
        let mut e = RowsEvent::default();
        e.use_decimal = true;

        let decimal_datas = get_decimal_data();
        b.iter(|| {
            for d in &decimal_datas {
                let _ = e.decode_value(&d.dump_data, mysql::MYSQL_TYPE_NEWDECIMAL, d.meta, false);
            }
        })
    }

    #[bench]
    fn benchmark_not_use_decimal(b: &mut test::Bencher) {
        let mut e = RowsEvent::default();
        e.use_decimal = false;

        let decimal_datas = get_decimal_data();
        b.iter(|| {
            for d in &decimal_datas {
                let _ = e.decode_value(&d.dump_data, mysql::MYSQL_TYPE_NEWDECIMAL, d.meta, false);
            }
        })
    }

    fn get_int_data() -> Vec<Vec<u8>> {
        vec![
            vec![1, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![3, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![5, 0, 0, 0],
            vec![6, 0, 0, 0],
            vec![7, 0, 0, 0],
            vec![8, 0, 0, 0],
            vec![9, 0, 0, 0],
            vec![10, 0, 0, 0],
            vec![11, 0, 0, 0],
            vec![12, 0, 0, 0],
        ]
    }

    #[bench]
    fn benchmark_int(b: &mut test::Bencher) {
        let mut e = RowsEvent::default();

        let int_datas = get_int_data();
        b.iter(|| {
            for d in &int_datas {
                let _ = e.decode_value(d, mysql::MYSQL_TYPE_LONG, 0, false);
            }
        })
    }
}
