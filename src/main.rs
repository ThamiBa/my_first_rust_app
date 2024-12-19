fn main() {
    let a: char = 'A';

    {
        let a: char = 'a'; // This 'a' shadows the outer 'a'
        println!("{}", a); // Prints 'a'
    }

    println!("{}", a); // Prints 'A'

    println!("{}", helper(true));  // Calls helper with true
    println!("{}", helper(false)); // Calls helper with false
}

// Here is one line comment

/* 
    Here is a multiple line comment
*/

// The helper function returns a String based on the boolean input
fn helper(fact: bool) -> String {
    if fact {
        String::from("Truth") // Corrected to use "Truth" as a string
    } else {
        String::from("Falsehood") // Return a string for the false case
    }
}
