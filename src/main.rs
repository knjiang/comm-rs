use std::env;
use std::fs::File;

fn comm(args: Vec<String>) -> Result<String, String> {
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

#[cfg(test)]
mod comm_tests {
    use super::*;
    #[test]
    fn test_files_opened() {
        let test_args = vec![
            "comm".to_string(),
            "./testdata/foo.txt".to_string(),
            "./testdata/bar.txt".to_string(),
        ];
        assert_eq!(comm(test_args), Ok("opened 2 files".to_string()));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match comm(args) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("error: {}", err),
    }
}
