use crate::mysql::{DEFAULT_MYSQL_STATE, MYSQL_ERR_NAME, MYSQL_STATE};
use crate::utils;
use scan_fmt::scan_fmt;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq)]
pub enum MysqlError {
    ErrBadConn,
    ErrMalformPacket,
    ErrTxDone,
}

impl std::error::Error for MysqlError {}

impl Display for MysqlError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            MysqlError::ErrBadConn => {
                write!(f, "{}", "connection was bad")
            }
            MysqlError::ErrMalformPacket => {
                write!(f, "{}", "Malform packet error")
            }
            MysqlError::ErrTxDone => {
                write!(
                    f,
                    "{}",
                    "sql: Transaction has already been committed or rolled back"
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MyError {
    pub code: u16,
    pub message: String,
    pub state: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "ERROR {code} ({state}): {message}",
            code = self.code,
            state = self.state,
            message = self.message
        )
    }
}

impl MyError {
    pub fn new_default(err_code: u16, args: &Vec<utils::format::FVariant>) -> Self {
        let mut e = MyError {
            code: err_code,
            message: "".to_string(),
            state: "".to_string(),
        };
        if let Some(s) = MYSQL_STATE.get(&err_code) {
            e.state = s.to_string()
        } else {
            e.state = DEFAULT_MYSQL_STATE.to_string()
        }

        if let Some(f) = MYSQL_ERR_NAME.get(&err_code) {
            e.message = utils::format::fmt_str_vec(f, args);
        } else {
            format!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        e
    }

    pub fn new(err_code: u16, message: &str) -> Self {
        let mut e = MyError {
            code: err_code,
            message: message.to_string(),
            state: "".to_string(),
        };
        if let Some(s) = MYSQL_STATE.get(&err_code) {
            e.state = s.to_string()
        } else {
            e.state = DEFAULT_MYSQL_STATE.to_string()
        }

        e
    }

    pub fn error_code(err_msg: &str) -> isize {
        let mut _tmp_str = "";
        let (_, code) =
            scan_fmt!(err_msg, "{}{}", String, isize).map_or(("".to_string(), 0), |data| data);
        code
    }
}

mod tests {
    use scan_fmt::scan_fmt;

    #[test]
    fn a() {
        let msg = "Kim is 22 years old";
        let (s, i) = scan_fmt!(msg, "{} is {} years old", String, isize).unwrap();
        println!("{} {}", s, i)
    }
}
