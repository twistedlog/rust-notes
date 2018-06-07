fn main() {
    //const x: i32 = 5;
    let x = 6;
    let x = x + 4;
    println!("The value of x is: {}", x);
    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    //let guess = "42".parse().expect("Not a number!");
    /*
     * You need to annotate variable with type info in case of parse
     * Parse is a generic over any type that implements FromStr
     *
     * Parse is one of those rere cases where you can use syntax knows
     * as turbofish ::<>
     * let four = "4".parse::<u32>();
     * or 
     * let four: u32 = "4".parse().unwrap();
     */
    let guess: u32 = "4".parse().expect("Not a number!");

    let t = true;
    let f = false;

    /*
     * Rust has following compound types
     * 1. tuple - can contain any types
     * 2. array - cannot expand or shrink and contain same type
     * 3. vector- can expand or shrink and contain same type
     */
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
