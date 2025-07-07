use axum::{
    extract::Json,
    http::{Method, StatusCode},
    response::Html,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::io::Write;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use std::f64::consts::PI;

#[derive(Debug, Deserialize)]
struct CalculationRequest {
    operation: String,
    a: Option<f64>,
    b: Option<f64>,
    value: Option<f64>,
}

#[derive(Debug, Serialize)]
struct CalculationResponse {
    result: f64,
    expression: String,
    success: bool,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn calculate(Json(request): Json<CalculationRequest>) -> Result<Json<CalculationResponse>, StatusCode> {
    let calculator = Calculator::new();
    
    let result = match request.operation.as_str() {
        "add" => {
            let a = request.a.ok_or(StatusCode::BAD_REQUEST)?;
            let b = request.b.ok_or(StatusCode::BAD_REQUEST)?;
            let result = calculator.add(a, b);
            Ok(CalculationResponse {
                result,
                expression: format!("{} + {}", a, b),
                success: true,
                error: None,
            })
        }
        "subtract" => {
            let a = request.a.ok_or(StatusCode::BAD_REQUEST)?;
            let b = request.b.ok_or(StatusCode::BAD_REQUEST)?;
            let result = calculator.subtract(a, b);
            Ok(CalculationResponse {
                result,
                expression: format!("{} - {}", a, b),
                success: true,
                error: None,
            })
        }
        "multiply" => {
            let a = request.a.ok_or(StatusCode::BAD_REQUEST)?;
            let b = request.b.ok_or(StatusCode::BAD_REQUEST)?;
            let result = calculator.multiply(a, b);
            Ok(CalculationResponse {
                result,
                expression: format!("{} × {}", a, b),
                success: true,
                error: None,
            })
        }
        "divide" => {
            let a = request.a.ok_or(StatusCode::BAD_REQUEST)?;
            let b = request.b.ok_or(StatusCode::BAD_REQUEST)?;
            match calculator.divide(a, b) {
                Ok(result) => Ok(CalculationResponse {
                    result,
                    expression: format!("{} ÷ {}", a, b),
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("{} ÷ {}", a, b),
                    success: false,
                    error: Some(e),
                }),
            }
        }
        "power" => {
            let a = request.a.ok_or(StatusCode::BAD_REQUEST)?;
            let b = request.b.ok_or(StatusCode::BAD_REQUEST)?;
            let result = calculator.power(a, b);
            Ok(CalculationResponse {
                result,
                expression: format!("{} ^ {}", a, b),
                success: true,
                error: None,
            })
        }
        "sqrt" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            match calculator.sqrt(value) {
                Ok(result) => Ok(CalculationResponse {
                    result,
                    expression: format!("√{}", value),
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("√{}", value),
                    success: false,
                    error: Some(e),
                }),
            }
        }
        "sin" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let radians = calculator.degrees_to_radians(value);
            let result = calculator.sin(radians);
            Ok(CalculationResponse {
                result,
                expression: format!("sin({}°)", value),
                success: true,
                error: None,
            })
        }
        "cos" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let radians = calculator.degrees_to_radians(value);
            let result = calculator.cos(radians);
            Ok(CalculationResponse {
                result,
                expression: format!("cos({}°)", value),
                success: true,
                error: None,
            })
        }
        "tan" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let radians = calculator.degrees_to_radians(value);
            match calculator.tan(radians) {
                Ok(result) => Ok(CalculationResponse {
                    result,
                    expression: format!("tan({}°)", value),
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("tan({}°)", value),
                    success: false,
                    error: Some(e),
                }),
            }
        }
        "ln" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            match calculator.ln(value) {
                Ok(result) => Ok(CalculationResponse {
                    result,
                    expression: format!("ln({})", value),
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("ln({})", value),
                    success: false,
                    error: Some(e),
                }),
            }
        }
        "log10" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            match calculator.log10(value) {
                Ok(result) => Ok(CalculationResponse {
                    result,
                    expression: format!("log({})", value),
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("log({})", value),
                    success: false,
                    error: Some(e),
                }),
            }
        }
        "degrees_to_radians" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let result = calculator.degrees_to_radians(value);
            Ok(CalculationResponse {
                result,
                expression: format!("{}° → rad", value),
                success: true,
                error: None,
            })
        }
        "radians_to_degrees" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let result = calculator.radians_to_degrees(value);
            Ok(CalculationResponse {
                result,
                expression: format!("{} rad → °", value),
                success: true,
                error: None,
            })
        }
        "square" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let result = value * value;
            Ok(CalculationResponse {
                result,
                expression: format!("{}²", value),
                success: true,
                error: None,
            })
        }
        "reciprocal" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            if value == 0.0 {
                Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("1/{}", value),
                    success: false,
                    error: Some("Cannot divide by zero".to_string()),
                })
            } else {
                let result = 1.0 / value;
                Ok(CalculationResponse {
                    result,
                    expression: format!("1/{}", value),
                    success: true,
                    error: None,
                })
            }
        }
        "factorial" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            if value < 0.0 || value != value.floor() {
                Ok(CalculationResponse {
                    result: 0.0,
                    expression: format!("{}!", value),
                    success: false,
                    error: Some("Factorial is only defined for non-negative integers".to_string()),
                })
            } else {
                let result = calculator.factorial(value as u64);
                Ok(CalculationResponse {
                    result: result as f64,
                    expression: format!("{}!", value),
                    success: true,
                    error: None,
                })
            }
        }
        "pi" => {
            Ok(CalculationResponse {
                result: PI,
                expression: "π".to_string(),
                success: true,
                error: None,
            })
        }
        "e" => {
            Ok(CalculationResponse {
                result: std::f64::consts::E,
                expression: "e".to_string(),
                success: true,
                error: None,
            })
        }
        "abs" => {
            let value = request.value.ok_or(StatusCode::BAD_REQUEST)?;
            let result = value.abs();
            Ok(CalculationResponse {
                result,
                expression: format!("|{}|", value),
                success: true,
                error: None,
            })
        }
        _ => Err(StatusCode::BAD_REQUEST),
    };

    result.map(Json)
}

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
        degrees * PI / 180.0
    }

    fn radians_to_degrees(&self, radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    fn factorial(&self, n: u64) -> u64 {
        if n == 0 || n == 1 {
            1
        } else {
            n * self.factorial(n - 1)
        }
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

    fn add_to_history(&mut self, operation: &str) {
        self.history.push(operation.to_string());
        if self.history.len() > 10 {
            self.history.remove(0);
        }
    }

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

    fn run(&mut self) {
        println!("Welcome to the Rust Scientific Calculator!");
        
        loop {
            self.show_menu();
            print!("Enter your choice (1-9): ");
            std::io::stdout().flush().unwrap();

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
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

    fn basic_operations(&mut self) {
        println!("\n=== Basic Operations ===");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        print!("Choose operation (1-4): ");
        std::io::stdout().flush().unwrap();

        let mut op_choice = String::new();
        std::io::stdin().read_line(&mut op_choice).unwrap();
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

    fn power_operation(&mut self) {
        println!("\n=== Power Operation ===");
        let (base, exponent) = self.get_two_numbers();
        let result = self.power(base, exponent);
        self.add_to_history(&format!("{} ^ {} = {}", base, exponent, result));
        println!("Result: {}", result);
    }

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

    fn trigonometric_operations(&mut self) {
        println!("\n=== Trigonometric Functions ===");
        println!("1. Sine");
        println!("2. Cosine");
        println!("3. Tangent");
        print!("Choose function (1-3): ");
        std::io::stdout().flush().unwrap();

        let mut func_choice = String::new();
        std::io::stdin().read_line(&mut func_choice).unwrap();
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

    fn logarithm_operations(&mut self) {
        println!("\n=== Logarithm Functions ===");
        println!("1. Natural Logarithm (ln)");
        println!("2. Base-10 Logarithm (log10)");
        print!("Choose function (1-2): ");
        std::io::stdout().flush().unwrap();

        let mut func_choice = String::new();
        std::io::stdin().read_line(&mut func_choice).unwrap();
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

    fn angle_conversion(&mut self) {
        println!("\n=== Angle Conversion ===");
        println!("1. Degrees to Radians");
        println!("2. Radians to Degrees");
        print!("Choose conversion (1-2): ");
        std::io::stdout().flush().unwrap();

        let mut conv_choice = String::new();
        std::io::stdin().read_line(&mut conv_choice).unwrap();
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

    fn memory_operations(&mut self) {
        println!("\n=== Memory Operations ===");
        println!("1. Store in Memory");
        println!("2. Recall from Memory");
        println!("3. Clear Memory");
        print!("Choose operation (1-3): ");
        std::io::stdout().flush().unwrap();

        let mut mem_choice = String::new();
        std::io::stdin().read_line(&mut mem_choice).unwrap();
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

    fn get_number(&self, prompt: &str) -> f64 {
        loop {
            print!("{}", prompt);
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
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

#[tokio::main]
async fn main() {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/api/calculate", post(calculate))
        .nest_service("/static", ServeDir::new("static"))
        .layer(cors);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("🚀 Calculator server starting on http://{}", addr);
    // println!("📱 Open your browser and navigate to http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Calculator server starting on http://{}", addr);
    println!("📱 Open your browser and navigate to http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}