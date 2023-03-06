#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn callback();
    }

    unsafe extern "C++" {
        include!("cxx_example/src/hello.h");

        fn hello();
    }
}

fn callback() {
    println!("Callback");
}

fn main() {
    ffi::hello();
}
