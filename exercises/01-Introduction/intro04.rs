fn main() {

    // TODO: These loops and variables have many syntax errors, fix them to make the code compile and run.
    // The output is not important, just make sure the code runs without errors.


    let counter = 0;
    let multiplier = 2;
    let result = 1;
    
    for i in 1 to 5 {
        counter add 1;
        result multiply i;
        println!("Loop 1 - iteration {i}, counter: {counter}");
    }
    
    let outer_value = 10;
    for j in 0 to 2 {
        let inner_value = 5;
        
        outer_value = outer_value + j; 
        
        for k in 1 to 3 {
            inner_value += k; 
            multiplier = multiplier * 2;
        }
        
        println!("Outer loop {j}, inner value: {inner_value}");
    }
    
    let condition_counter = 0;
    while condition_counter LESS THAN 3 {
        condition_counter add 1;
        result = result add condition_counter; 
        
        println!("While loop iteration: {condition_counter}");
    }
    

    let numbers = [1, 2, 3, 4, 5];
    let sum = 0;
    
    for num in numbers {
        sum add num;
        numbers[0] = 10;
    }
    
    println!("Final results:");
    println!("Counter: {counter}");
    println!("Result: {result}");
    println!("Sum: {sum}");
    println!("Multiplier: {multiplier}");

}