fn main() {
    
     let mut args: std::collections::VecDeque<String> = std::env::args().collect();
     args.pop_front();
     let env_vars: Vec<_> = std::env::vars().collect();
     let mut result: Vec<_> = Vec::new();
     let colored_text_beginning = "\x1b[92m";
     let colored_text_end = "\x1b[0m";

     for arg in &args {

        for env_var in &env_vars {
            if env_var.0.contains(arg) {
                result.push(env_var);
            }
        }

        if result.len() > 0 {
            result.sort();
            for result_env_arg in &result {
                if !result_env_arg.1.contains(":") {
                    println!("{} = {}{}{}", result_env_arg.0, colored_text_beginning, result_env_arg.1, colored_text_end);
                }
                else {
                    let env_var_values: Vec<_> = result_env_arg.1.split(':').collect();
                    println!("{} =", result_env_arg.0);
                    
                    for value in env_var_values {
                        println!("\t{}{}{}", colored_text_beginning, value, colored_text_end);
                    }
                }
             }
        }
        else {
            println!("{} = {}{}{}", arg, colored_text_beginning, "NONE", colored_text_end);
        }

        result.clear();
        println!();

     }

}