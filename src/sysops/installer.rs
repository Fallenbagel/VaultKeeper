use std::{env, path::Path};

use color_eyre::eyre::eyre;

pub fn install_binary() -> Result<(), color_eyre::Report> {
    let binary = env::current_exe()?;

    let path = env::var("PATH")?;

    for path in env::split_paths(&path) {
        let binary_path = Path::new(binary.file_name().unwrap());

        if binary_path.exists() {
            return Err(eyre!("Binary already exists in {}", path.display()));
        }

        if let Err(e) = std::fs::copy(&binary, path.join(binary_path)) {
            return Err(eyre!("Could not copy binary to {}: {}", path.display(), e));
        }
    }
    Ok(())
}
