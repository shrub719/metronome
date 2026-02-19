use std::process::Command;
use std::fs;
use std::path::Path;
use std::io::Write;

fn convert_icon() {
    let output = Command::new("nwlink")
        .args(&["png-nwi", "assets/icons/icon_nwa.png", "target/icon.nwi"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

fn convert_map(input: &str, output: &str) {
    let txt = fs::read_to_string(input).unwrap();
    let mut bin = fs::File::create(output).unwrap();
    
    for line in txt.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue
        }

        let mut parts = line.split_whitespace();
        let class = parts.next().unwrap();
        let ms: u32 = parts.next().unwrap().parse().unwrap();
        let x: f32 = parts.next().unwrap().parse().unwrap();
        
        match class {
            "t" => {
                bin.write_all(b"t").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&x.to_le_bytes()).unwrap();
                bin.write_all(b"beef").unwrap();

            },
            "h" => {
                let ms_end: u32 = parts.next().unwrap().parse().unwrap();

                bin.write_all(b"h").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&x.to_le_bytes()).unwrap();
                bin.write_all(&ms_end.to_le_bytes()).unwrap();
            },
            _ => ()
        };
    }
}

fn convert_maps() {
    let out_dir = "target/maps";
    let in_dir = "assets/maps";
    
    fs::create_dir_all(out_dir).unwrap();

    for entry in fs::read_dir(in_dir).unwrap() {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            // println!("cargo:rerun-if-changed={}", path.display());

            let filename = path.file_stem().unwrap().to_str().unwrap();
            let output = Path::new(&out_dir).join(format!("{filename}.bin"));

            convert_map(path.to_str().unwrap(), output.to_str().unwrap());
        }
    }
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

    println!("cargo:rerun-if-changed=NULL");    // hack
    convert_maps();

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        compile_storage_c();
    }
}
