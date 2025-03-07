use std::env;

use findtext_sheet::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <keyword> <file.xlsx>", args[0]);
        return;
    }

    let keyword = &args[1];
    let filepath = &args[2];
    let ret = search(keyword.as_str(), filepath.as_str());

    match ret {
        Ok(ret) => ret.iter().for_each(|x| println!("{}", x)),
        Err(err) => eprint!("{}", err),
    }
}
