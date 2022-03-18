use std::fs;
use std::path::Path;

fn show_entry(title: &str, option_s: bool, path: &Path) -> std::io::Result<()>{
    let last_modified = fs::metadata(&path)?.modified()?.elapsed().unwrap().as_secs();

    if title == "File" {
        print!("\x1b[92m{}\x1b[0m", title);
    }
    else{
        print!("{}", title);
    }

    if option_s {
        print!(": {} \x1b[93m{}B\x1b[0m \x1b[94m{}s\x1b[0m\n", path.display(), fs::metadata(&path)?.len(), last_modified);
    }
    else {
        print!(": {} \x1b[94m{}s\x1b[0m\n", path.display(), last_modified);
    }
    Ok(())
}

fn show_paths(direction: &Path, option_r: bool, option_d: bool, option_s: bool, option_sort: &str) -> std::io::Result<()>{

    if direction.is_dir(){

        let mut paths: Vec<_> = fs::read_dir(direction).unwrap().map(|r| r.unwrap()).collect();

        if option_sort == "date" {
            paths.sort_by_key(|dir| fs::metadata(dir.path()).unwrap().modified().unwrap().elapsed().unwrap());
        }
        else if option_sort == "alpha" {
            paths.sort_by_key(|dir| dir.path());
        }

        for entry in paths {

            let path = entry.path();

            if path.is_dir() {  
                show_entry("Folder", false, &path)?;
                
                if option_r {
                    show_paths(&path, option_r, option_d, option_s, option_sort)?;
                }
                
            }else if !option_d{
                show_entry("File", option_s, &path)?;
            }
        }
        println!();
    }
    Ok(())
}


fn main(){

    let mut args: std::collections::VecDeque<String> = std::env::args().collect();
    args.pop_front();

    let mut is_r = false;
    let mut is_d = false;
    let mut is_s = false;
    let mut is_sort : &str= "none";

    for argument in args {
        match argument.as_str(){
            "-R" =>  is_r = true,
            "-d" =>  is_d = true,
            "-s" =>  is_s = true,
            "--sort=alpha" =>  is_sort = "alpha",
            "--sort=date" =>  is_sort = "date",
            _ => {
                println!("\x1b[91m{}\x1b[0m", 
            "Wrong option; avaible options:
            -R -> recursive show;
            -d -> only folders; 
            -s -> show size of files in bytes;
            --sort=alpha | --sort=date -> sort by name, date of modification"); 
            std::process::exit(1)
        },
        }
    }

    let res = show_paths(Path::new("./"), is_r, is_d, is_s, &is_sort);

    if let Err(err) = res {
        println!("Error: {}", err);
    }
    
}