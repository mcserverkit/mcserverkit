fn main() {
    println!("Hello World!");

    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    let _output = if cfg!(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos"
    )) {
    } else {
        println!("Operating system not supported");
        return;
    };
}
