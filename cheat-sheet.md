Ich mache CodinGame jetzt Live auf https://www.twitch.tv/mortoray

I'm now battling CodinGame live at https://www.twitch.tv/mortoray


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


TODO:
- repeat a string