// fn main() {
//         let x: i32 = 5; // This is how we declare variables: let <variable_name>: <variable_type> = <data>
//         let _y: i32;   // An underscore means the variable is unused

//         assert_eq!(x, 5);
//         println!("Success!");
// }



// fn main() {
//     let mut x: i32 = 1; // Variables, by default are immutable which means they can't be changed after assignment, we have to declare a variable as mutable with the "mut" keyword
//     x += 2; // If we get rid of the mut keyword, this line doesn't work

//     assert_eq!(x, 3);
//     println!("Success!");
// }



// fn main() {
//     let x: i32 = 10;
//     // let y: i32 = 5; // To fix this we need to declare y outside of the inner scope

//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     // println!("The value of x is {} and value of y is {}", x, y); // This line won't work bcs y is out of the scope
// }


// fn main() {
//     define_x();
// }

// fn define_x() { // This is a function
//     let x: &str = "hello";

//     println!("{}, world", x);
// }


// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12; // We don't have to declare a type but it is safer and better to do so
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x: i32 = 42;
//     println!("{}", x); // Prints "42"
// }


fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x; // Don't use this bcs we are declaring it as immutable
    x += 3;

    let y: i32 = 4;
    // Shadowing
    let y: &str = "I can also be bound to text!";

    println!("Success!");
}