use std::fs::Metadata;
use std::io::Error;


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
        let paths: Vec<String> = self.collect_paths()?;
        Self::write_output(paths);
        Ok(())
    }

    fn collect_paths(&self, ) -> Result<Vec<String>, Error> {
        let current_dir = String::from("./");
        let mut paths: Vec<String> = Vec::new();

        if self.config.args.contains(&String::from("-a"))
            && self.config.args.len() == 2
        {
            return Ok(Self::read_everything(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-d"))
            && self.config.args.len() == 2
        {
            return Ok(Self::read_directories(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-f"))
            && self.config.args.len() == 2
        {
            return Ok(Self::read_files(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-h"))
            && self.config.args.len() == 2
        {
            return  Ok(Self::read_hidden(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-d"))
            && self.config.args.contains(&String::from("-h"))
            && self.config.args.len() == 3
        {
            return Ok(Self::read_hidden_directories(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-d"))
            && self.config.args.contains(&String::from("--with-hidden"))
            && self.config.args.len() == 3
        {
            return Ok(Self::read_directories_with_hidden(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-f"))
            && self.config.args.contains(&String::from("--with-hidden"))
            && self.config.args.len() == 3
        {
            return Ok(Self::read_files_with_hidden(&current_dir)?)
        }

        if self.config.args.contains(&String::from("-f"))
            && self.config.args.contains(&String::from("-h"))
            && self.config.args.len() == 3
        {
            return Ok(Self::read_hidden_files(&current_dir)?)
        }

        paths.append(&mut Self::read_directories(&current_dir)?);
        paths.append(&mut Self::read_files(&current_dir)?);

        Ok(paths)
    }

    fn write_output(paths: Vec<String>) -> () {
        for path in paths {
            println!("{path}")
        }
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

    fn read_all_directories(path: &String) -> Result<Vec<String>, Error> {
        let mut directories: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata = Self::read_metadata(&path)?;
            let clean_name: String = String::from(&path[2..]);
            if metadata.is_dir() {
                directories.push(clean_name);
            }
        }

        Ok(directories)
    }

    fn read_directories(path: &String) -> Result<Vec<String>, Error> {
        let paths = Self::read_all_directories(path)?;

        Ok(
            paths.into_iter()
                .filter(|path| !path.starts_with("."))
                .map(|path| format!("{}/{}{}", "\x1b[38;5;25m", path, "\x1b[0m"))
                .collect()
        )
    }

    fn read_directories_with_hidden(path: &String) -> Result<Vec<String>, Error> {
        let paths = Self::read_all_directories(path)?;
        Ok(
            paths.into_iter()
                .map(|path| format!("{}/{}{}", "\x1b[38;5;25m", path, "\x1b[0m"))
                .collect()
        )
    }

    fn read_all_files(path: &String) -> Result<Vec<String>, Error> {
        let mut files: Vec<String> = Vec::new();
        let paths = Self::read_everything(path)?;
        for path in paths {
            let metadata = Self::read_metadata(&path)?;
            let clean_name: String = String::from(&path[2..]);
            if metadata.is_file() {
                files.push(clean_name);
            }
        }

        Ok(files)
    }

    fn read_files(path: &String) -> Result<Vec<String>, Error> {
        let paths = Self::read_all_files(path)?;
        Ok(
            paths.into_iter()
                .filter(|path| !path.starts_with("."))
                .map(|path| format!("{}{}{}", "\x1b[38;5;210m", path, "\x1b[0m"))
                .collect()
        )
    }

    fn read_files_with_hidden(path: &String) -> Result<Vec<String>, Error> {
        let paths = Self::read_all_files(path)?;
        Ok(
            paths.into_iter()
                .map(|path| format!("{}{}{}", "\x1b[38;5;210m", path, "\x1b[0m"))
                .collect()
        )
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn directories() {
        let path: String = String::from("./");
        let mut expected: Vec<String> = vec!(String::from("./src"), String::from("./target"));
        let mut result: Vec<String> = App::read_directories(&path).unwrap();
        expected.sort();
        result.sort();
        assert_eq!(expected, result)
    }

    #[test]
    fn directories_with_hidden() {
        let path: String = String::from("./");
        let mut expected: Vec<String> = vec!(
            String::from("./.git"),
            String::from("./.idea"),
            String::from("./src"),
            String::from("./target")
        );
        let mut result: Vec<String> = App::read_directories_with_hidden(&path).unwrap();
        expected.sort();
        result.sort();
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
        let mut expected: Vec<String> = vec!(String::from("./Cargo.lock"), String::from("./Cargo.toml"));
        let mut result: Vec<String> = App::read_files(&path).unwrap();
        expected.sort();
        result.sort();
        assert_eq!(expected, result)
    }

    #[test]
    fn files_with_hidden() {
        let path: String = String::from("./");
        let mut expected: Vec<String> = vec!(
            String::from("./.gitignore"),
            String::from("./Cargo.lock"),
            String::from("./Cargo.toml")
        );
        let mut result: Vec<String> = App::read_files_with_hidden(&path).unwrap();
        expected.sort();
        result.sort();
        assert_eq!(expected, result)
    }

    #[test]
    fn everything_hidden() {
        let path: String = String::from("./");
        let mut expected: Vec<String> = vec!(
            String::from("./.git"),
            String::from("./.gitignore"),
            String::from("./.idea")
        );
        let mut result: Vec<String> = App::read_hidden(&path).unwrap();
        expected.sort();
        result.sort();
        assert_eq!(expected, result)
    }

    #[test]
    fn hidden_directories() {
        let path: String = String::from("./");
        let mut expected: Vec<String> = vec!(
            String::from("./.git"),
            String::from("./.idea")
        );
        let mut result: Vec<String> = App::read_hidden_directories(&path).unwrap();
        expected.sort();
        result.sort();
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