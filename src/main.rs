mod ffi {
    #[link(name = "hello")]
    extern "C" {
        pub fn hello();
    }
}

fn main() {
    unsafe {
        ffi::hello();
    }
}
