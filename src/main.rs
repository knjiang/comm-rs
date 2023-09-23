use std::env;
use std::fs::File;

fn comm() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("comm needs at least two arguments to compare files!".to_string());
    }

    let files: usize = args.len();
    let mut files_opened = 0;

    for i in 1..files {
        let filename: &String = &args[i];
        let file = File::open(filename);
        match file {
            Ok(_) => {
                files_opened += 1;
            }
            Err(error) => {
                println!("error opening file {}: {}", filename, error)
            }
        }
    }

    return Ok(format!("opened {} files", files_opened));
}

fn main() {
    match comm() {
        Ok(result) => println!("{}", result),
        Err(err) => println!("error: {}", err),
    }
}
