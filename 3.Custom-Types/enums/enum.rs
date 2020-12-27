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

// Type aliases
enum VeryVerboseEnumOfThingToDoWithNumbers {
    Add,
    Subtract,
}

// Creating an alias of above enum
type Operations = VeryVerboseEnumOfThingToDoWithNumbers;

// Most common example of type alias
impl VeryVerboseEnumOfThingToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,            
        }
    }
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

    // Refering the type alias we created for enum
    let x = Operations::Add;
}