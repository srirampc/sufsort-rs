//extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

// macro_rules! t {
//     ($e:expr) => (match $e {
//         Ok(n) => n,
//         Err(e) => fail(&format!("\n{} failed with {}\n", stringify!($e), e)),
//     })
// }

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn main() {
    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let _ = fs::create_dir(&dst.join("build"));

    //let mut cmd = Command::new("cmake");
    run(Command::new("cmake")
             .arg(&src.join("libdivsufsort"))
             .current_dir(&dst.join("build")), "cmake");

    run(Command::new("cmake")
            .arg("--build").arg(".")
            .current_dir(&dst.join("build")), "cmake");

    println!("cargo:root={}", dst.display());
    println!("cargo:rustc-flags=-l static=divsufsort");
    println!("cargo:rustc-flags=-l static=divsufsort64");
    if cfg!(windows) {
        println!("cargo:rustc-flags=-L {}", dst.join("build").join("lib").join("Debug").display());
    } else {
        println!("cargo:rustc-flags=-L {}", dst.join("build").join("lib").display());
    }
}
