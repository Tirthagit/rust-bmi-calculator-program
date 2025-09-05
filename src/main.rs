use std::io;
fn main() {
    let mut weight_input = String::new();
    let mut height_input = String::new();

    println!("Enter your weight in kilograms:");
    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to get weight");

    println!("Enter your height in metres:");
    io::stdin()
        .read_line(&mut height_input)
        .expect("Please enter valid height");

    let weight: f64 = weight_input
        .trim()
        .parse()
        .expect("Please enter a valid weight");
    let height: f64 = height_input
        .trim()
        .parse()
        .expect("Please enter a valid weight");

    bmi_calculator(weight, height);
}

fn bmi_calculator(weight: f64, height: f64) {
    if weight < 0.0 || weight > 500.0 {
        println!("Weight cannot be below 0 or above 500!")
    }
    if height < 0.5 || height > 3.0 {
        println!("Height cannot be lower than 0.5 metres or higher than 3.0 metres!")
    }
    let mut bmi = weight / (height * height);
    bmi = (bmi * 100.0).round() / 100.0;
    println!("{bmi}");
}
