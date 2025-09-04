use std::io;

fn fill_array(mut arr: [String; 5]) -> [String; 5] {
    let mut buf = String::new();
    for i in 0..arr.len() {
        io::stdin().read_line(&mut buf).unwrap();
        arr[i] = buf.trim().parse().unwrap();
        buf.clear();
    }
    arr
}

fn main() {
    let mut original_items = [String::new(), String::new(), String::new(), String::new(), String::new()];
    let mut incorrect_items = [String::new(), String::new(), String::new(), String::new(), String::new()];
    original_items = fill_array(original_items);
    incorrect_items = fill_array(incorrect_items);
    if original_items == incorrect_items {
        println!("0");
    } else {
        for i in 0..original_items.len() {
            if original_items[i] != incorrect_items[i] {
                println!("{}", i + 1);
            }
        }
    }
}