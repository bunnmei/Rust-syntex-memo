use std::fs::DirBuilder;
use std::env;
use std::path::{PathBuf, Path};

pub fn test() {
  println!("test ifled");
}

pub fn ifled() {
  let mut builder: DirBuilder = DirBuilder::new();
  let current_path: Result<PathBuf, std::io::Error> = env::current_dir();

  let path: &str = "hoge/abc";
  
  if let Ok(mut c_path) = current_path {
    println!("{}", c_path.display());
    c_path.push(path);
    println!("{}", c_path.display());
    let r_path: Result<(), std::io::Error> = builder.recursive(true).create(c_path);

    match r_path {
        Ok(()) => println!("folderが作成されました"),
        Err(_) => println!("folderの作成に失敗")
    };
  }
}