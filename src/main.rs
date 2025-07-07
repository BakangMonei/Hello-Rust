use std::io::{self, Write};

struct Calculator {
    memory: f64,
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            memory: 0.0,
            history: Vec::new(),
        }
    }
    fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero is not allowed".to_string())
        } else {
            Ok(a / b)
        }
    }

    fn power(&self, base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }

    fn sqrt(&self, x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err("Cannot calculate square root of negative number".to_string())
        } else {
            Ok(x.sqrt())
        }
    }

    fn sin(&self, x: f64) -> f64 {
        x.sin()
    }

    fn cos(&self, x: f64) -> f64 {
        x.cos()
    }

    fn tan(&self, x: f64) -> Result<f64, String> {
        if x.cos() == 0.0 {
            Err("Tangent is undefined for this angle".to_string())
        } else {
            Ok(x.tan())
        }
    }
    fn ln(&self, x: f64) -> Result<f64, String> {
        if x <= 0.0 {
            Err("Natural logarithm is only defined for positive numbers".to_string())
        } else {
            Ok(x.ln())
        }
    }

    fn log10(&self, x: f64) -> Result<f64, String> {
        if x <= 0.0 {
            Err("Logarithm is only defined for positive numbers".to_string())
        } else {
            Ok(x.log10())
        }
    }

    fn degrees_to_radians(&self, degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    fn radians_to_degrees(&self, radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    fn store_memory(&mut self, value: f64) {
        self.memory = value;
        self.add_to_history(&format!("Stored {} in memory", value));
    }

    fn recall_memory(&self) -> f64 {
        self.memory
    }

    fn clear_memory(&mut self) {
        self.memory = 0.0;
        self.add_to_history("Memory cleared");
    }

    /// Adds an operation to history
    fn add_to_history(&mut self, operation: &str) {
        self.history.push(operation.to_string());
        // Keep only the last 10 operations
        if self.history.len() > 10 {
            self.history.remove(0);
        }
    }

    /// Displays the calculation history
    fn show_history(&self) {
        println!("\n=== Calculation History ===");
        if self.history.is_empty() {
            println!("No operations in history");
        } else {
            for (i, operation) in self.history.iter().enumerate() {
                println!("{}. {}", i + 1, operation);
            }
        }
        println!("==========================\n");
    }

    /// Displays the calculator menu
    fn show_menu(&self) {
        println!("\n=== Scientific Calculator ===");
        println!("1. Basic Operations (+, -, *, /)");
        println!("2. Power");
        println!("3. Square Root");
        println!("4. Trigonometric Functions");
        println!("5. Logarithms");
        println!("6. Angle Conversion");
        println!("7. Memory Operations");
        println!("8. Show History");
        println!("9. Exit");
        println!("=============================");
    }

    /// Runs the calculator interface
    fn run(&mut self) {
        println!("Welcome to the Rust Scientific Calculator!");
        
        loop {
            self.show_menu();
            print!("Enter your choice (1-9): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => self.basic_operations(),
                "2" => self.power_operation(),
                "3" => self.sqrt_operation(),
                "4" => self.trigonometric_operations(),
                "5" => self.logarithm_operations(),
                "6" => self.angle_conversion(),
                "7" => self.memory_operations(),
                "8" => self.show_history(),
                "9" => {
                    println!("Thank you for using the calculator!");
                    break;
                }
                _ => println!("Invalid choice. Please try again."),
            }
        }
    }

    /// Handles basic arithmetic operations
    fn basic_operations(&mut self) {
        println!("\n=== Basic Operations ===");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        print!("Choose operation (1-4): ");
        io::stdout().flush().unwrap();

        let mut op_choice = String::new();
        io::stdin().read_line(&mut op_choice).unwrap();
        let op_choice = op_choice.trim();

        let (a, b) = self.get_two_numbers();
        let result = match op_choice {
            "1" => {
                let result = self.add(a, b);
                self.add_to_history(&format!("{} + {} = {}", a, b, result));
                Ok(result)
            }
            "2" => {
                let result = self.subtract(a, b);
                self.add_to_history(&format!("{} - {} = {}", a, b, result));
                Ok(result)
            }
            "3" => {
                let result = self.multiply(a, b);
                self.add_to_history(&format!("{} * {} = {}", a, b, result));
                Ok(result)
            }
            "4" => {
                match self.divide(a, b) {
                    Ok(result) => {
                        self.add_to_history(&format!("{} / {} = {}", a, b, result));
                        Ok(result)
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err("Invalid operation choice".to_string()),
        };

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }

    /// Handles power operations
    fn power_operation(&mut self) {
        println!("\n=== Power Operation ===");
        let (base, exponent) = self.get_two_numbers();
        let result = self.power(base, exponent);
        self.add_to_history(&format!("{} ^ {} = {}", base, exponent, result));
        println!("Result: {}", result);
    }

    /// Handles square root operations
    fn sqrt_operation(&mut self) {
        println!("\n=== Square Root ===");
        let number = self.get_number("Enter a number: ");
        match self.sqrt(number) {
            Ok(result) => {
                self.add_to_history(&format!("√{} = {}", number, result));
                println!("Result: {}", result);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    /// Handles trigonometric operations
    fn trigonometric_operations(&mut self) {
        println!("\n=== Trigonometric Functions ===");
        println!("1. Sine");
        println!("2. Cosine");
        println!("3. Tangent");
        print!("Choose function (1-3): ");
        io::stdout().flush().unwrap();

        let mut func_choice = String::new();
        io::stdin().read_line(&mut func_choice).unwrap();
        let func_choice = func_choice.trim();

        let angle = self.get_number("Enter angle in degrees: ");
        let radians = self.degrees_to_radians(angle);

        let result = match func_choice {
            "1" => {
                let result = self.sin(radians);
                self.add_to_history(&format!("sin({}°) = {}", angle, result));
                Ok(result)
            }
            "2" => {
                let result = self.cos(radians);
                self.add_to_history(&format!("cos({}°) = {}", angle, result));
                Ok(result)
            }
            "3" => {
                match self.tan(radians) {
                    Ok(result) => {
                        self.add_to_history(&format!("tan({}°) = {}", angle, result));
                        Ok(result)
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err("Invalid function choice".to_string()),
        };

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }

    /// Handles logarithm operations
    fn logarithm_operations(&mut self) {
        println!("\n=== Logarithm Functions ===");
        println!("1. Natural Logarithm (ln)");
        println!("2. Base-10 Logarithm (log10)");
        print!("Choose function (1-2): ");
        io::stdout().flush().unwrap();

        let mut func_choice = String::new();
        io::stdin().read_line(&mut func_choice).unwrap();
        let func_choice = func_choice.trim();

        let number = self.get_number("Enter a number: ");

        let result = match func_choice {
            "1" => {
                match self.ln(number) {
                    Ok(result) => {
                        self.add_to_history(&format!("ln({}) = {}", number, result));
                        Ok(result)
                    }
                    Err(e) => Err(e),
                }
            }
            "2" => {
                match self.log10(number) {
                    Ok(result) => {
                        self.add_to_history(&format!("log10({}) = {}", number, result));
                        Ok(result)
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err("Invalid function choice".to_string()),
        };

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }

    /// Handles angle conversion operations
    fn angle_conversion(&mut self) {
        println!("\n=== Angle Conversion ===");
        println!("1. Degrees to Radians");
        println!("2. Radians to Degrees");
        print!("Choose conversion (1-2): ");
        io::stdout().flush().unwrap();

        let mut conv_choice = String::new();
        io::stdin().read_line(&mut conv_choice).unwrap();
        let conv_choice = conv_choice.trim();

        let value = self.get_number("Enter value: ");

        let result = match conv_choice {
            "1" => {
                let result = self.degrees_to_radians(value);
                self.add_to_history(&format!("{}° = {} radians", value, result));
                result
            }
            "2" => {
                let result = self.radians_to_degrees(value);
                self.add_to_history(&format!("{} radians = {}°", value, result));
                result
            }
            _ => {
                println!("Invalid conversion choice");
                return;
            }
        };

        println!("Result: {}", result);
    }

    /// Handles memory operations
    fn memory_operations(&mut self) {
        println!("\n=== Memory Operations ===");
        println!("1. Store in Memory");
        println!("2. Recall from Memory");
        println!("3. Clear Memory");
        print!("Choose operation (1-3): ");
        io::stdout().flush().unwrap();

        let mut mem_choice = String::new();
        io::stdin().read_line(&mut mem_choice).unwrap();
        let mem_choice = mem_choice.trim();

        match mem_choice {
            "1" => {
                let value = self.get_number("Enter value to store: ");
                self.store_memory(value);
                println!("Value stored in memory");
            }
            "2" => {
                println!("Memory value: {}", self.recall_memory());
            }
            "3" => {
                self.clear_memory();
                println!("Memory cleared");
            }
            _ => println!("Invalid memory operation choice"),
        }
    }

    /// Gets a single number from user input
    fn get_number(&self, prompt: &str) -> f64 {
        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input.parse::<f64>() {
                Ok(number) => return number,
                Err(_) => println!("Invalid number. Please try again."),
            }
        }
    }

    fn get_two_numbers(&self) -> (f64, f64) {
        let a = self.get_number("Enter first number: ");
        let b = self.get_number("Enter second number: ");
        (a, b)
    }
}

fn main() {
    let mut calculator = Calculator::new();
    calculator.run();
}