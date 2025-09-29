// Oh god I get back to Rust and now we're doing unsafe code.
// I specifically signed up for Rust because it was safe.
// I feel like I've been betrayed.

// Calls the slice function from the standard library. Slice is used to handle
// dynamically sized arrays.
use std::slice;

// A mutable static variable. A static variable is a variable that is
// valid for the entire duration of the program. Mutable static variables
// are unsafe because they can be changed from multiple threads at the same time.

static mut COUNTER: u32 = 0;

// An external function. This is a function that is defined in another
// language, in this case C. The `extern "C"` part specifies that the
// function uses the C calling convention. The `unsafe` part specifies
// that calling this function is unsafe because it is not checked by
// the Rust compiler. god damnit.

// How many comments am I gonna have to write here to prove I understand why this is all unsafe.
// Crossing the road without looking both ways is also unsafe. You know. Just while we're discussing unsafe things.
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

// Unsafe trait. An unsafe trait is a trait that has some invariants that
// the implementer must uphold. Implementing an unsafe trait is unsafe
// because the compiler cannot check that the implementer has upheld them.
unsafe trait Foo {
    // methods go here
}

// And now another one for i32. Great. I hate this.
unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    
    // Raw pointers. Raw pointers are like references, but they are not
    // checked by the Rust compiler. They can be null, they can be dangling,
    // and they can be unaligned. Dereferencing a raw pointer is unsafe
    // because the compiler cannot check that the pointer is valid.
    let address = 0x01234usize;
    let r = address as *mut i32;

    // Creating a slice from a raw pointer. This is unsafe because the
    // compiler cannot check that the pointer is valid or that the length
    // is correct.
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // Declares a mutable number that equals five. I cannot wait to find out why this is also unsafe.
    let mut num = 5;

    // Creates a raw pointer to the number. One is immutable, one is mutable.
    // This is unsafe because the compiler cannot check that the pointers are valid.
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // Unsafe printing. Amazing. 
    // Dereferencing raw pointers is unsafe because the compiler cannot
    // check that the pointers are valid.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calls the split_at_mut function. This is safe because the function
    // checks that the mid index is valid.
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // This is safe because we know that 3 is a valid index.
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Calls the external C function. This is unsafe because the compiler
    // cannot check that the function is valid or that the arguments
    // are correct. I'm just gonna avoid using unsafe tbh.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Modifies the mutable static variable. This is unsafe because
    // the compiler cannot check that the variable is not being
    // modified from multiple threads at the same time.

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

// A safe abstraction over unsafe code. This function takes a mutable
// slice and an index, and returns two mutable slices that split
// the original slice at the given index. This is safe because the
// function checks that the index is valid.

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // This is unsafe because the compiler cannot check that the
    // pointers are valid or that the lengths are correct.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// An unsafe function. An unsafe function is a function that has
// some invariants that the caller must uphold. Calling an unsafe
// function is unsafe because the compiler cannot check that the
// caller has upheld them.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// I'm going to go play some Kingdom Come Deliverance 2 now. Henry makes me feel safe.