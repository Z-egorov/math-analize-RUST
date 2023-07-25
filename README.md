# Math Analize Library
This is __Math Analize Library__, you can use it to simplify your work with mathematical functions. Library is ported from C++ for the Rust language
___
## How-to-use
- Run the following Cargo command in your project directory: cargo add math-analize
- Or add the following line to your Cargo.toml: math-analize = "0.0.1"

To use in your program, add this 
```rust
use math-analize::ma;
```
__or__
```rust
use math-analize::ma { functions, methods, options };
```

Then you will be able to use all the functionality
___
## Features

In this library there are ___5 structures of mathematical functions___ and ___5 base methods___ to work with them:
  - Mathematical functions:
    - Linear,
    - Quadratic,
    - Power,
    - Fractional-Linear,
    - Exponential
  - Methods:
    - Calculate integral,
    - Calculate derivative points,
    - Calculate Extremum points,
    - Calculate Extremums,
    - Evenness
   
All sturctures are using public Trait "Function". You can use it to create your own mathematical function structure. You can also use Limits structure for convenient use
#### Warning! Some functions and methods can return Result<> or Option<> types, so be careful
## Example
#### Program, that prints all function's data
```rust

use math_analize::ma::{ functions, methods, options };

fn main() {
    let function = functions::Quadratic::new(1.0, 6.0, 9.0, 0.0, 10.0);
    
    let integral_result = methods::calculate_integral(options::CalculatingMethod::NEWTON, &function);
    let derivative_points_result = methods::calculate_derivative_points(&function);
    let extremums_result = methods::calculate_extremums(&function);
    let evenness_result = methods::evenness(&function);

    println!("Integral of this quadratic function - {}", integral_result);
    println!("Evenness of this quadratic function - {}", evenness_result);
    
    println!("Derivative points of this quadratic function:");
    for i in derivative_points_result {
        print!("{}", i);
    }
    
    println!("Extremums of this quadratic function:");
    for i in extremums_result {
        print!("{} {}", i.0, i.1);
    }

}

```
