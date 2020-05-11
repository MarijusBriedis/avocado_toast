use std::path::Path;
use std::error::Error;
use std::process::Command;

fn check_install(command: &str, args: &[&str], expected: &[&str]) -> Option<()> {
    // Invoke the command
    let res = Command::new(command).args(args).output().ok()?;

    // Check if command was successful
    if !res.status.success() {
        return None;
    }

    // Convert the stdout bytes to a string
    let stdout = std::str::from_utf8(&res.stdout).ok()?;

    // Mak sure `stdout` contains everything else
    if expected.iter().all(|x| stdout.contains(x)) {
        Some(())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Check for NASM
    check_install("nasm", &["-v"], &["NASM version"]).ok_or("nasm not present in the path")?;

    // Check for RUST and targets
    // I should add i586-pc-windows-msvc and x86_64-pc-windows-msvc
    check_install(
        "rustup",
        &["target", "list"],
        &["x86_64-unknown-linux-gnu (installed)"],
    )
    .ok_or("RUST targets are not installed")?;

    // Check for ld.lld
    check_install("ld.lld", &["--version"], &["LLD "]).ok_or("ld.lld not present in the path")?;

    // Build the stage0
    let stage0 = Path::new("bootloader").join("src").join("stage0.asm");
    println!("{:?}\n", stage0);
    Ok(())
}
