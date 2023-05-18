// use macro_rule! <name of macro> { <Body> }
macro_rules! add {
    // match like arm for macro
    ($a:expr, $b:expr) => {{
        // $a and $b will be templated using the value / variable provided to macro
        $a + $b
    }};

    // Second arm match add!(1), add!(2) etc
    ($a:expr) => {{
        $a
    }};
}

// use macro_rule! <name of macro> { <Body> }
macro_rules! add_as {
    // using a ty token type for matching datatypes passed to macro
    // ($a:expr,$b:expr,$typ:ty) => {
    //     $a as $typ + $b as $typ
    // };

    ($typ:ty, $($a:expr),*) => {
        {
            0 as $typ
            $(+$a as $typ)*
        }
    }
}

fn main() {
    // println!("{}", add!(1, 2));
    let x = 0;
    println!("{}", add!(1, 2));
    println!("{}", add!(x));
    println!("{}", add_as!(u8, 0, 2, 3));
}
