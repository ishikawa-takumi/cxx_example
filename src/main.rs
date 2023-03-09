#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx_example/src/hello_wrapper.h");

        fn hello_wrapper();
    }
}

fn main() {
    ffi::hello_wrapper();
}
