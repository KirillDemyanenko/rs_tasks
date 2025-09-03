use std::io;

fn main() {

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Ошибка чтения строки");

    let number: i32 = number.trim().parse().expect("Ошибка преобразования в число");

    let result = match number {
        n if n == 0 => "Число равно нулю",
        n if n % 2 == 0 => "Число четное",
        n if n % 2 != 0 => "Число нечетное",
        _ => "Неизвестный вариант"
    };
    println!("{}", result);
}