use std::process::Stdio;

fn main() {
    let mut cmd = std::process::Command::new("make");
    cmd.args(["-C", "./cios-lib"]);
    cmd.stderr(Stdio::inherit());

    let out = cmd.output().expect("failed to spawn `make -C ./cios-lib`");
    if !out.status.success() {
        panic!("failed to run `make -C ./cios-lib`");
    }

    println!("cargo:rustc-link-search=native=./cios-lib");
    println!("cargo:rustc-link-lib=static=cios");
}
