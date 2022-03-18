fn main(){
    let env_args: Vec<_> = std::env::vars().collect();
    let args: Vec<String> = std::env::args().collect();

    env_args.iter().for_each(|arg| print!("{}=\x1b[92m{}\x1b[0m ", arg.0, arg.1)); 
    
    println!();
    println!();

    args.iter().for_each(|arg| print!("{} ", arg));

    println!();
}