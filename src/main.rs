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

    let weight: f64 = weight_input_parsing(weight_input);
    let height: f64 = height_input_parsing(height_input);

    bmi_calculator(weight, height);
}

// Weight bondary constants
const MIN_WEIGHT: f64 = 0.0;
const MAX_WEIGHT: f64 = 500.0;

// Height boundary constants
const MIN_HEIGHT: f64 = 0.5;
const MAX_HEIGHT: f64 = 3.0;

fn bmi_calculator(weight: f64, height: f64) {
    if weight < MIN_WEIGHT || weight > MAX_WEIGHT {
        println!("Weight cannot be below 0 or above 500!")
    }
    if height < MIN_HEIGHT || height > MAX_HEIGHT {
        println!("Height cannot be lower than 0.5 metres or higher than 3.0 metres!")
    }
    let mut bmi = weight / (height * height);
    let message = bmi_message(bmi);
    bmi = (bmi * 100.0).round() / 100.0;
    println!("Your B.M.I is: {bmi} --> {message}");
}

fn bmi_message(bmi: f64) -> &'static str {
    if bmi < 16.0 {
        "Severe Thinness"
    } else if bmi >= 16.0 && bmi < 17.0 {
        "Moderate Thinness"
    } else if bmi >= 17.0 && bmi < 18.5 {
        "Mild Thinness"
    } else if bmi >= 18.5 && bmi < 25.0 {
        "Normal"
    } else if bmi >= 25.0 && bmi < 30.0 {
        "Overweight"
    } else if bmi >= 30.0 && bmi < 35.0 {
        "Obese Class I"
    } else if bmi >= 35.0 && bmi < 40.0 {
        "Obese Class II"
    } else if bmi >= 40.0 {
        "Obese Class III"
    } else {
        "Invalid"
    }
}

fn weight_input_parsing(input: String) -> f64 {
    return input
        .trim()
        .parse()
        .expect("Please enter a valid a valid weight");
}

fn height_input_parsing(input: String) -> f64 {
    return input
        .trim()
        .parse()
        .expect("Please enter a valid a valid height");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_input_parsing_valid_number_returns_f64() {
        let result = weight_input_parsing(String::from("65"));
        assert_eq!(result, 65.0);
    }
    
    #[test]
    fn height_input_parsing_valid_number_returns_f64() {
        let result = weight_input_parsing(String::from("1.75"));
        assert_eq!(result, 1.75);
    }

    // #[test]
    // fn returns_message_after_evaluating_bmi_16_5(){
    //     let message = bmi_message(16.5);
    //     assert_eq!(message, "Moderate Thinness");
    // }
}
