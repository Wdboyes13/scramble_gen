#![feature(int_roundings)]
#![feature(random)]

use std::fmt::{Display, Write};
use clap::Parser;
use std::random::random;

fn random_bool() -> bool {
    random(..)
}

fn random_int(min: i32, max: i32) -> i32 {
    let range = max - min;
    let threshold = i32::MAX - (i32::MAX % range);
    loop {
        let x: i32 = random(..);
        let x = x.abs();
        if x < threshold {
            return x % range + min;
        }
    }
}

#[derive(Copy, Clone)]
enum Face {
    R, L, U, D, B, F
}

impl Face {
    fn to_char(self) -> char {
        const FACES: [char; 6] = ['R', 'L', 'U', 'D', 'B', 'F'];
        FACES[self as usize]
    }
    fn from_i32(x: i32) -> Self {
        const FACES: [Face; 6] = [Face::R, Face::L, Face::U, Face::D, Face::B, Face::F];
        FACES[x as usize]
    }
}

struct Mods {
    x2: bool,
    prime: bool,
    wide: bool,
    slice: Option<u32>
}

impl Mods {
    fn new() -> Self {
        Self {
            x2: false,
            prime: false, 
            wide: false,
            slice: None
        }
    }

    fn rand(x: i32) -> Self {
        let mut ret = Self::new();

        ret.x2 = random_bool();
        ret.prime = random_bool();

        if x >= 4 {
            ret.wide = random_bool();
        } 
        
        if x >= 5 && random_bool() {
            ret.slice = Some(random_int(3, x.div_ceil(2)) as u32);
        }

        // we dont want both lol
        if ret.wide && ret.slice.is_some() {
            let wide_or_slice = random_bool();
            if wide_or_slice {
                ret.slice = None;
            } else {
                ret.wide = false;
            }
        }

        ret
    }
}

struct Move {
    face: Face,
    mods: Mods
}

struct Seq {
    val: Vec<Move>
}

impl Seq {
    fn generate(len: usize, cubex: i32) -> Self {
        let mut sq = Vec::<Move>::with_capacity(len);

        let mut pface: i32 = -1;
        for _ in 0..len {
            let mut face: i32 = random_int(0, 6);
            while face == pface {
                face = random_int(0, 6);
            }

            sq.push(Move{
                face: Face::from_i32(face),
                mods: Mods::rand(cubex)
            });

            pface = face;
        }

        Self{val: sq}
    }
}

impl Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        

        for mv in &self.val {
            if let Some(slice) = mv.mods.slice {
                f.write_str(format!("{}", slice).as_str())?;
            }

            f.write_char(mv.face.to_char())?;

            if mv.mods.wide || mv.mods.slice.is_some() {
                f.write_char('w')?;
            }

            if mv.mods.x2 {
                f.write_char('2')?;
            }

            if mv.mods.prime {
                f.write_char('\'')?;
            }

            f.write_char(' ')?;
        }

        Ok(())
    }
}

#[derive(Parser)]
#[command(after_help = r#"Notation:
Uses the standard WCA notation
Faces:
    R - Right face
    L - Left face
    U - Up/top face
    D - Down/bottom face
    F - Front face
    B - Back face
Modifiers:
    '    - Rotate counter-clockwise
    2    - Rotate twice (180°)
    XFw  - Rotate X layers of F side
    Fw   - Rotate 2 layers of F side
    None - Rotate clockwise once (90°)
"#)]
struct Cli {
    #[arg(short = 'l', long = "length", help = "Generate scramble with <LEN> moves", default_value_t = 20)]
    len: usize,
    #[arg(short = 's', long = "size", 
    help = "Generate a scramble for X size cube, for example if value is 5 the scramble will be for a 5x5 cube", default_value_t = 3)]
    cubesize: i32,
    #[arg(short = 'n', long = "num", help = "Generate x number of scrambles", default_value_t = 1)]
    num: i32,
}

fn main() {
    let cli = Cli::parse();
    for _ in 0..cli.num {
        println!("{}", Seq::generate(cli.len, cli.cubesize));
    }
}
