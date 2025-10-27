use std::process::Command;

fn convert_icon() {
    let output = Command::new("nwlink")
        .args(&["png-nwi", "assets/icon.png", "target/icon.nwi"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.png");
    convert_icon();
}
