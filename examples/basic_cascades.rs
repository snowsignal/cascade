#[macro_use]
extern crate cascade;

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
    height: u32
}

impl Person {
    pub fn blank() -> Person {
        Person {
            name: "".to_string(),
            age: 0,
            height: 0
        }
    }
}

#[derive(Clone, Debug)]
struct Chain {
    links: Vec<u32>
}

impl Chain {
    fn blank() -> Chain {
        Chain {
            links: vec!()
        }
    }
    fn add(mut self, link: u32) -> Self {
        self.links.push(link);
        self
    }
}

fn main() {
  // Cascades can be used recursively!
  let people = cascade! {
     Vec::new();
     .. push(cascade! {
         Person::blank();
         .. name = "John Smith".to_string();
         .. height = 72; // 6 feet, or 72 inches tall
         .. age = 34;
     });
     // This is what an expanded cascade looks like.
     ..push({
        let mut __tmp = Person::blank();
        __tmp.name = "Jason Smith".to_string();
        __tmp.height = 64;
        __tmp.age = 34;
        __tmp
     });
  };
  // Any expression works as the first statement of a cascade.
  let other_person = cascade! {
     people[0].clone();
     .. name = "Bob Smith".to_string();
     .. height = 66;
  };
  // You can also use +=, -=, *=, /= for operators
  let another_person = cascade! {
     other_person.clone();
     .. name = "Joan Smith".to_string();
     .. age += 3;
     .. height -= 4;
  };
  // You can use | instead of .. to specify a regular statement, instead of a .. member accessor.
  let yet_another_person = cascade! {
     people[0].clone();
     .. name = "James Smith".to_string();
     .. age = 27;
     | println!("Cascades in Rust are cool!");
     .. height -= 3;
  };
  // You can bind the initial value of the cascade to an identifier, which reflects the current state of the cascaded value.
  let one_more_person = cascade! {
    person: people[0].clone();
    | println!("'person' was equal to: {:?}", person);
    .. name = "James Smith".to_string();
    .. height = ((person.height as f32) * 0.8) as u32;
    | println!("'person' is now equal to: {:?}", person);
  };
  // As of version 0.1.2, you can also chain methods together. Observe:
  let method_chain_example = cascade! {
    ch: Chain::blank();
    ..add(5).add(6).add(7); // In this case, ch is consumed here. So we have to shadow ch to avoid an error. Obviously, this isn't the most useful way to method-chain.
    | let ch = ();
  };
  println!("People: {:?}, {:?}, {:?}, {:?}, {:?}", people, other_person, another_person, yet_another_person, one_more_person);
}
