use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_broken_symlinks(path: &Path) -> Vec<BrokenSymlink> {
    let output = std::process::Command::new("find")
        .arg(path)
        .args(["-type", "l"])
        .args(["!", "-exec", "test", "-e", "{}", ";", "-print"])
        .output()
        .expect("failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .filter_map(|line| {
            let link = PathBuf::from(line);
            let target = fs::read_link(&link).ok()?;
            Some(BrokenSymlink { link, target })
        })
        .collect()
}

pub struct BrokenSymlink {
    pub link: PathBuf,
    pub target: PathBuf,
}

impl fmt::Display for BrokenSymlink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.link.display(), self.target.display())
    }
}
