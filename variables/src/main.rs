/*
const (constant) is always immutable and type must be annotated
ex. const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

let is immutable by default but can be change to mutable by adding mut
ex. let mut x = 10
*/

/*
 Rust’s naming convention for constants
 is to use all uppercase with underscores between words
*/

/*
Shadowing is different from marking a variable as mut
because we’ll get a compile-time error if we accidentally try to reassign to this variable
without using the let keyword.

By using let, we can perform a few transformations
on a value but have the variable be immutable after those transformations
have been completed.
*/

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of inner scope is {x}");
    }

    println!("The value of outer scope is {x}");
}
