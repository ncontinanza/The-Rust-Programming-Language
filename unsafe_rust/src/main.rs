unsafe fn dangerous() {}

// We can use extern to call external code
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;
    
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer

    let address = 0x012345usize;
    let r = address as *const i32; // raw pointer to a location in memory
    /*
     * Trying to use arbitrary memory is undefined: there might or might not be data there.
     * The compiler might optimize the code so there is no memory access, or the program
     * might error with a segmentation fault. 
     */
    
     unsafe { // To dereference a raw pointer and read the data being pointed to we must annotate the block as unsafe.
         println!("r1 is: {}", *r1);
         println!("r2 is: {}", *r2);
     }

     unsafe { // Calling unsafe functions also needs an unsafe block.
         dangerous();
     }

     // Using an external function is always unsafe.
     unsafe {
         println!("Absolute value of -3 according to C: {}", abs(-3));
     }
}
