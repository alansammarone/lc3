// https://docs.rs/strum_macros/0.26.4/strum_macros/derive.EnumIter.html

// You need to bring the trait into scope to use it!
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
enum Color {
    Red,
    Green { range: usize },
    Blue(usize),
    Yellow,
}

fn main() {
    // It's simple to iterate over the variants of an enum.
    for color in Color::iter() {
        println!("My favorite color is {:?}", color);
    }

    let mut ci = Color::iter();
    assert_eq!(Some(Color::Red), ci.next());
    assert_eq!(Some(Color::Green { range: 0 }), ci.next());
    assert_eq!(Some(Color::Blue(0)), ci.next());
    assert_eq!(Some(Color::Yellow), ci.next());
    assert_eq!(None, ci.next());
}
