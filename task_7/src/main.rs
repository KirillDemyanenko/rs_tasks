use std::io;

fn main() {
    let friends: [&str; 4] = ["Алиса", "Борис", "Виктория", "Григорий"];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Пожалуйста, введите корректное число.");
            return;
        }
    };

    if index <= friends.len() {
        println!("Друг номер {}: {}", index, friends[index - 1]);
    } else {
        println!("Некорректный номер друга.");
    }
}