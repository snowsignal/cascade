# `cascade`: Cascade expressions in Rust!
`cascade` is a macro library for Rust that makes it easy and ergonomic
to use cascade-like expressions, similar to Dart.

```rust
#[macro_use]
extern crate cascade;

fn main() {
    let cascaded_list = cascade! {
      Vec::new();
      ..push("Cascades");
      ..push("reduce");
      ..push("boilerplate");
    };
    println!("{:?}", cascaded_list); // Will print '["Cascades", "reduce", "boilerplate"]'
}
```

This is only a small example of what `cascade` lets you do:
the `basic_cascades` example in this repository covers the other
cool features of the `cascade!` macro.

#### Why does this need to exist?
Cascades reduce boilerplate by eliminating the need for a 'temporary'
variable when making several method calls in a row, and it also
helps make struct member assignments look more ergonomic. For
example:
```rust
#[macro_use]
extern crate cascade;

#[derive(Clone, Debug)]
struct Person {
  pub name: String,
  pub age: u32,
  pub height: u32,
  pub friend_names: Vec<String>
}

fn main() {
    // Without cascades
    let person = Person {
      name: "John Smith",
      age: 17,
      height: 68, // 5' 8"
      friend_names: {
        let mut tmp_names = Vec::new();
        tmp_names.push("James Smith".to_string());
        tmp_names.push("Bob Jones".to_string());
        tmp_names.push("Billy Jones".to_string());
        tmp_names
      }
    };
    // With cascades
    let person = Person {
      name: "John Smith",
      age: 17,
      height: 68,
      friend_names: cascade! {
        Vec::new();
        ..push("James Smith".to_string());
        ..push("Bob Jones".to_string());
        ..push("Billy Jones".to_string());
      }
    };
    // Cascades also let you do cool stuff like this
    let person_one_year_later = cascade! {
      person;
      ..age += 1;
      ..height += 2;
    };
}
```

In addition, cascades make it easier to design fluent interfaces.
No more returning `self` with every single function!

### Changelog
**0.1.3**: The ? operator now works with cascades, for scenarios like this:
```rust
fn file_read() -> Result<SomeFileClass, ErrorMsg> {
    cascade! {
      SomeFileClass::create_file_reader("test.txt");
      ..load()?;
      ..check_validity()?;
    }
}
```

**0.1.2**: You can now chain methods together, like this:
```rust
fn chain_example() {
  cascade! {
    FnChainTest::new();
    ..chain().chain().chain();
  }
}
```

### Credits
Written by Jackson Lewis