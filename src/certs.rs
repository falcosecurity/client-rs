use crate::{Result, ResultExt};
use std::{fs::File, io::Read, path::Path};

pub fn check_pem_file(path: &Path) -> Result<File> {
    Ok(File::open(path).context(internal_err!("failed to open {}", path.display()))?)
}

pub fn load_pem_file(path: &Path) -> Result<Vec<u8>> {
    let mut file = check_pem_file(path)?;
    let mut key = vec![];
    file.read_to_end(&mut key)
        .context(internal_err!("failed to open {}", path.display()))?;
    Ok(key)
}
