use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Rectangle {
    width: u32,
    height: u32,
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    #[derive(Hash)]
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }

    let person1 = Person {
        id: 5,
        name: "Janet".to_string(),
        phone: 555_666_7777,
    };
    let person2 = Person {
        id: 5,
        name: "Bob".to_string(),
        phone: 555_666_7777,
    };
    let person3 = Person {
        id: 5,
        name: "Bab".to_string(),
        phone: 555_666_7777,
    };


    println!("{}", calculate_hash(&person1));
    println!("{}", calculate_hash(&person2));
    println!("{}", calculate_hash(&person3));

    assert!(calculate_hash(&person1) != calculate_hash(&person2));

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
