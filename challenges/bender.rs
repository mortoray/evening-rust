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

#[derive(Clone,Copy,Debug,PartialEq)]
enum cell_type {
    Empty,
    Suicide, //?x
    ObstacleHard,
    ObstacleSoft, //can be destroyed
    South,
    East,
    North,
    West,
    Inverter,
    Beer,
    Teleporter, //2x
}

#[derive(Clone,Copy,Debug, PartialEq)]
enum direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone,Copy,Debug, PartialEq)]
struct point(i32, i32);

#[derive(Copy, Clone, PartialEq)]
struct bender_state {
    pos : point,
    inverted : bool,
    breaker : bool,
    dir : direction,
    dead : bool,
    version: i32,
}

impl bender_state {
    fn new() -> bender_state {
        bender_state { pos: point(0,0),
            inverted: false, 
            breaker: false, 
            dir: direction::South,
            dead : false,
            version: 0i32,
        }
    }
}

struct Matrix {
    cells : Vec<cell_type>,
    width : i32,
    height : i32,
}

impl Matrix {
    fn new( x : i32, y : i32 ) -> Matrix {
        Matrix {
            cells: Vec::new(),
            width: x,
            height: y,
        }
    }
    
    fn cur_push_pos(&self) -> point {
        let l = self.cells.len() as i32;
        point( ((l as usize) % (self.width as usize)) as i32, l / self.width)
    }
    
    fn push(&mut self, x : cell_type ) {
        self.cells.push(x);
    }
    
    fn get( &self, pos : point ) -> cell_type {
        self.cells[ (pos.1*self.width + pos.0) as usize ].clone()
    }
    
    fn set( &mut self, pos : point, v : cell_type ) {
        self.cells[ (pos.1*self.width + pos.0) as usize ] = v;
    }
    
    fn is_valid( &self, x : i32, y : i32 ) -> bool {
        return x >= 0 && x < self.width && 
            y >= 0 && y < self.height;
    }
    
    fn find_teleporter( &self, not_here : point ) -> point {
        for y in 0..self.height {
            for x in 0..self.width {
                if not_here.0 == x && not_here.1 == y {
                    continue;
                }
                
                if self.get( point(x,y) ) == cell_type::Teleporter {
                    return point(x,y);
                }
            }
        }
        
        print_errln!("Unreachable unless input broken");
        return point(0,0);
    }
    
    fn dump( &self, bp : point ) {
        for y in 0..self.height {
            for x in 0..self.width {
                if bp.0 == x && bp.1 == y {
                    print_err!( "@" );
                    continue;
                }
                print_err!("{}", match self.get( point(x,y) ) {
                    cell_type::Empty => " ",
                    cell_type::Suicide => "$",
                    cell_type::ObstacleHard => "#",
                    cell_type::ObstacleSoft => "X",
                    cell_type::South => "S",
                    cell_type::East => "E",
                    cell_type::North => "N",
                    cell_type::West => "W",
                    cell_type::Inverter => "I",
                    cell_type::Beer => "B",
                    cell_type::Teleporter => "T",
                })
            }
            print_errln!("");
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], i32);
    let c = parse_input!(inputs[1], i32);
    
    let mut town : Matrix = Matrix::new(c,l);
    let mut state : bender_state = bender_state::new();
    
    for _ in 0..l as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_right().to_string();
        for c in row.chars() {
            let ct = match c {
                '@' => {
                    //TODO: syntax?
                    state.pos = town.cur_push_pos();
                    cell_type::Empty
                },
                '$' => cell_type::Suicide,
                '#' => cell_type::ObstacleHard,
                'X' => cell_type::ObstacleSoft,
                'S' => cell_type::South,
                'E' => cell_type::East,
                'N' => cell_type::North,
                'W' => cell_type::West,
                'I' => cell_type::Inverter,
                'B' => cell_type::Beer,
                'T' => cell_type::Teleporter,
                _ => cell_type::Empty,
           };
           town.push(ct);
        }
    }
    
    town.dump(state.pos);

    let mut history : Vec<bender_state> = Vec::new();
    
    loop {
        state = step( &state, &town );
        if have_history( &state, &history ) {
            history.clear();
            println!("LOOP");
            break;
        }
        history.push( state );
        
        state = land( &state, &mut town );
        if state.dead {
            break;
        }
    }
    
    for h in history {
        println!("{}", match h.dir {
            direction::South => "SOUTH",
            direction::East => "EAST",
            direction::North => "NORTH",
            direction::West => "WEST",
        });
    }
    
    town.dump(state.pos);
}


fn step( state : &bender_state, town : &Matrix ) -> bender_state {
    let mut bs = state.clone();
    
    let next_pos = get_next_pos( &bs );
    if is_obstacle( &bs, town.get( next_pos ) ) {
        let dirs = &[ direction::South, direction::East, direction::North, direction::West ];
        let inv_dirs = &[ direction::West, direction::North, direction::East, direction::South ];
        for d in if bs.inverted { inv_dirs } else { dirs } {
            bs.dir = *d;
            let next_pos = get_next_pos(&bs);
            if !is_obstacle( &bs, town.get( next_pos ) ) {
                bs.pos = next_pos;
                break;
            }
        }
    } else {
        bs.pos = next_pos;
    }
    
    bs
}

fn land( state : &bender_state, town : &mut Matrix ) -> bender_state {
    let mut bs = state.clone();
    
    //TODO: why does this "not work" without cell_type::, but not compiler error
    let sq = town.get(bs.pos);
    match sq {
        cell_type::Empty => {
        },
        cell_type::Suicide => { 
            bs.dead = true;
        },
        cell_type::ObstacleHard => {
            print_errln!("On Obstable");
        },
        cell_type::ObstacleSoft => {
            town.set( bs.pos, cell_type::Empty );
            bs.version += 1;
        },
        cell_type::South => {
            bs.dir = direction::South;
        },
        cell_type::East => {
            bs.dir = direction::East;
        },
        cell_type::North => {
            bs.dir = direction::North;
        },
        cell_type::West => {
            bs.dir = direction::West;
        },
        cell_type::Inverter => {
            bs.inverted = !bs.inverted;
        },
        cell_type::Beer => {
            bs.breaker = !bs.breaker;
        },
        cell_type::Teleporter => {
            bs.pos = town.find_teleporter( bs.pos );
        },
    };
    
    bs
}

fn get_next_pos( bs : &bender_state ) -> point {
    match bs.dir {
        direction::North => point( bs.pos.0, bs.pos.1-1),
        direction::East => point( bs.pos.0+1, bs.pos.1 ),
        direction::West => point( bs.pos.0-1, bs.pos.1 ),
        direction::South => point( bs.pos.0, bs.pos.1+1 ),
    }
}

fn is_obstacle( bs : &bender_state, ct : cell_type ) -> bool {
    
    match ct {
        cell_type::ObstacleHard => true,
        cell_type::ObstacleSoft => !bs.breaker,
        _ => false,
    }
}

fn have_history( bs : &bender_state, history: &Vec<bender_state> ) -> bool {
    for h in history {
        if bs == h {
            return true;
        }
    }
    return false;
}
