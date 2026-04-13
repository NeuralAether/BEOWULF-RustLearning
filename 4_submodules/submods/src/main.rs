use crate::cars::speedster::Speedster;
pub mod cars;

fn main() {
    let my_speedster = Speedster {
        top_speed: 200,
        zero_to_sixty: 3.5,
    };
    println!("My speedster has a top speed of {} mph and can go from 0 to 60 in {} seconds.", my_speedster.top_speed, my_speedster.zero_to_sixty);
}
