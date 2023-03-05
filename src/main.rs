#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx_example/src/hello.h");

        fn hello();
    }
}

fn main() {
    ffi::hello();
}
