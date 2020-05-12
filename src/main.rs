fn main() {
    //changing what x means by having the reference be mutable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    //creating new variables of x and shadowing the previous definition
    //  the references are themselves immutable in this instance.
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x);

    // shadowing values where the redefinition is a different type is also possible

    // first, a string;
    let spaces = "  ";

    // then, shadow this definition where the new spaces variable is a usize:
    let spaces = spaces.len();

    println!("The value of spaces is {}", spaces);
}
