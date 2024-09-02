// 返回数据库连接的函数
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection(url: String) -> PgConnection {
    PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}