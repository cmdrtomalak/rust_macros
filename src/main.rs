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

fn main() {
    // println!("{}", add!(1, 2));
    let x = 0;
    println!("{}", add!(1, 2));
    println!("{}", add!(x));
}
