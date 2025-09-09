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

    let weight: f64 = match weight_input_parsing(weight_input) {
        Ok(w) => w,
        Err(msg) => {
            println!("{msg}");
            return;
        }
    };

    let height: f64 = match height_input_parsing(height_input) {
        Ok(h) => h,
        Err(msg) => {
            println!("{msg}");
            return;
        }
    };

    match bmi_calculator(weight, height) {
        Ok(bmi) => {
            let message = bmi_message(bmi);
            println!("Your B.M.I is: {bmi} --> {message}");
        }
        Err(err_msg) => {
            println!("{err_msg}")
        }
    }
}

// Weight bondary constants
const MIN_WEIGHT: f64 = 0.1;
const MAX_WEIGHT: f64 = 500.0;

// Height boundary constants
const MIN_HEIGHT: f64 = 0.5;
const MAX_HEIGHT: f64 = 3.0;

fn bmi_calculator(weight: f64, height: f64) -> Result<f64, &'static str> {
    if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&height) {
        return Err("Height cannot be lower than 0.5 metres or higher than 3.0 metres!");
    }
    if !(MIN_WEIGHT..=MAX_WEIGHT).contains(&weight) {
        return Err("Weight cannot be below 0 or above 500!");
    }
    let bmi = compute_bmi(weight, height);
    Ok(bmi)
}

fn compute_bmi(weight: f64, height: f64) -> f64 {
    ((weight / (height * height)) * 100.0).round() / 100.0
}

fn bmi_message(bmi: f64) -> &'static str {
    if bmi < 16.0 {
        "Severe Thinness"
    } else if (16.0..17.0).contains(&bmi) {
        "Moderate Thinness"
    } else if (17.0..18.5).contains(&bmi) {
        "Mild Thinness"
    } else if (18.5..25.0).contains(&bmi) {
        "Normal"
    } else if (25.0..30.0).contains(&bmi) {
        "Overweight"
    } else if (30.0..35.0).contains(&bmi) {
        "Obese Class I"
    } else if (35.0..40.0).contains(&bmi) {
        "Obese Class II"
    } else {
        "Obese Class III"
    }
}

fn weight_input_parsing(input: String) -> Result<f64, &'static str> {
    input
        .trim()
        .parse()
        .map_err(|_| "Please enter a valid a valid weight")
}

fn height_input_parsing(input: String) -> Result<f64, &'static str> {
    input
        .trim()
        .parse()
        .map_err(|_| "Please enter a valid a valid height")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_input_parsing_valid_number_returns_f64() {
        let result = weight_input_parsing(String::from("65"));
        assert_eq!(result, Ok(65.0));
    }

    #[test]
    fn height_input_parsing_valid_number_returns_f64() {
        let result = weight_input_parsing(String::from("1.75"));
        assert_eq!(result, Ok(1.75));
    }

    // #[test]
    // fn returns_message_after_evaluating_bmi_16_5(){
    //     let message = bmi_message(16.5);
    //     assert_eq!(message, "Moderate Thinness");
    // }
}
