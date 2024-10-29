use std::fs::Metadata;
use std::io::Error;


fn get_type(path: &String) -> () {
    let metadata = std::fs::metadata(&path).unwrap();
    let is_dir = metadata.is_dir();
    let is_file = metadata.is_file();
    println!("{:?}", metadata);
    println!("Is directory: {:?}", is_dir);
    println!("Is file: {:?}", is_file);
}


struct Config {
    args: Vec<String>
}

pub struct App {
    config: Config
}

// TODO: implement struct that describes each path like { is_dir: bool, is_file: bool, size: usize, ... }

impl App {
    pub fn build() -> App {
        let config = Config::new();
        App { config }
    }

    pub fn run(&self) -> Result<(), Error> {
        let current_dir = String::from("./");
        let paths = Self::read_files_with_hidden(&current_dir)?;
        for path in paths {
            println!("Path: {}", path)
        }
        Ok(())
    }

    fn read_everything(path: &String) -> Result<Vec<String>, Error> {
        let mut results: Vec<String> = Vec::new();
        let paths = std::fs::read_dir(path)?;
        for path in paths {
            let path_str: String = path?
                .path()
                .to_str()
                .unwrap()
                .to_string();
            results.push(path_str);
        }
        Ok(results)
    }

    fn read_directories_with_hidden(path: &String) -> Result<Vec<String>, Error> {
        let mut directories: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata = Self::read_metadata(&path)?;
            if metadata.is_dir() {
                println!("{:?}", metadata.file_type());
                directories.push(path)
            }
        }

        Ok(directories)
    }

    fn read_directories(path: &String) -> Result<Vec<String>, Error> {
        let mut directories: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata = Self::read_metadata(&path)?;
            let clean_name: String = String::from(&path[2..]);
            if metadata.is_dir() && !clean_name.starts_with(".") {
                directories.push(path);
            }
        }

        Ok(directories)
    }

    fn read_files_with_hidden(path: &String) -> Result<Vec<String>, Error> {
        let mut files: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata = Self::read_metadata(&path)?;
            if metadata.is_file(){
                files.push(path);
            }
        }

        Ok(files)
    }

    fn read_files(path: &String) -> Result<Vec<String>, Error> {
        let mut files: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata = Self::read_metadata(&path)?;
            let clean_name: String = String::from(&path[2..]);
            if metadata.is_file() && !clean_name.starts_with(".") {
                files.push(path);
            }
        }

        Ok(files)
    }

    fn read_hidden(path: &String) -> Result<Vec<String>, Error> {
        let mut hidden: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let clean_name = String::from(&path[2..]);
            if clean_name.starts_with(".") {
                hidden.push(path)
            }
        }

        Ok(hidden)
    }

    fn read_hidden_directories(path: &String) -> Result<Vec<String>, Error> {
        let mut hidden_directories: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata: Metadata = Self::read_metadata(&path)?;
            let clean_name = String::from(&path[2..]);
            if clean_name.starts_with(".") && metadata.is_dir(){
                hidden_directories.push(path)
            }
        }

        Ok(hidden_directories)
    }

    fn read_hidden_files(path: &String) -> Result<Vec<String>, Error> {
        let mut hidden_files: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata: Metadata = Self::read_metadata(&path)?;
            let clean_name = String::from(&path[2..]);
            if clean_name.starts_with(".") && metadata.is_file(){
                hidden_files.push(path)
            }
        }

        Ok(hidden_files)
    }

    fn read_metadata(path: &String) -> Result<std::fs::Metadata, Error> {
        let metadata = std::fs::metadata(path).unwrap();
        Ok(metadata)
    }
}


impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = std::env::args().collect();
        Config { args }
    }
}

mod test {
    use super::*;

    #[test]
    fn directories() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(String::from("./src"), String::from("./target"));
        let result: Vec<String> = App::read_directories(&path).unwrap();
        assert_eq!(expected, result)
    }

    #[test]
    fn directories_with_hidden() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(
            String::from("./.git"),
            String::from("./.idea"),
            String::from("./src"),
            String::from("./target")
        );
        let result: Vec<String> = App::read_directories_with_hidden(&path).unwrap();
        assert_eq!(expected, result)
    }

    #[test]
    fn directories_fail() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(String::from("./src"));
        let result: Vec<String> = App::read_directories(&path).unwrap();
        assert_ne!(expected, result)
    }

    #[test]
    fn files() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(String::from("./Cargo.lock"), String::from("./Cargo.toml"));
        let result: Vec<String> = App::read_files(&path).unwrap();
        assert_eq!(expected, result)
    }

    #[test]
    fn files_with_hidden() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(
            String::from("./.gitignore"),
            String::from("./Cargo.lock"),
            String::from("./Cargo.toml")
        );
        let result: Vec<String> = App::read_files_with_hidden(&path).unwrap();
        assert_eq!(expected, result)
    }

    #[test]
    fn everything_hidden() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(
            String::from("./.git"),
            String::from("./.gitignore"),
            String::from("./.idea")
        );
        let result: Vec<String> = App::read_hidden(&path).unwrap();
        assert_eq!(expected, result)
    }

    #[test]
    fn hidden_directories() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(
            String::from("./.git"),
            String::from("./.idea")
        );
        let result: Vec<String> = App::read_hidden_directories(&path).unwrap();
        assert_eq!(expected, result)
    }

    #[test]
    fn hidden_files() {
        let path: String = String::from("./");
        let expected: Vec<String> = vec!(
            String::from("./.gitignore")
        );
        let result: Vec<String> = App::read_hidden_files(&path).unwrap();
        assert_eq!(expected, result)
    }
}