use {
    std::{
        env,
        io,
        fs,
        path::Path
    },
    winresource::WindowsResource,
};


fn main() -> io::Result<()> {
    if Ok("debug".to_owned()) == env::var("PROFILE") {
        return Ok(());
    }

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()?;
    }

    let s = Path::new("target/debug/turing-machine-simulator.exe");
    let d = Path::new("target/debug/Turing Machine Simulator!.exe");
    if s.exists(){
        fs::rename(s, d).expect("failed to rename executable");
    }
    let s = Path::new("target/release/turing-machine-simulator.exe");
    let d = Path::new("target/release/Turing Machine Simulator!.exe");
    if s.exists(){
        fs::rename(s, d).expect("failed to rename executable");
    }
    Ok(())
}