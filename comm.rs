use std::env;

fn comm() -> Result<String,String>{
    let args : Vec<_> = env::args().collect();
    if args.len() != 3{
        return Err("comm needs two arguments to compare files!".to_string())
    }
    return Ok("Hello, World".to_string());
}

fn main(){
    match comm(){
        Ok(result) => println!("{}", result),
        Err(err) => println!("error: {}",err),
    }
}