use std::env;

fn main() {
    let project_dir = env::current_dir().unwrap();
    println!(
        "cargo:rustc-link-search=native={}/src",
        project_dir.display()
    );

    cxx_build::bridge("src/main.rs")
        .file("src/hello_wrapper.cpp")
        .flag_if_supported("-std=c++20")
        .compile("cxx-example");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=static=hello");

    println!("cargo:rerun-if-changed=/src/*");
}
