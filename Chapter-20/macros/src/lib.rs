// Oh god we're doing macros now. God help me.
// This is a simple implementation of a vec! macro that allows us to create vectors easily.
// It takes a variable number of arguments and creates a Vec containing those elements.

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}