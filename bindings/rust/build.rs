fn main() {
    let architecture = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        return;
    };
    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        println!("Operating system not supported");
        return;
    };

    let asset = format!("mcserverkit.{os}-{architecture}.zip");
    let url =
        format!("https://github.com/mcserverkit/mcserverkit/releases/latest/download/{asset}");

    println!("{url}");
}
