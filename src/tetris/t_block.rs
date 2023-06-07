use rand::{thread_rng, Rng};

use super::{t_pos::Pos};
use crate::tetris::t_built_in::built_in::make_shape;
use super::t_move::Move;

#[derive(Debug, Clone)]
pub struct Tblock {
    pub shape: Vec<Vec<usize>>,
    pub pos: Pos,
    pub id: usize,
    pub deg: usize
}

impl Tblock {
    pub fn new(id: usize, pos: Option<Pos>, deg: usize) -> Self{
        let pos = pos.unwrap_or_else(||Pos{x: 3, y: 0});
        let shape = make_shape(id, pos, deg).unwrap();

        Self{
            shape: shape,
            pos: pos,
            id: id,
            deg: deg
        }
    }
    pub fn chang_en(id: usize) -> &'static str{
        match id{
            1 => "I",
            2 => "T",
            3 => "O",
            4 => "J",
            5 => "L",
            6 => "Z",
            7 => "S",
            _ => "?"
        }
    }
    
    pub fn t_move(&mut self, direction: Move){
        match direction{
            Move::Left => {
                self.pos.x -= 1;
                match make_shape(self.id, self.pos, self.deg) {
                    Ok(ok) => { self.shape = ok }
                    Err(_) => { self.pos.x += 1 }
                }
            }
            Move::Right => {
                self.pos.x += 1;
                match make_shape(self.id, self.pos, self.deg) {
                    Ok(ok) => { self.shape = ok }
                    Err(_) => { self.pos.x -= 1 }
                }
            }
        }
    }

    pub fn is_none(&self) -> bool{
        match make_shape(0, self.pos, 0){
            Ok(ok) => {self.shape.clone() == ok}
            Err(_) => {
                false
            }
        }
    }

    pub fn t_spin(&mut self){
        if self.deg + 1 >= 4{
            self.deg = 0;
        }else{
            self.deg += 1
        }
        match make_shape(self.id, self.pos, self.deg) {
            Ok(ok) => { self.shape = ok }
            Err(_) => {
                if self.deg == 0{
                    self.deg = 3
                }else{
                    self.deg -= 1
                }
            }
        }
    }
}