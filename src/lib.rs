use std::{fs,io};

/// main entrypoint of the binary
pub fn run(cfg: &Config) -> Result<(), io::Error >{
    // read the file:
    let data = fs::read_to_string(&cfg.path)?;

    // get lines that include our search-string
    let lines = if cfg.case_insensitive {
        search_str_case_insensitive(&cfg.query, &data)
    } else {
        search_str(&cfg.query, &data)
    };

    // print out the lines
    for line in lines {
        println!("{line}")
    }
    Ok(())
}

/// searches a string for a substring. Returns array of lines that include substing.
pub fn search_str<'a>(substr: &str, data: &'a str) -> Vec<&'a str> {
    let mut found_lines:Vec<&str> = vec![];
    for line in data.lines() {
        if line.contains(substr) {
            found_lines.push(line);
        }
    }
    found_lines
}

/// like seach_str() but case insensitive
pub fn search_str_case_insensitive<'a>(substr: &str, data: &'a str) -> Vec<&'a str> {
    let substr = substr.to_lowercase();
    let mut found_lines = Vec::new();
    for line in data.lines() {
        if line.to_lowercase().contains(&substr) {
            found_lines.push(line);
        }
    }
    found_lines
}


/// Config that holds Arg-params and ENV params
pub struct Config {
    query: String,
    path: String,
    case_insensitive: bool,
}
impl Config {
    /// constructor for the Config struct
    pub fn init() -> Result<Config, &'static str> {
        // read the Terminal Args. (args[0] is the binary that is ran)
        use std::env;
        let args: Vec<String> = env::args().collect();  //-> ["pathToBinary/rsgrep", "Searchstring", "file.txt"]
        if args.len() != 3 {
            return Err("need 2 args to run rsgrep successfully")
        }
        let (query, path) = Config::parse_args(&args);

        // in this case we just check if there is any ENV of the name:
        let ignore_case = env::var("CASE_INSENSITIVE").is_ok();

        // return our config
        Ok(Config{
            query: query.to_string(),
            path: path.to_string(),
            case_insensitive: ignore_case,
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
    fn case_sensitive() {
        let data = "\
Lorem Blah:
ipsum, blah lbah.
Lastline three.";
        let query = "blah";
        assert_eq!(vec!["ipsum, blah lbah."], search_str(query, data));
    }

    #[test]
    fn case_insensitive(){
        let data = "\
Lorem Blah:
ipsum, blah lbah.
Lastline three.";
        let query = "BlAh";
        assert_eq!(
            vec!["Lorem Blah:","ipsum, blah lbah."], 
            search_str_case_insensitive(query, data)
        );

    }
}