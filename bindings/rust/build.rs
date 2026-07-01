use std::process::Command;

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

    let curl = if cfg!(target_os = "windows") {
        "curl.exe"
    } else {
        "curl"
    };

    // https://doc.rust-lang.org/std/process/struct.Command.html
    let _output = Command::new(curl)
        .args(["-LO", &url])
        .output()
        .expect("failed to execute process");

    if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args([
                "-Command",
                &format!("Expand-Archive -Path '{asset}' -DestinationPath . -Force"),
            ])
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("unzip")
            .args([&asset, "-o"])
            .output()
            .expect("failed to execute process");
    }

    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=mcserverkit");
}
