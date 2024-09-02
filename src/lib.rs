#[macro_use] extern crate diesel;
extern crate dotenv; // 本示例并没有用到dotenv，因为没有用环境变量，是用的Python系统传入了数据库URL
use diesel::prelude::*;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

mod database;
mod schema;
mod models;

use database::establish_connection;
use models::FibEntry;
use schema::fib_entries;

/// 获取并返回所有斐波那契记录
#[pyfunction]
fn get_fib_entries(url: String, py: Python) -> Vec<Bound<PyDict>>{
    let mut connection = establish_connection(url);

    // 按照input_number字段从小到大排序（ASC）
    let fibs = fib_entries::table.order(fib_entries::columns::input_number.asc()).load::<FibEntry>(&mut connection).unwrap();

    let mut buffer = Vec::new();

    for i in fibs{
        let placeholder = PyDict::new_bound(py);
        placeholder.set_item("input number", i.input_number.unwrap()).unwrap();
        placeholder.set_item("fib number", i.calculated_number.unwrap()).unwrap();
        buffer.push(placeholder);
    }

    buffer
}

#[pymodule]
fn rust_db_diesel(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = module.add_function(wrap_pyfunction!(get_fib_entries, module)?);
    Ok(())
}