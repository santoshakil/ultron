use std::{fs, io, path::Path};

use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Subcommand, Debug)]
pub enum Io {
    CopyAllFiles(CopyAllFiles),
}

#[derive(Parser, Debug)]
pub struct CopyAllFiles {
    /// source directory to copy files from.
    #[clap(short, long)]
    src: String,

    /// destination directory to copy files to.
    #[clap(short, long)]
    dst: String,
}

impl CopyAllFiles {
    pub fn execute(&self) -> Result<(), io::Error> {
        let src = Path::new(&self.src);
        let dst = Path::new(&self.dst);

        if !src.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("src directory '{}' does not exist", self.src),
            ));
        }

        if !dst.exists() {
            fs::create_dir_all(&dst)?;
        }

        WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let src = e.path();
                let dst = dst.join(src.strip_prefix(src).unwrap());
                _ = fs::copy(src, dst);
            });

        Ok(())
    }
}
