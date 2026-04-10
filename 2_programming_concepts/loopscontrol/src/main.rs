fn main() {
    let number = 3;

    // IF
    if number < 5 { // mix between python (no parentheses) and C++ (curly braces)
        println!("condition was true");
    } else if number == 5 {
        println!("condition was too true to be true");
    } else {
        println!("condition was false");
    }

    let condition = true ;
    let condition2 = true;
    let number = if condition { 5 } else if condition2 { 6 } else { 7 }; // just like the scope but it returns a value !
    println!("The value of number is: {number}");

    // Pure LOOP (like a while true)
    let mut counter = 0; 
    loop {
        println!("again!");
        counter += 1;
        if counter == 10 {
            break; // you need to break the loop otherwise it will run forever ! 
        }
    }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // you can also return a value from the loop !
        }
    }; // break becomes like a return statement for the loop !
    println!("The result is: {result}");

    // WHILE
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // FOR
    let a = [10, 20, 30, 40, 50]; // 
    for element in a { // like python 
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    
}
