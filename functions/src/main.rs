fn main() {
    println!("Hello, world!");
    //let x = (let y = 6);
    /*
     *    Compiling functions v0.1.0 (file:///Users/sahil.chug/Workspace/private/rust-workshop/functions)
     error: expected expression, found statement (`let`)
     --> src/main.rs:3:14
     |
     3 |     let x = (let y = 6);
     |              ^^^ expected expression
     |
     = note: variable declaration using `let` is a statement

     error: aborting due to previous error

     error: Could not compile `functions`.

     To learn more, run the command again with --verbose.
     */
    another_function(4);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let z = plus_one(5);
    println!("The value of z is: {}", z);
}

fn another_function(_x: i32) {
    println!("Another function.");
}

fn plus_one(x: i32) -> i32 {
    /*
     * error[E0308]: mismatched types
     --> src/main.rs:38:28
     |
     38 |   fn plus_one(x: i32) -> i32 {
     |  ____________________________^
     39 | |     x + 1;
     | |          - help: consider removing this semicolon
     40 | | }
     | |_^ expected i32, found ()
     |
     = note: expected type `i32`
     found type `()`

     error: aborting due to previous error

     For more information about this error, try `rustc --explain E0308`.
     error: Could not compile `functions`.
     */
    // NOTE: the last colon means a statement not an expressions
    // so this will return () empty tuple
    // but we have specified i32 as return type
    // This is weird have to get used to it. FML
    x + 1;
}
