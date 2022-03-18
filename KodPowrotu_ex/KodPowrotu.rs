use std::io;


fn main() {
    let mut args: std::collections::VecDeque<String> = std::env::args().collect();
    args.pop_front();

    let args: Vec<String> = args.iter().map(|elem| elem.to_lowercase()).collect();

    let mut text = String::new();

    match io::stdin().read_line(&mut text){
        Err(err) => panic!("Error while reading line: {}", err),
        Ok(_) => (),
    }

    let mut chars = text.chars();
    chars.next_back();
    let text = chars.as_str();

    let text: Vec<_> = text.split([',', ';', ' ', '.'].as_ref()).collect();
    let text: Vec<_> = text.iter().filter(|elem| **elem != "").collect();
    let text: Vec<_> = text.iter().map(|elem| elem.to_lowercase()).collect();

    let mut max_result:i32 = 0;
    let mut max_index:i32 = 0;
    let mut index = 0;
    let mut current_max:i32 = 0;

    let mut args_with_apperance : Vec<(String, i32)> = Vec::new();

    for key in &args {
        index+=1;
        for text_element in &text {
            if text_element == key {
                current_max+=1;
            }
        }

        args_with_apperance.push((key.to_string(), current_max));

        if current_max > max_result {
            max_result = current_max;
            max_index = index;
        }

        current_max = 0;
    }

    // println!("{:?}",text);
    // println!("{:?}",args_with_apperance);
    // println!("{}", max_index);
    std::process::exit(max_index);



}