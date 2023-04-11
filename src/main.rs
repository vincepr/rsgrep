use std::{env, fs};

fn main() {
    // reading args
    let args: Vec<String> = env::args().collect();  //-> ["pathToBinary/rsgrep", "searchstring", "file.txt"]
    if args.len() < 2 {
        panic!("rsgrep: Error need 2 args to run rsgrep")
    }
    let search = &args[1];
    let path = &args[2];
    println!("rsgrep: searching for: {}", search);
    println!("rsgrep: in file: {}", path);

    //reading a file:
    let data = fs::read_to_string(path)
        .expect("rsgrep: Error: cant read file at: {path}");
    println!("{data}");
}
