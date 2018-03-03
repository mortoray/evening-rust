Rust Lernen auf CodinGame, jetzt Live auf https://www.twitch.tv/mortoray

Battling CodinGame with Rust, now live on https://www.twitch.tv/mortoray


#![deny(warnings)]

```rs
let dirs = vec![ "NW", "N", "NE",
	"W", "", "E",
	"SW", "S", "SE" ];
```


```rs
loop {
}
```


```rs
let mut step_x : isize = 0;
let dir_ndx : usize = expr as usize;
```

```rs
println!("{},{}", whatever, type);
```

```{.rs}
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
```

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```


            let row_str = &letters[row];
            //let substr = row_str.chars()[from..to].collect();
            //let substr = &row_str[from..to];
            let substr : String = row_str.chars().skip(from as usize).take(
                (to-from) as usize).collect();


```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

```rs
use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut s = input_line.trim().to_string();

    let g = "I am not a good speller Sometimes I mix up the spelling of words and sometimes I even forget to put periods Please count the number of words I have misspelled and the number of periods I have missed";
    let d = s.chars().filter(|&c| c == '.').count();
    
    s = s.replace(".","");
    
    let w = g.split(' ').zip(s.split(' ')).filter(|&(a, b)| a != b).count();
    
    println!("{} {}", w, 3 - d);
}
```

```rs
fn ref_multiple<'a, T>(vec: &'a mut Vec<T>, index_1: usize, index_2: usize) -> (&'a mut T, &'a mut T) {
   if index1 == index2 {
       panic!("Indices were the same, dummy");
   };

   let item_1 = unsafe { &mut *(&mut vec[index_1] as *mut T) };
   let item_2 = unsafe { &mut *(&mut vec[index_2] as *mut T) };

   (item_1, item_2)
}```

```rs
enum BouncyIteratorDirection {
   Right,
   Left,
}

struct BouncyIterator<T> {
   inner: Iterator<T>, // Take items from inner iterator until exhausted
   cache: Vec<T>, // Stuff items from iterator into a cache so we can go back over them in the other direction
   direction: BouncyIteratorDirection,
}

impl<T> BouncyIterator<T> {
   pub fn new(iter: Iterator<T>) -> BouncyIterator<T> {
       BouncyIterator {
           inner: iter,
           cache: Vec::new(),
           direction: BouncyIteratorDirection::Right,
       }
   }
}

impl Iterator for BouncyIterator
 ...
 ```

 https://doc.rust-lang.org/std/iter/index.html#implementing-iterator

TODO:
- repeat a string
- increment by 2 in a for loop
- global mutable variable (like a counter)
- slice a string
- difference between str, string, String, "string", etc.
