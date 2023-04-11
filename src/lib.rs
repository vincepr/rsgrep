use std::{fs,io};

// main entrypoint of the binary
pub fn run(cfg: &Config) -> Result<(), io::Error >{
    // reading the file:
    let data = fs::read_to_string(&cfg.path)?;
        

    let lines = search_str(&cfg.query, &data);
    for line in lines {
        println!("{line}")
    }
    Ok(())
}

// searches a string for a substring. Returns array of lines that include substing.
pub fn search_str<'a>(substr: &str, data: &'a str) -> Vec<&'a str> {
    let mut found_lines:Vec<&str> = vec![];
    for line in data.lines() {
        if line.contains(substr) {
            found_lines.push(line);
        }
    }
    found_lines
}

// Config that holds Args params as Strings
pub struct Config {
    query: String,
    path: String,
}
impl Config {
    pub fn init() -> Result<Config, &'static str> {
        use std::env;
        let args: Vec<String> = env::args().collect();  //-> ["pathToBinary/rsgrep", "Searchstring", "file.txt"]
        if args.len() != 3 {
            return Err("need 2 args to run rsgrep successfully")
        }
        let (query, path) = Config::parse_args(&args);
        Ok(Config{
            query: query.to_string(),
            path: path.to_string(),
        })
    }
    fn parse_args(args: &Vec<String>) -> (&str, &str){
        let query = &args[1];
        let path = &args[2];
        (query, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "blah";
        let data = "\
Lorem:
ipsum, blah lbah.
Lastline three.";
        assert_eq!(vec!["ipsum, blah lbah."], search_str(query, data));
    }
}