use std::process::Command;

pub fn term_size() -> Option<(u16, u16)> {
    if cfg!(windows) {
        // Use `mode` command on Windows
        let output = Command::new("mode")
            .args(&["con", "|", "find", "/I", "columns"])
            .output()
            .ok()?;
        if !output.status.success() {
            eprintln!("Error running `mode` command: {:?}", output);
            return None;
        }
        let output_str = String::from_utf8_lossy(&output.stdout);
        let width = output_str
            .split_whitespace()
            .skip(1)
            .next()?
            .parse::<u16>()
            .ok()?;
        let output = Command::new("mode")
            .args(&["con", "|", "find", "/I", "lines"])
            .output()
            .ok()?;
        if !output.status.success() {
            eprintln!("Error running `mode` command: {:?}", output);
            return None;
        }
        let output_str = String::from_utf8_lossy(&output.stdout);
        let height = output_str
            .split_whitespace()
            .skip(1)
            .next()?
            .parse::<u16>()
            .ok()?;
        Some((width, height))
    } else {
        // Use `tput` command on other systems
        let output = Command::new("sh")
            .arg("-c")
            .arg("tput cols")
            .output()
            .ok()?;
        if !output.status.success() {
            eprintln!("Error running `tput` command: {:?}", output);
            return None;
        }
        let width = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u16>()
            .ok()?;
        let output = Command::new("sh")
            .arg("-c")
            .arg("tput lines")
            .output()
            .ok()?;
        if !output.status.success() {
            eprintln!("Error running `tput` command: {:?}", output);
            return None;
        }
        let height = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u16>()
            .ok()?;
        Some((width, height))
    }
}
