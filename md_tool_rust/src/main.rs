use std::env;
mod pic_down;
mod pic_clean;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need 1 param by option");
    }

    let mut option = String::new();
    option.push_str(&(args[1]));

    match option.as_str() {
        "pic_down" => pic_down::main(args),
        "pic_clean" => pic_clean::main(args),
        _=> println!("option not support"),
    }
}
