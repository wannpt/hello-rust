fn main() {
    // rust take {} as an argument
    println!("{} days", 31);

    // rust can understand {int} as positional arguments
    println!("{0}, This is {1}. {1}, this is {0}", "Alice", "Bob");

    // rust can understand {name} as named arguments
    // note that position is not counted
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}
