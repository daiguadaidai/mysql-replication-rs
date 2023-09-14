use crate::mysql;
use std::collections::HashMap;

pub const DEFAULT_MYSQL_STATE: &str = "HY000";

lazy_static! {
    pub static ref MYSQL_STATE: HashMap<u16, String> = {
        let mut map = HashMap::new();
        map.insert(mysql::ER_DUP_KEY, String::from("23000"));
        map.insert(mysql::ER_OUTOFMEMORY, String::from("HY001"));
        map.insert(mysql::ER_OUT_OF_SORTMEMORY, String::from("HY001"));
        map.insert(mysql::ER_CON_COUNT_ERROR, String::from("08004"));
        map.insert(mysql::ER_BAD_HOST_ERROR, String::from("08S01"));
        map.insert(mysql::ER_HANDSHAKE_ERROR, String::from("08S01"));
        map.insert(mysql::ER_DBACCESS_DENIED_ERROR, String::from("42000"));
        map.insert(mysql::ER_ACCESS_DENIED_ERROR, String::from("28000"));
        map.insert(mysql::ER_NO_DB_ERROR, String::from("3D000"));
        map.insert(mysql::ER_UNKNOWN_COM_ERROR, String::from("08S01"));
        map.insert(mysql::ER_BAD_NULL_ERROR, String::from("23000"));
        map.insert(mysql::ER_BAD_DB_ERROR, String::from("42000"));
        map.insert(mysql::ER_TABLE_EXISTS_ERROR, String::from("42S01"));
        map.insert(mysql::ER_BAD_TABLE_ERROR, String::from("42S02"));
        map.insert(mysql::ER_NON_UNIQ_ERROR, String::from("23000"));
        map.insert(mysql::ER_SERVER_SHUTDOWN, String::from("08S01"));
        map.insert(mysql::ER_BAD_FIELD_ERROR, String::from("42S22"));
        map.insert(mysql::ER_WRONG_FIELD_WITH_GROUP, String::from("42000"));
        map.insert(mysql::ER_WRONG_SUM_SELECT, String::from("42000"));
        map.insert(mysql::ER_WRONG_GROUP_FIELD, String::from("42000"));
        map.insert(mysql::ER_WRONG_VALUE_COUNT, String::from("21S01"));
        map.insert(mysql::ER_TOO_LONG_IDENT, String::from("42000"));
        map.insert(mysql::ER_DUP_FIELDNAME, String::from("42S21"));
        map.insert(mysql::ER_DUP_KEYNAME, String::from("42000"));
        map.insert(mysql::ER_DUP_ENTRY, String::from("23000"));
        map.insert(mysql::ER_WRONG_FIELD_SPEC, String::from("42000"));
        map.insert(mysql::ER_PARSE_ERROR, String::from("42000"));
        map.insert(mysql::ER_EMPTY_QUERY, String::from("42000"));
        map.insert(mysql::ER_NONUNIQ_TABLE, String::from("42000"));
        map.insert(mysql::ER_INVALID_DEFAULT, String::from("42000"));
        map.insert(mysql::ER_MULTIPLE_PRI_KEY, String::from("42000"));
        map.insert(mysql::ER_TOO_MANY_KEYS, String::from("42000"));
        map.insert(mysql::ER_TOO_MANY_KEY_PARTS, String::from("42000"));
        map.insert(mysql::ER_TOO_LONG_KEY, String::from("42000"));
        map.insert(mysql::ER_KEY_COLUMN_DOES_NOT_EXITS, String::from("42000"));
        map.insert(mysql::ER_BLOB_USED_AS_KEY, String::from("42000"));
        map.insert(mysql::ER_TOO_BIG_FIELDLENGTH, String::from("42000"));
        map.insert(mysql::ER_WRONG_AUTO_KEY, String::from("42000"));
        map.insert(mysql::ER_FORCING_CLOSE, String::from("08S01"));
        map.insert(mysql::ER_IPSOCK_ERROR, String::from("08S01"));
        map.insert(mysql::ER_NO_SUCH_INDEX, String::from("42S12"));
        map.insert(mysql::ER_WRONG_FIELD_TERMINATORS, String::from("42000"));
        map.insert(mysql::ER_BLOBS_AND_NO_TERMINATED, String::from("42000"));
        map.insert(mysql::ER_CANT_REMOVE_ALL_FIELDS, String::from("42000"));
        map.insert(mysql::ER_CANT_DROP_FIELD_OR_KEY, String::from("42000"));
        map.insert(mysql::ER_BLOB_CANT_HAVE_DEFAULT, String::from("42000"));
        map.insert(mysql::ER_WRONG_DB_NAME, String::from("42000"));
        map.insert(mysql::ER_WRONG_TABLE_NAME, String::from("42000"));
        map.insert(mysql::ER_TOO_BIG_SELECT, String::from("42000"));
        map.insert(mysql::ER_UNKNOWN_PROCEDURE, String::from("42000"));
        map.insert(
            mysql::ER_WRONG_PARAMCOUNT_TO_PROCEDURE,
            String::from("42000"),
        );
        map.insert(mysql::ER_UNKNOWN_TABLE, String::from("42S02"));
        map.insert(mysql::ER_FIELD_SPECIFIED_TWICE, String::from("42000"));
        map.insert(mysql::ER_UNSUPPORTED_EXTENSION, String::from("42000"));
        map.insert(mysql::ER_TABLE_MUST_HAVE_COLUMNS, String::from("42000"));
        map.insert(mysql::ER_UNKNOWN_CHARACTER_SET, String::from("42000"));
        map.insert(mysql::ER_TOO_BIG_ROWSIZE, String::from("42000"));
        map.insert(mysql::ER_WRONG_OUTER_JOIN, String::from("42000"));
        map.insert(mysql::ER_NULL_COLUMN_IN_INDEX, String::from("42000"));
        map.insert(mysql::ER_PASSWORD_ANONYMOUS_USER, String::from("42000"));
        map.insert(mysql::ER_PASSWORD_NOT_ALLOWED, String::from("42000"));
        map.insert(mysql::ER_PASSWORD_NO_MATCH, String::from("42000"));
        map.insert(mysql::ER_WRONG_VALUE_COUNT_ON_ROW, String::from("21S01"));
        map.insert(mysql::ER_INVALID_USE_OF_NULL, String::from("22004"));
        map.insert(mysql::ER_REGEXP_ERROR, String::from("42000"));
        map.insert(
            mysql::ER_MIX_OF_GROUP_FUNC_AND_FIELDS,
            String::from("42000"),
        );
        map.insert(mysql::ER_NONEXISTING_GRANT, String::from("42000"));
        map.insert(mysql::ER_TABLEACCESS_DENIED_ERROR, String::from("42000"));
        map.insert(mysql::ER_COLUMNACCESS_DENIED_ERROR, String::from("42000"));
        map.insert(mysql::ER_ILLEGAL_GRANT_FOR_TABLE, String::from("42000"));
        map.insert(mysql::ER_GRANT_WRONG_HOST_OR_USER, String::from("42000"));
        map.insert(mysql::ER_NO_SUCH_TABLE, String::from("42S02"));
        map.insert(mysql::ER_NONEXISTING_TABLE_GRANT, String::from("42000"));
        map.insert(mysql::ER_NOT_ALLOWED_COMMAND, String::from("42000"));
        map.insert(mysql::ER_SYNTAX_ERROR, String::from("42000"));
        map.insert(mysql::ER_ABORTING_CONNECTION, String::from("08S01"));
        map.insert(mysql::ER_NET_PACKET_TOO_LARGE, String::from("08S01"));
        map.insert(mysql::ER_NET_READ_ERROR_FROM_PIPE, String::from("08S01"));
        map.insert(mysql::ER_NET_FCNTL_ERROR, String::from("08S01"));
        map.insert(mysql::ER_NET_PACKETS_OUT_OF_ORDER, String::from("08S01"));
        map.insert(mysql::ER_NET_UNCOMPRESS_ERROR, String::from("08S01"));
        map.insert(mysql::ER_NET_READ_ERROR, String::from("08S01"));
        map.insert(mysql::ER_NET_READ_INTERRUPTED, String::from("08S01"));
        map.insert(mysql::ER_NET_ERROR_ON_WRITE, String::from("08S01"));
        map.insert(mysql::ER_NET_WRITE_INTERRUPTED, String::from("08S01"));
        map.insert(mysql::ER_TOO_LONG_STRING, String::from("42000"));
        map.insert(mysql::ER_TABLE_CANT_HANDLE_BLOB, String::from("42000"));
        map.insert(
            mysql::ER_TABLE_CANT_HANDLE_AUTO_INCREMENT,
            String::from("42000"),
        );
        map.insert(mysql::ER_WRONG_COLUMN_NAME, String::from("42000"));
        map.insert(mysql::ER_WRONG_KEY_COLUMN, String::from("42000"));
        map.insert(mysql::ER_DUP_UNIQUE, String::from("23000"));
        map.insert(mysql::ER_BLOB_KEY_WITHOUT_LENGTH, String::from("42000"));
        map.insert(mysql::ER_PRIMARY_CANT_HAVE_NULL, String::from("42000"));
        map.insert(mysql::ER_TOO_MANY_ROWS, String::from("42000"));
        map.insert(mysql::ER_REQUIRES_PRIMARY_KEY, String::from("42000"));
        map.insert(mysql::ER_KEY_DOES_NOT_EXITS, String::from("42000"));
        map.insert(mysql::ER_CHECK_NO_SUCH_TABLE, String::from("42000"));
        map.insert(mysql::ER_CHECK_NOT_IMPLEMENTED, String::from("42000"));
        map.insert(
            mysql::ER_CANT_DO_THIS_DURING_AN_TRANSACTION,
            String::from("25000"),
        );
        map.insert(mysql::ER_NEW_ABORTING_CONNECTION, String::from("08S01"));
        map.insert(mysql::ER_MASTER_NET_READ, String::from("08S01"));
        map.insert(mysql::ER_MASTER_NET_WRITE, String::from("08S01"));
        map.insert(mysql::ER_TOO_MANY_USER_CONNECTIONS, String::from("42000"));
        map.insert(mysql::ER_READ_ONLY_TRANSACTION, String::from("25000"));
        map.insert(
            mysql::ER_NO_PERMISSION_TO_CREATE_USER,
            String::from("42000"),
        );
        map.insert(mysql::ER_LOCK_DEADLOCK, String::from("40001"));
        map.insert(mysql::ER_NO_REFERENCED_ROW, String::from("23000"));
        map.insert(mysql::ER_ROW_IS_REFERENCED, String::from("23000"));
        map.insert(mysql::ER_CONNECT_TO_MASTER, String::from("08S01"));
        map.insert(
            mysql::ER_WRONG_NUMBER_OF_COLUMNS_IN_SELECT,
            String::from("21000"),
        );
        map.insert(mysql::ER_USER_LIMIT_REACHED, String::from("42000"));
        map.insert(
            mysql::ER_SPECIFIC_ACCESS_DENIED_ERROR,
            String::from("42000"),
        );
        map.insert(mysql::ER_NO_DEFAULT, String::from("42000"));
        map.insert(mysql::ER_WRONG_VALUE_FOR_VAR, String::from("42000"));
        map.insert(mysql::ER_WRONG_TYPE_FOR_VAR, String::from("42000"));
        map.insert(mysql::ER_CANT_USE_OPTION_HERE, String::from("42000"));
        map.insert(mysql::ER_NOT_SUPPORTED_YET, String::from("42000"));
        map.insert(mysql::ER_WRONG_FK_DEF, String::from("42000"));
        map.insert(mysql::ER_OPERAND_COLUMNS, String::from("21000"));
        map.insert(mysql::ER_SUBQUERY_NO_1_ROW, String::from("21000"));
        map.insert(mysql::ER_ILLEGAL_REFERENCE, String::from("42S22"));
        map.insert(mysql::ER_DERIVED_MUST_HAVE_ALIAS, String::from("42000"));
        map.insert(mysql::ER_SELECT_REDUCED, String::from("01000"));
        map.insert(mysql::ER_TABLENAME_NOT_ALLOWED_HERE, String::from("42000"));
        map.insert(mysql::ER_NOT_SUPPORTED_AUTH_MODE, String::from("08004"));
        map.insert(mysql::ER_SPATIAL_CANT_HAVE_NULL, String::from("42000"));
        map.insert(mysql::ER_COLLATION_CHARSET_MISMATCH, String::from("42000"));
        map.insert(mysql::ER_WARN_TOO_FEW_RECORDS, String::from("01000"));
        map.insert(mysql::ER_WARN_TOO_MANY_RECORDS, String::from("01000"));
        map.insert(mysql::ER_WARN_NULL_TO_NOTNULL, String::from("22004"));
        map.insert(mysql::ER_WARN_DATA_OUT_OF_RANGE, String::from("22003"));
        map.insert(mysql::WARN_DATA_TRUNCATED, String::from("01000"));
        map.insert(mysql::ER_WRONG_NAME_FOR_INDEX, String::from("42000"));
        map.insert(mysql::ER_WRONG_NAME_FOR_CATALOG, String::from("42000"));
        map.insert(mysql::ER_UNKNOWN_STORAGE_ENGINE, String::from("42000"));
        map.insert(mysql::ER_TRUNCATED_WRONG_VALUE, String::from("22007"));
        map.insert(mysql::ER_SP_NO_RECURSIVE_CREATE, String::from("2F003"));
        map.insert(mysql::ER_SP_ALREADY_EXISTS, String::from("42000"));
        map.insert(mysql::ER_SP_DOES_NOT_EXIST, String::from("42000"));
        map.insert(mysql::ER_SP_LILABEL_MISMATCH, String::from("42000"));
        map.insert(mysql::ER_SP_LABEL_REDEFINE, String::from("42000"));
        map.insert(mysql::ER_SP_LABEL_MISMATCH, String::from("42000"));
        map.insert(mysql::ER_SP_UNINIT_VAR, String::from("01000"));
        map.insert(mysql::ER_SP_BADSELECT, String::from("0A000"));
        map.insert(mysql::ER_SP_BADRETURN, String::from("42000"));
        map.insert(mysql::ER_SP_BADSTATEMENT, String::from("0A000"));
        map.insert(
            mysql::ER_UPDATE_LOG_DEPRECATED_IGNORED,
            String::from("42000"),
        );
        map.insert(
            mysql::ER_UPDATE_LOG_DEPRECATED_TRANSLATED,
            String::from("42000"),
        );
        map.insert(mysql::ER_QUERY_INTERRUPTED, String::from("70100"));
        map.insert(mysql::ER_SP_WRONG_NO_OF_ARGS, String::from("42000"));
        map.insert(mysql::ER_SP_COND_MISMATCH, String::from("42000"));
        map.insert(mysql::ER_SP_NORETURN, String::from("42000"));
        map.insert(mysql::ER_SP_NORETURNEND, String::from("2F005"));
        map.insert(mysql::ER_SP_BAD_CURSOR_QUERY, String::from("42000"));
        map.insert(mysql::ER_SP_BAD_CURSOR_SELECT, String::from("42000"));
        map.insert(mysql::ER_SP_CURSOR_MISMATCH, String::from("42000"));
        map.insert(mysql::ER_SP_CURSOR_ALREADY_OPEN, String::from("24000"));
        map.insert(mysql::ER_SP_CURSOR_NOT_OPEN, String::from("24000"));
        map.insert(mysql::ER_SP_UNDECLARED_VAR, String::from("42000"));
        map.insert(mysql::ER_SP_FETCH_NO_DATA, String::from("02000"));
        map.insert(mysql::ER_SP_DUP_PARAM, String::from("42000"));
        map.insert(mysql::ER_SP_DUP_VAR, String::from("42000"));
        map.insert(mysql::ER_SP_DUP_COND, String::from("42000"));
        map.insert(mysql::ER_SP_DUP_CURS, String::from("42000"));
        map.insert(mysql::ER_SP_SUBSELECT_NYI, String::from("0A000"));
        map.insert(
            mysql::ER_STMT_NOT_ALLOWED_IN_SF_OR_TRG,
            String::from("0A000"),
        );
        map.insert(mysql::ER_SP_VARCOND_AFTER_CURSHNDLR, String::from("42000"));
        map.insert(mysql::ER_SP_CURSOR_AFTER_HANDLER, String::from("42000"));
        map.insert(mysql::ER_SP_CASE_NOT_FOUND, String::from("20000"));
        map.insert(mysql::ER_DIVISION_BY_ZERO, String::from("22012"));
        map.insert(mysql::ER_ILLEGAL_VALUE_FOR_TYPE, String::from("22007"));
        map.insert(mysql::ER_PROCACCESS_DENIED_ERROR, String::from("42000"));
        map.insert(mysql::ER_XAER_NOTA, String::from("XAE04"));
        map.insert(mysql::ER_XAER_INVAL, String::from("XAE05"));
        map.insert(mysql::ER_XAER_RMFAIL, String::from("XAE07"));
        map.insert(mysql::ER_XAER_OUTSIDE, String::from("XAE09"));
        map.insert(mysql::ER_XAER_RMERR, String::from("XAE03"));
        map.insert(mysql::ER_XA_RBROLLBACK, String::from("XA100"));
        map.insert(mysql::ER_NONEXISTING_PROC_GRANT, String::from("42000"));
        map.insert(mysql::ER_DATA_TOO_LONG, String::from("22001"));
        map.insert(mysql::ER_SP_BAD_SQLSTATE, String::from("42000"));
        map.insert(mysql::ER_CANT_CREATE_USER_WITH_GRANT, String::from("42000"));
        map.insert(mysql::ER_SP_DUP_HANDLER, String::from("42000"));
        map.insert(mysql::ER_SP_NOT_VAR_ARG, String::from("42000"));
        map.insert(mysql::ER_SP_NO_RETSET, String::from("0A000"));
        map.insert(mysql::ER_CANT_CREATE_GEOMETRY_OBJECT, String::from("22003"));
        map.insert(mysql::ER_TOO_BIG_SCALE, String::from("42000"));
        map.insert(mysql::ER_TOO_BIG_PRECISION, String::from("42000"));
        map.insert(mysql::ER_M_BIGGER_THAN_D, String::from("42000"));
        map.insert(mysql::ER_TOO_LONG_BODY, String::from("42000"));
        map.insert(mysql::ER_TOO_BIG_DISPLAYWIDTH, String::from("42000"));
        map.insert(mysql::ER_XAER_DUPID, String::from("XAE08"));
        map.insert(mysql::ER_DATETIME_FUNCTION_OVERFLOW, String::from("22008"));
        map.insert(mysql::ER_ROW_IS_REFERENCED_2, String::from("23000"));
        map.insert(mysql::ER_NO_REFERENCED_ROW_2, String::from("23000"));
        map.insert(mysql::ER_SP_BAD_VAR_SHADOW, String::from("42000"));
        map.insert(mysql::ER_SP_WRONG_NAME, String::from("42000"));
        map.insert(mysql::ER_SP_NO_AGGREGATE, String::from("42000"));
        map.insert(
            mysql::ER_MAX_PREPARED_STMT_COUNT_REACHED,
            String::from("42000"),
        );
        map.insert(mysql::ER_NON_GROUPING_FIELD_USED, String::from("42000"));
        map.insert(
            mysql::ER_FOREIGN_DUPLICATE_KEY_OLD_UNUSED,
            String::from("23000"),
        );
        map.insert(
            mysql::ER_CANT_CHANGE_TX_CHARACTERISTICS,
            String::from("25001"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMCOUNT_TO_NATIVE_FCT,
            String::from("42000"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMETERS_TO_NATIVE_FCT,
            String::from("42000"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMETERS_TO_STORED_FCT,
            String::from("42000"),
        );
        map.insert(mysql::ER_DUP_ENTRY_WITH_KEY_NAME, String::from("23000"));
        map.insert(mysql::ER_XA_RBTIMEOUT, String::from("XA106"));
        map.insert(mysql::ER_XA_RBDEADLOCK, String::from("XA102"));
        map.insert(
            mysql::ER_FUNC_INEXISTENT_NAME_COLLISION,
            String::from("42000"),
        );
        map.insert(mysql::ER_DUP_SIGNAL_SET, String::from("42000"));
        map.insert(mysql::ER_SIGNAL_WARN, String::from("01000"));
        map.insert(mysql::ER_SIGNAL_NOT_FOUND, String::from("02000"));
        map.insert(mysql::ER_SIGNAL_EXCEPTION, String::from("HY000"));
        map.insert(
            mysql::ER_RESIGNAL_WITHOUT_ACTIVE_HANDLER,
            String::from("0K000"),
        );
        map.insert(mysql::ER_SPATIAL_MUST_HAVE_GEOM_COL, String::from("42000"));
        map.insert(mysql::ER_DATA_OUT_OF_RANGE, String::from("22003"));
        map.insert(
            mysql::ER_ACCESS_DENIED_NO_PASSWORD_ERROR,
            String::from("28000"),
        );
        map.insert(mysql::ER_TRUNCATE_ILLEGAL_FK, String::from("42000"));
        map.insert(mysql::ER_DA_INVALID_CONDITION_NUMBER, String::from("35000"));
        map.insert(
            mysql::ER_FOREIGN_DUPLICATE_KEY_WITH_CHILD_INFO,
            String::from("23000"),
        );
        map.insert(
            mysql::ER_FOREIGN_DUPLICATE_KEY_WITHOUT_CHILD_INFO,
            String::from("23000"),
        );
        map.insert(
            mysql::ER_CANT_EXECUTE_IN_READ_ONLY_TRANSACTION,
            String::from("25006"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED,
            String::from("0A000"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON,
            String::from("0A000"),
        );
        map.insert(mysql::ER_DUP_UNKNOWN_IN_INDEX, String::from("23000"));
        map
    };
}
