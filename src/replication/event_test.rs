#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use crate::mysql::MariadbGTID;
    use crate::replication::IntVarEventType;
    use crate::replication::{
        Event, GTIDEvent, IntVarEvent, MariadbGTIDEvent, MariadbGTIDListEvent,
    };

    #[test]
    fn test_mariadb_gtid_list_event() -> Result<(), ReplicationError> {
        // single GTID, 1-2-3
        let data = vec![
            1_u8, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut ev = MariadbGTIDListEvent::default();
        ev.decode(&data)?;
        assert_eq!(ev.gtids.len(), 1);
        assert_eq!(ev.gtids[0].domain_id, 1_u32);
        assert_eq!(ev.gtids[0].server_id, 2_u32);
        assert_eq!(ev.gtids[0].sequence_number, 3_u64);

        // multi GTIDs, 1-2-3,4-5-6,7-8-9
        let data = vec![
            3_u8, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0,
            6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut ev = MariadbGTIDListEvent::default();
        ev.decode(&data)?;
        assert_eq!(ev.gtids.len(), 3);
        for i in 0..3 {
            assert_eq!(ev.gtids[i].domain_id, (1 + 3 * i) as u32);
            assert_eq!(ev.gtids[i].server_id, (2 + 3 * i) as u32);
            assert_eq!(ev.gtids[i].sequence_number, (3 + 3 * i) as u64);
        }

        Ok(())
    }

    #[test]
    fn test_mariadb_gtid_event() -> Result<(), ReplicationError> {
        let data = vec![
            1_u8, 2, 3, 4, 5, 6, 7, 8, // SequenceNumber
            0x2a, 1, 0x3b, 4,    // DomainID
            0xff, // Flags
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, // commitID
        ];

        let mut ev = MariadbGTIDEvent {
            gtid: MariadbGTID {
                domain_id: 0,
                server_id: 0,
                sequence_number: 0,
            },
            flags: 0,
            commit_id: 0,
        };
        ev.decode(&data)?;

        assert_eq!(0x0807060504030201_u64, ev.gtid.sequence_number);
        assert_eq!(0x043b012a_u32, ev.gtid.domain_id);
        assert_eq!(0xff_u8, ev.flags);
        assert_eq!(true, ev.is_ddl());
        assert_eq!(true, ev.is_standalone());
        assert_eq!(true, ev.is_group_commit());
        assert_eq!(0x1716151413121110_u64, ev.commit_id);

        Ok(())
    }

    #[test]
    fn test_gtid_event_mysql8_new_fields() -> Result<(), ReplicationError> {
        struct Case {
            pub data: Vec<u8>,
            pub expect_immediate_commit_timestamp: u64,
            pub expect_original_commit_timestamp: u64,
            pub expect_transaction_length: u64,
            pub expect_immediate_server_version: u32,
            pub expect_original_server_version: u32,
        }

        let cases = vec![
            Case {
                data: vec![
                    0, 90, 167, 42, 127, 68, 168, 17, 234, 148, 127, 2, 66, 172, 25, 0, 2, 2, 1, 0,
                    0, 0, 0, 0, 0, 2, 118, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 0, 0, 0, 0, 0, 193, 71,
                    129, 22, 120, 160, 133, 0, 0, 0, 0, 0, 0, 0, 252, 197, 3, 147, 56, 1, 128, 0,
                    0, 0, 0,
                ],
                expect_immediate_commit_timestamp: 1583812517644225,
                expect_original_commit_timestamp: 0,
                expect_transaction_length: 965,
                expect_immediate_server_version: 80019,
                expect_original_server_version: 0,
            },
            Case {
                data: vec![
                    0, 90, 167, 42, 127, 68, 168, 17, 234, 148, 127, 2, 66, 172, 25, 0, 2, 3, 1, 0,
                    0, 0, 0, 0, 0, 2, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0,
                ],
                expect_immediate_commit_timestamp: 0,
                expect_original_commit_timestamp: 0,
                expect_transaction_length: 0,
                expect_immediate_server_version: 0,
                expect_original_server_version: 0,
            },
            Case {
                data: vec![
                    0, 92, 204, 16, 51, 68, 168, 17, 234, 189, 89, 2, 66, 172, 25, 0, 3, 119, 0, 0,
                    0, 0, 0, 0, 0, 2, 120, 0, 0, 0, 0, 0, 0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 106, 48,
                    177, 62, 120, 160, 5, 252, 195, 3, 147, 56, 1, 0,
                ],
                expect_immediate_commit_timestamp: 1583813191872618,
                expect_original_commit_timestamp: 1583813191872618,
                expect_transaction_length: 963,
                expect_immediate_server_version: 80019,
                expect_original_server_version: 80019,
            },
        ];

        for tc in &cases {
            let mut ev = GTIDEvent::default();
            ev.decode(&tc.data)?;
            assert_eq!(
                tc.expect_immediate_commit_timestamp,
                ev.immediate_commit_timestamp
            );
            assert_eq!(
                tc.expect_original_commit_timestamp,
                ev.original_commit_timestamp
            );
            assert_eq!(tc.expect_transaction_length, ev.transaction_length);
            assert_eq!(
                tc.expect_immediate_server_version,
                ev.immediate_server_version
            );
            assert_eq!(
                tc.expect_original_server_version,
                ev.original_server_version
            );
        }
        Ok(())
    }

    #[test]
    fn test_int_var_event() -> Result<(), ReplicationError> {
        // IntVarEvent Type LastInsertID, Value 13
        let data = vec![1, 13, 0, 0, 0, 0, 0, 0, 0];
        let mut ev = IntVarEvent::default();
        ev.decode(&data)?;
        assert_eq!(IntVarEventType::LastInsertId, ev.type_i);
        assert_eq!(13_u64, ev.value);

        // IntVarEvent Type InsertID, Value 23
        let data = vec![2, 23, 0, 0, 0, 0, 0, 0, 0];
        let mut ev = IntVarEvent::default();
        ev.decode(&data)?;
        assert_eq!(IntVarEventType::InsertId, ev.type_i);
        assert_eq!(23_u64, ev.value);

        Ok(())
    }
}
