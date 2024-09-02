# rust-db-diesel
使用Rust Diesel自动编写数据库model和schema，管理数据库连接；  
using the diesel Rust crate to automatically write our schema and database models in Rust. manage the connection to the database

## macos 安装diesel
1. 安装postgresql  
brew install postgresql  
2. 安装diesel命令行  
cargo install diesel_cli --no-default-features --features postgres  
3. 安装diesel命令行拓展工具  
cargo install diesel_cli_ext



## 启动docker-compose中的postgresql
https://github.com/lijiachang/flask_with_rust  
在根目录中执行`docker-compose up`  

## Diesel根据数据库模式(schema)自动生成对应的 Rust 结构体  
首先执行`source .env`加载环境变量DATABASE_URL  

`diesel print-schema >src/schema.rs`  
这个命令会连接到数据库、读取表结构，然后生成对应的 Rust 代码。生成的 schema.rs 文件会包含表的定义和字段信息

`diesel_ext > src/models.rs`  
这个命令可以帮助自动创建模型