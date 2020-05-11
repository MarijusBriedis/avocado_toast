use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    macro_rules! check_install {
        ($command:expr, $args:expr,
         $expected_in_stdout:expr, $error_message:expr) => {{
            // Invoke the command
            Command::new($command)
                .args($args)
                .output()
                .ok()
                .and_then(|res| {
                    // Convert the stdout bytes to a string
                    std::str::from_utf8(&res.stdout).ok().and_then(|stdout| {
                        // Make sure the command returned successfully
                        if !res.status.success() {
                            return None;
                        }

                        // Make sure all expected strings are present in stdout
                        for expected in $expected_in_stdout {
                            if !stdout.contains(expected) {
                                return None;
                            }
                        }

                        Some(())
                    })
                })
                .ok_or($error_message)
        }};
    }

    // Check for NASM
    check_install!(
        "nasm",
        &["-v"],
        &["NASM version"],
        "nasm not present in the path"
    )?;

    // Check for RUST and targets
    // I should add i586-pc-windows-msvc and x86_64-pc-windows-msvc
    check_install!(
        "rustup",
        &["target", "list"],
        &["x86_64-unknown-linux-gnu (installed)"],
        "RUST targets are not installed"
    )?;

    // Check for ld.lld
    check_install!(
        "ld.lld",
        &["--version"],
        &["LLD "],
        "ld.lld not present in the path"
    )?;

    Ok(())
}
