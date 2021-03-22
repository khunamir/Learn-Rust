fn main() {
    let logical : bool  = true;

    if logical {
        println!("Statement is true!");
    }

    let a_float: f64 = 1.0;  // Regular annotation
    let _an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;

    if mutable {
        println!("{}, {}, {}, {}, {}", a_float, _an_integer, default_float, default_integer, inferred_type);
    }
}