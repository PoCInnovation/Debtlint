use std::fs::{self, ReadDir};
use std::path::PathBuf;

pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
}

const CODE_EXTENSIONS: [&str; 13] = ["py", "js", "rs", "c", "cpp", "go", "java", "php", "lua", "ts", "rb", "jsx", "tsx"];

fn ingest_codebase(dir: ReadDir) -> std::io::Result<()>
{
    for e in dir {
        let entry = e?;
        let file_type = entry.file_type()?.is_dir();
        let file_name = entry.file_name();

        if let Some(name) = file_name.to_str(){
            if let Some(extension) = name.split('.').last(){
                for code_ext in CODE_EXTENSIONS {
                    if extension == code_ext{
                        println!("{}, {}", file_name.to_str().unwrap(), file_type);
                    }
                }
            }
        }

        if file_type && file_name.to_str().unwrap().chars().next().unwrap() != '.' {
            println!("will read directory: {}, {}", file_name.to_str().unwrap(), file_type);
            ingest_codebase(entry.path().read_dir()?)?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {

    let dir = fs::read_dir("./")?;
    ingest_codebase(dir)?;
    Ok(())

}
