fn main() {
    let x = 5;

    let x = x + 1; // This is called "shadowing". The new variable "x" shadows the previous variable "x".

    {
        let x = x * 2; // This is also shadowing. The new variable "x" shadows the previous variable "x" within this block.
        println!("The value of x in the inner scope is: {x}");  
    }

    println!("The value of x is: {x}"); // This will print 6, because the inner variable "x" is only valid within the inner scope.
}
