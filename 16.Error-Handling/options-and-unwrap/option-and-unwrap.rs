// The commoner has seen it all, and can handle any gift well.
// All gifts are handled explicitly by `match`.
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner)   => println!("{}? how nice.", inner),
        None          => println!("No gift? Oh well."), 
    }
}

// Our shelter royal will `panic` at the sight of snake.
// ALl gifts are handled implicitly by `unwrap`.
fn give_royal(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it recieves a `None`
    let indside = gift.unwrap();
    if indside == "snake" {panic!("AAAAaaaa!!!");}


    println!("I love {}s!!!", indside);
}

fn main() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_royal(bird);
    give_royal(nothing);
}