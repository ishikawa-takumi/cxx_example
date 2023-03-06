fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/hello.cpp")
        .flag_if_supported("-std=c++20")
        .compile("cxx-example");
    println!("cargo:rerun-if-changed=/src/*");
    println!("cargo:rerun-if-changed=/build.rs");
}
