use std::process::Command;

fn main() {
    println!("Hello World!");

    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    let _output = if cfg!(target_os = "windows") {
    } else if cfg!(target_os = "linux") {
    } else if cfg!(target_os = "macos") {
    } else {
        println!("Operating system not supported");
        return;
    };
}
