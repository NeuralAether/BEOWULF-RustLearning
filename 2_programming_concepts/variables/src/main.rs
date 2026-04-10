fn main() {
    // Mutable variables need mut 
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const MAX_POINTS: u32 = 100000; // You can add _ like 100_000 to make it more readable
    println!("The value of MAX_POINTS is: {MAX_POINTS}");
    // Shadowing allows you to reuse the same variable name
    let x = x + 1; 
    { // This is a new scope ! 
        let x = x * 2; // This x is only valid in this scope ! 
        println!("The value of x in the inner scope is: {x}"); // outputs 14
    }
    println!("The value of x is: {x}"); // outputs 7 (x is unaffected by the inner scope)
    // Some strings 
    let spaces = "   "; 
    let spaces = spaces.len(); // Changes the type. only for non mutable variables ! Mutable strings would not allow this !
    println!("The number of spaces is: {spaces}"); 
    let guess : u32 = "42a".parse().expect("Not a number!");
    println!("The guess is: {guess}");
}
