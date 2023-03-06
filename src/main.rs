use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx_example/src/hello.h");

        fn hello(name: &CxxString) -> &CxxString;
    }
}

fn main() {
    let_cxx_string!(name = "Taro");
    let message = ffi::hello(&name);
    println!("{}", message);
}
