# bin-calculator
### Evaluate expressions with binary numbers

This is a command line tool written in Rust that allows you to do math with binary numbers.  
  
For example 
```
./bin-calculator
Enter your expression: (101 + 11) * 10
10000
```
  
  
  
You can also provide the expression through the command line arguments, but you will need to escape special characters.   
For example in bash
```
./bin-calculator \(101 + 11\) \* 10
10000
```

Five operations are supported:
* Multiplication (`*`)
* Division (`/`)
* Addition (`+`)
* Subtraction (binary `-`)
* Negation (unary `-`)

Binary numbers are interpreted as two's complement 32-bit integers (`i32` in Rust)
```
./bin-calculator -10
11111111111111111111111111111110
```

## Build Instructions
1. Clone the repository.
2. Run `cargo build --release` in the directory you cloned to.
3. The executable will be in `bin-calculator/target/release/bin-calculator`, (`bin-calculator.exe` for Windows) feel free to move it to where you need it.
4. Execute the executable!
5. You can also run `cargo run` to build and execute the program in one step.
