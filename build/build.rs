use std::process::Command;

fn convert_icon() {
    let output = Command::new("nwlink")
        .args(&["png-nwi", "assets/icons/icon_nwa.png", "target/icon.nwi"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

// thanks to yannis300307 for build.rs compile_c_libs function
fn compile_storage_c() {
    unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

    let nwlink_flags = String::from_utf8(
        Command::new("nwlink")
            .args(["eadk-cflags"])
            .output()
            .expect("Failed to get nwlink eadk-cflags")
            .stdout,
    )
    .expect("Invalid UTF-8 in nwlink flags");

    let mut build = cc::Build::new();
    build.file("src/eadk/storage/storage.c");
    build.flag("-std=c99");
    build.flag("-Os");
    build.flag("-Wall");
    build.flag("-ggdb");
    build.warnings(false);

    for flag in nwlink_flags.split_whitespace() {
        build.flag(flag);
    }

    build.compile("storage_c");
}

fn main() {
    println!("cargo:rerun-if-changed=assets/icons/icon_nwa.png");
    convert_icon();

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        compile_storage_c();
    }
}
