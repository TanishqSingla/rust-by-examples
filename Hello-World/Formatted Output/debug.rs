/*
    Rust has some really cool and useful options when it comes to formatted output
    here are some of them
*/


fn main() {
    // In general `{}` is used as a placeholder in strings to replace it with a value

    //Example
    println!("Novemeber has {} days", 30);

    // You can store a number in `{}` like this `{1}` to refer the position of a value

    //Example
    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");

    /* 
        The above statement would output:
        Alice this is Bob, Bob this is Alice 
    */

    //Special formatting can be applied using `:` symbol inside `{}`

    //Example
    println!("{} of {:b}, know binary", 1, 2);
    
    /*
        This would output
        1 of 10 people know binary

        Note: 10 here is the binary representation of 2
    */

    // you can also put a named argument inside `{}`

    //Example
    println!(
        "{subject} {verb} {object}", 
        object = "the lazy dog", 
        verb = "jumps over", 
        subject = "the quick brown fox"
    );

    //Specify width to align your text differntly by specifying `:width` in `{}`
    println!("{number:>width$}", number=1, width=6); 

    /*
        This will output the number after 5 spaces i.e "     1"
    */

}