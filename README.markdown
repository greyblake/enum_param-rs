# EnumParam

Allows you to iterate through cartesian product of vectors of any type.
Powered by Rust generics.

```rust
extern crate enum_param;
use enum_param::*;

fn main() {
    let digits = EnumParam::new(vec![1, 2, 3]);
    let letters = EnumParam::new(vec!['a', 'b', 'c']);

    // Combine digits and numbers
    let digits_letters = EnumParam2::new(digits, letters);

    println!("\nDigits and letters");
    for item in digits_letters.iter() {
        println!("{:?}", item);
    }

    let words = EnumParam::new(vec!["hi", "bye"]);

    // Combine digits, numbers and words
    let digits_letters_words = EnumParam2::new(digits_letters, words);

    println!("\nDigits, letters and words");
    for item in digits_letters_words.iter() {
        println!("{:?}", item);
    }
}
```

Output:

```
Digits and letters
(1, 'a')
(1, 'b')
(1, 'c')
(2, 'a')
(2, 'b')
(2, 'c')
(3, 'a')
(3, 'b')
(3, 'c')

Digits, letters and words
((1, 'a'), "hi")
((1, 'a'), "bye")
((1, 'b'), "hi")
((1, 'b'), "bye")
((1, 'c'), "hi")
((1, 'c'), "bye")
((2, 'a'), "hi")
((2, 'a'), "bye")
((2, 'b'), "hi")
((2, 'b'), "bye")
((2, 'c'), "hi")
((2, 'c'), "bye")
((3, 'a'), "hi")
((3, 'a'), "bye")
((3, 'b'), "hi")
((3, 'b'), "bye")
((3, 'c'), "hi")
((3, 'c'), "bye")

```
