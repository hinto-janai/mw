use std::process::*;

fn main() {
    let mut cmd = Command::new("path/to/monero-wallet-cli")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
