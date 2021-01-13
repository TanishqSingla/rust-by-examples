fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {panic!("AAAAaaaaa!")}

    println!("Some refreshing {} is all I need", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}