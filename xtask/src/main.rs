use std::{os::unix::process::CommandExt, process::Command};

fn main() {
    let mut args = std::env::args();

    if args.len() != 2 {
        eprintln!("Sorry, you can only do tui or rtui, for debug and release mode respectivly");
        return;
    }

    let arg = args.nth(1).unwrap();

    let mut cmd = Command::new("cargo");
    match arg.as_str() {
        "rtui" | "rratadraw-tui" => {
                cmd.args(["run","--bin","ratadraw-tui","--release"])
        },
        "tui" | "ratadraw-tui" => {
                cmd.args(["run","--bin","ratadraw-tui"])
        },
        _ => {
            panic!("Can only pass rtui, or tui")
        }
    };

    cmd.exec();
}
