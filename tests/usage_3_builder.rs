use abin::{Bin, BinBuilder, Factory, NewBin, NewStr, Str, StrBuilder, StrFactory, StrSegment};

use crate::Action::{GreetTwo, SayGoodByeTo, SayHelloTo, ShutUp};

/// This is more or less efficient (does not require additional allocation).
#[test]
fn builder_with_single_item() {
    // binary
    let mut builder = NewBin::builder();
    builder.push(NewBin::from_static("Hello, world!".as_bytes()));
    let built_bin = builder.build();
    assert_eq!(built_bin, NewBin::from_static("Hello, world!".as_bytes()));

    // string
    let mut builder = NewStr::builder();
    builder.push(NewStr::from_static("Hello, world!"));
    let built_str = builder.build();
    assert_eq!(built_str, NewStr::from_static("Hello, world!"));
}

/// How to build strings dynamically using a builder.
#[test]
fn text_from_action() {
    assert_eq!(
        "Hello, World!",
        process_action(SayHelloTo(NewStr::from_static("World"))).as_str()
    );
    assert_eq!(
        "See you tomorrow, Lucie.",
        process_action(SayGoodByeTo(NewStr::from_static("Lucie"))).as_str()
    );
    assert_eq!(
        "Nice to see you, Simon and Garfunkel!",
        process_action(GreetTwo(
            NewStr::from_static("Simon"),
            NewStr::from_static("Garfunkel")
        ))
        .as_str()
    );
    assert_eq!("", process_action(ShutUp).as_str());
}

enum Action {
    SayHelloTo(Str),
    SayGoodByeTo(Str),
    GreetTwo(Str, Str),
    ShutUp,
}

fn process_action(action: Action) -> Str {
    let mut builder = NewStr::builder();
    match action {
        Action::SayHelloTo(name) => {
            builder.push_static("Hello, ");
            builder.push(name);
            builder.push_static("!");
        }
        Action::SayGoodByeTo(name) => {
            builder.push_static("See you tomorrow, ");
            builder.push(name);
            builder.push_static(".");
        }
        Action::GreetTwo(name1, name2) => {
            builder.push_static("Nice to see you, ");
            builder.push(name1);
            builder.push_static(" and ");
            builder.push(name2);
            builder.push_static("!");
        }
        Action::ShutUp => {
            // <..>
        }
    }
    builder.build()
}
