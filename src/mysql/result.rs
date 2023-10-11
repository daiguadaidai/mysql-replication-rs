use crate::mysql::resultset::ResultSet;

#[derive(Debug, Default, Clone)]
pub struct MysqlResult {
    pub status: u16,
    pub warnings: u16,
    pub insert_id: u64,
    pub affected_rows: u64,
    pub result_set: Option<ResultSet>,
}

impl MysqlResult {
    pub fn close(&mut self) {
        if !self.result_set.is_none() {
            self.result_set = None
        }
    }
}

/*
type Executer interface {
    Execute(query string, args ...interface{}) (*Result, error)
}
 */
