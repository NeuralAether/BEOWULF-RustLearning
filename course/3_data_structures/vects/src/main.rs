fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1i32, 2i32, 3i32];
    println!("{:?}", v);
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    println!("The first element is {}", v[0]);    
    
}