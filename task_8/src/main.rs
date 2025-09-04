fn main() {
    let grades: [u8; 20] = [5, 4, 3, 5, 2, 4, 5, 3, 4, 5, 5, 4, 3, 2, 5, 4, 5, 5, 4, 3];
    println!("Отличников: {}", grades.iter().filter(|x| **x == 5).count());
    println!("Хорошистов: {}", grades.iter().filter(|x| **x == 4).count());
    println!("Троечников: {}", grades.iter().filter(|x| **x == 3).count());
    println!("Двоечников: {}", grades.iter().filter(|x| **x == 2).count());
}