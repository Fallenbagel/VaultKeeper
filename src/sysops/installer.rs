use std::{env, path::Path};

use color_eyre::eyre::eyre;

pub fn install_binary() -> Result<(), color_eyre::Report> {
    let binary = env::current_exe()?;

    let path = Path::new("/usr/local/bin");

    let binary_path = path.join(binary.file_name().unwrap());

    println!("{:#?}", binary_path);

    if binary_path.exists() {
        return Err(eyre!("Binary already exists in {}", binary_path.display()));
    }

    if let Err(e) = std::fs::copy(&binary, path.join(binary_path)) {
        return Err(eyre!("Could not copy binary to {}: {}", path.display(), e));
    }

    Ok(())
}
