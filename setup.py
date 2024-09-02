from setuptools import dist

dist.Distribution().fetch_build_eggs(["setuptools_rust"])
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rust-database-diesel",
    version="0.1",
    rust_extensions=[
        RustExtension(
            ".rust_db_diesel.rust_db_diesel", path="Cargo.toml", binding=Binding.PyO3
        )
    ],
    # 第一个rust_db_diesel是下面packages定义的名字（也是项目中的rust_db_diesel目录），
    # 第二个rust_db_diesel是在 lib.rs 中的#[pymodule]定义的fn rust_db_diesel(module: &Bound<'_, PyModule>)
    packages=["rust_db_diesel"],
    zip_safe=False,
)
