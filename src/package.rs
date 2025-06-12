use std::{error::Error, fs::{create_dir_all, File}, path::Path};
use apple_xar::reader::XarReader;

pub struct Package<'a> {
    path: &'a Path,
}

impl<'a> Package<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
    
    pub fn unpack_into(&self, output: &Path) -> Result<(), Box<dyn Error>> {
        create_dir_all(output)?;
        let mut reader = XarReader::new(File::open(self.path)?)?;
        reader.unpack(output)?;
        Ok(())
    }
}
