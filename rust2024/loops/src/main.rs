use std::fmt;

// Define an enum to represent the temperature units clearly.
// Added derive attributes for easier debugging, comparison, and copying.
#[derive(Debug, PartialEq, Copy, Clone)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

// Implement the Display trait for the enum to print units nicely (e.g., "°C").
impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureUnit::Celsius => write!(f, "°C"),
            TemperatureUnit::Fahrenheit => write!(f, "°F"),
        }
    }
}

// Single function to convert temperature.
// Takes a value and its unit, returns the converted value and the new unit.
fn convert_temperature(value: f64, unit: TemperatureUnit) -> (f64, TemperatureUnit) {
    match unit {
        TemperatureUnit::Celsius => {
            // Convert Celsius to Fahrenheit: F = C * 9/5 + 32
            // Use floating point numbers (e.g., 9.0) for calculation.
            let fahrenheit = value * (9.0 / 5.0) + 32.0;
            (fahrenheit, TemperatureUnit::Fahrenheit) // Return converted value and new unit
        }
        TemperatureUnit::Fahrenheit => {
            // Convert Fahrenheit to Celsius: C = (F - 32) * 5/9
            let celsius = (value - 32.0) * (5.0 / 9.0);
            (celsius, TemperatureUnit::Celsius) // Return converted value and new unit
        }
    }
}

fn main() {
    println!("--- Temperature Conversion Examples ---");

    // --- Example 1: Celsius to Fahrenheit ---
    let temp_c1 = 0.0; // Freezing point in Celsius
    let unit_c1 = TemperatureUnit::Celsius;
    // Call the conversion function
    let (converted_value1, converted_unit1) = convert_temperature(temp_c1, unit_c1);
    // Print the result using the Display impl for the unit
    println!("{} {} is {:.2} {}", temp_c1, unit_c1, converted_value1, converted_unit1);

    // --- Example 2: Celsius to Fahrenheit ---
    let temp_c2 = 100.0; // Boiling point in Celsius
    let unit_c2 = TemperatureUnit::Celsius;
    let (converted_value2, converted_unit2) = convert_temperature(temp_c2, unit_c2);
    println!("{} {} is {:.2} {}", temp_c2, unit_c2, converted_value2, converted_unit2);

    // --- Example 3: Fahrenheit to Celsius ---
    let temp_f1 = 32.0; // Freezing point in Fahrenheit
    let unit_f1 = TemperatureUnit::Fahrenheit;
    let (converted_value3, converted_unit3) = convert_temperature(temp_f1, unit_f1);
    println!("{} {} is {:.2} {}", temp_f1, unit_f1, converted_value3, converted_unit3);

    // --- Example 4: Fahrenheit to Celsius ---
    let temp_f2 = 212.0; // Boiling point in Fahrenheit
    let unit_f2 = TemperatureUnit::Fahrenheit;
    let (converted_value4, converted_unit4) = convert_temperature(temp_f2, unit_f2);
    println!("{} {} is {:.2} {}", temp_f2, unit_f2, converted_value4, converted_unit4);

     // --- Example 5: Fahrenheit to Celsius ---
    let temp_f3 = 68.0; // A common room temperature
    let unit_f3 = TemperatureUnit::Fahrenheit;
    let (converted_value5, converted_unit5) = convert_temperature(temp_f3, unit_f3);
    println!("{} {} is {:.2} {}", temp_f3, unit_f3, converted_value5, converted_unit5);

     // --- Example 6: Celsius to Fahrenheit ---
    let temp_c3 = 20.0; // A common room temperature
    let unit_c3 = TemperatureUnit::Celsius;
    let (converted_value6, converted_unit6) = convert_temperature(temp_c3, unit_c3);
    println!("{} {} is {:.2} {}", temp_c3, unit_c3, converted_value6, converted_unit6);

}