use crate::Result;
use std::{fs::File, io::Read, path::Path};

pub fn check_pem_file(path: &Path) -> Result<File> {
    File::open(path).map_err(|e| internal_err!("failed to open {}: {:?}", path.display(), e))
}

pub fn load_pem_file(path: &Path) -> Result<Vec<u8>> {
    let mut file = check_pem_file(path)?;
    let mut key = vec![];
    file.read_to_end(&mut key)
        .map_err(|e| internal_err!("failed to open {}: {:?}", path.display(), e))
        .map(|_| key)
}
