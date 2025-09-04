fn sum_even_numbers(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&n| n % 2 == 0)
        .sum()

}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let sum = sum_even_numbers(&numbers);
    println!("{}", sum);
}