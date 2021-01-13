macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Forces types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // Decompose mutiple `eval's` recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! {eval $e}
        calculate! {$(eval $es),+ }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}