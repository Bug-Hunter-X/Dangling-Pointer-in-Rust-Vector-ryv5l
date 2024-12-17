fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    { // Create a new scope
        let x = &vec[0];
        println!("{}", x); 
    } //Reference x is dropped here
    vec.push(3); 
    println!("Vec: {:?}",vec);
}