use std::env;
use std::fs::File;

fn comm() -> Result<String,String>{
    let args : Vec<String> = env::args().collect();
    if args.len() != 3{
        return Err("comm needs two arguments to compare files!".to_string())
    }
    let filename1 : &String = &args[1];
    let filename2 : &String = &args[1];
    
    let _file1 = File::open(filename1).expect("unable to open first file");
    let _file2 = File::open(filename2).expect("unable to open second file");
    return Ok("Hello, World".to_string());
}

fn main(){
    match comm(){
        Ok(result) => println!("{}", result),
        Err(err) => println!("error: {}",err),
    }
}