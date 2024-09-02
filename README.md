# rust-db-diesel
使用Rust Diesel自动编写数据库model和schema，管理数据库连接；  
using the diesel Rust crate to automatically write our schema and database models in Rust. manage the connection to the database

该项目用于 https://github.com/lijiachang/flask_with_rust 的依赖

## macos 安装diesel
1. 安装postgresql  
brew install postgresql  
    添加环境变量  
    ```text
    ➜  rust-db-diesel git:(main) ✗ tail ~/.zshrc                      
    
    # postgresql env
    export PATH="/opt/homebrew/opt/postgresql@14/bin:$PATH"
    export LDFLAGS="-L/opt/homebrew/opt/postgresql@14/lib"
    export CPPFLAGS="-I/opt/homebrew/opt/postgresql@14/include"
    export LIBRARY_PATH="/opt/homebrew/opt/postgresql@14/lib:$LIBRARY_PATH"
    export PKG_CONFIG_PATH="/opt/homebrew/opt/postgresql@14/lib/pkgconfig:$PKG_CONFIG_PATH"
    ```
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
要注意的是：在创建好models.rs之后，需要修改。因为diesel默认假定表名为模型模型的复数，  
如果我们的不是，需要添加#[diesel(table_name = xxx)] //自定义表名