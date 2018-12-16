use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let st = Command::new("flatc")
        .args(&["-r", "-o", &out_dir])
        .args(&["-b", "fbs/addressbook.fbs"])
        .status()
        .expect("flatc command failed");
    if !st.success() {
        panic!("flatc failed: {}", st.code().unwrap());
    }
}
