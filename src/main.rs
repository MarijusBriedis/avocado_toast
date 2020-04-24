use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    {
        // Check if NASM is installed and is in the path
        let valio = Command::new("nasm")
            .arg("-v")
            .output()
            .ok()
            .and_then(|res| {
                std::str::from_utf8(&res.stdout).ok().and_then(|stdout| {
                    if res.status.success() && stdout.contains("NASM version") {
                        Some(())
                    } else {
                        None
                    }
                })
            })
            .ok_or("nasm is not installed or in the path")?;
    }

    Ok(())
}
