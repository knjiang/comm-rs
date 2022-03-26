use std::env;

fn main(){
    let mut args = Vec::new()
    args = env::args().collect()
    
    let mut file1 = String::new();

    io::stdin().read_line(&mut input)?;
    println!("Hello, {}!", input.trim());
    Ok(())
}