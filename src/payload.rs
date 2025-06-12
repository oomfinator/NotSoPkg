use std::{
    error::Error, fs::{create_dir_all, File}, io::{Read, Write}, path::Path
};

#[cfg(target_family = "unix")]
use std::fs::{Permissions, set_permissions};
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use cpio_reader::{Mode, iter_files};
use flate2::read::GzDecoder;

pub struct Payload<'a> {
    path: &'a Path,
}

impl<'a> Payload<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn unpack_into(&self, output: &Path) -> Result<(), Box<dyn Error>> {
        create_dir_all(output)?;
        
        let file = File::open(&self.path)?;
        let mut decoder = GzDecoder::new(file);
        let mut cpio_data = Vec::new();

        decoder.read_to_end(&mut cpio_data)?;

        for entry in iter_files(&cpio_data) {
            let path = output.join(entry.name());
            let mode = entry.mode();
            let content = entry.file();

            if mode.contains(Mode::DIRECTORY) {
                create_dir_all(&path)?;
            } else {
                if let Some(parent) = path.parent() {
                    create_dir_all(parent)?;
                }
                let mut file = File::create(&path)?;
                file.write_all(content)?;
            }
            #[cfg(unix)]
            {
                let perms = Permissions::from_mode(mode.bits() & 0o777);
                set_permissions(&path, perms)?;
            }
        }
        
        Ok(())
    }
}
