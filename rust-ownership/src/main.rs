fn main() {
    { // Start of new scope
        //s has not been created, not valid yet
        let _s = "hello";
        //Now it is valid
    } // End of scope, s cannot be used

    {
        // This will request memory needed for the string "hello"
    let mut s = String::from("hello"); //Mutable, growable. Size not known at compile time
    s.push_str(", world!"); // Adds onto the string object in the heap
    println!("{}", s);  
    }// s is now out scope, and drop() will be called on it automatically

    {
        let x = 5;
        let _y = x; //y is also given a value of 5. Both are fixed size, on the stack.
        
        let s1 = String::from("hello");
        let s2 = s1;
        //println!("{}", s1); This line won't work, because s1 was 'moved' to s2. 
        // If it wasn't, when s2 and s1 went out of scope, they would both free the memory of the string
        // Need to now use s2, as if s1 was no longer in the program.
        println!("{}", s2);
    }

    let string = String::from("howdy");
    takes_ownership(string);
    // println!("{}", string); This line will not compile because the value is moved into the func
    // To keep the value in the scope of main, it needs to be borrowed by the func.

    let x = 20;
    makes_copy(x);
    println!("{} but in the main", x); // No problems here, x can be copied into a new variable easily.

    let string2 = String::from("transferred");
    let _string2 = takes_and_returns_ownership(string2);// This returns a string, and also its ownership

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Passes a reference to s1, a pointer to where the structure is.

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // If this line was missing, this would not compile.
    //the references are out of scope after their last use. If there is no use, there is no problem.

    let r3 = &mut s;
    println!("{}", r3);

    let _ref_to_nothing = dangle();
    
}

fn dangle() -> /*&*/String {
    let s = String::from("dangler");
    //If the ref operator is used, this memory will drop after dangle() is over, and the reference will point to nothing. 
    /*&*/s
}

fn calculate_length(s: &String) -> usize { //it takes a reference, which allows us to access the methods and values.
    s.len()
}

fn takes_ownership(some_string: String) { // Ownership of the value is transferred to some_string here. Will drop at the end of the function.
    println!("{}", some_string);
}

fn takes_and_returns_ownership(some_string: String) -> String{ //The ownership is moved to some_string, but upon return is transferred back.
    some_string
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}