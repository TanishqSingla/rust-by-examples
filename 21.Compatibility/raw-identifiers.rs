extern crate foo;

fn main() {
    // Error: expected identifier found keyword
    // foo::try();

    foo::r#try();
}