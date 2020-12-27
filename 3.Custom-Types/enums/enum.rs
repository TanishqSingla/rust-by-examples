// Creating an `enum` to classify a web event.
// Each element is different and independent
enum WebEvent {
    // An `enum` may either be like `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or C-like structures.
    Click {x: i64, y: i64},
}

// A function which takes WebEvent enum as argument and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned string from string slice.
    let pasted = WebEvent::Paste("my text".to_owned()); 
    let click = WebEvent::Click {x: 20, y: 20};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload); 
}