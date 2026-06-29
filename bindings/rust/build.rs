fn main() {
    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    let _filename = if cfg!(target_os = "windows") {
        "mcserverkit.windows-x86_64.zip";
    } else if cfg!(target_os = "linux") {
        "mcserverkit.linux-x86_64.zip";
    } else if cfg!(target_os = "macos") {
        "mcserverkit.macos-x86_64.zip";
    } else {
        println!("Operating system not supported");
        return;
    };
}
