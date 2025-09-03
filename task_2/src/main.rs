use std::io;

fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Ошибка чтения строки");
    let number: i32 = number.trim().parse().expect("Ошибка преобразования в число");

    let result = match number {
        n if n == 0 => "Ноль",
        n if n > 0 => "Положительное число",
        n if n < 0 => "Отрицательное число",
        _ => "Неизвестное число",
    };

    println!("Результат: {}", result);
}