use std::{fs, io, path};

pub struct FileCache {
    root: path::PathBuf,
}

impl FileCache {
    pub fn open(root: impl Into<path::PathBuf>) -> io::Result<Self> {
        let root: path::PathBuf = root.into();

        if !root.exists() || root.is_file() {
            fs::create_dir(&root)?;
        }

        Ok(Self { root })
    }

    pub fn get(&self, file: impl Into<path::PathBuf>) -> io::Result<String> {
        let path = self.root.join(file.into());
        fs::read_to_string(path)
    }

    pub fn set(
        &self,
        file: impl Into<path::PathBuf>,
        contents: impl AsRef<[u8]>,
    ) -> io::Result<()> {
        let path = self.root.join(file.into());
        fs::write(path, contents)
    }
}
