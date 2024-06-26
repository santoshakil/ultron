use clap::Parser;
use ultron::{args::Args, file::file::Io};

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    match args.io {
        Io::CopyAllFiles(copy_all_files) => {
            copy_all_files.execute().unwrap();
        }
        Io::CopyAllFilesToExtFolder(copy_all_files_to_ext_folder) => {
            copy_all_files_to_ext_folder.execute().unwrap();
        }
        Io::MoveAllFilesToExtFolder(move_all_files_to_ext_folder) => {
            move_all_files_to_ext_folder.execute().unwrap();
        }
    }
    println!("Finished!");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
