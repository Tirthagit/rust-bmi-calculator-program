# BMI Calculator CLI

A simple command-line BMI (Body Mass Index) calculator written in Rust.  
Users can input their weight and height, and the program outputs their BMI along with the corresponding category.

## Features

- Parses weight and height input from the user
- Calculates BMI and rounds it to 2 decimal places
- Outputs BMI category (e.g., Normal, Overweight, Obese Class I)
- Validates realistic weight and height ranges
- Modular Rust code with functions for parsing, calculation, and categorization

## Running the App

1. Clone the repository:

   ```bash  
    git clone https://github.com/Tirthagit/rust-bmi-calculator-program.git 
    ```  

2. Change directory to project directory

    ```bash
    cd bmi-calculator
    ```  

3. Run using Cargo

    ```bash
    cargo run  
    ```

## Example Usage

```markdown
## Example

Enter your weight in kilograms: 70
Enter your height in meters: 1.75
Your B.M.I is: 22.86 --> Normal
```

## Future Improvements

- Add command-line arguments for weight and height
- Implement error handling without panics
- Add history tracking or logging of previous BMI calculations
