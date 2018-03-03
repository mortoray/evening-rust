use std::io;

macro_rules! print_errln {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            write!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// -1 == unknonw
// -2 == bomb
struct Matrix {
    rooms : Vec<i32>,
    width : i32,
    height : i32,
    
}

impl Matrix {
    fn new( x : i32, y : i32 ) -> Matrix {
        Matrix {
            rooms: Vec::new(),
            width: x,
            height: y,
        }
    }
    
    fn push(&mut self, x : i32 ) {
        self.rooms.push(x);
    }
    
    fn get( &self, x : i32, y : i32 ) -> i32 {
        self.rooms[ (y*self.width + x) as usize ]
    }
    
    fn set( &mut self, x : i32, y : i32, v : i32 ) {
        self.rooms[ (y*self.width + x) as usize ] = v;
    }
    
    fn is_valid( &self, x : i32, y : i32 ) -> bool {
        return x >= 0 && x < self.width && 
            y >= 0 && y < self.height;
    }
    
    fn enum_cell<F>( &self, x: i32, y : i32, mut cb : F) 
        where F : FnMut(i32,i32,i32) {
        for cx in (x-1)..(x+2) {
            for cy  in (y-1)..(y+2) {
                if !self.is_valid(cx,cy) {
                    continue;
                }
                cb( cx, cy, self.get(cx,cy) );
            }
        }
    }
    
    fn mut_enum_cell<F>( &mut self, x: i32, y : i32, mut cb : F) 
        where F : FnMut(i32,i32,i32, &mut Matrix) {
        for cx in (x-1)..(x+2) {
            for cy  in (y-1)..(y+2) {
                if !self.is_valid(cx,cy) {
                    continue;
                }
                cb( cx, cy, self.get(cx,cy), self );
            }
        }
    }
    
    // fn mut_enum_grid<F>( &mut self, mut cb : F )
    //     where F : FnMut(i32,i32,&mut Matrix) {
        
    //     for 
    
    fn count_unknown( &self, x : i32, y : i32 ) -> i32 {
         let mut c = 0;
         self.enum_cell( x, y, | x, y, v | {
            if v == -1 {
                c += 1;
            }
        });
        return c;
    }
}

fn place_bomb( grid : &mut Matrix, x : i32, y : i32 ) {
    print_errln!("place_bomb {} {}", x, y);
    grid.set( x, y, -2 );
    
    grid.mut_enum_cell( x, y, | cx, cy, v, grid | {
        if v > 0 {
            grid.set( cx, cy, v - 1);
        }
    })
}

fn place_bombs( grid: &mut Matrix, x : i32, y : i32 ) {
    print_errln!("place_bombs {} {}", x, y);
    grid.mut_enum_cell( x, y, | cx, cy, v, grid | {
        if v == -1 {
            place_bomb( grid, cx, cy );
        }
    });
}

fn has_zero( grid: &Matrix, x : i32, y: i32) -> bool {
    let mut result = false;
    grid.enum_cell( x, y, | cx, cy, v | {
        if v == 0 {
            result = true;
        }
    });
    return result;
}

fn find_bombs( grid : &mut Matrix, nb : &mut i32 ) -> bool {
    //find known bombs
    for r in 0..grid.height {
       for c in 0..grid.width {
            let v = grid.get(c,r);
            if v < 1 {
                continue;
            }
            
            let qs = grid.count_unknown(c,r);
            if qs == v {
                *nb -= v;
                place_bombs( grid, c, r );
                return true;
            }
        }
    }
    
    //replace ? beside 0 with 0
    for c in 0..grid.width {
        for r in 0..grid.height {
            let v = grid.get(c,r);
            if v != -1 {
                continue;
            }
            if has_zero( &grid, c, r ) {
                grid.set(c,r, -3);
                return true;
            }
        }
    }
    
    return false;
}

fn print_grid( grid: & Matrix ) {
    for r in 0..grid.height {
        for c in 0..grid.width {
            let c = grid.get(c,r);
            if c == -1 {
                print_err!("?");
            } else if c == -2 {
                print_err!("*");
            } else if c == -3 {
                print_err!("-");
            } else if c == 0 {
                print_err!(".");
            } else {
                print_err!("{}", c);
            }
        }
        print_errln!("");
    }
    print_errln!("");
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let h = parse_input!(inputs[0], i32);
    let w = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut nb = parse_input!(input_line, i32);
    
    let mut grid = Matrix::new(w,h);
    for i in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_right().to_string();
        for c in line.chars() {
            if c == '.' {
                grid.push(0);
            } else if c == '?' {
                grid.push(-1);
            } else {
                grid.push( (c as i32) - ('0' as i32) );
            }
        }
    }

    print_grid(&grid); 
    while find_bombs(&mut grid, &mut nb) {
        print_grid(&grid);
    }
    
    //needed for hidden bombs
    if nb > 0 {
        for c in 0..grid.width {
            for r in 0..grid.height {
                if grid.get(c,r) == -1 {
                    place_bomb( &mut grid, c,r );
                }
            }
        }
    }
    
    //print bombs
    for c in 0..grid.width {
        for r in 0..grid.height {
            if grid.get(c,r) == -2 {
                println!("{} {}", c, r);
            }
        }
    }
}