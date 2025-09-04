fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    swap_first_and_last(&mut numbers);
    println!("{:?}", numbers);
}

fn swap_first_and_last(arr: &mut [i32]) {
    arr.swap(0, arr.len() - 1);
}