use std::os::unix::prelude::CommandExt;
use std::process::Command;

fn main() {
    std::env::set_var("CARGO_TARGET_DIR", "../front/target");
    
    Command::new("trunk")
        .arg("build")
        .current_dir("../front")
        .exec();
}
