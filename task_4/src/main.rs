fn process_thermostat_data() {
    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).unwrap();
    let temperature: i32 = temperature.trim().parse().unwrap();

    let mut humidity = String::new();
    std::io::stdin().read_line(&mut humidity).unwrap();
    let humidity: i32 = humidity.trim().parse().unwrap();

    let data = (temperature, humidity);

    if let (t @ 51.., _) = data {
            println!("Слишком жарко! Температура: {}", t)
    } else { println!("0") }
}

fn main() {
    process_thermostat_data();
}