use crate::mysql;
use std::collections::HashMap;

lazy_static! {
    pub static ref MYSQL_ERR_NAME: HashMap<u16, String> = {
        let mut map = HashMap::new();
        map.insert(mysql::ER_HASHCHK, String::from("hashchk"));
        map.insert(mysql::ER_NISAMCHK, String::from("isamchk"));
        map.insert(mysql::ER_NO, String::from("NO"));
        map.insert(mysql::ER_YES, String::from("YES"));
        map.insert(
            mysql::ER_CANT_CREATE_FILE,
            String::from("Can't create file '{:<.200}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_CREATE_TABLE,
            String::from("Can't create table '{:<.200}' (errno: {})"),
        );
        map.insert(
            mysql::ER_CANT_CREATE_DB,
            String::from("Can't create database '{:<.192}' (errno: {})"),
        );
        map.insert(
            mysql::ER_DB_CREATE_EXISTS,
            String::from("Can't create database '{:<.192}'; database exists"),
        );
        map.insert(
            mysql::ER_DB_DROP_EXISTS,
            String::from("Can't drop database '{:<.192}'; database doesn't exist"),
        );
        map.insert(
            mysql::ER_DB_DROP_DELETE,
            String::from("Error dropping database (can't delete '{:<.192}', errno: {})"),
        );
        map.insert(
            mysql::ER_DB_DROP_RMDIR,
            String::from("Error dropping database (can't rmdir '{:<.192}', errno: {})"),
        );
        map.insert(
            mysql::ER_CANT_DELETE_FILE,
            String::from("Error on delete of '{:<.192}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_FIND_SYSTEM_REC,
            String::from("Can't read record in system table"),
        );
        map.insert(
            mysql::ER_CANT_GET_STAT,
            String::from("Can't get status of '{:<.200}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_GET_WD,
            String::from("Can't get working directory (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_LOCK,
            String::from("Can't lock file (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_OPEN_FILE,
            String::from("Can't open file: '{:<.200}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_FILE_NOT_FOUND,
            String::from("Can't find file: '{:<.200}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_READ_DIR,
            String::from("Can't read dir of '{:<.192}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CANT_SET_WD,
            String::from("Can't change dir to '{:<.192}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CHECKREAD,
            String::from("Record has changed since last read in table '{:<.192}'"),
        );
        map.insert(
            mysql::ER_DISK_FULL,
            String::from(
                "Disk full ({}); waiting for someone to free some space... (errno: {} - {})",
            ),
        );
        map.insert(
            mysql::ER_DUP_KEY,
            String::from("Can't write; duplicate key in table '{:<.192}'"),
        );
        map.insert(
            mysql::ER_ERROR_ON_CLOSE,
            String::from("Error on close of '{:<.192}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_ERROR_ON_READ,
            String::from("Error reading file '{:<.200}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_ERROR_ON_RENAME,
            String::from("Error on rename of '{:<.210}' to '{:<.210}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_ERROR_ON_WRITE,
            String::from("Error writing file '{:<.200}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_FILE_USED,
            String::from("'{:<.192}' is locked against change"),
        );
        map.insert(mysql::ER_FILSORT_ABORT, String::from("Sort aborted"));
        map.insert(
            mysql::ER_FORM_NOT_FOUND,
            String::from("View '{:<.192}' doesn't exist for '{:<.192}'"),
        );
        map.insert(
            mysql::ER_GET_ERRNO,
            String::from("Got error {} from storage engine"),
        );
        map.insert(
            mysql::ER_ILLEGAL_HA,
            String::from("Table storage engine for '{:<.192}' doesn't have this option"),
        );
        map.insert(
            mysql::ER_KEY_NOT_FOUND,
            String::from("Can't find record in '{:<.192}'"),
        );
        map.insert(
            mysql::ER_NOT_FORM_FILE,
            String::from("Incorrect information in file: '{:<.200}'"),
        );
        map.insert(
            mysql::ER_NOT_KEYFILE,
            String::from("Incorrect key file for table '{:<.200}'; try to repair it"),
        );
        map.insert(
            mysql::ER_OLD_KEYFILE,
            String::from("Old key file for table '{:<.192}'; repair it!"),
        );
        map.insert(
            mysql::ER_OPEN_AS_READONLY,
            String::from("Table '{:<.192}' is read only"),
        );
        map.insert(
            mysql::ER_OUTOFMEMORY,
            String::from("Out of memory; restart server and try again (needed {} bytes)"),
        );
        map.insert(
            mysql::ER_OUT_OF_SORTMEMORY,
            String::from("Out of sort memory, consider increasing server sort buffer size"),
        );
        map.insert(
            mysql::ER_UNEXPECTED_EOF,
            String::from("Unexpected EOF found when reading file '{:<.192}' (errno: {} - {})"),
        );
        map.insert(
            mysql::ER_CON_COUNT_ERROR,
            String::from("Too many connections"),
        );
        map.insert(mysql::ER_OUT_OF_RESOURCES, String::from("Out of memory; check if mysqld or some other process uses all available memory; if not, you may have to use 'ulimit' to allow mysqld to use more memory or you can add more swap space"));
        map.insert(
            mysql::ER_BAD_HOST_ERROR,
            String::from("Can't get hostname for your address"),
        );
        map.insert(mysql::ER_HANDSHAKE_ERROR, String::from("Bad handshake"));
        map.insert(
            mysql::ER_DBACCESS_DENIED_ERROR,
            String::from("Access denied for user '{:<.48}'@'{:<.64}' to database '{:<.192}'"),
        );
        map.insert(
            mysql::ER_ACCESS_DENIED_ERROR,
            String::from("Access denied for user '{:<.48}'@'{:<.64}' (using password: {})"),
        );
        map.insert(mysql::ER_NO_DB_ERROR, String::from("No database selected"));
        map.insert(mysql::ER_UNKNOWN_COM_ERROR, String::from("Unknown command"));
        map.insert(
            mysql::ER_BAD_NULL_ERROR,
            String::from("Column '{:<.192}' cannot be null"),
        );
        map.insert(
            mysql::ER_BAD_DB_ERROR,
            String::from("Unknown database '{:<.192}'"),
        );
        map.insert(
            mysql::ER_TABLE_EXISTS_ERROR,
            String::from("Table '{:<.192}' already exists"),
        );
        map.insert(
            mysql::ER_BAD_TABLE_ERROR,
            String::from("Unknown table '{:<.100}'"),
        );
        map.insert(
            mysql::ER_NON_UNIQ_ERROR,
            String::from("Column '{:<.192}' in {:<.192} is ambiguous"),
        );
        map.insert(
            mysql::ER_SERVER_SHUTDOWN,
            String::from("Server shutdown in progress"),
        );
        map.insert(
            mysql::ER_BAD_FIELD_ERROR,
            String::from("Unknown column '{:<.192}' in '{:<.192}'"),
        );
        map.insert(
            mysql::ER_WRONG_FIELD_WITH_GROUP,
            String::from("'{:<.192}' isn't in GROUP BY"),
        );
        map.insert(
            mysql::ER_WRONG_GROUP_FIELD,
            String::from("Can't group on '{:<.192}'"),
        );
        map.insert(
            mysql::ER_WRONG_SUM_SELECT,
            String::from("Statement has sum functions and columns in same statement"),
        );
        map.insert(
            mysql::ER_WRONG_VALUE_COUNT,
            String::from("Column count doesn't match value count"),
        );
        map.insert(
            mysql::ER_TOO_LONG_IDENT,
            String::from("Identifier name '{:<.100}' is too long"),
        );
        map.insert(
            mysql::ER_DUP_FIELDNAME,
            String::from("Duplicate column name '{:<.192}'"),
        );
        map.insert(
            mysql::ER_DUP_KEYNAME,
            String::from("Duplicate key name '{:<.192}'"),
        );
        map.insert(
            mysql::ER_DUP_ENTRY,
            String::from("Duplicate entry '{:<.192}' for key {}"),
        );
        map.insert(
            mysql::ER_WRONG_FIELD_SPEC,
            String::from("Incorrect column specifier for column '{:<.192}'"),
        );
        map.insert(
            mysql::ER_PARSE_ERROR,
            String::from("{} near '{:<.80}' at line {}"),
        );
        map.insert(mysql::ER_EMPTY_QUERY, String::from("Query was empty"));
        map.insert(
            mysql::ER_NONUNIQ_TABLE,
            String::from("Not unique table/alias: '{:<.192}'"),
        );
        map.insert(
            mysql::ER_INVALID_DEFAULT,
            String::from("Invalid default value for '{:<.192}'"),
        );
        map.insert(
            mysql::ER_MULTIPLE_PRI_KEY,
            String::from("Multiple primary key defined"),
        );
        map.insert(
            mysql::ER_TOO_MANY_KEYS,
            String::from("Too many keys specified; max {} keys allowed"),
        );
        map.insert(
            mysql::ER_TOO_MANY_KEY_PARTS,
            String::from("Too many key parts specified; max {} parts allowed"),
        );
        map.insert(
            mysql::ER_TOO_LONG_KEY,
            String::from("Specified key was too long; max key length is {} bytes"),
        );
        map.insert(
            mysql::ER_KEY_COLUMN_DOES_NOT_EXITS,
            String::from("Key column '{:<.192}' doesn't exist in table"),
        );
        map.insert(mysql::ER_BLOB_USED_AS_KEY, String::from("BLOB column '{:<.192}' can't be used in key specification with the used table type"));
        map.insert(
            mysql::ER_TOO_BIG_FIELDLENGTH,
            String::from(
                "Column length too big for column '{:<.192}' (max = {}); use BLOB or TEXT instead",
            ),
        );
        map.insert(mysql::ER_WRONG_AUTO_KEY, String::from("Incorrect table definition; there can be only one auto column and it must be defined as a key"));
        map.insert(
            mysql::ER_READY,
            String::from("{}: ready for connections.\nVersion: '{}'  socket: '{}'  port: {}"),
        );
        map.insert(
            mysql::ER_NORMAL_SHUTDOWN,
            String::from("{}: Normal shutdown\n"),
        );
        map.insert(
            mysql::ER_GOT_SIGNAL,
            String::from("{}: Got signal {}. Aborting!\n"),
        );
        map.insert(
            mysql::ER_SHUTDOWN_COMPLETE,
            String::from("{}: Shutdown complete\n"),
        );
        map.insert(
            mysql::ER_FORCING_CLOSE,
            String::from("{}: Forcing close of thread {}  user: '{:<.48}'\n"),
        );
        map.insert(
            mysql::ER_IPSOCK_ERROR,
            String::from("Can't create IP socket"),
        );
        map.insert(mysql::ER_NO_SUCH_INDEX, String::from("Table '{:<.192}' has no index like the one used in CREATE INDEX; recreate the table"));
        map.insert(
            mysql::ER_WRONG_FIELD_TERMINATORS,
            String::from("Field separator argument is not what is expected; check the manual"),
        );
        map.insert(
            mysql::ER_BLOBS_AND_NO_TERMINATED,
            String::from(
                "You can't use fixed rowlength with BLOBs; please use 'fields terminated by'",
            ),
        );
        map.insert(
            mysql::ER_TEXTFILE_NOT_READABLE,
            String::from(
                "The file '{:<.128}' must be in the database directory or be readable by all",
            ),
        );
        map.insert(
            mysql::ER_FILE_EXISTS_ERROR,
            String::from("File '{:<.200}' already exists"),
        );
        map.insert(
            mysql::ER_LOAD_INFO,
            String::from("Records: {}  Deleted: {}  Skipped: {}  Warnings: {}"),
        );
        map.insert(
            mysql::ER_ALTER_INFO,
            String::from("Records: {}  Duplicates: {}"),
        );
        map.insert(mysql::ER_WRONG_SUB_KEY, String::from("Incorrect prefix key; the used key part isn't a string, the used length is longer than the key part, or the storage engine doesn't support unique prefix keys"));
        map.insert(
            mysql::ER_CANT_REMOVE_ALL_FIELDS,
            String::from("You can't delete all columns with ALTER TABLE; use DROP TABLE instead"),
        );
        map.insert(
            mysql::ER_CANT_DROP_FIELD_OR_KEY,
            String::from("Can't DROP '{:<.192}'; check that column/key exists"),
        );
        map.insert(
            mysql::ER_INSERT_INFO,
            String::from("Records: {}  Duplicates: {}  Warnings: {}"),
        );
        map.insert(
            mysql::ER_UPDATE_TABLE_USED,
            String::from("You can't specify target table '{:<.192}' for update in FROM clause"),
        );
        map.insert(
            mysql::ER_NO_SUCH_THREAD,
            String::from("Unknown thread id: {}"),
        );
        map.insert(
            mysql::ER_KILL_DENIED_ERROR,
            String::from("You are not owner of thread {}"),
        );
        map.insert(mysql::ER_NO_TABLES_USED, String::from("No tables used"));
        map.insert(
            mysql::ER_TOO_BIG_SET,
            String::from("Too many strings for column {:<.192} and SET"),
        );
        map.insert(
            mysql::ER_NO_UNIQUE_LOGFILE,
            String::from("Can't generate a unique log-filename {:<.200}.(1-999)\n"),
        );
        map.insert(
            mysql::ER_TABLE_NOT_LOCKED_FOR_WRITE,
            String::from("Table '{:<.192}' was locked with a READ lock and can't be updated"),
        );
        map.insert(
            mysql::ER_TABLE_NOT_LOCKED,
            String::from("Table '{:<.192}' was not locked with LOCK TABLES"),
        );
        map.insert(
            mysql::ER_BLOB_CANT_HAVE_DEFAULT,
            String::from("BLOB/TEXT column '{:<.192}' can't have a default value"),
        );
        map.insert(
            mysql::ER_WRONG_DB_NAME,
            String::from("Incorrect database name '{:<.100}'"),
        );
        map.insert(
            mysql::ER_WRONG_TABLE_NAME,
            String::from("Incorrect table name '{:<.100}'"),
        );
        map.insert(mysql::ER_TOO_BIG_SELECT, String::from("The SELECT would examine more than MAX_JOIN_SIZE rows; check your WHERE and use SET SQL_BIG_SELECTS=1 or SET MAX_JOIN_SIZE=# if the SELECT is okay"));
        map.insert(mysql::ER_UNKNOWN_ERROR, String::from("Unknown error"));
        map.insert(
            mysql::ER_UNKNOWN_PROCEDURE,
            String::from("Unknown procedure '{:<.192}'"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMCOUNT_TO_PROCEDURE,
            String::from("Incorrect parameter count to procedure '{:<.192}'"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMETERS_TO_PROCEDURE,
            String::from("Incorrect parameters to procedure '{:<.192}'"),
        );
        map.insert(
            mysql::ER_UNKNOWN_TABLE,
            String::from("Unknown table '{:<.192}' in {:<.32}"),
        );
        map.insert(
            mysql::ER_FIELD_SPECIFIED_TWICE,
            String::from("Column '{:<.192}' specified twice"),
        );
        map.insert(
            mysql::ER_INVALID_GROUP_FUNC_USE,
            String::from("Invalid use of group function"),
        );
        map.insert(
            mysql::ER_UNSUPPORTED_EXTENSION,
            String::from(
                "Table '{:<.192}' uses an extension that doesn't exist in this MySQL version",
            ),
        );
        map.insert(
            mysql::ER_TABLE_MUST_HAVE_COLUMNS,
            String::from("A table must have at least 1 column"),
        );
        map.insert(
            mysql::ER_RECORD_FILE_FULL,
            String::from("The table '{:<.192}' is full"),
        );
        map.insert(
            mysql::ER_UNKNOWN_CHARACTER_SET,
            String::from("Unknown character set: '{:<.64}'"),
        );
        map.insert(
            mysql::ER_TOO_MANY_TABLES,
            String::from("Too many tables; MySQL can only use {} tables in a join"),
        );
        map.insert(mysql::ER_TOO_MANY_FIELDS, String::from("Too many columns"));
        map.insert(mysql::ER_TOO_BIG_ROWSIZE, String::from("Row size too large. The maximum row size for the used table type, not counting BLOBs, is {}. This includes storage overhead, check the manual. You have to change some columns to TEXT or BLOBs"));
        map.insert(mysql::ER_STACK_OVERRUN, String::from("Thread stack overrun:  Used: {} of a {} stack.  Use 'mysqld --thread_stack=#' to specify a bigger stack if needed"));
        map.insert(
            mysql::ER_WRONG_OUTER_JOIN,
            String::from("Cross dependency found in OUTER JOIN; examine your ON conditions"),
        );
        map.insert(mysql::ER_NULL_COLUMN_IN_INDEX, String::from("Table handler doesn't support NULL in given index. Please change column '{:<.192}' to be NOT NULL or use another handler"));
        map.insert(
            mysql::ER_CANT_FIND_UDF,
            String::from("Can't load function '{:<.192}'"),
        );
        map.insert(
            mysql::ER_CANT_INITIALIZE_UDF,
            String::from("Can't initialize function '{:<.192}'; {:<.80}"),
        );
        map.insert(
            mysql::ER_UDF_NO_PATHS,
            String::from("No paths allowed for shared library"),
        );
        map.insert(
            mysql::ER_UDF_EXISTS,
            String::from("Function '{:<.192}' already exists"),
        );
        map.insert(
            mysql::ER_CANT_OPEN_LIBRARY,
            String::from("Can't open shared library '{:<.192}' (errno: {} {:<.128})"),
        );
        map.insert(
            mysql::ER_CANT_FIND_DL_ENTRY,
            String::from("Can't find symbol '{:<.128}' in library"),
        );
        map.insert(
            mysql::ER_FUNCTION_NOT_DEFINED,
            String::from("Function '{:<.192}' is not defined"),
        );
        map.insert(mysql::ER_HOST_IS_BLOCKED, String::from("Host '{:<.64}' is blocked because of many connection errors; unblock with 'mysqladmin flush-hosts'"));
        map.insert(
            mysql::ER_HOST_NOT_PRIVILEGED,
            String::from("Host '{:<.64}' is not allowed to connect to this MySQL server"),
        );
        map.insert(mysql::ER_PASSWORD_ANONYMOUS_USER, String::from("You are using MySQL as an anonymous user and anonymous users are not allowed to change passwords"));
        map.insert(mysql::ER_PASSWORD_NOT_ALLOWED, String::from("You must have privileges to update tables in the mysql database to be able to change passwords for others"));
        map.insert(
            mysql::ER_PASSWORD_NO_MATCH,
            String::from("Can't find any matching row in the user table"),
        );
        map.insert(
            mysql::ER_UPDATE_INFO,
            String::from("Rows matched: {}  Changed: {}  Warnings: {}"),
        );
        map.insert(mysql::ER_CANT_CREATE_THREAD, String::from("Can't create a new thread (errno {}); if you are not out of available memory, you can consult the manual for a possible OS-dependent bug"));
        map.insert(
            mysql::ER_WRONG_VALUE_COUNT_ON_ROW,
            String::from("Column count doesn't match value count at row {}"),
        );
        map.insert(
            mysql::ER_CANT_REOPEN_TABLE,
            String::from("Can't reopen table: '{:<.192}'"),
        );
        map.insert(
            mysql::ER_INVALID_USE_OF_NULL,
            String::from("Invalid use of NULL value"),
        );
        map.insert(
            mysql::ER_REGEXP_ERROR,
            String::from("Got error '{:<.64}' from regexp"),
        );
        map.insert(mysql::ER_MIX_OF_GROUP_FUNC_AND_FIELDS, String::from("Mixing of GROUP columns (MIN(),MAX(),COUNT(),...) with no GROUP columns is illegal if there is no GROUP BY clause"));
        map.insert(
            mysql::ER_NONEXISTING_GRANT,
            String::from("There is no such grant defined for user '{:<.48}' on host '{:<.64}'"),
        );
        map.insert(
            mysql::ER_TABLEACCESS_DENIED_ERROR,
            String::from("{:<.128} command denied to user '{:<.48}'@'{:<.64}' for table '{:<.64}'"),
        );
        map.insert(mysql::ER_COLUMNACCESS_DENIED_ERROR, String::from("{:<.16} command denied to user '{:<.48}'@'{:<.64}' for column '{:<.192}' in table '{:<.192}'"));
        map.insert(mysql::ER_ILLEGAL_GRANT_FOR_TABLE, String::from("Illegal GRANT/REVOKE command; please consult the manual to see which privileges can be used"));
        map.insert(
            mysql::ER_GRANT_WRONG_HOST_OR_USER,
            String::from("The host or user argument to GRANT is too long"),
        );
        map.insert(
            mysql::ER_NO_SUCH_TABLE,
            String::from("Table '{:<.192}.{:<.192}' doesn't exist"),
        );
        map.insert(mysql::ER_NONEXISTING_TABLE_GRANT, String::from("There is no such grant defined for user '{:<.48}' on host '{:<.64}' on table '{:<.192}'"));
        map.insert(
            mysql::ER_NOT_ALLOWED_COMMAND,
            String::from("The used command is not allowed with this MySQL version"),
        );
        map.insert(mysql::ER_SYNTAX_ERROR, String::from("You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use"));
        map.insert(
            mysql::ER_DELAYED_CANT_CHANGE_LOCK,
            String::from("Delayed insert thread couldn't get requested lock for table {:<.192}"),
        );
        map.insert(
            mysql::ER_TOO_MANY_DELAYED_THREADS,
            String::from("Too many delayed threads in use"),
        );
        map.insert(
            mysql::ER_ABORTING_CONNECTION,
            String::from("Aborted connection {} to db: '{:<.192}' user: '{:<.48}' ({:<.64})"),
        );
        map.insert(
            mysql::ER_NET_PACKET_TOO_LARGE,
            String::from("Got a packet bigger than 'max_allowed_packet' bytes"),
        );
        map.insert(
            mysql::ER_NET_READ_ERROR_FROM_PIPE,
            String::from("Got a read error from the connection pipe"),
        );
        map.insert(
            mysql::ER_NET_FCNTL_ERROR,
            String::from("Got an error from fcntl()"),
        );
        map.insert(
            mysql::ER_NET_PACKETS_OUT_OF_ORDER,
            String::from("Got packets out of order"),
        );
        map.insert(
            mysql::ER_NET_UNCOMPRESS_ERROR,
            String::from("Couldn't uncompress communication packet"),
        );
        map.insert(
            mysql::ER_NET_READ_ERROR,
            String::from("Got an error reading communication packets"),
        );
        map.insert(
            mysql::ER_NET_READ_INTERRUPTED,
            String::from("Got timeout reading communication packets"),
        );
        map.insert(
            mysql::ER_NET_ERROR_ON_WRITE,
            String::from("Got an error writing communication packets"),
        );
        map.insert(
            mysql::ER_NET_WRITE_INTERRUPTED,
            String::from("Got timeout writing communication packets"),
        );
        map.insert(
            mysql::ER_TOO_LONG_STRING,
            String::from("Result string is longer than 'max_allowed_packet' bytes"),
        );
        map.insert(
            mysql::ER_TABLE_CANT_HANDLE_BLOB,
            String::from("The used table type doesn't support BLOB/TEXT columns"),
        );
        map.insert(
            mysql::ER_TABLE_CANT_HANDLE_AUTO_INCREMENT,
            String::from("The used table type doesn't support AUTO_INCREMENT columns"),
        );
        map.insert(mysql::ER_DELAYED_INSERT_TABLE_LOCKED, String::from("INSERT DELAYED can't be used with table '{:<.192}' because it is locked with LOCK TABLES"));
        map.insert(
            mysql::ER_WRONG_COLUMN_NAME,
            String::from("Incorrect column name '{:<.100}'"),
        );
        map.insert(
            mysql::ER_WRONG_KEY_COLUMN,
            String::from("The used storage engine can't index column '{:<.192}'"),
        );
        map.insert(mysql::ER_WRONG_MRG_TABLE, String::from("Unable to open underlying table which is differently defined or of non-MyISAM type or doesn't exist"));
        map.insert(
            mysql::ER_DUP_UNIQUE,
            String::from("Can't write, because of unique constraint, to table '{:<.192}'"),
        );
        map.insert(
            mysql::ER_BLOB_KEY_WITHOUT_LENGTH,
            String::from(
                "BLOB/TEXT column '{:<.192}' used in key specification without a key length",
            ),
        );
        map.insert(mysql::ER_PRIMARY_CANT_HAVE_NULL, String::from("All parts of a PRIMARY KEY must be NOT NULL; if you need NULL in a key, use UNIQUE instead"));
        map.insert(
            mysql::ER_TOO_MANY_ROWS,
            String::from("Result consisted of more than one row"),
        );
        map.insert(
            mysql::ER_REQUIRES_PRIMARY_KEY,
            String::from("This table type requires a primary key"),
        );
        map.insert(
            mysql::ER_NO_RAID_COMPILED,
            String::from("This version of MySQL is not compiled with RAID support"),
        );
        map.insert(mysql::ER_UPDATE_WITHOUT_KEY_IN_SAFE_MODE, String::from("You are using safe update mode and you tried to update a table without a WHERE that uses a KEY column"));
        map.insert(
            mysql::ER_KEY_DOES_NOT_EXITS,
            String::from("Key '{:<.192}' doesn't exist in table '{:<.192}'"),
        );
        map.insert(
            mysql::ER_CHECK_NO_SUCH_TABLE,
            String::from("Can't open table"),
        );
        map.insert(
            mysql::ER_CHECK_NOT_IMPLEMENTED,
            String::from("The storage engine for the table doesn't support {}"),
        );
        map.insert(
            mysql::ER_CANT_DO_THIS_DURING_AN_TRANSACTION,
            String::from("You are not allowed to execute this command in a transaction"),
        );
        map.insert(
            mysql::ER_ERROR_DURING_COMMIT,
            String::from("Got error {} during COMMIT"),
        );
        map.insert(
            mysql::ER_ERROR_DURING_ROLLBACK,
            String::from("Got error {} during ROLLBACK"),
        );
        map.insert(
            mysql::ER_ERROR_DURING_FLUSH_LOGS,
            String::from("Got error {} during FLUSH_LOGS"),
        );
        map.insert(
            mysql::ER_ERROR_DURING_CHECKPOINT,
            String::from("Got error {} during CHECKPOINT"),
        );
        map.insert(
            mysql::ER_NEW_ABORTING_CONNECTION,
            String::from(
                "Aborted connection {} to db: '{:<.192}' user: '{:<.48}' host: '{:<.64}' ({:<.64})",
            ),
        );
        map.insert(
            mysql::ER_DUMP_NOT_IMPLEMENTED,
            String::from("The storage engine for the table does not support binary table dump"),
        );
        map.insert(
            mysql::ER_FLUSH_MASTER_BINLOG_CLOSED,
            String::from("Binlog closed, cannot RESET MASTER"),
        );
        map.insert(
            mysql::ER_INDEX_REBUILD,
            String::from("Failed rebuilding the index of  dumped table '{:<.192}'"),
        );
        map.insert(
            mysql::ER_MASTER,
            String::from("Error from master: '{:<.64}'"),
        );
        map.insert(
            mysql::ER_MASTER_NET_READ,
            String::from("Net error reading from master"),
        );
        map.insert(
            mysql::ER_MASTER_NET_WRITE,
            String::from("Net error writing to master"),
        );
        map.insert(
            mysql::ER_FT_MATCHING_KEY_NOT_FOUND,
            String::from("Can't find FULLTEXT index matching the column list"),
        );
        map.insert(mysql::ER_LOCK_OR_ACTIVE_TRANSACTION, String::from("Can't execute the given command because you have active locked tables or an active transaction"));
        map.insert(
            mysql::ER_UNKNOWN_SYSTEM_VARIABLE,
            String::from("Unknown system variable '{:<.64}'"),
        );
        map.insert(
            mysql::ER_CRASHED_ON_USAGE,
            String::from("Table '{:<.192}' is marked as crashed and should be repaired"),
        );
        map.insert(
            mysql::ER_CRASHED_ON_REPAIR,
            String::from(
                "Table '{:<.192}' is marked as crashed and last (automatic?) repair failed",
            ),
        );
        map.insert(
            mysql::ER_WARNING_NOT_COMPLETE_ROLLBACK,
            String::from("Some non-transactional changed tables couldn't be rolled back"),
        );
        map.insert(mysql::ER_TRANS_CACHE_FULL, String::from("Multi-statement transaction required more than 'max_binlog_cache_size' bytes of storage; increase this mysqld variable and try again"));
        map.insert(
            mysql::ER_SLAVE_MUST_STOP,
            String::from(
                "This operation cannot be performed with a running slave; run STOP SLAVE first",
            ),
        );
        map.insert(
            mysql::ER_SLAVE_NOT_RUNNING,
            String::from(
                "This operation requires a running slave; configure slave and do START SLAVE",
            ),
        );
        map.insert(mysql::ER_BAD_SLAVE, String::from("The server is not configured as slave; fix in config file or with CHANGE MASTER TO"));
        map.insert(mysql::ER_MASTER_INFO, String::from("Could not initialize master info structure; more error messages can be found in the MySQL error log"));
        map.insert(
            mysql::ER_SLAVE_THREAD,
            String::from("Could not create slave thread; check system resources"),
        );
        map.insert(
            mysql::ER_TOO_MANY_USER_CONNECTIONS,
            String::from(
                "User {:<.64} already has more than 'max_user_connections' active connections",
            ),
        );
        map.insert(
            mysql::ER_SET_CONSTANTS_ONLY,
            String::from("You may only use constant expressions with SET"),
        );
        map.insert(
            mysql::ER_LOCK_WAIT_TIMEOUT,
            String::from("Lock wait timeout exceeded; try restarting transaction"),
        );
        map.insert(
            mysql::ER_LOCK_TABLE_FULL,
            String::from("The total number of locks exceeds the lock table size"),
        );
        map.insert(
            mysql::ER_READ_ONLY_TRANSACTION,
            String::from("Update locks cannot be acquired during a READ UNCOMMITTED transaction"),
        );
        map.insert(
            mysql::ER_DROP_DB_WITH_READ_LOCK,
            String::from("DROP DATABASE not allowed while thread is holding global read lock"),
        );
        map.insert(
            mysql::ER_CREATE_DB_WITH_READ_LOCK,
            String::from("CREATE DATABASE not allowed while thread is holding global read lock"),
        );
        map.insert(
            mysql::ER_WRONG_ARGUMENTS,
            String::from("Incorrect arguments to {}"),
        );
        map.insert(
            mysql::ER_NO_PERMISSION_TO_CREATE_USER,
            String::from("'{:<.48}'@'{:<.64}' is not allowed to create new users"),
        );
        map.insert(
            mysql::ER_UNION_TABLES_IN_DIFFERENT_DIR,
            String::from(
                "Incorrect table definition; all MERGE tables must be in the same database",
            ),
        );
        map.insert(
            mysql::ER_LOCK_DEADLOCK,
            String::from("Deadlock found when trying to get lock; try restarting transaction"),
        );
        map.insert(
            mysql::ER_TABLE_CANT_HANDLE_FT,
            String::from("The used table type doesn't support FULLTEXT indexes"),
        );
        map.insert(
            mysql::ER_CANNOT_ADD_FOREIGN,
            String::from("Cannot add foreign key constraint"),
        );
        map.insert(
            mysql::ER_NO_REFERENCED_ROW,
            String::from("Cannot add or update a child row: a foreign key constraint fails"),
        );
        map.insert(
            mysql::ER_ROW_IS_REFERENCED,
            String::from("Cannot delete or update a parent row: a foreign key constraint fails"),
        );
        map.insert(
            mysql::ER_CONNECT_TO_MASTER,
            String::from("Error connecting to master: {:<.128}"),
        );
        map.insert(
            mysql::ER_QUERY_ON_MASTER,
            String::from("Error running query on master: {:<.128}"),
        );
        map.insert(
            mysql::ER_ERROR_WHEN_EXECUTING_COMMAND,
            String::from("Error when executing command {}: {:<.128}"),
        );
        map.insert(
            mysql::ER_WRONG_USAGE,
            String::from("Incorrect usage of {} and {}"),
        );
        map.insert(
            mysql::ER_WRONG_NUMBER_OF_COLUMNS_IN_SELECT,
            String::from("The used SELECT statements have a different number of columns"),
        );
        map.insert(
            mysql::ER_CANT_UPDATE_WITH_READLOCK,
            String::from("Can't execute the query because you have a conflicting read lock"),
        );
        map.insert(
            mysql::ER_MIXING_NOT_ALLOWED,
            String::from("Mixing of transactional and non-transactional tables is disabled"),
        );
        map.insert(
            mysql::ER_DUP_ARGUMENT,
            String::from("Option '{}' used twice in statement"),
        );
        map.insert(
            mysql::ER_USER_LIMIT_REACHED,
            String::from("User '{:<.64}' has exceeded the '{}' resource (current value: {})"),
        );
        map.insert(mysql::ER_SPECIFIC_ACCESS_DENIED_ERROR, String::from("Access denied; you need (at least one of) the {:<.128} privilege(s) for this operation"));
        map.insert(
            mysql::ER_LOCAL_VARIABLE,
            String::from(
                "Variable '{:<.64}' is a SESSION variable and can't be used with SET GLOBAL",
            ),
        );
        map.insert(
            mysql::ER_GLOBAL_VARIABLE,
            String::from(
                "Variable '{:<.64}' is a GLOBAL variable and should be set with SET GLOBAL",
            ),
        );
        map.insert(
            mysql::ER_NO_DEFAULT,
            String::from("Variable '{:<.64}' doesn't have a default value"),
        );
        map.insert(
            mysql::ER_WRONG_VALUE_FOR_VAR,
            String::from("Variable '{:<.64}' can't be set to the value of '{:<.200}'"),
        );
        map.insert(
            mysql::ER_WRONG_TYPE_FOR_VAR,
            String::from("Incorrect argument type to variable '{:<.64}'"),
        );
        map.insert(
            mysql::ER_VAR_CANT_BE_READ,
            String::from("Variable '{:<.64}' can only be set, not read"),
        );
        map.insert(
            mysql::ER_CANT_USE_OPTION_HERE,
            String::from("Incorrect usage/placement of '{}'"),
        );
        map.insert(
            mysql::ER_NOT_SUPPORTED_YET,
            String::from("This version of MySQL doesn't yet support '{}'"),
        );
        map.insert(
            mysql::ER_MASTER_FATAL_ERROR_READING_BINLOG,
            String::from(
                "Got fatal error {} from master when reading data from binary log: '{:<.320}'",
            ),
        );
        map.insert(
            mysql::ER_SLAVE_IGNORED_TABLE,
            String::from("Slave SQL thread ignored the query because of replicate-*-table rules"),
        );
        map.insert(
            mysql::ER_INCORRECT_GLOBAL_LOCAL_VAR,
            String::from("Variable '{:<.192}' is a {} variable"),
        );
        map.insert(
            mysql::ER_WRONG_FK_DEF,
            String::from("Incorrect foreign key definition for '{:<.192}': {}"),
        );
        map.insert(
            mysql::ER_KEY_REF_DO_NOT_MATCH_TABLE_REF,
            String::from("Key reference and table reference don't match"),
        );
        map.insert(
            mysql::ER_OPERAND_COLUMNS,
            String::from("Operand should contain {} column(s)"),
        );
        map.insert(
            mysql::ER_SUBQUERY_NO_1_ROW,
            String::from("Subquery returns more than 1 row"),
        );
        map.insert(
            mysql::ER_UNKNOWN_STMT_HANDLER,
            String::from("Unknown prepared statement handler ({:.*}) given to {}"),
        );
        map.insert(
            mysql::ER_CORRUPT_HELP_DB,
            String::from("Help database is corrupt or does not exist"),
        );
        map.insert(
            mysql::ER_CYCLIC_REFERENCE,
            String::from("Cyclic reference on subqueries"),
        );
        map.insert(
            mysql::ER_AUTO_CONVERT,
            String::from("Converting column '{}' from {} to {}"),
        );
        map.insert(
            mysql::ER_ILLEGAL_REFERENCE,
            String::from("Reference '{:<.64}' not supported ({})"),
        );
        map.insert(
            mysql::ER_DERIVED_MUST_HAVE_ALIAS,
            String::from("Every derived table must have its own alias"),
        );
        map.insert(
            mysql::ER_SELECT_REDUCED,
            String::from("Select {} was reduced during optimization"),
        );
        map.insert(
            mysql::ER_TABLENAME_NOT_ALLOWED_HERE,
            String::from("Table '{:<.192}' from one of the SELECTs cannot be used in {:<.32}"),
        );
        map.insert(mysql::ER_NOT_SUPPORTED_AUTH_MODE, String::from("Client does not support authentication protocol requested by server; consider upgrading MySQL client"));
        map.insert(
            mysql::ER_SPATIAL_CANT_HAVE_NULL,
            String::from("All parts of a SPATIAL index must be NOT NULL"),
        );
        map.insert(
            mysql::ER_COLLATION_CHARSET_MISMATCH,
            String::from("COLLATION '{}' is not valid for CHARACTER SET '{}'"),
        );
        map.insert(
            mysql::ER_SLAVE_WAS_RUNNING,
            String::from("Slave is already running"),
        );
        map.insert(
            mysql::ER_SLAVE_WAS_NOT_RUNNING,
            String::from("Slave already has been stopped"),
        );
        map.insert(mysql::ER_TOO_BIG_FOR_UNCOMPRESS, String::from("Uncompressed data size too large; the maximum size is {} (probably, length of uncompressed data was corrupted)"));
        map.insert(
            mysql::ER_ZLIB_Z_MEM_ERROR,
            String::from("ZLIB: Not enough memory"),
        );
        map.insert(mysql::ER_ZLIB_Z_BUF_ERROR, String::from("ZLIB: Not enough room in the output buffer (probably, length of uncompressed data was corrupted)"));
        map.insert(
            mysql::ER_ZLIB_Z_DATA_ERROR,
            String::from("ZLIB: Input data corrupted"),
        );
        map.insert(
            mysql::ER_CUT_VALUE_GROUP_CONCAT,
            String::from("Row {} was cut by GROUP_CONCAT()"),
        );
        map.insert(
            mysql::ER_WARN_TOO_FEW_RECORDS,
            String::from("Row {} doesn't contain data for all columns"),
        );
        map.insert(
            mysql::ER_WARN_TOO_MANY_RECORDS,
            String::from(
                "Row {} was truncated; it contained more data than there were input columns",
            ),
        );
        map.insert(
            mysql::ER_WARN_NULL_TO_NOTNULL,
            String::from(
                "Column set to default value; NULL supplied to NOT NULL column '{}' at row {}",
            ),
        );
        map.insert(
            mysql::ER_WARN_DATA_OUT_OF_RANGE,
            String::from("Out of range value for column '{}' at row {}"),
        );
        map.insert(
            mysql::WARN_DATA_TRUNCATED,
            String::from("Data truncated for column '{}' at row {}"),
        );
        map.insert(
            mysql::ER_WARN_USING_OTHER_HANDLER,
            String::from("Using storage engine {} for table '{}'"),
        );
        map.insert(
            mysql::ER_CANT_AGGREGATE_2COLLATIONS,
            String::from("Illegal mix of collations ({},{}) and ({},{}) for operation '{}'"),
        );
        map.insert(
            mysql::ER_DROP_USER,
            String::from("Cannot drop one or more of the requested users"),
        );
        map.insert(
            mysql::ER_REVOKE_GRANTS,
            String::from("Can't revoke all privileges for one or more of the requested users"),
        );
        map.insert(
            mysql::ER_CANT_AGGREGATE_3COLLATIONS,
            String::from("Illegal mix of collations ({},{}), ({},{}), ({},{}) for operation '{}'"),
        );
        map.insert(
            mysql::ER_CANT_AGGREGATE_NCOLLATIONS,
            String::from("Illegal mix of collations for operation '{}'"),
        );
        map.insert(mysql::ER_VARIABLE_IS_NOT_STRUCT, String::from("Variable '{:<.64}' is not a variable component (can't be used as XXXX.variable_name)"));
        map.insert(
            mysql::ER_UNKNOWN_COLLATION,
            String::from("Unknown collation: '{:<.64}'"),
        );
        map.insert(mysql::ER_SLAVE_IGNORED_SSL_PARAMS, String::from("SSL parameters in CHANGE MASTER are ignored because this MySQL slave was compiled without SSL support; they can be used later if MySQL slave with SSL is started"));
        map.insert(mysql::ER_SERVER_IS_IN_SECURE_AUTH_MODE, String::from("Server is running in --secure-auth mode, but '{}'@'{}' has a password in the old format; please change the password to the new format"));
        map.insert(mysql::ER_WARN_FIELD_RESOLVED, String::from("Field or reference '{:<.192}{}{:<.192}{}{:<.192}' of SELECT #{} was resolved in SELECT #{}"));
        map.insert(
            mysql::ER_BAD_SLAVE_UNTIL_COND,
            String::from("Incorrect parameter or combination of parameters for START SLAVE UNTIL"),
        );
        map.insert(mysql::ER_MISSING_SKIP_SLAVE, String::from("It is recommended to use --skip-slave-start when doing step-by-step replication with START SLAVE UNTIL; otherwise, you will get problems if you get an unexpected slave's mysqld restart"));
        map.insert(
            mysql::ER_UNTIL_COND_IGNORED,
            String::from("SQL thread is not to be started so UNTIL options are ignored"),
        );
        map.insert(
            mysql::ER_WRONG_NAME_FOR_INDEX,
            String::from("Incorrect index name '{:<.100}'"),
        );
        map.insert(
            mysql::ER_WRONG_NAME_FOR_CATALOG,
            String::from("Incorrect catalog name '{:<.100}'"),
        );
        map.insert(
            mysql::ER_WARN_QC_RESIZE,
            String::from("Query cache failed to set size {}; new query cache size is {}"),
        );
        map.insert(
            mysql::ER_BAD_FT_COLUMN,
            String::from("Column '{:<.192}' cannot be part of FULLTEXT index"),
        );
        map.insert(
            mysql::ER_UNKNOWN_KEY_CACHE,
            String::from("Unknown key cache '{:<.100}'"),
        );
        map.insert(mysql::ER_WARN_HOSTNAME_WONT_WORK, String::from("MySQL is started in --skip-name-resolve mode; you must restart it without this switch for this grant to work"));
        map.insert(
            mysql::ER_UNKNOWN_STORAGE_ENGINE,
            String::from("Unknown storage engine '{}'"),
        );
        map.insert(
            mysql::ER_WARN_DEPRECATED_SYNTAX,
            String::from(
                "'{}' is deprecated and will be removed in a future release. Please use {} instead",
            ),
        );
        map.insert(
            mysql::ER_NON_UPDATABLE_TABLE,
            String::from("The target table {:<.100} of the {} is not updatable"),
        );
        map.insert(
            mysql::ER_FEATURE_DISABLED,
            String::from(
                "The '{}' feature is disabled; you need MySQL built with '{}' to have it working",
            ),
        );
        map.insert(mysql::ER_OPTION_PREVENTS_STATEMENT, String::from("The MySQL server is running with the {} option so it cannot execute this statement"));
        map.insert(
            mysql::ER_DUPLICATED_VALUE_IN_TYPE,
            String::from("Column '{:<.100}' has duplicated value '{:<.64}' in {}"),
        );
        map.insert(
            mysql::ER_TRUNCATED_WRONG_VALUE,
            String::from("Truncated incorrect {:<.32} value: '{:<.128}'"),
        );
        map.insert(mysql::ER_TOO_MUCH_AUTO_TIMESTAMP_COLS, String::from("Incorrect table definition; there can be only one TIMESTAMP column with CURRENT_TIMESTAMP in DEFAULT or ON UPDATE clause"));
        map.insert(
            mysql::ER_INVALID_ON_UPDATE,
            String::from("Invalid ON UPDATE clause for '{:<.192}' column"),
        );
        map.insert(
            mysql::ER_UNSUPPORTED_PS,
            String::from("This command is not supported in the prepared statement protocol yet"),
        );
        map.insert(
            mysql::ER_GET_ERRMSG,
            String::from("Got error {} '{:<.100}' from {}"),
        );
        map.insert(
            mysql::ER_GET_TEMPORARY_ERRMSG,
            String::from("Got temporary error {} '{:<.100}' from {}"),
        );
        map.insert(
            mysql::ER_UNKNOWN_TIME_ZONE,
            String::from("Unknown or incorrect time zone: '{:<.64}'"),
        );
        map.insert(
            mysql::ER_WARN_INVALID_TIMESTAMP,
            String::from("Invalid TIMESTAMP value in column '{}' at row {}"),
        );
        map.insert(
            mysql::ER_INVALID_CHARACTER_STRING,
            String::from("Invalid {} character string: '{:.64}'"),
        );
        map.insert(
            mysql::ER_WARN_ALLOWED_PACKET_OVERFLOWED,
            String::from("Result of {}() was larger than max_allowed_packet ({}) - truncated"),
        );
        map.insert(
            mysql::ER_CONFLICTING_DECLARATIONS,
            String::from("Conflicting declarations: '{}{}' and '{}{}'"),
        );
        map.insert(
            mysql::ER_SP_NO_RECURSIVE_CREATE,
            String::from("Can't create a {} from within another stored routine"),
        );
        map.insert(
            mysql::ER_SP_ALREADY_EXISTS,
            String::from("{} {} already exists"),
        );
        map.insert(
            mysql::ER_SP_DOES_NOT_EXIST,
            String::from("{} {} does not exist"),
        );
        map.insert(
            mysql::ER_SP_DROP_FAILED,
            String::from("Failed to DROP {} {}"),
        );
        map.insert(
            mysql::ER_SP_STORE_FAILED,
            String::from("Failed to CREATE {} {}"),
        );
        map.insert(
            mysql::ER_SP_LILABEL_MISMATCH,
            String::from("{} with no matching label: {}"),
        );
        map.insert(
            mysql::ER_SP_LABEL_REDEFINE,
            String::from("Redefining label {}"),
        );
        map.insert(
            mysql::ER_SP_LABEL_MISMATCH,
            String::from("End-label {} without match"),
        );
        map.insert(
            mysql::ER_SP_UNINIT_VAR,
            String::from("Referring to uninitialized variable {}"),
        );
        map.insert(
            mysql::ER_SP_BADSELECT,
            String::from("PROCEDURE {} can't return a result set in the given context"),
        );
        map.insert(
            mysql::ER_SP_BADRETURN,
            String::from("RETURN is only allowed in a FUNCTION"),
        );
        map.insert(
            mysql::ER_SP_BADSTATEMENT,
            String::from("{} is not allowed in stored procedures"),
        );
        map.insert(mysql::ER_UPDATE_LOG_DEPRECATED_IGNORED, String::from("The update log is deprecated and replaced by the binary log; SET SQL_LOG_UPDATE has been ignored."));
        map.insert(mysql::ER_UPDATE_LOG_DEPRECATED_TRANSLATED, String::from("The update log is deprecated and replaced by the binary log; SET SQL_LOG_UPDATE has been translated to SET SQL_LOG_BIN."));
        map.insert(
            mysql::ER_QUERY_INTERRUPTED,
            String::from("Query execution was interrupted"),
        );
        map.insert(
            mysql::ER_SP_WRONG_NO_OF_ARGS,
            String::from("Incorrect number of arguments for {} {}; expected {}, got {}"),
        );
        map.insert(
            mysql::ER_SP_COND_MISMATCH,
            String::from("Undefined CONDITION: {}"),
        );
        map.insert(
            mysql::ER_SP_NORETURN,
            String::from("No RETURN found in FUNCTION {}"),
        );
        map.insert(
            mysql::ER_SP_NORETURNEND,
            String::from("FUNCTION {} ended without RETURN"),
        );
        map.insert(
            mysql::ER_SP_BAD_CURSOR_QUERY,
            String::from("Cursor statement must be a SELECT"),
        );
        map.insert(
            mysql::ER_SP_BAD_CURSOR_SELECT,
            String::from("Cursor SELECT must not have INTO"),
        );
        map.insert(
            mysql::ER_SP_CURSOR_MISMATCH,
            String::from("Undefined CURSOR: {}"),
        );
        map.insert(
            mysql::ER_SP_CURSOR_ALREADY_OPEN,
            String::from("Cursor is already open"),
        );
        map.insert(
            mysql::ER_SP_CURSOR_NOT_OPEN,
            String::from("Cursor is not open"),
        );
        map.insert(
            mysql::ER_SP_UNDECLARED_VAR,
            String::from("Undeclared variable: {}"),
        );
        map.insert(
            mysql::ER_SP_WRONG_NO_OF_FETCH_ARGS,
            String::from("Incorrect number of FETCH variables"),
        );
        map.insert(
            mysql::ER_SP_FETCH_NO_DATA,
            String::from("No data - zero rows fetched, selected, or processed"),
        );
        map.insert(
            mysql::ER_SP_DUP_PARAM,
            String::from("Duplicate parameter: {}"),
        );
        map.insert(mysql::ER_SP_DUP_VAR, String::from("Duplicate variable: {}"));
        map.insert(
            mysql::ER_SP_DUP_COND,
            String::from("Duplicate condition: {}"),
        );
        map.insert(mysql::ER_SP_DUP_CURS, String::from("Duplicate cursor: {}"));
        map.insert(
            mysql::ER_SP_CANT_ALTER,
            String::from("Failed to ALTER {} {}"),
        );
        map.insert(
            mysql::ER_SP_SUBSELECT_NYI,
            String::from("Subquery value not supported"),
        );
        map.insert(
            mysql::ER_STMT_NOT_ALLOWED_IN_SF_OR_TRG,
            String::from("{} is not allowed in stored function or trigger"),
        );
        map.insert(
            mysql::ER_SP_VARCOND_AFTER_CURSHNDLR,
            String::from("Variable or condition declaration after cursor or handler declaration"),
        );
        map.insert(
            mysql::ER_SP_CURSOR_AFTER_HANDLER,
            String::from("Cursor declaration after handler declaration"),
        );
        map.insert(
            mysql::ER_SP_CASE_NOT_FOUND,
            String::from("Case not found for CASE statement"),
        );
        map.insert(
            mysql::ER_FPARSER_TOO_BIG_FILE,
            String::from("Configuration file '{:<.192}' is too big"),
        );
        map.insert(
            mysql::ER_FPARSER_BAD_HEADER,
            String::from("Malformed file type header in file '{:<.192}'"),
        );
        map.insert(
            mysql::ER_FPARSER_EOF_IN_COMMENT,
            String::from("Unexpected end of file while parsing comment '{:<.200}'"),
        );
        map.insert(
            mysql::ER_FPARSER_ERROR_IN_PARAMETER,
            String::from("Error while parsing parameter '{:<.192}' (line: '{:<.192}')"),
        );
        map.insert(
            mysql::ER_FPARSER_EOF_IN_UNKNOWN_PARAMETER,
            String::from("Unexpected end of file while skipping unknown parameter '{:<.192}'"),
        );
        map.insert(
            mysql::ER_VIEW_NO_EXPLAIN,
            String::from("EXPLAIN/SHOW can not be issued; lacking privileges for underlying table"),
        );
        map.insert(
            mysql::ER_FRM_UNKNOWN_TYPE,
            String::from("File '{:<.192}' has unknown type '{:<.64}' in its header"),
        );
        map.insert(
            mysql::ER_WRONG_OBJECT,
            String::from("'{:<.192}.{:<.192}' is not {}"),
        );
        map.insert(
            mysql::ER_NONUPDATEABLE_COLUMN,
            String::from("Column '{:<.192}' is not updatable"),
        );
        map.insert(
            mysql::ER_VIEW_SELECT_DERIVED,
            String::from("View's SELECT contains a subquery in the FROM clause"),
        );
        map.insert(
            mysql::ER_VIEW_SELECT_CLAUSE,
            String::from("View's SELECT contains a '{}' clause"),
        );
        map.insert(
            mysql::ER_VIEW_SELECT_VARIABLE,
            String::from("View's SELECT contains a variable or parameter"),
        );
        map.insert(
            mysql::ER_VIEW_SELECT_TMPTABLE,
            String::from("View's SELECT refers to a temporary table '{:<.192}'"),
        );
        map.insert(
            mysql::ER_VIEW_WRONG_LIST,
            String::from("View's SELECT and view's field list have different column counts"),
        );
        map.insert(
            mysql::ER_WARN_VIEW_MERGE,
            String::from(
                "View merge algorithm can't be used here for now (assumed undefined algorithm)",
            ),
        );
        map.insert(
            mysql::ER_WARN_VIEW_WITHOUT_KEY,
            String::from("View being updated does not have complete key of underlying table in it"),
        );
        map.insert(mysql::ER_VIEW_INVALID, String::from("View '{:<.192}.{:<.192}' references invalid table(s) or column(s) or function(s) or definer/invoker of view lack rights to use them"));
        map.insert(
            mysql::ER_SP_NO_DROP_SP,
            String::from("Can't drop or alter a {} from within another stored routine"),
        );
        map.insert(
            mysql::ER_SP_GOTO_IN_HNDLR,
            String::from("GOTO is not allowed in a stored procedure handler"),
        );
        map.insert(
            mysql::ER_TRG_ALREADY_EXISTS,
            String::from("Trigger already exists"),
        );
        map.insert(
            mysql::ER_TRG_DOES_NOT_EXIST,
            String::from("Trigger does not exist"),
        );
        map.insert(
            mysql::ER_TRG_ON_VIEW_OR_TEMP_TABLE,
            String::from("Trigger's '{:<.192}' is view or temporary table"),
        );
        map.insert(
            mysql::ER_TRG_CANT_CHANGE_ROW,
            String::from("Updating of {} row is not allowed in {}trigger"),
        );
        map.insert(
            mysql::ER_TRG_NO_SUCH_ROW_IN_TRG,
            String::from("There is no {} row in {} trigger"),
        );
        map.insert(
            mysql::ER_NO_DEFAULT_FOR_FIELD,
            String::from("Field '{:<.192}' doesn't have a default value"),
        );
        map.insert(mysql::ER_DIVISION_BY_ZERO, String::from("Division by 0"));
        map.insert(
            mysql::ER_TRUNCATED_WRONG_VALUE_FOR_FIELD,
            String::from("Incorrect {:<.32} value: '{:<.128}' for column '{:.192}' at row {}"),
        );
        map.insert(
            mysql::ER_ILLEGAL_VALUE_FOR_TYPE,
            String::from("Illegal {} '{:<.192}' value found during parsing"),
        );
        map.insert(
            mysql::ER_VIEW_NONUPD_CHECK,
            String::from("CHECK OPTION on non-updatable view '{:<.192}.{:<.192}'"),
        );
        map.insert(
            mysql::ER_VIEW_CHECK_FAILED,
            String::from("CHECK OPTION failed '{:<.192}.{:<.192}'"),
        );
        map.insert(
            mysql::ER_PROCACCESS_DENIED_ERROR,
            String::from(
                "{:<.16} command denied to user '{:<.48}'@'{:<.64}' for routine '{:<.192}'",
            ),
        );
        map.insert(
            mysql::ER_RELAY_LOG_FAIL,
            String::from("Failed purging old relay logs: {}"),
        );
        map.insert(
            mysql::ER_PASSWD_LENGTH,
            String::from("Password hash should be a {}-digit hexadecimal number"),
        );
        map.insert(
            mysql::ER_UNKNOWN_TARGET_BINLOG,
            String::from("Target log not found in binlog index"),
        );
        map.insert(
            mysql::ER_IO_ERR_LOG_INDEX_READ,
            String::from("I/O error reading log index file"),
        );
        map.insert(
            mysql::ER_BINLOG_PURGE_PROHIBITED,
            String::from("Server configuration does not permit binlog purge"),
        );
        map.insert(mysql::ER_FSEEK_FAIL, String::from("Failed on fseek()"));
        map.insert(
            mysql::ER_BINLOG_PURGE_FATAL_ERR,
            String::from("Fatal error during log purge"),
        );
        map.insert(
            mysql::ER_LOG_IN_USE,
            String::from("A purgeable log is in use, will not purge"),
        );
        map.insert(
            mysql::ER_LOG_PURGE_UNKNOWN_ERR,
            String::from("Unknown error during log purge"),
        );
        map.insert(
            mysql::ER_RELAY_LOG_INIT,
            String::from("Failed initializing relay log position: {}"),
        );
        map.insert(
            mysql::ER_NO_BINARY_LOGGING,
            String::from("You are not using binary logging"),
        );
        map.insert(
            mysql::ER_RESERVED_SYNTAX,
            String::from(
                "The '{:<.64}' syntax is reserved for purposes internal to the MySQL server",
            ),
        );
        map.insert(mysql::ER_WSAS_FAILED, String::from("WSAStartup Failed"));
        map.insert(
            mysql::ER_DIFF_GROUPS_PROC,
            String::from("Can't handle procedures with different groups yet"),
        );
        map.insert(
            mysql::ER_NO_GROUP_FOR_PROC,
            String::from("Select must have a group with this procedure"),
        );
        map.insert(
            mysql::ER_ORDER_WITH_PROC,
            String::from("Can't use ORDER clause with this procedure"),
        );
        map.insert(
            mysql::ER_LOGGING_PROHIBIT_CHANGING_OF,
            String::from("Binary logging and replication forbid changing the global server {}"),
        );
        map.insert(
            mysql::ER_NO_FILE_MAPPING,
            String::from("Can't map file: {:<.200}, errno: {}"),
        );
        map.insert(
            mysql::ER_WRONG_MAGIC,
            String::from("Wrong magic in {:<.64}"),
        );
        map.insert(
            mysql::ER_PS_MANY_PARAM,
            String::from("Prepared statement contains too many placeholders"),
        );
        map.insert(
            mysql::ER_KEY_PART_0,
            String::from("Key part '{:<.192}' length cannot be 0"),
        );
        map.insert(
            mysql::ER_VIEW_CHECKSUM,
            String::from("View text checksum failed"),
        );
        map.insert(
            mysql::ER_VIEW_MULTIUPDATE,
            String::from(
                "Can not modify more than one base table through a join view '{:<.192}.{:<.192}'",
            ),
        );
        map.insert(
            mysql::ER_VIEW_NO_INSERT_FIELD_LIST,
            String::from("Can not insert into join view '{:<.192}.{:<.192}' without fields list"),
        );
        map.insert(
            mysql::ER_VIEW_DELETE_MERGE_VIEW,
            String::from("Can not delete from join view '{:<.192}.{:<.192}'"),
        );
        map.insert(
            mysql::ER_CANNOT_USER,
            String::from("Operation {} failed for {:.256}"),
        );
        map.insert(mysql::ER_XAER_NOTA, String::from("XAER_NOTA: Unknown XID"));
        map.insert(
            mysql::ER_XAER_INVAL,
            String::from("XAER_INVAL: Invalid arguments (or unsupported command)"),
        );
        map.insert(mysql::ER_XAER_RMFAIL, String::from("XAER_RMFAIL: The command cannot be executed when global transaction is in the  {:.64} state"));
        map.insert(
            mysql::ER_XAER_OUTSIDE,
            String::from("XAER_OUTSIDE: Some work is done outside global transaction"),
        );
        map.insert(mysql::ER_XAER_RMERR, String::from("XAER_RMERR: Fatal error occurred in the transaction branch - check your data for consistency"));
        map.insert(
            mysql::ER_XA_RBROLLBACK,
            String::from("XA_RBROLLBACK: Transaction branch was rolled back"),
        );
        map.insert(mysql::ER_NONEXISTING_PROC_GRANT, String::from("There is no such grant defined for user '{:<.48}' on host '{:<.64}' on routine '{:<.192}'"));
        map.insert(
            mysql::ER_PROC_AUTO_GRANT_FAIL,
            String::from("Failed to grant EXECUTE and ALTER ROUTINE privileges"),
        );
        map.insert(
            mysql::ER_PROC_AUTO_REVOKE_FAIL,
            String::from("Failed to revoke all privileges to dropped routine"),
        );
        map.insert(
            mysql::ER_DATA_TOO_LONG,
            String::from("Data too long for column '{}' at row {}"),
        );
        map.insert(
            mysql::ER_SP_BAD_SQLSTATE,
            String::from("Bad SQLSTATE: '{}'"),
        );
        map.insert(
            mysql::ER_STARTUP,
            String::from("{}: ready for connections.\nVersion: '{}'  socket: '{}'  port: {}  {}"),
        );
        map.insert(
            mysql::ER_LOAD_FROM_FIXED_SIZE_ROWS_TO_VAR,
            String::from("Can't load value from file with fixed size rows to variable"),
        );
        map.insert(
            mysql::ER_CANT_CREATE_USER_WITH_GRANT,
            String::from("You are not allowed to create a user with GRANT"),
        );
        map.insert(
            mysql::ER_WRONG_VALUE_FOR_TYPE,
            String::from("Incorrect {:<.32} value: '{:<.128}' for function {:<.32}"),
        );
        map.insert(
            mysql::ER_TABLE_DEF_CHANGED,
            String::from("Table definition has changed, please retry transaction"),
        );
        map.insert(
            mysql::ER_SP_DUP_HANDLER,
            String::from("Duplicate handler declared in the same block"),
        );
        map.insert(mysql::ER_SP_NOT_VAR_ARG, String::from("OUT or INOUT argument {} for routine {} is not a variable or NEW pseudo-variable in BEFORE trigger"));
        map.insert(
            mysql::ER_SP_NO_RETSET,
            String::from("Not allowed to return a result set from a {}"),
        );
        map.insert(
            mysql::ER_CANT_CREATE_GEOMETRY_OBJECT,
            String::from("Cannot get geometry object from data you send to the GEOMETRY field"),
        );
        map.insert(mysql::ER_FAILED_ROUTINE_BREAK_BINLOG, String::from("A routine failed and has neither NO SQL nor READS SQL DATA in its declaration and binary logging is enabled; if non-transactional tables were updated, the binary log will miss their changes"));
        map.insert(mysql::ER_BINLOG_UNSAFE_ROUTINE, String::from("This function has none of DETERMINISTIC, NO SQL, or READS SQL DATA in its declaration and binary logging is enabled (you *might* want to use the less safe log_bin_trust_function_creators variable)"));
        map.insert(mysql::ER_BINLOG_CREATE_ROUTINE_NEED_SUPER, String::from("You do not have the SUPER privilege and binary logging is enabled (you *might* want to use the less safe log_bin_trust_function_creators variable)"));
        map.insert(mysql::ER_EXEC_STMT_WITH_OPEN_CURSOR, String::from("You can't execute a prepared statement which has an open cursor associated with it. Reset the statement to re-execute it."));
        map.insert(
            mysql::ER_STMT_HAS_NO_OPEN_CURSOR,
            String::from("The statement ({}) has no open cursor."),
        );
        map.insert(
            mysql::ER_COMMIT_NOT_ALLOWED_IN_SF_OR_TRG,
            String::from(
                "Explicit or implicit commit is not allowed in stored function or trigger.",
            ),
        );
        map.insert(
            mysql::ER_NO_DEFAULT_FOR_VIEW_FIELD,
            String::from(
                "Field of view '{:<.192}.{:<.192}' underlying table doesn't have a default value",
            ),
        );
        map.insert(
            mysql::ER_SP_NO_RECURSION,
            String::from("Recursive stored functions and triggers are not allowed."),
        );
        map.insert(
            mysql::ER_TOO_BIG_SCALE,
            String::from("Too big scale {} specified for column '{:<.192}'. Maximum is {}."),
        );
        map.insert(
            mysql::ER_TOO_BIG_PRECISION,
            String::from("Too big precision {} specified for column '{:<.192}'. Maximum is {}."),
        );
        map.insert(
            mysql::ER_M_BIGGER_THAN_D,
            String::from(
                "For float(M,D), double(M,D) or decimal(M,D), M must be >= D (column '{:<.192}').",
            ),
        );
        map.insert(
            mysql::ER_WRONG_LOCK_OF_SYSTEM_TABLE,
            String::from(
                "You can't combine write-locking of system tables with other tables or lock types",
            ),
        );
        map.insert(
            mysql::ER_CONNECT_TO_FOREIGN_DATA_SOURCE,
            String::from("Unable to connect to foreign data source: {:.64}"),
        );
        map.insert(mysql::ER_QUERY_ON_FOREIGN_DATA_SOURCE, String::from("There was a problem processing the query on the foreign data source. Data source error: {:<.64}"));
        map.insert(mysql::ER_FOREIGN_DATA_SOURCE_DOESNT_EXIST, String::from("The foreign data source you are trying to reference does not exist. Data source error:  {:<.64}"));
        map.insert(mysql::ER_FOREIGN_DATA_STRING_INVALID_CANT_CREATE, String::from("Can't create federated table. The data source connection string '{:<.64}' is not in the correct format"));
        map.insert(
            mysql::ER_FOREIGN_DATA_STRING_INVALID,
            String::from(
                "The data source connection string '{:<.64}' is not in the correct format",
            ),
        );
        map.insert(
            mysql::ER_CANT_CREATE_FEDERATED_TABLE,
            String::from("Can't create federated table. Foreign data src error:  {:<.64}"),
        );
        map.insert(
            mysql::ER_TRG_IN_WRONG_SCHEMA,
            String::from("Trigger in wrong schema"),
        );
        map.insert(mysql::ER_STACK_OVERRUN_NEED_MORE, String::from("Thread stack overrun:  {} bytes used of a {} byte stack, and {} bytes needed.  Use 'mysqld --thread_stack=#' to specify a bigger stack."));
        map.insert(
            mysql::ER_TOO_LONG_BODY,
            String::from("Routine body for '{:<.100}' is too long"),
        );
        map.insert(
            mysql::ER_WARN_CANT_DROP_DEFAULT_KEYCACHE,
            String::from("Cannot drop default keycache"),
        );
        map.insert(
            mysql::ER_TOO_BIG_DISPLAYWIDTH,
            String::from("Display width out of range for column '{:<.192}' (max = {})"),
        );
        map.insert(
            mysql::ER_XAER_DUPID,
            String::from("XAER_DUPID: The XID already exists"),
        );
        map.insert(
            mysql::ER_DATETIME_FUNCTION_OVERFLOW,
            String::from("Datetime function: {:<.32} field overflow"),
        );
        map.insert(mysql::ER_CANT_UPDATE_USED_TABLE_IN_SF_OR_TRG, String::from("Can't update table '{:<.192}' in stored function/trigger because it is already used by statement which invoked this stored function/trigger."));
        map.insert(mysql::ER_VIEW_PREVENT_UPDATE, String::from("The definition of table '{:<.192}' prevents operation {:.192} on table '{:<.192}'."));
        map.insert(mysql::ER_PS_NO_RECURSION, String::from("The prepared statement contains a stored routine call that refers to that same statement. It's not allowed to execute a prepared statement in such a recursive manner"));
        map.insert(
            mysql::ER_SP_CANT_SET_AUTOCOMMIT,
            String::from("Not allowed to set autocommit from a stored function or trigger"),
        );
        map.insert(
            mysql::ER_MALFORMED_DEFINER,
            String::from("Definer is not fully qualified"),
        );
        map.insert(mysql::ER_VIEW_FRM_NO_USER, String::from("View '{:<.192}'.'{:<.192}' has no definer information (old table format). Current user is used as definer. Please recreate the view!"));
        map.insert(
            mysql::ER_VIEW_OTHER_USER,
            String::from(
                "You need the SUPER privilege for creation view with '{:<.192}'@'{:<.192}' definer",
            ),
        );
        map.insert(
            mysql::ER_NO_SUCH_USER,
            String::from("The user specified as a definer ('{:<.64}'@'{:<.64}') does not exist"),
        );
        map.insert(
            mysql::ER_FORBID_SCHEMA_CHANGE,
            String::from("Changing schema from '{:<.192}' to '{:<.192}' is not allowed."),
        );
        map.insert(
            mysql::ER_ROW_IS_REFERENCED_2,
            String::from(
                "Cannot delete or update a parent row: a foreign key constraint fails ({:.192})",
            ),
        );
        map.insert(
            mysql::ER_NO_REFERENCED_ROW_2,
            String::from(
                "Cannot add or update a child row: a foreign key constraint fails ({:.192})",
            ),
        );
        map.insert(
            mysql::ER_SP_BAD_VAR_SHADOW,
            String::from("Variable '{:<.64}' must be quoted with `...`, or renamed"),
        );
        map.insert(mysql::ER_TRG_NO_DEFINER, String::from("No definer attribute for trigger '{:<.192}'.'{:<.192}'. The trigger will be activated under the authorization of the invoker, which may have insufficient privileges. Please recreate the trigger."));
        map.insert(
            mysql::ER_OLD_FILE_FORMAT,
            String::from("'{:<.192}' has an old format, you should re-create the '{}' object(s)"),
        );
        map.insert(mysql::ER_SP_RECURSION_LIMIT, String::from("Recursive limit {} (as set by the max_sp_recursion_depth variable) was exceeded for routine {:.192}"));
        map.insert(mysql::ER_SP_PROC_TABLE_CORRUPT, String::from("Failed to load routine {:<.192}. The table mysql.proc is missing, corrupt, or contains bad data (internal code {})"));
        map.insert(
            mysql::ER_SP_WRONG_NAME,
            String::from("Incorrect routine name '{:<.192}'"),
        );
        map.insert(mysql::ER_TABLE_NEEDS_UPGRADE, String::from("Table upgrade required. Please do \"REPAIR TABLE `{:<.32}`\" or dump/reload to fix it!"));
        map.insert(
            mysql::ER_SP_NO_AGGREGATE,
            String::from("AGGREGATE is not supported for stored functions"),
        );
        map.insert(
            mysql::ER_MAX_PREPARED_STMT_COUNT_REACHED,
            String::from(
                "Can't create more than max_prepared_stmt_count statements (current value: {})",
            ),
        );
        map.insert(
            mysql::ER_VIEW_RECURSIVE,
            String::from("`{:<.192}`.`{:<.192}` contains view recursion"),
        );
        map.insert(
            mysql::ER_NON_GROUPING_FIELD_USED,
            String::from("Non-grouping field '{:<.192}' is used in {:<.64} clause"),
        );
        map.insert(
            mysql::ER_TABLE_CANT_HANDLE_SPKEYS,
            String::from("The used table type doesn't support SPATIAL indexes"),
        );
        map.insert(
            mysql::ER_NO_TRIGGERS_ON_SYSTEM_SCHEMA,
            String::from("Triggers can not be created on system tables"),
        );
        map.insert(
            mysql::ER_REMOVED_SPACES,
            String::from("Leading spaces are removed from name '{}'"),
        );
        map.insert(
            mysql::ER_AUTOINC_READ_FAILED,
            String::from("Failed to read auto-increment value from storage engine"),
        );
        map.insert(mysql::ER_USERNAME, String::from("user name"));
        map.insert(mysql::ER_HOSTNAME, String::from("host name"));
        map.insert(
            mysql::ER_WRONG_STRING_LENGTH,
            String::from("String '{:<.70}' is too long for {} (should be no longer than {})"),
        );
        map.insert(
            mysql::ER_NON_INSERTABLE_TABLE,
            String::from("The target table {:<.100} of the {} is not insertable-into"),
        );
        map.insert(
            mysql::ER_ADMIN_WRONG_MRG_TABLE,
            String::from(
                "Table '{:<.64}' is differently defined or of non-MyISAM type or doesn't exist",
            ),
        );
        map.insert(
            mysql::ER_TOO_HIGH_LEVEL_OF_NESTING_FOR_SELECT,
            String::from("Too high level of nesting for select"),
        );
        map.insert(
            mysql::ER_NAME_BECOMES_EMPTY,
            String::from("Name '{:<.64}' has become ''"),
        );
        map.insert(mysql::ER_AMBIGUOUS_FIELD_TERM, String::from("First character of the FIELDS TERMINATED string is ambiguous; please use non-optional and non-empty FIELDS ENCLOSED BY"));
        map.insert(
            mysql::ER_FOREIGN_SERVER_EXISTS,
            String::from("The foreign server, {}, you are trying to create already exists."),
        );
        map.insert(mysql::ER_FOREIGN_SERVER_DOESNT_EXIST, String::from("The foreign server name you are trying to reference does not exist. Data source error:  {:<.64}"));
        map.insert(
            mysql::ER_ILLEGAL_HA_CREATE_OPTION,
            String::from(
                "Table storage engine '{:<.64}' does not support the create option '{:.64}'",
            ),
        );
        map.insert(mysql::ER_PARTITION_REQUIRES_VALUES_ERROR, String::from("Syntax error: {:<.64} PARTITIONING requires definition of VALUES {:<.64} for each partition"));
        map.insert(
            mysql::ER_PARTITION_WRONG_VALUES_ERROR,
            String::from(
                "Only {:<.64} PARTITIONING can use VALUES {:<.64} in partition definition",
            ),
        );
        map.insert(
            mysql::ER_PARTITION_MAXVALUE_ERROR,
            String::from("MAXVALUE can only be used in last partition definition"),
        );
        map.insert(
            mysql::ER_PARTITION_SUBPARTITION_ERROR,
            String::from("Subpartitions can only be hash partitions and by key"),
        );
        map.insert(
            mysql::ER_PARTITION_SUBPART_MIX_ERROR,
            String::from("Must define subpartitions on all partitions if on one partition"),
        );
        map.insert(
            mysql::ER_PARTITION_WRONG_NO_PART_ERROR,
            String::from("Wrong number of partitions defined, mismatch with previous setting"),
        );
        map.insert(
            mysql::ER_PARTITION_WRONG_NO_SUBPART_ERROR,
            String::from("Wrong number of subpartitions defined, mismatch with previous setting"),
        );
        map.insert(mysql::ER_WRONG_EXPR_IN_PARTITION_FUNC_ERROR, String::from("Constant, random or timezone-dependent expressions in (sub)partitioning function are not allowed"));
        map.insert(
            mysql::ER_NO_CONST_EXPR_IN_RANGE_OR_LIST_ERROR,
            String::from("Expression in RANGE/LIST VALUES must be constant"),
        );
        map.insert(
            mysql::ER_FIELD_NOT_FOUND_PART_ERROR,
            String::from("Field in list of fields for partition function not found in table"),
        );
        map.insert(
            mysql::ER_LIST_OF_FIELDS_ONLY_IN_HASH_ERROR,
            String::from("List of fields is only allowed in KEY partitions"),
        );
        map.insert(mysql::ER_INCONSISTENT_PARTITION_INFO_ERROR, String::from("The partition info in the frm file is not consistent with what can be written into the frm file"));
        map.insert(
            mysql::ER_PARTITION_FUNC_NOT_ALLOWED_ERROR,
            String::from("The {:<.192} function returns the wrong type"),
        );
        map.insert(
            mysql::ER_PARTITIONS_MUST_BE_DEFINED_ERROR,
            String::from("For {:<.64} partitions each partition must be defined"),
        );
        map.insert(
            mysql::ER_RANGE_NOT_INCREASING_ERROR,
            String::from("VALUES LESS THAN value must be strictly increasing for each partition"),
        );
        map.insert(
            mysql::ER_INCONSISTENT_TYPE_OF_FUNCTIONS_ERROR,
            String::from("VALUES value must be of same type as partition function"),
        );
        map.insert(
            mysql::ER_MULTIPLE_DEF_CONST_IN_LIST_PART_ERROR,
            String::from("Multiple definition of same constant in list partitioning"),
        );
        map.insert(
            mysql::ER_PARTITION_ENTRY_ERROR,
            String::from("Partitioning can not be used stand-alone in query"),
        );
        map.insert(
            mysql::ER_MIX_HANDLER_ERROR,
            String::from(
                "The mix of handlers in the partitions is not allowed in this version of MySQL",
            ),
        );
        map.insert(
            mysql::ER_PARTITION_NOT_DEFINED_ERROR,
            String::from("For the partitioned engine it is necessary to define all {:<.64}"),
        );
        map.insert(
            mysql::ER_TOO_MANY_PARTITIONS_ERROR,
            String::from("Too many partitions (including subpartitions) were defined"),
        );
        map.insert(mysql::ER_SUBPARTITION_ERROR, String::from("It is only possible to mix RANGE/LIST partitioning with HASH/KEY partitioning for subpartitioning"));
        map.insert(
            mysql::ER_CANT_CREATE_HANDLER_FILE,
            String::from("Failed to create specific handler file"),
        );
        map.insert(
            mysql::ER_BLOB_FIELD_IN_PART_FUNC_ERROR,
            String::from("A BLOB field is not allowed in partition function"),
        );
        map.insert(
            mysql::ER_UNIQUE_KEY_NEED_ALL_FIELDS_IN_PF,
            String::from(
                "A {:<.192} must include all columns in the table's partitioning function",
            ),
        );
        map.insert(
            mysql::ER_NO_PARTS_ERROR,
            String::from("Number of {:<.64} = 0 is not an allowed value"),
        );
        map.insert(
            mysql::ER_PARTITION_MGMT_ON_NONPARTITIONED,
            String::from("Partition management on a not partitioned table is not possible"),
        );
        map.insert(
            mysql::ER_FOREIGN_KEY_ON_PARTITIONED,
            String::from(
                "Foreign key clause is not yet supported in conjunction with partitioning",
            ),
        );
        map.insert(
            mysql::ER_DROP_PARTITION_NON_EXISTENT,
            String::from("Error in list of partitions to {:<.64}"),
        );
        map.insert(
            mysql::ER_DROP_LAST_PARTITION,
            String::from("Cannot remove all partitions, use DROP TABLE instead"),
        );
        map.insert(
            mysql::ER_COALESCE_ONLY_ON_HASH_PARTITION,
            String::from("COALESCE PARTITION can only be used on HASH/KEY partitions"),
        );
        map.insert(mysql::ER_REORG_HASH_ONLY_ON_SAME_NO, String::from("REORGANIZE PARTITION can only be used to reorganize partitions not to change their numbers"));
        map.insert(mysql::ER_REORG_NO_PARAM_ERROR, String::from("REORGANIZE PARTITION without parameters can only be used on auto-partitioned tables using HASH PARTITIONs"));
        map.insert(
            mysql::ER_ONLY_ON_RANGE_LIST_PARTITION,
            String::from("{:<.64} PARTITION can only be used on RANGE/LIST partitions"),
        );
        map.insert(
            mysql::ER_ADD_PARTITION_SUBPART_ERROR,
            String::from("Trying to Add partition(s) with wrong number of subpartitions"),
        );
        map.insert(
            mysql::ER_ADD_PARTITION_NO_NEW_PARTITION,
            String::from("At least one partition must be added"),
        );
        map.insert(
            mysql::ER_COALESCE_PARTITION_NO_PARTITION,
            String::from("At least one partition must be coalesced"),
        );
        map.insert(
            mysql::ER_REORG_PARTITION_NOT_EXIST,
            String::from("More partitions to reorganize than there are partitions"),
        );
        map.insert(
            mysql::ER_SAME_NAME_PARTITION,
            String::from("Duplicate partition name {:<.192}"),
        );
        map.insert(
            mysql::ER_NO_BINLOG_ERROR,
            String::from("It is not allowed to shut off binlog on this command"),
        );
        map.insert(
            mysql::ER_CONSECUTIVE_REORG_PARTITIONS,
            String::from("When reorganizing a set of partitions they must be in consecutive order"),
        );
        map.insert(mysql::ER_REORG_OUTSIDE_RANGE, String::from("Reorganize of range partitions cannot change total ranges except for last partition where it can extend the range"));
        map.insert(
            mysql::ER_PARTITION_FUNCTION_FAILURE,
            String::from("Partition function not supported in this version for this handler"),
        );
        map.insert(
            mysql::ER_PART_STATE_ERROR,
            String::from("Partition state cannot be defined from CREATE/ALTER TABLE"),
        );
        map.insert(
            mysql::ER_LIMITED_PART_RANGE,
            String::from("The {:<.64} handler only supports 32 bit integers in VALUES"),
        );
        map.insert(
            mysql::ER_PLUGIN_IS_NOT_LOADED,
            String::from("Plugin '{:<.192}' is not loaded"),
        );
        map.insert(
            mysql::ER_WRONG_VALUE,
            String::from("Incorrect {:<.32} value: '{:<.128}'"),
        );
        map.insert(
            mysql::ER_NO_PARTITION_FOR_GIVEN_VALUE,
            String::from("Table has no partition for value {:<.64}"),
        );
        map.insert(
            mysql::ER_FILEGROUP_OPTION_ONLY_ONCE,
            String::from("It is not allowed to specify {} more than once"),
        );
        map.insert(
            mysql::ER_CREATE_FILEGROUP_FAILED,
            String::from("Failed to create {}"),
        );
        map.insert(
            mysql::ER_DROP_FILEGROUP_FAILED,
            String::from("Failed to drop {}"),
        );
        map.insert(
            mysql::ER_TABLESPACE_AUTO_EXTEND_ERROR,
            String::from("The handler doesn't support autoextend of tablespaces"),
        );
        map.insert(
            mysql::ER_WRONG_SIZE_NUMBER,
            String::from(
                "A size parameter was incorrectly specified, either number or on the form 10M",
            ),
        );
        map.insert(mysql::ER_SIZE_OVERFLOW_ERROR, String::from("The size number was correct but we don't allow the digit part to be more than 2 billion"));
        map.insert(
            mysql::ER_ALTER_FILEGROUP_FAILED,
            String::from("Failed to alter: {}"),
        );
        map.insert(
            mysql::ER_BINLOG_ROW_LOGGING_FAILED,
            String::from("Writing one row to the row-based binary log failed"),
        );
        map.insert(
            mysql::ER_BINLOG_ROW_WRONG_TABLE_DEF,
            String::from("Table definition on master and slave does not match: {}"),
        );
        map.insert(mysql::ER_BINLOG_ROW_RBR_TO_SBR, String::from("Slave running with --log-slave-updates must use row-based binary logging to be able to replicate row-based binary log events"));
        map.insert(
            mysql::ER_EVENT_ALREADY_EXISTS,
            String::from("Event '{:<.192}' already exists"),
        );
        map.insert(
            mysql::ER_EVENT_STORE_FAILED,
            String::from("Failed to store event {}. Error code {} from storage engine."),
        );
        map.insert(
            mysql::ER_EVENT_DOES_NOT_EXIST,
            String::from("Unknown event '{:<.192}'"),
        );
        map.insert(
            mysql::ER_EVENT_CANT_ALTER,
            String::from("Failed to alter event '{:<.192}'"),
        );
        map.insert(
            mysql::ER_EVENT_DROP_FAILED,
            String::from("Failed to drop {}"),
        );
        map.insert(
            mysql::ER_EVENT_INTERVAL_NOT_POSITIVE_OR_TOO_BIG,
            String::from("INTERVAL is either not positive or too big"),
        );
        map.insert(
            mysql::ER_EVENT_ENDS_BEFORE_STARTS,
            String::from("ENDS is either invalid or before STARTS"),
        );
        map.insert(
            mysql::ER_EVENT_EXEC_TIME_IN_THE_PAST,
            String::from("Event execution time is in the past. Event has been disabled"),
        );
        map.insert(
            mysql::ER_EVENT_OPEN_TABLE_FAILED,
            String::from("Failed to open mysql.event"),
        );
        map.insert(
            mysql::ER_EVENT_NEITHER_M_EXPR_NOR_M_AT,
            String::from("No datetime expression provided"),
        );
        map.insert(mysql::ER_OBSOLETE_COL_COUNT_DOESNT_MATCH_CORRUPTED, String::from("Column count of mysql.{} is wrong. Expected {}, found {}. The table is probably corrupted"));
        map.insert(
            mysql::ER_OBSOLETE_CANNOT_LOAD_FROM_TABLE,
            String::from("Cannot load from mysql.{}. The table is probably corrupted"),
        );
        map.insert(
            mysql::ER_EVENT_CANNOT_DELETE,
            String::from("Failed to delete the event from mysql.event"),
        );
        map.insert(
            mysql::ER_EVENT_COMPILE_ERROR,
            String::from("Error during compilation of event's body"),
        );
        map.insert(
            mysql::ER_EVENT_SAME_NAME,
            String::from("Same old and new event name"),
        );
        map.insert(
            mysql::ER_EVENT_DATA_TOO_LONG,
            String::from("Data for column '{}' too long"),
        );
        map.insert(
            mysql::ER_DROP_INDEX_FK,
            String::from("Cannot drop index '{:<.192}': needed in a foreign key constraint"),
        );
        map.insert(mysql::ER_WARN_DEPRECATED_SYNTAX_WITH_VER, String::from("The syntax '{}' is deprecated and will be removed in MySQL {}. Please use {} instead"));
        map.insert(
            mysql::ER_CANT_WRITE_LOCK_LOG_TABLE,
            String::from("You can't write-lock a log table. Only read access is possible"),
        );
        map.insert(
            mysql::ER_CANT_LOCK_LOG_TABLE,
            String::from("You can't use locks with log tables."),
        );
        map.insert(mysql::ER_FOREIGN_DUPLICATE_KEY_OLD_UNUSED, String::from("Upholding foreign key constraints for table '{:.192}', entry '{:<.192}', key {} would lead to a duplicate entry"));
        map.insert(mysql::ER_COL_COUNT_DOESNT_MATCH_PLEASE_UPDATE, String::from("Column count of mysql.{} is wrong. Expected {}, found {}. Created with MySQL {}, now running {}. Please use mysql_upgrade to fix this error."));
        map.insert(mysql::ER_TEMP_TABLE_PREVENTS_SWITCH_OUT_OF_RBR, String::from("Cannot switch out of the row-based binary log format when the session has open temporary tables"));
        map.insert(
            mysql::ER_STORED_FUNCTION_PREVENTS_SWITCH_BINLOG_FORMAT,
            String::from(
                "Cannot change the binary logging format inside a stored function or trigger",
            ),
        );
        map.insert(
            mysql::ER_NDB_CANT_SWITCH_BINLOG_FORMAT,
            String::from(
                "The NDB cluster engine does not support changing the binlog format on the fly yet",
            ),
        );
        map.insert(
            mysql::ER_PARTITION_NO_TEMPORARY,
            String::from("Cannot create temporary table with partitions"),
        );
        map.insert(
            mysql::ER_PARTITION_CONST_DOMAIN_ERROR,
            String::from("Partition constant is out of partition function domain"),
        );
        map.insert(
            mysql::ER_PARTITION_FUNCTION_IS_NOT_ALLOWED,
            String::from("This partition function is not allowed"),
        );
        map.insert(mysql::ER_DDL_LOG_ERROR, String::from("Error in DDL log"));
        map.insert(
            mysql::ER_NULL_IN_VALUES_LESS_THAN,
            String::from("Not allowed to use NULL value in VALUES LESS THAN"),
        );
        map.insert(
            mysql::ER_WRONG_PARTITION_NAME,
            String::from("Incorrect partition name"),
        );
        map.insert(
            mysql::ER_CANT_CHANGE_TX_CHARACTERISTICS,
            String::from(
                "Transaction characteristics can't be changed while a transaction is in progress",
            ),
        );
        map.insert(mysql::ER_DUP_ENTRY_AUTOINCREMENT_CASE, String::from("ALTER TABLE causes auto_increment resequencing, resulting in duplicate entry '{:<.192}' for key '{:<.192}'"));
        map.insert(
            mysql::ER_EVENT_MODIFY_QUEUE_ERROR,
            String::from("Internal scheduler error {}"),
        );
        map.insert(
            mysql::ER_EVENT_SET_VAR_ERROR,
            String::from("Error during starting/stopping of the scheduler. Error code {}"),
        );
        map.insert(
            mysql::ER_PARTITION_MERGE_ERROR,
            String::from("Engine cannot be used in partitioned tables"),
        );
        map.insert(
            mysql::ER_CANT_ACTIVATE_LOG,
            String::from("Cannot activate '{:<.64}' log"),
        );
        map.insert(
            mysql::ER_RBR_NOT_AVAILABLE,
            String::from("The server was not built with row-based replication"),
        );
        map.insert(
            mysql::ER_BASE64_DECODE_ERROR,
            String::from("Decoding of base64 string failed"),
        );
        map.insert(
            mysql::ER_EVENT_RECURSION_FORBIDDEN,
            String::from("Recursion of EVENT DDL statements is forbidden when body is present"),
        );
        map.insert(mysql::ER_EVENTS_DB_ERROR, String::from("Cannot proceed because system tables used by Event Scheduler were found damaged at server start"));
        map.insert(
            mysql::ER_ONLY_INTEGERS_ALLOWED,
            String::from("Only integers allowed as number here"),
        );
        map.insert(
            mysql::ER_UNSUPORTED_LOG_ENGINE,
            String::from("This storage engine cannot be used for log tables\""),
        );
        map.insert(
            mysql::ER_BAD_LOG_STATEMENT,
            String::from("You cannot '{}' a log table if logging is enabled"),
        );
        map.insert(mysql::ER_CANT_RENAME_LOG_TABLE, String::from("Cannot rename '{}'. When logging enabled, rename to/from log table must rename two tables: the log table to an archive table and another table back to '{}'"));
        map.insert(
            mysql::ER_WRONG_PARAMCOUNT_TO_NATIVE_FCT,
            String::from("Incorrect parameter count in the call to native function '{:<.192}'"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMETERS_TO_NATIVE_FCT,
            String::from("Incorrect parameters in the call to native function '{:<.192}'"),
        );
        map.insert(
            mysql::ER_WRONG_PARAMETERS_TO_STORED_FCT,
            String::from("Incorrect parameters in the call to stored function '{:<.192}'"),
        );
        map.insert(
            mysql::ER_NATIVE_FCT_NAME_COLLISION,
            String::from("This function '{:<.192}' has the same name as a native function"),
        );
        map.insert(
            mysql::ER_DUP_ENTRY_WITH_KEY_NAME,
            String::from("Duplicate entry '{:<.64}' for key '{:<.192}'"),
        );
        map.insert(
            mysql::ER_BINLOG_PURGE_EMFILE,
            String::from("Too many files opened, please execute the command again"),
        );
        map.insert(mysql::ER_EVENT_CANNOT_CREATE_IN_THE_PAST, String::from("Event execution time is in the past and ON COMPLETION NOT PRESERVE is set. The event was dropped immediately after creation."));
        map.insert(mysql::ER_EVENT_CANNOT_ALTER_IN_THE_PAST, String::from("Event execution time is in the past and ON COMPLETION NOT PRESERVE is set. The event was not changed. Specify a time in the future."));
        map.insert(
            mysql::ER_SLAVE_INCIDENT,
            String::from("The incident {} occurred on the master. Message: {:<.64}"),
        );
        map.insert(
            mysql::ER_NO_PARTITION_FOR_GIVEN_VALUE_SILENT,
            String::from("Table has no partition for some existing values"),
        );
        map.insert(mysql::ER_BINLOG_UNSAFE_STATEMENT, String::from("Unsafe statement written to the binary log using statement format since BINLOG_FORMAT = STATEMENT. {}"));
        map.insert(mysql::ER_SLAVE_FATAL_ERROR, String::from("Fatal error: {}"));
        map.insert(
            mysql::ER_SLAVE_RELAY_LOG_READ_FAILURE,
            String::from("Relay log read failure: {}"),
        );
        map.insert(
            mysql::ER_SLAVE_RELAY_LOG_WRITE_FAILURE,
            String::from("Relay log write failure: {}"),
        );
        map.insert(
            mysql::ER_SLAVE_CREATE_EVENT_FAILURE,
            String::from("Failed to create {}"),
        );
        map.insert(
            mysql::ER_SLAVE_MASTER_COM_FAILURE,
            String::from("Master command {} failed: {}"),
        );
        map.insert(
            mysql::ER_BINLOG_LOGGING_IMPOSSIBLE,
            String::from("Binary logging not possible. Message: {}"),
        );
        map.insert(
            mysql::ER_VIEW_NO_CREATION_CTX,
            String::from("View `{:<.64}`.`{:<.64}` has no creation context"),
        );
        map.insert(
            mysql::ER_VIEW_INVALID_CREATION_CTX,
            String::from("Creation context of view `{:<.64}`.`{:<.64}' is invalid"),
        );
        map.insert(
            mysql::ER_SR_INVALID_CREATION_CTX,
            String::from("Creation context of stored routine `{:<.64}`.`{:<.64}` is invalid"),
        );
        map.insert(
            mysql::ER_TRG_CORRUPTED_FILE,
            String::from("Corrupted TRG file for table `{:<.64}`.`{:<.64}`"),
        );
        map.insert(
            mysql::ER_TRG_NO_CREATION_CTX,
            String::from("Triggers for table `{:<.64}`.`{:<.64}` have no creation context"),
        );
        map.insert(
            mysql::ER_TRG_INVALID_CREATION_CTX,
            String::from("Trigger creation context of table `{:<.64}`.`{:<.64}` is invalid"),
        );
        map.insert(
            mysql::ER_EVENT_INVALID_CREATION_CTX,
            String::from("Creation context of event `{:<.64}`.`{:<.64}` is invalid"),
        );
        map.insert(
            mysql::ER_TRG_CANT_OPEN_TABLE,
            String::from("Cannot open table for trigger `{:<.64}`.`{:<.64}`"),
        );
        map.insert(
            mysql::ER_CANT_CREATE_SROUTINE,
            String::from("Cannot create stored routine `{:<.64}`. Check warnings"),
        );
        map.insert(
            mysql::ER_NEVER_USED,
            String::from("Ambiguous slave modes combination. {}"),
        );
        map.insert(mysql::ER_NO_FORMAT_DESCRIPTION_EVENT_BEFORE_BINLOG_STATEMENT, String::from("The BINLOG statement of type `{}` was not preceded by a format description BINLOG statement."));
        map.insert(
            mysql::ER_SLAVE_CORRUPT_EVENT,
            String::from("Corrupted replication event was detected"),
        );
        map.insert(
            mysql::ER_LOAD_DATA_INVALID_COLUMN,
            String::from("Invalid column reference ({:<.64}) in LOAD DATA"),
        );
        map.insert(
            mysql::ER_LOG_PURGE_NO_FILE,
            String::from("Being purged log {} was not found"),
        );
        map.insert(
            mysql::ER_XA_RBTIMEOUT,
            String::from("XA_RBTIMEOUT: Transaction branch was rolled back: took too long"),
        );
        map.insert(
            mysql::ER_XA_RBDEADLOCK,
            String::from(
                "XA_RBDEADLOCK: Transaction branch was rolled back: deadlock was detected",
            ),
        );
        map.insert(
            mysql::ER_NEED_REPREPARE,
            String::from("Prepared statement needs to be re-prepared"),
        );
        map.insert(
            mysql::ER_DELAYED_NOT_SUPPORTED,
            String::from("DELAYED option not supported for table '{:<.192}'"),
        );
        map.insert(
            mysql::WARN_NO_MASTER_INFO,
            String::from("The master info structure does not exist"),
        );
        map.insert(
            mysql::WARN_OPTION_IGNORED,
            String::from("<{:<.64}> option ignored"),
        );
        map.insert(
            mysql::WARN_PLUGIN_DELETE_BUILTIN,
            String::from("Built-in plugins cannot be deleted"),
        );
        map.insert(
            mysql::WARN_PLUGIN_BUSY,
            String::from("Plugin is busy and will be uninstalled on shutdown"),
        );
        map.insert(
            mysql::ER_VARIABLE_IS_READONLY,
            String::from("{} variable '{}' is read-only. Use SET {} to assign the value"),
        );
        map.insert(mysql::ER_WARN_ENGINE_TRANSACTION_ROLLBACK, String::from("Storage engine {} does not support rollback for this statement. Transaction rolled back and must be restarted"));
        map.insert(
            mysql::ER_SLAVE_HEARTBEAT_FAILURE,
            String::from("Unexpected master's heartbeat data: {}"),
        );
        map.insert(mysql::ER_SLAVE_HEARTBEAT_VALUE_OUT_OF_RANGE, String::from("The requested value for the heartbeat period is either negative or exceeds the maximum allowed ({} seconds)."));
        map.insert(
            mysql::ER_NDB_REPLICATION_SCHEMA_ERROR,
            String::from("Bad schema for mysql.ndb_replication table. Message: {:<.64}"),
        );
        map.insert(
            mysql::ER_CONFLICT_FN_PARSE_ERROR,
            String::from("Error in parsing conflict function. Message: {:<.64}"),
        );
        map.insert(
            mysql::ER_EXCEPTIONS_WRITE_ERROR,
            String::from("Write to exceptions table failed. Message: {:<.128}\""),
        );
        map.insert(
            mysql::ER_TOO_LONG_TABLE_COMMENT,
            String::from("Comment for table '{:<.64}' is too long (max = {})"),
        );
        map.insert(
            mysql::ER_TOO_LONG_FIELD_COMMENT,
            String::from("Comment for field '{:<.64}' is too long (max = {})"),
        );
        map.insert(mysql::ER_FUNC_INEXISTENT_NAME_COLLISION, String::from("FUNCTION {} does not exist. Check the 'Function Name Parsing and Resolution' section in the Reference Manual"));
        map.insert(mysql::ER_DATABASE_NAME, String::from("Database"));
        map.insert(mysql::ER_TABLE_NAME, String::from("Table"));
        map.insert(mysql::ER_PARTITION_NAME, String::from("Partition"));
        map.insert(mysql::ER_SUBPARTITION_NAME, String::from("Subpartition"));
        map.insert(mysql::ER_TEMPORARY_NAME, String::from("Temporary"));
        map.insert(mysql::ER_RENAMED_NAME, String::from("Renamed"));
        map.insert(
            mysql::ER_TOO_MANY_CONCURRENT_TRXS,
            String::from("Too many active concurrent transactions"),
        );
        map.insert(
            mysql::WARN_NON_ASCII_SEPARATOR_NOT_IMPLEMENTED,
            String::from("Non-ASCII separator arguments are not fully supported"),
        );
        map.insert(
            mysql::ER_DEBUG_SYNC_TIMEOUT,
            String::from("debug sync point wait timed out"),
        );
        map.insert(
            mysql::ER_DEBUG_SYNC_HIT_LIMIT,
            String::from("debug sync point hit limit reached"),
        );
        map.insert(
            mysql::ER_DUP_SIGNAL_SET,
            String::from("Duplicate condition information item '{}'"),
        );
        map.insert(
            mysql::ER_SIGNAL_WARN,
            String::from("Unhandled user-defined warning condition"),
        );
        map.insert(
            mysql::ER_SIGNAL_NOT_FOUND,
            String::from("Unhandled user-defined not found condition"),
        );
        map.insert(
            mysql::ER_SIGNAL_EXCEPTION,
            String::from("Unhandled user-defined exception condition"),
        );
        map.insert(
            mysql::ER_RESIGNAL_WITHOUT_ACTIVE_HANDLER,
            String::from("RESIGNAL when handler not active"),
        );
        map.insert(
            mysql::ER_SIGNAL_BAD_CONDITION_TYPE,
            String::from("SIGNAL/RESIGNAL can only use a CONDITION defined with SQLSTATE"),
        );
        map.insert(
            mysql::WARN_COND_ITEM_TRUNCATED,
            String::from("Data truncated for condition item '{}'"),
        );
        map.insert(
            mysql::ER_COND_ITEM_TOO_LONG,
            String::from("Data too long for condition item '{}'"),
        );
        map.insert(
            mysql::ER_UNKNOWN_LOCALE,
            String::from("Unknown locale: '{:<.64}'"),
        );
        map.insert(mysql::ER_SLAVE_IGNORE_SERVER_IDS, String::from("The requested server id {} clashes with the slave startup option --replicate-same-server-id"));
        map.insert(
            mysql::ER_QUERY_CACHE_DISABLED,
            String::from(
                "Query cache is disabled; restart the server with query_cache_type=1 to enable it",
            ),
        );
        map.insert(
            mysql::ER_SAME_NAME_PARTITION_FIELD,
            String::from("Duplicate partition field name '{:<.192}'"),
        );
        map.insert(
            mysql::ER_PARTITION_COLUMN_LIST_ERROR,
            String::from("Inconsistency in usage of column lists for partitioning"),
        );
        map.insert(
            mysql::ER_WRONG_TYPE_COLUMN_VALUE_ERROR,
            String::from("Partition column values of incorrect type"),
        );
        map.insert(
            mysql::ER_TOO_MANY_PARTITION_FUNC_FIELDS_ERROR,
            String::from("Too many fields in '{:<.192}'"),
        );
        map.insert(
            mysql::ER_MAXVALUE_IN_VALUES_IN,
            String::from("Cannot use MAXVALUE as value in VALUES IN"),
        );
        map.insert(
            mysql::ER_TOO_MANY_VALUES_ERROR,
            String::from("Cannot have more than one value for this type of {:<.64} partitioning"),
        );
        map.insert(
            mysql::ER_ROW_SINGLE_PARTITION_FIELD_ERROR,
            String::from(
                "Row expressions in VALUES IN only allowed for multi-field column partitioning",
            ),
        );
        map.insert(
            mysql::ER_FIELD_TYPE_NOT_ALLOWED_AS_PARTITION_FIELD,
            String::from("Field '{:<.192}' is of a not allowed type for this type of partitioning"),
        );
        map.insert(
            mysql::ER_PARTITION_FIELDS_TOO_LONG,
            String::from("The total length of the partitioning fields is too large"),
        );
        map.insert(mysql::ER_BINLOG_ROW_ENGINE_AND_STMT_ENGINE, String::from("Cannot execute statement: impossible to write to binary log since both row-incapable engines and statement-incapable engines are involved."));
        map.insert(mysql::ER_BINLOG_ROW_MODE_AND_STMT_ENGINE, String::from("Cannot execute statement: impossible to write to binary log since BINLOG_FORMAT = ROW and at least one table uses a storage engine limited to statement-based logging."));
        map.insert(mysql::ER_BINLOG_UNSAFE_AND_STMT_ENGINE, String::from("Cannot execute statement: impossible to write to binary log since statement is unsafe, storage engine is limited to statement-based logging, and BINLOG_FORMAT = MIXED. {}"));
        map.insert(mysql::ER_BINLOG_ROW_INJECTION_AND_STMT_ENGINE, String::from("Cannot execute statement: impossible to write to binary log since statement is in row format and at least one table uses a storage engine limited to statement-based logging."));
        map.insert(mysql::ER_BINLOG_STMT_MODE_AND_ROW_ENGINE, String::from("Cannot execute statement: impossible to write to binary log since BINLOG_FORMAT = STATEMENT and at least one table uses a storage engine limited to row-based logging.{}"));
        map.insert(mysql::ER_BINLOG_ROW_INJECTION_AND_STMT_MODE, String::from("Cannot execute statement: impossible to write to binary log since statement is in row format and BINLOG_FORMAT = STATEMENT."));
        map.insert(mysql::ER_BINLOG_MULTIPLE_ENGINES_AND_SELF_LOGGING_ENGINE, String::from("Cannot execute statement: impossible to write to binary log since more than one engine is involved and at least one engine is self-logging."));
        map.insert(mysql::ER_BINLOG_UNSAFE_LIMIT, String::from("The statement is unsafe because it uses a LIMIT clause. This is unsafe because the set of rows included cannot be predicted."));
        map.insert(mysql::ER_BINLOG_UNSAFE_INSERT_DELAYED, String::from("The statement is unsafe because it uses INSERT DELAYED. This is unsafe because the times when rows are inserted cannot be predicted."));
        map.insert(mysql::ER_BINLOG_UNSAFE_SYSTEM_TABLE, String::from("The statement is unsafe because it uses the general log, slow query log, or performance_schema table(s). This is unsafe because system tables may differ on slaves."));
        map.insert(mysql::ER_BINLOG_UNSAFE_AUTOINC_COLUMNS, String::from("Statement is unsafe because it invokes a trigger or a stored function that inserts into an AUTO_INCREMENT column. Inserted values cannot be logged correctly."));
        map.insert(mysql::ER_BINLOG_UNSAFE_UDF, String::from("Statement is unsafe because it uses a UDF which may not return the same value on the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_SYSTEM_VARIABLE, String::from("Statement is unsafe because it uses a system variable that may have a different value on the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_SYSTEM_FUNCTION, String::from("Statement is unsafe because it uses a system function that may return a different value on the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_NONTRANS_AFTER_TRANS, String::from("Statement is unsafe because it accesses a non-transactional table after accessing a transactional table within the same transaction."));
        map.insert(
            mysql::ER_MESSAGE_AND_STATEMENT,
            String::from("{} Statement: {}"),
        );
        map.insert(mysql::ER_SLAVE_CONVERSION_FAILED, String::from("Column {} of table '{:<.192}.{:<.192}' cannot be converted from type '{:<.32}' to type '{:<.32}'"));
        map.insert(
            mysql::ER_SLAVE_CANT_CREATE_CONVERSION,
            String::from("Can't create conversion table for table '{:<.192}.{:<.192}'"),
        );
        map.insert(
            mysql::ER_INSIDE_TRANSACTION_PREVENTS_SWITCH_BINLOG_FORMAT,
            String::from("Cannot modify @@session.binlog_format inside a transaction"),
        );
        map.insert(
            mysql::ER_PATH_LENGTH,
            String::from("The path specified for {:.64} is too long."),
        );
        map.insert(
            mysql::ER_WARN_DEPRECATED_SYNTAX_NO_REPLACEMENT,
            String::from("'{}' is deprecated and will be removed in a future release."),
        );
        map.insert(
            mysql::ER_WRONG_NATIVE_TABLE_STRUCTURE,
            String::from("Native table '{:<.64}'.'{:<.64}' has the wrong structure"),
        );
        map.insert(
            mysql::ER_WRONG_PERFSCHEMA_USAGE,
            String::from("Invalid performance_schema usage."),
        );
        map.insert(mysql::ER_WARN_I_S_SKIPPED_TABLE, String::from("Table '{}'.'{}' was skipped since its definition is being modified by concurrent DDL statement"));
        map.insert(mysql::ER_INSIDE_TRANSACTION_PREVENTS_SWITCH_BINLOG_DIRECT, String::from("Cannot modify @@session.binlog_direct_non_transactional_updates inside a transaction"));
        map.insert(
            mysql::ER_STORED_FUNCTION_PREVENTS_SWITCH_BINLOG_DIRECT,
            String::from(
                "Cannot change the binlog direct flag inside a stored function or trigger",
            ),
        );
        map.insert(
            mysql::ER_SPATIAL_MUST_HAVE_GEOM_COL,
            String::from("A SPATIAL index may only contain a geometrical type column"),
        );
        map.insert(
            mysql::ER_TOO_LONG_INDEX_COMMENT,
            String::from("Comment for index '{:<.64}' is too long (max = {})"),
        );
        map.insert(
            mysql::ER_LOCK_ABORTED,
            String::from("Wait on a lock was aborted due to a pending exclusive lock"),
        );
        map.insert(
            mysql::ER_DATA_OUT_OF_RANGE,
            String::from("{} value is out of range in '{}'"),
        );
        map.insert(
            mysql::ER_WRONG_SPVAR_TYPE_IN_LIMIT,
            String::from("A variable of a non-integer based type in LIMIT clause"),
        );
        map.insert(
            mysql::ER_BINLOG_UNSAFE_MULTIPLE_ENGINES_AND_SELF_LOGGING_ENGINE,
            String::from(
                "Mixing self-logging and non-self-logging engines in a statement is unsafe.",
            ),
        );
        map.insert(mysql::ER_BINLOG_UNSAFE_MIXED_STATEMENT, String::from("Statement accesses nontransactional table as well as transactional or temporary table, and writes to any of them."));
        map.insert(
            mysql::ER_INSIDE_TRANSACTION_PREVENTS_SWITCH_SQL_LOG_BIN,
            String::from("Cannot modify @@session.sql_log_bin inside a transaction"),
        );
        map.insert(
            mysql::ER_STORED_FUNCTION_PREVENTS_SWITCH_SQL_LOG_BIN,
            String::from("Cannot change the sql_log_bin inside a stored function or trigger"),
        );
        map.insert(
            mysql::ER_FAILED_READ_FROM_PAR_FILE,
            String::from("Failed to read from the .par file"),
        );
        map.insert(
            mysql::ER_VALUES_IS_NOT_INT_TYPE_ERROR,
            String::from("VALUES value for partition '{:<.64}' must have type INT"),
        );
        map.insert(
            mysql::ER_ACCESS_DENIED_NO_PASSWORD_ERROR,
            String::from("Access denied for user '{:<.48}'@'{:<.64}'"),
        );
        map.insert(
            mysql::ER_SET_PASSWORD_AUTH_PLUGIN,
            String::from("SET PASSWORD has no significance for users authenticating via plugins"),
        );
        map.insert(
            mysql::ER_GRANT_PLUGIN_USER_EXISTS,
            String::from(
                "GRANT with IDENTIFIED WITH is illegal because the user {:<.*} already exists",
            ),
        );
        map.insert(
            mysql::ER_TRUNCATE_ILLEGAL_FK,
            String::from(
                "Cannot truncate a table referenced in a foreign key constraint ({:.192})",
            ),
        );
        map.insert(
            mysql::ER_PLUGIN_IS_PERMANENT,
            String::from("Plugin '{}' is force_plus_permanent and can not be unloaded"),
        );
        map.insert(mysql::ER_SLAVE_HEARTBEAT_VALUE_OUT_OF_RANGE_MIN, String::from("The requested value for the heartbeat period is less than 1 millisecond. The value is reset to 0, meaning that heartbeating will effectively be disabled."));
        map.insert(mysql::ER_SLAVE_HEARTBEAT_VALUE_OUT_OF_RANGE_MAX, String::from("The requested value for the heartbeat period exceeds the value of `slave_net_timeout' seconds. A sensible value for the period should be less than the timeout."));
        map.insert(mysql::ER_STMT_CACHE_FULL, String::from("Multi-row statements required more than 'max_binlog_stmt_cache_size' bytes of storage; increase this mysqld variable and try again"));
        map.insert(mysql::ER_MULTI_UPDATE_KEY_CONFLICT, String::from("Primary key/partition key update is not allowed since the table is updated both as '{:<.192}' and '{:<.192}'."));
        map.insert(mysql::ER_TABLE_NEEDS_REBUILD, String::from("Table rebuild required. Please do \"ALTER TABLE `{:<.32}` FORCE\" or dump/reload to fix it!"));
        map.insert(
            mysql::WARN_OPTION_BELOW_LIMIT,
            String::from("The value of '{}' should be no less than the value of '{}'"),
        );
        map.insert(
            mysql::ER_INDEX_COLUMN_TOO_LONG,
            String::from("Index column size too large. The maximum column size is {} bytes."),
        );
        map.insert(
            mysql::ER_ERROR_IN_TRIGGER_BODY,
            String::from("Trigger '{:<.64}' has an error in its body: '{:<.256}'"),
        );
        map.insert(
            mysql::ER_ERROR_IN_UNKNOWN_TRIGGER_BODY,
            String::from("Unknown trigger has an error in its body: '{:<.256}'"),
        );
        map.insert(
            mysql::ER_INDEX_CORRUPT,
            String::from("Index {} is corrupted"),
        );
        map.insert(
            mysql::ER_UNDO_RECORD_TOO_BIG,
            String::from("Undo log record is too big."),
        );
        map.insert(mysql::ER_BINLOG_UNSAFE_INSERT_IGNORE_SELECT, String::from("INSERT IGNORE... SELECT is unsafe because the order in which rows are retrieved by the SELECT determines which (if any) rows are ignored. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_INSERT_SELECT_UPDATE, String::from("INSERT... SELECT... ON DUPLICATE KEY UPDATE is unsafe because the order in which rows are retrieved by the SELECT determines which (if any) rows are updated. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_REPLACE_SELECT, String::from("REPLACE... SELECT is unsafe because the order in which rows are retrieved by the SELECT determines which (if any) rows are replaced. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_CREATE_IGNORE_SELECT, String::from("CREATE... IGNORE SELECT is unsafe because the order in which rows are retrieved by the SELECT determines which (if any) rows are ignored. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_CREATE_REPLACE_SELECT, String::from("CREATE... REPLACE SELECT is unsafe because the order in which rows are retrieved by the SELECT determines which (if any) rows are replaced. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_UPDATE_IGNORE, String::from("UPDATE IGNORE is unsafe because the order in which rows are updated determines which (if any) rows are ignored. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_PLUGIN_NO_UNINSTALL, String::from("Plugin '{}' is marked as not dynamically uninstallable. You have to stop the server to uninstall it."));
        map.insert(mysql::ER_PLUGIN_NO_INSTALL, String::from("Plugin '{}' is marked as not dynamically installable. You have to stop the server to install it."));
        map.insert(mysql::ER_BINLOG_UNSAFE_WRITE_AUTOINC_SELECT, String::from("Statements writing to a table with an auto-increment column after selecting from another table are unsafe because the order in which rows are retrieved determines what (if any) rows will be written. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_CREATE_SELECT_AUTOINC, String::from("CREATE TABLE... SELECT...  on a table with an auto-increment column is unsafe because the order in which rows are retrieved by the SELECT determines which (if any) rows are inserted. This order cannot be predicted and may differ on master and the slave."));
        map.insert(mysql::ER_BINLOG_UNSAFE_INSERT_TWO_KEYS, String::from("INSERT... ON DUPLICATE KEY UPDATE  on a table with more than one UNIQUE KEY is unsafe"));
        map.insert(
            mysql::ER_TABLE_IN_FK_CHECK,
            String::from("Table is being used in foreign key check."),
        );
        map.insert(
            mysql::ER_UNSUPPORTED_ENGINE,
            String::from("Storage engine '{}' does not support system tables. [{}.{}]"),
        );
        map.insert(mysql::ER_BINLOG_UNSAFE_AUTOINC_NOT_FIRST, String::from("INSERT into autoincrement field which is not the first part in the composed primary key is unsafe."));
        map.insert(
            mysql::ER_CANNOT_LOAD_FROM_TABLE_V2,
            String::from("Cannot load from {}.{}. The table is probably corrupted"),
        );
        map.insert(
            mysql::ER_MASTER_DELAY_VALUE_OUT_OF_RANGE,
            String::from("The requested value {} for the master delay exceeds the maximum {}"),
        );
        map.insert(mysql::ER_ONLY_FD_AND_RBR_EVENTS_ALLOWED_IN_BINLOG_STATEMENT, String::from("Only Format_description_log_event and row events are allowed in BINLOG statements (but {} was provided)"));
        map.insert(
            mysql::ER_PARTITION_EXCHANGE_DIFFERENT_OPTION,
            String::from("Non matching attribute '{:<.64}' between partition and table"),
        );
        map.insert(
            mysql::ER_PARTITION_EXCHANGE_PART_TABLE,
            String::from("Table to exchange with partition is partitioned: '{:<.64}'"),
        );
        map.insert(
            mysql::ER_PARTITION_EXCHANGE_TEMP_TABLE,
            String::from("Table to exchange with partition is temporary: '{:<.64}'"),
        );
        map.insert(
            mysql::ER_PARTITION_INSTEAD_OF_SUBPARTITION,
            String::from("Subpartitioned table, use subpartition instead of partition"),
        );
        map.insert(
            mysql::ER_UNKNOWN_PARTITION,
            String::from("Unknown partition '{:<.64}' in table '{:<.64}'"),
        );
        map.insert(
            mysql::ER_TABLES_DIFFERENT_METADATA,
            String::from("Tables have different definitions"),
        );
        map.insert(
            mysql::ER_ROW_DOES_NOT_MATCH_PARTITION,
            String::from("Found a row that does not match the partition"),
        );
        map.insert(mysql::ER_BINLOG_CACHE_SIZE_GREATER_THAN_MAX, String::from("Option binlog_cache_size ({}) is greater than max_binlog_cache_size ({}); setting binlog_cache_size equal to max_binlog_cache_size."));
        map.insert(mysql::ER_WARN_INDEX_NOT_APPLICABLE, String::from("Cannot use {:<.64} access on index '{:<.64}' due to type or collation conversion on field '{:<.64}'"));
        map.insert(
            mysql::ER_PARTITION_EXCHANGE_FOREIGN_KEY,
            String::from("Table to exchange with partition has foreign key references: '{:<.64}'"),
        );
        map.insert(
            mysql::ER_NO_SUCH_KEY_VALUE,
            String::from("Key value '{:<.192}' was not found in table '{:<.192}.{:<.192}'"),
        );
        map.insert(
            mysql::ER_RPL_INFO_DATA_TOO_LONG,
            String::from("Data for column '{}' too long"),
        );
        map.insert(
            mysql::ER_NETWORK_READ_EVENT_CHECKSUM_FAILURE,
            String::from(
                "Replication event checksum verification failed while reading from network.",
            ),
        );
        map.insert(
            mysql::ER_BINLOG_READ_EVENT_CHECKSUM_FAILURE,
            String::from(
                "Replication event checksum verification failed while reading from a log file.",
            ),
        );
        map.insert(mysql::ER_BINLOG_STMT_CACHE_SIZE_GREATER_THAN_MAX, String::from("Option binlog_stmt_cache_size ({}) is greater than max_binlog_stmt_cache_size ({}); setting binlog_stmt_cache_size equal to max_binlog_stmt_cache_size."));
        map.insert(
            mysql::ER_CANT_UPDATE_TABLE_IN_CREATE_TABLE_SELECT,
            String::from("Can't update table '{:<.192}' while '{:<.192}' is being created."),
        );
        map.insert(
            mysql::ER_PARTITION_CLAUSE_ON_NONPARTITIONED,
            String::from("PARTITION () clause on non partitioned table"),
        );
        map.insert(
            mysql::ER_ROW_DOES_NOT_MATCH_GIVEN_PARTITION_SET,
            String::from("Found a row not matching the given partition set"),
        );
        map.insert(
            mysql::ER_NO_SUCH_PARTITION__UNUSED,
            String::from("partition '{:<.64}' doesn't exist"),
        );
        map.insert(
            mysql::ER_CHANGE_RPL_INFO_REPOSITORY_FAILURE,
            String::from("Failure while changing the type of replication repository: {}."),
        );
        map.insert(
            mysql::ER_WARNING_NOT_COMPLETE_ROLLBACK_WITH_CREATED_TEMP_TABLE,
            String::from("The creation of some temporary tables could not be rolled back."),
        );
        map.insert(mysql::ER_WARNING_NOT_COMPLETE_ROLLBACK_WITH_DROPPED_TEMP_TABLE, String::from("Some temporary tables were dropped, but these operations could not be rolled back."));
        map.insert(
            mysql::ER_MTS_FEATURE_IS_NOT_SUPPORTED,
            String::from("{} is not supported in multi-threaded slave mode. {}"),
        );
        map.insert(mysql::ER_MTS_UPDATED_DBS_GREATER_MAX, String::from("The number of modified databases exceeds the maximum {}; the database names will not be included in the replication event metadata."));
        map.insert(mysql::ER_MTS_CANT_PARALLEL, String::from("Cannot execute the current event group in the parallel mode. Encountered event {}, relay-log name {}, position {} which prevents execution of this event group in parallel mode. Reason: {}."));
        map.insert(mysql::ER_MTS_INCONSISTENT_DATA, String::from("{}"));
        map.insert(
            mysql::ER_FULLTEXT_NOT_SUPPORTED_WITH_PARTITIONING,
            String::from("FULLTEXT index is not supported for partitioned tables."),
        );
        map.insert(
            mysql::ER_DA_INVALID_CONDITION_NUMBER,
            String::from("Invalid condition number"),
        );
        map.insert(
            mysql::ER_INSECURE_PLAIN_TEXT,
            String::from("Sending passwords in plain text without SSL/TLS is extremely insecure."),
        );
        map.insert(mysql::ER_INSECURE_CHANGE_MASTER, String::from("Storing MySQL user name or password information in the master.info repository is not secure and is therefore not recommended. Please see the MySQL Manual for more about this issue and possible alternatives."));
        map.insert(mysql::ER_FOREIGN_DUPLICATE_KEY_WITH_CHILD_INFO, String::from("Foreign key constraint for table '{:.192}', record '{:<.192}' would lead to a duplicate entry in table '{:.192}', key '{:.192}'"));
        map.insert(mysql::ER_FOREIGN_DUPLICATE_KEY_WITHOUT_CHILD_INFO, String::from("Foreign key constraint for table '{:.192}', record '{:<.192}' would lead to a duplicate entry in a child table"));
        map.insert(mysql::ER_SQLTHREAD_WITH_SECURE_SLAVE, String::from("Setting authentication options is not possible when only the Slave SQL Thread is being started."));
        map.insert(
            mysql::ER_TABLE_HAS_NO_FT,
            String::from("The table does not have FULLTEXT index to support this query"),
        );
        map.insert(
            mysql::ER_VARIABLE_NOT_SETTABLE_IN_SF_OR_TRIGGER,
            String::from(
                "The system variable {:<.200} cannot be set in stored functions or triggers.",
            ),
        );
        map.insert(
            mysql::ER_VARIABLE_NOT_SETTABLE_IN_TRANSACTION,
            String::from(
                "The system variable {:<.200} cannot be set when there is an ongoing transaction.",
            ),
        );
        map.insert(mysql::ER_GTID_NEXT_IS_NOT_IN_GTID_NEXT_LIST, String::from("The system variable @@SESSION.GTID_NEXT has the value {:<.200}, which is not listed in @@SESSION.GTID_NEXT_LIST."));
        map.insert(mysql::ER_CANT_CHANGE_GTID_NEXT_IN_TRANSACTION_WHEN_GTID_NEXT_LIST_IS_NULL, String::from("When @@SESSION.GTID_NEXT_LIST == NULL, the system variable @@SESSION.GTID_NEXT cannot change inside a transaction."));
        map.insert(
            mysql::ER_SET_STATEMENT_CANNOT_INVOKE_FUNCTION,
            String::from("The statement 'SET {:<.200}' cannot invoke a stored function."),
        );
        map.insert(mysql::ER_GTID_NEXT_CANT_BE_AUTOMATIC_IF_GTID_NEXT_LIST_IS_NON_NULL, String::from("The system variable @@SESSION.GTID_NEXT cannot be 'AUTOMATIC' when @@SESSION.GTID_NEXT_LIST is non-NULL."));
        map.insert(
            mysql::ER_SKIPPING_LOGGED_TRANSACTION,
            String::from(
                "Skipping transaction {:<.200} because it has already been executed and logged.",
            ),
        );
        map.insert(
            mysql::ER_MALFORMED_GTID_SET_SPECIFICATION,
            String::from("Malformed GTID set specification '{:<.200}'."),
        );
        map.insert(
            mysql::ER_MALFORMED_GTID_SET_ENCODING,
            String::from("Malformed GTID set encoding."),
        );
        map.insert(
            mysql::ER_MALFORMED_GTID_SPECIFICATION,
            String::from("Malformed GTID specification '{:<.200}'."),
        );
        map.insert(mysql::ER_GNO_EXHAUSTED, String::from("Impossible to generate Global Transaction Identifier: the integer component reached the maximal value. Restart the server with a new server_uuid."));
        map.insert(mysql::ER_BAD_SLAVE_AUTO_POSITION, String::from("Parameters MASTER_LOG_FILE, MASTER_LOG_POS, RELAY_LOG_FILE and RELAY_LOG_POS cannot be set when MASTER_AUTO_POSITION is active."));
        map.insert(mysql::ER_AUTO_POSITION_REQUIRES_GTID_MODE_ON, String::from("CHANGE MASTER TO MASTER_AUTO_POSITION = 1 can only be executed when @@GLOBAL.GTID_MODE = ON."));
        map.insert(mysql::ER_CANT_DO_IMPLICIT_COMMIT_IN_TRX_WHEN_GTID_NEXT_IS_SET, String::from("Cannot execute statements with implicit commit inside a transaction when @@SESSION.GTID_NEXT != AUTOMATIC or @@SESSION.GTID_NEXT_LIST != NULL."));
        map.insert(mysql::ER_GTID_MODE_2_OR_3_REQUIRES_ENFORCE_GTID_CONSISTENCY_ON, String::from("@@GLOBAL.GTID_MODE = ON or UPGRADE_STEP_2 requires @@GLOBAL.ENFORCE_GTID_CONSISTENCY = 1."));
        map.insert(mysql::ER_GTID_MODE_REQUIRES_BINLOG, String::from("@@GLOBAL.GTID_MODE = ON or UPGRADE_STEP_1 or UPGRADE_STEP_2 requires --log-bin and --log-slave-updates."));
        map.insert(
            mysql::ER_CANT_SET_GTID_NEXT_TO_GTID_WHEN_GTID_MODE_IS_OFF,
            String::from(
                "@@SESSION.GTID_NEXT cannot be set to UUID:NUMBER when @@GLOBAL.GTID_MODE = OFF.",
            ),
        );
        map.insert(
            mysql::ER_CANT_SET_GTID_NEXT_TO_ANONYMOUS_WHEN_GTID_MODE_IS_ON,
            String::from(
                "@@SESSION.GTID_NEXT cannot be set to ANONYMOUS when @@GLOBAL.GTID_MODE = ON.",
            ),
        );
        map.insert(mysql::ER_CANT_SET_GTID_NEXT_LIST_TO_NON_NULL_WHEN_GTID_MODE_IS_OFF, String::from("@@SESSION.GTID_NEXT_LIST cannot be set to a non-NULL value when @@GLOBAL.GTID_MODE = OFF."));
        map.insert(
            mysql::ER_FOUND_GTID_EVENT_WHEN_GTID_MODE_IS_OFF,
            String::from(
                "Found a Gtid_log_event or Previous_gtids_log_event when @@GLOBAL.GTID_MODE = OFF.",
            ),
        );
        map.insert(mysql::ER_GTID_UNSAFE_NON_TRANSACTIONAL_TABLE, String::from("When @@GLOBAL.ENFORCE_GTID_CONSISTENCY = 1, updates to non-transactional tables can only be done in either autocommitted statements or single-statement transactions, and never in the same statement as updates to transactional tables."));
        map.insert(
            mysql::ER_GTID_UNSAFE_CREATE_SELECT,
            String::from(
                "CREATE TABLE ... SELECT is forbidden when @@GLOBAL.ENFORCE_GTID_CONSISTENCY = 1.",
            ),
        );
        map.insert(mysql::ER_GTID_UNSAFE_CREATE_DROP_TEMPORARY_TABLE_IN_TRANSACTION, String::from("When @@GLOBAL.ENFORCE_GTID_CONSISTENCY = 1, the statements CREATE TEMPORARY TABLE and DROP TEMPORARY TABLE can be executed in a non-transactional context only, and require that AUTOCOMMIT = 1."));
        map.insert(mysql::ER_GTID_MODE_CAN_ONLY_CHANGE_ONE_STEP_AT_A_TIME, String::from("The value of @@GLOBAL.GTID_MODE can only change one step at a time: OFF <-> UPGRADE_STEP_1 <-> UPGRADE_STEP_2 <-> ON. Also note that this value must be stepped up or down simultaneously on all servers; see the Manual for instructions."));
        map.insert(mysql::ER_MASTER_HAS_PURGED_REQUIRED_GTIDS, String::from("The slave is connecting using CHANGE MASTER TO MASTER_AUTO_POSITION = 1, but the master has purged binary logs containing GTIDs that the slave requires."));
        map.insert(mysql::ER_CANT_SET_GTID_NEXT_WHEN_OWNING_GTID, String::from("@@SESSION.GTID_NEXT cannot be changed by a client that owns a GTID. The client owns {}. Ownership is released on COMMIT or ROLLBACK."));
        map.insert(
            mysql::ER_UNKNOWN_EXPLAIN_FORMAT,
            String::from("Unknown EXPLAIN format name: '{}'"),
        );
        map.insert(
            mysql::ER_CANT_EXECUTE_IN_READ_ONLY_TRANSACTION,
            String::from("Cannot execute statement in a READ ONLY transaction."),
        );
        map.insert(
            mysql::ER_TOO_LONG_TABLE_PARTITION_COMMENT,
            String::from("Comment for table partition '{:<.64}' is too long (max = {})"),
        );
        map.insert(mysql::ER_SLAVE_CONFIGURATION, String::from("Slave is not configured or failed to initialize properly. You must at least set --server-id to enable either a master or a slave. Additional error messages can be found in the MySQL error log."));
        map.insert(
            mysql::ER_INNODB_FT_LIMIT,
            String::from("InnoDB presently supports one FULLTEXT index creation at a time"),
        );
        map.insert(
            mysql::ER_INNODB_NO_FT_TEMP_TABLE,
            String::from("Cannot create FULLTEXT index on temporary InnoDB table"),
        );
        map.insert(
            mysql::ER_INNODB_FT_WRONG_DOCID_COLUMN,
            String::from("Column '{:<.192}' is of wrong type for an InnoDB FULLTEXT index"),
        );
        map.insert(
            mysql::ER_INNODB_FT_WRONG_DOCID_INDEX,
            String::from("Index '{:<.192}' is of wrong type for an InnoDB FULLTEXT index"),
        );
        map.insert(mysql::ER_INNODB_ONLINE_LOG_TOO_BIG, String::from("Creating index '{:<.192}' required more than 'innodb_online_alter_log_max_size' bytes of modification log. Please try again."));
        map.insert(
            mysql::ER_UNKNOWN_ALTER_ALGORITHM,
            String::from("Unknown ALGORITHM '{}'"),
        );
        map.insert(
            mysql::ER_UNKNOWN_ALTER_LOCK,
            String::from("Unknown LOCK type '{}'"),
        );
        map.insert(mysql::ER_MTS_CHANGE_MASTER_CANT_RUN_WITH_GAPS, String::from("CHANGE MASTER cannot be executed when the slave was stopped with an error or killed in MTS mode. Consider using RESET SLAVE or START SLAVE UNTIL."));
        map.insert(mysql::ER_MTS_RECOVERY_FAILURE, String::from("Cannot recover after SLAVE errored out in parallel execution mode. Additional error messages can be found in the MySQL error log."));
        map.insert(mysql::ER_MTS_RESET_WORKERS, String::from("Cannot clean up worker info tables. Additional error messages can be found in the MySQL error log."));
        map.insert(mysql::ER_COL_COUNT_DOESNT_MATCH_CORRUPTED_V2, String::from("Column count of {}.{} is wrong. Expected {}, found {}. The table is probably corrupted"));
        map.insert(
            mysql::ER_SLAVE_SILENT_RETRY_TRANSACTION,
            String::from("Slave must silently retry current transaction"),
        );
        map.insert(mysql::ER_DISCARD_FK_CHECKS_RUNNING, String::from("There is a foreign key check running on table '{:<.192}'. Cannot discard the table."));
        map.insert(
            mysql::ER_TABLE_SCHEMA_MISMATCH,
            String::from("Schema mismatch ({})"),
        );
        map.insert(
            mysql::ER_TABLE_IN_SYSTEM_TABLESPACE,
            String::from("Table '{:<.192}' in system tablespace"),
        );
        map.insert(
            mysql::ER_IO_READ_ERROR,
            String::from("IO Read error: ({}, {}) {}"),
        );
        map.insert(
            mysql::ER_IO_WRITE_ERROR,
            String::from("IO Write error: ({}, {}) {}"),
        );
        map.insert(
            mysql::ER_TABLESPACE_MISSING,
            String::from("Tablespace is missing for table '{:<.192}'"),
        );
        map.insert(mysql::ER_TABLESPACE_EXISTS, String::from("Tablespace for table '{:<.192}' exists. Please DISCARD the tablespace before IMPORT."));
        map.insert(
            mysql::ER_TABLESPACE_DISCARDED,
            String::from("Tablespace has been discarded for table '{:<.192}'"),
        );
        map.insert(mysql::ER_INTERNAL_ERROR, String::from("Internal error: {}"));
        map.insert(
            mysql::ER_INNODB_IMPORT_ERROR,
            String::from("ALTER TABLE '{:<.192}' IMPORT TABLESPACE failed with error {} : '{}'"),
        );
        map.insert(
            mysql::ER_INNODB_INDEX_CORRUPT,
            String::from("Index corrupt: {}"),
        );
        map.insert(
            mysql::ER_INVALID_YEAR_COLUMN_LENGTH,
            String::from("YEAR({}) column type is deprecated. Creating YEAR(4) column instead."),
        );
        map.insert(
            mysql::ER_NOT_VALID_PASSWORD,
            String::from("Your password does not satisfy the current policy requirements"),
        );
        map.insert(
            mysql::ER_MUST_CHANGE_PASSWORD,
            String::from("You must SET PASSWORD before executing this statement"),
        );
        map.insert(mysql::ER_FK_NO_INDEX_CHILD, String::from("Failed to add the foreign key constaint. Missing index for constraint '{}' in the foreign table '{}'"));
        map.insert(mysql::ER_FK_NO_INDEX_PARENT, String::from("Failed to add the foreign key constaint. Missing index for constraint '{}' in the referenced table '{}'"));
        map.insert(
            mysql::ER_FK_FAIL_ADD_SYSTEM,
            String::from("Failed to add the foreign key constraint '{}' to system tables"),
        );
        map.insert(
            mysql::ER_FK_CANNOT_OPEN_PARENT,
            String::from("Failed to open the referenced table '{}'"),
        );
        map.insert(mysql::ER_FK_INCORRECT_OPTION, String::from("Failed to add the foreign key constraint on table '{}'. Incorrect options in FOREIGN KEY constraint '{}'"));
        map.insert(
            mysql::ER_FK_DUP_NAME,
            String::from("Duplicate foreign key constraint name '{}'"),
        );
        map.insert(mysql::ER_PASSWORD_FORMAT, String::from("The password hash doesn't have the expected format. Check if the correct password algorithm is being used with the PASSWORD() function."));
        map.insert(
            mysql::ER_FK_COLUMN_CANNOT_DROP,
            String::from(
                "Cannot drop column '{:<.192}': needed in a foreign key constraint '{:<.192}'",
            ),
        );
        map.insert(mysql::ER_FK_COLUMN_CANNOT_DROP_CHILD, String::from("Cannot drop column '{:<.192}': needed in a foreign key constraint '{:<.192}' of table '{:<.192}'"));
        map.insert(mysql::ER_FK_COLUMN_NOT_NULL, String::from("Column '{:<.192}' cannot be NOT NULL: needed in a foreign key constraint '{:<.192}' SET NULL"));
        map.insert(mysql::ER_DUP_INDEX, String::from("Duplicate index '{:<.64}' defined on the table '{:<.64}.{:<.64}'. This is deprecated and will be disallowed in a future release."));
        map.insert(
            mysql::ER_FK_COLUMN_CANNOT_CHANGE,
            String::from(
                "Cannot change column '{:<.192}': used in a foreign key constraint '{:<.192}'",
            ),
        );
        map.insert(mysql::ER_FK_COLUMN_CANNOT_CHANGE_CHILD, String::from("Cannot change column '{:<.192}': used in a foreign key constraint '{:<.192}' of table '{:<.192}'"));
        map.insert(mysql::ER_FK_CANNOT_DELETE_PARENT, String::from("Cannot delete rows from table which is parent in a foreign key constraint '{:<.192}' of table '{:<.192}'"));
        map.insert(
            mysql::ER_MALFORMED_PACKET,
            String::from("Malformed communication packet."),
        );
        map.insert(
            mysql::ER_READ_ONLY_MODE,
            String::from("Running in read-only mode"),
        );
        map.insert(mysql::ER_GTID_NEXT_TYPE_UNDEFINED_GROUP, String::from("When @@SESSION.GTID_NEXT is set to a GTID, you must explicitly set it again after a COMMIT or ROLLBACK. If you see this error message in the slave SQL thread, it means that a table in the current transaction is transactional on the master and non-transactional on the slave. In a client connection, it means that you executed SET @@SESSION.GTID_NEXT before a transaction and forgot to set @@SESSION.GTID_NEXT to a different identifier or to 'AUTOMATIC' after COMMIT or ROLLBACK. Current @@SESSION.GTID_NEXT is '{}'."));
        map.insert(
            mysql::ER_VARIABLE_NOT_SETTABLE_IN_SP,
            String::from("The system variable {:<.200} cannot be set in stored procedures."),
        );
        map.insert(
            mysql::ER_CANT_SET_GTID_PURGED_WHEN_GTID_MODE_IS_OFF,
            String::from("@@GLOBAL.GTID_PURGED can only be set when @@GLOBAL.GTID_MODE = ON."),
        );
        map.insert(
            mysql::ER_CANT_SET_GTID_PURGED_WHEN_GTID_EXECUTED_IS_NOT_EMPTY,
            String::from(
                "@@GLOBAL.GTID_PURGED can only be set when @@GLOBAL.GTID_EXECUTED is empty.",
            ),
        );
        map.insert(mysql::ER_CANT_SET_GTID_PURGED_WHEN_OWNED_GTIDS_IS_NOT_EMPTY, String::from("@@GLOBAL.GTID_PURGED can only be set when there are no ongoing transactions (not even in other clients)."));
        map.insert(
            mysql::ER_GTID_PURGED_WAS_CHANGED,
            String::from("@@GLOBAL.GTID_PURGED was changed from '{}' to '{}'."),
        );
        map.insert(
            mysql::ER_GTID_EXECUTED_WAS_CHANGED,
            String::from("@@GLOBAL.GTID_EXECUTED was changed from '{}' to '{}'."),
        );
        map.insert(mysql::ER_BINLOG_STMT_MODE_AND_NO_REPL_TABLES, String::from("Cannot execute statement: impossible to write to binary log since BINLOG_FORMAT = STATEMENT, and both replicated and non replicated tables are written to."));
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED,
            String::from("{} is not supported for this operation. Try {}."),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON,
            String::from("{} is not supported. Reason: {}. Try {}."),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_COPY,
            String::from("COPY algorithm requires a lock"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_PARTITION,
            String::from("Partition specific operations do not yet support LOCK/ALGORITHM"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_FK_RENAME,
            String::from("Columns participating in a foreign key are renamed"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_COLUMN_TYPE,
            String::from("Cannot change column type INPLACE"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_FK_CHECK,
            String::from("Adding foreign keys needs foreign_key_checks=OFF"),
        );
        map.insert(mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_IGNORE, String::from("Creating unique indexes with IGNORE requires COPY algorithm to remove duplicate rows"));
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_NOPK,
            String::from(
                "Dropping a primary key is not allowed without also adding a new primary key",
            ),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_AUTOINC,
            String::from("Adding an auto-increment column requires a lock"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_HIDDEN_FTS,
            String::from("Cannot replace hidden FTS_DOC_ID with a user-visible one"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_CHANGE_FTS,
            String::from("Cannot drop or rename FTS_DOC_ID"),
        );
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_FTS,
            String::from("Fulltext index creation requires a lock"),
        );
        map.insert(mysql::ER_SQL_SLAVE_SKIP_COUNTER_NOT_SETTABLE_IN_GTID_MODE, String::from("sql_slave_skip_counter can not be set when the server is running with @@GLOBAL.GTID_MODE = ON. Instead, for each transaction that you want to skip, generate an empty transaction with the same GTID as the transaction"));
        map.insert(
            mysql::ER_DUP_UNKNOWN_IN_INDEX,
            String::from("Duplicate entry for key '{:<.192}'"),
        );
        map.insert(mysql::ER_IDENT_CAUSES_TOO_LONG_PATH, String::from("Long database name and identifier for object resulted in path length exceeding {} characters. Path: '{}'."));
        map.insert(
            mysql::ER_ALTER_OPERATION_NOT_SUPPORTED_REASON_NOT_NULL,
            String::from("cannot silently convert NULL values, as required in this SQL_MODE"),
        );
        map.insert(mysql::ER_MUST_CHANGE_PASSWORD_LOGIN, String::from("Your password has expired. To log in you must change it using a client that supports expired passwords."));
        map.insert(
            mysql::ER_ROW_IN_WRONG_PARTITION,
            String::from("Found a row in wrong partition {}"),
        );

        map
    };
}
