// Calling the C stdlib is the most common external lang to access
// IT's considered unsafe, so you need to define a wrapper around it 
// (that's our abs function here)
// To run it, it has to be within an unsafe block too
// Overall, this isn't recommended at all, but is here to show that it _is_ possible
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C stdlib: {}", abs(-3));
    }
}
