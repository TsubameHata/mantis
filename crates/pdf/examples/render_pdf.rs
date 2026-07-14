use std::env;
use std::fs;

use pdf::{render, Pages};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let data = fs::read(&args[1]).unwrap();
    let pages = render(data, 2000, 4000, Pages::All);

    for (i, page) in pages.iter().enumerate() {
        page.save(format!("{}.page{}.png", &args[1], i)).unwrap();
    }
}