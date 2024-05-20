use std::{fs, io, path::Path};

use clap::{Parser, Subcommand};
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

#[derive(Subcommand, Debug)]
pub enum Io {
    CopyAllFiles(CopyAllFiles),
    CopyAllFilesToExtFolder(CopyAllFilesToExtFolder),
    MoveAllFilesToExtFolder(MoveAllFilesToExtFolder),
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
            println!("Creating destination directory '{}'", self.dst);
            fs::create_dir_all(&dst)?;
        }

        println!("Copying all files from '{}' to '{}'", self.src, self.dst);

        WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .par_bridge()
            .for_each(|e| {
                println!("Copying file: {:?}", e.path());
                _ = fs::copy(e.path(), dst.join(e.file_name()));
            });

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct CopyAllFilesToExtFolder {
    /// source directory to copy files from.
    #[clap(short, long)]
    src: String,

    /// destination directory to copy files to.
    #[clap(short, long)]
    dst: String,

    /// exclude files with the given extensions separated by comma.
    #[clap(short, long)]
    exclude: Option<String>,
}

impl CopyAllFilesToExtFolder {
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
            println!("Creating destination directory '{}'", self.dst);
            fs::create_dir_all(&dst)?;
        }

        println!("Copying all files from '{}' to '{}'", self.src, self.dst);

        WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .par_bridge()
            .for_each(|e| {
                let ext = e
                    .path()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("");
                if let Some(exclude) = &self.exclude {
                    if exclude.split(',').any(|e| e == ext) {
                        return;
                    }
                }
                let new_dst = dst.join(ext);
                _ = fs::create_dir_all(&new_dst);
                println!("Copying file: {:?} to {:?}", e.path(), new_dst);
                _ = fs::copy(e.path(), new_dst.join(e.file_name()));
            });

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct MoveAllFilesToExtFolder {
    /// source directory to copy files from.
    #[clap(short, long)]
    src: String,

    /// destination directory to copy files to.
    #[clap(short, long)]
    dst: String,

    /// exclude files with the given extensions separated by comma.
    #[clap(short, long)]
    exclude: Option<String>,
}

impl MoveAllFilesToExtFolder {
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
            println!("Creating destination directory '{}'", self.dst);
            fs::create_dir_all(&dst)?;
        }

        println!("Moving all files from '{}' to '{}'", self.src, self.dst);

        WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .par_bridge()
            .for_each(|e| {
                let ext = e
                    .path()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("");
                if let Some(exclude) = &self.exclude {
                    if exclude.split(',').any(|e| e == ext) {
                        return;
                    }
                }
                let new_dst = dst.join(ext);
                _ = fs::create_dir_all(&new_dst);
                println!("Moving file: {:?} to {:?}", e.path(), new_dst);
                _ = fs::rename(e.path(), new_dst.join(e.file_name()));
            });

        Ok(())
    }
}
