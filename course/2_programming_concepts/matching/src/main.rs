enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    HalfDollar,
    Dollar
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // like a switch statement in other languages
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::HalfDollar => 50,
        Coin::Dollar => {
            println!("This is a full dollar coin!");
            100
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

fn main() {
    let coin = Coin::Dollar;
    println!("The value of the coin is {} cents.", value_in_cents(coin));
    let five = Some(5); // Some is a variant of the Option enum that holds a value, while None is a variant that does not hold a value
    let six = plus_one(five);
    let none = plus_one(None);
    println!("5 plus one is {:?}.", six);
    println!("None plus one is {:?}.", none);
}