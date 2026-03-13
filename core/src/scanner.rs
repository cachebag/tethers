use std::path::Path;

pub fn find_broken_symlinks(path: &Path) -> Vec<String> {
    let output = std::process::Command::new("find")
        .arg(path)
        .args(["-type", "l"])
        .args(["!", "-exec", "test", "-e", "{}", ";", "-print"])
        .output()
        .expect("failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().map(|s| s.to_string()).collect()
}
