/*
OWNERSHIP IN RUST:-
In Rust, a value can only be held by one variable at a time, and the value is lost from the original variable if it is
reassigned to another variable. This is called "moving" a value in Rust. Rust does not do a clone or shallow copy but instead it
"transfers ownership" of the value to another variable
Another concept is that if the value is passed to a variable, it is lost or "dropped" after the variable goes out of scope
*/


fn main() {
    let s1: String = String::from("Hello, World!"); //Initialises a string var
    let s2: String = s1; //Transfers ownership of String literal to s2 var
    println!("{}", s2); //Prints value of s2, which contains literal of s1
    //println!("{}", s1); If this statement were to be executed it would provide an error because the value of s1 is transferred
    //to s2

    //Giving ownership to a function local var
    let s3: String = String::from("hello");
    take_ownership(s3);

    //Taking ownership of a local var in a function
    let s4: String = give_ownership();
    println!("{}", s4);

    /*
    REFERENCES:-
    When you want to work with values but don't want to take the ownership of the value, that's where references come in
    A reference can be called a pointer to the variable being passed
    Let's say, we want to know the length of a String and we use a function to do that-
    */

    let s5: String = String::from("Hello There!");
    let (strlength, s6) = calc_len(s5);
    println!("Length of string \"{}\" = {}", s6, strlength);

    /*
    The above code can work fine, but the syntax is weird and the process is complicated in general, as it includes taking and
    giving back of ownership of a string variable
    Instead, we could achieve the same with references but with much less hassle
     */

    let s7: String = String::from("Poopyhead");
    let strlen2 = calculate_length(&s7); //As we can see, we pass the string s7 as a REFERENCE using the (&) operator
    println!("{}", strlen2);

    /*
    MUTABLE AND IMMUTABLE REFERENCES:-
    In Rust, a mutable variable can only have ONE reference to it. This is because with mutable references, you can read and
    write to the same value simultaneously, and this can lead to bad memory bugs and make the program crash.
    This is why Rust only allows one reference to a mutable value at one time so you can only read OR write from the value
    */

    let mut s8 = String::from("Its cold");
    let mut s9 = &mut s8;
    println!("{}", s9);
    //let mut s10: &mut String = &mut s8 This statement of code would lead to an error saying you can only have one reference
    //to the mutable variable s8

    /*
    However, you can have multiple references to an immutable value as that would not lead to a race between pointers to read
    and write to the variable, because immutable references cannot modify the value of a variable
     */

    let s10 = String::from("Its getting colder!");
    let _s11 = &s10; //Multiple references to immutable value
    let s12 = &s10;
    println!("{}", s12);

    /*
    SLICES IN RUST:-
    In Rust, when you want a part of your data to be stored in a collection, you use the slice type
    The slice type enables you to specify the starting index and the ending index of the part of the collection you want to store
    The slice type also enables us to keep in sync of the actual collection without facing any errors or clashes
    Let us enumerate this using an example: let's say we want to return the first word in a string sequence
     */

    let hello_world = String::from("Hello World!");
    let f_word: &str = first_word(&hello_world);
    println!("{}", f_word);

    //We can also return a slice to an array:

    let arr: [i32; 10] = [2,4,3,5,3,1,4,5,6,3]; //An array containing 10 elements
    let arr_slice = &arr[2..7]; //Slice containing elements 3 through 7 of array
    println!("{:?}", arr);
    println!("{:?}", arr_slice);
}

//Function for taking ownership of a String var
fn take_ownership(s: String){ //This function takes the ownership of the passed string
    println!("{}", s); //Prints out the passed string
} //As the function scope ends, the value of 's' "drops" or just gets erased from memory

//Function for giving ownership to a String var
fn give_ownership() -> String{ //This gives the ownership to the assigned var when function is called
    let str: String = String::from("Oh hello there"); //String is instantiated
    str //String is returned and ownership of string is given to called var
}

//Function for calculating length of a String
fn calc_len(s: String) -> (u32, String){ //This function returns a tuple containing the length of string and the string itself
    let len: u32 = s.len() as u32; //Function to calculate the length of string
    (len, s) //Returns the length of string along with returning the ownership of the passed string, also called "borrowing"
}

//Function to achieve the same length calculating, but enhanced and using references
 fn calculate_length(s: &String) -> u32{ //As we can see here, we take the REFERENCE of a string instead of taking the OWNERSHIP
     let len: u32 = s.len() as u32;      //of the string
     len //returns the length of the string
} // As the scope ends in this function, the reference to the string drops and not the original string

//Function to return a slice type containing the first word of a string
fn first_word(s: &String) -> &str{ //Return type of the function is a slice to string (&str)
    let bytes = s.as_bytes(); //Transforms the String to bytes to improve readability of characters

    for (i, &element) in bytes.iter().enumerate(){ //The enumerate() method returns a tuple containing the current
        if element == b' '{                                    //iteration and value of the collection
            return &s[0..i]; //Returns the slice containing the first word when first space character is encountered
        }
    }

    &s[..] //Returns whole string if space is not encountered
} //Drops the reference to the string as scope ends