macro_rules! quine {
    ($t:expr) => {($t)(stringify!($t));}
}

fn main() {
    quine!(|s|
    {
        println!("macro_rules! quine {{
    ($t:expr) => {{($t)(stringify!($t));}}
}}

fn main() {{
    quine!({});
}}",
                 s)
    });
}
