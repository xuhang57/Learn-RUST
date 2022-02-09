// The relationships between various data types in Rust are established using traits.

// Trait Objects
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vev<&Show> = vec![&answer, &maybe_pi];
    for d in v.iter() {
        println!("show {}", d.show());
    }
}

// show four-byte signed 42
// show eight-byte float 3.14

// note that i32 and f64 have no relationship to each other, but they both understand the show method because they both implement the same trait.
// This method is virtual, because the actual method has different code for different types, and yet the correct method is invoked based on runtime

// A box contains a reference to data allocated on th heap, and acts very much like a reference --> smart pointer
let answer = Box::new(42);
let maybe_pi = Box::new(3.14);

// The difference is that you can now take this vector, pass it as a reference or give it away without having to track any borrowed references.
let show_list: Vec<Box<Show>> = vec![answer, maybe_pi];
for d in &show_list {
    println!("show {}", d.show());
}

// Animals
trait Quack {
    fn quack(&self);
}

struct Duck ();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

struct RandomBird {
    is_a_parrot: bool
}

impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

let duck1 = Duck();
let duck2 = RandomBird{is_a_parrot: false};
let parrot = RandomBird{is_a_parrot: true};

let ducks: Vec<&Quack> = vec![&duck1, &duck2, &parrot];

for d in &ducks {
    d.quack();
}

// quack!
// quack!
// squawk!
