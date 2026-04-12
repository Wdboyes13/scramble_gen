#![feature(int_roundings)]
#![feature(random)]

use std::fmt::{Display, Write};
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
pub enum Face {
    R, L, U, D, B, F
}

impl Face {
    pub fn to_char(self) -> char {
        const FACES: [char; 6] = ['R', 'L', 'U', 'D', 'B', 'F'];
        FACES[self as usize]
    }
    pub fn from_i32(x: i32) -> Self {
        const FACES: [Face; 6] = [Face::R, Face::L, Face::U, Face::D, Face::B, Face::F];
        FACES[x as usize]
    }
}

pub struct Mods {
    pub x2: bool,
    pub prime: bool,
    pub wide: bool,
    pub slice: Option<u32>
}

impl Mods {
    pub fn rand(x: i32) -> Self {
        let mut ret = Self {
            x2: random_bool(),
            prime: random_bool(),
            wide: false,
            slice: None
        };

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

pub struct Move {
    pub face: Face,
    pub mods: Mods
}

pub struct Seq {
    pub val: Vec<Move>
}

impl Seq {
    pub fn generate(len: usize, cubex: i32) -> Self {
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