// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::fib_entries;
use crate::schema::alembic_version;

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(version_num))]
#[diesel(table_name = alembic_version)] //自定义表名
pub struct AlembicVersion {
    pub version_num: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = fib_entries)] //自定义表名
pub struct FibEntry {
    pub id: i32,
    pub input_number: Option<i32>,
    pub calculated_number: Option<i32>,
}

