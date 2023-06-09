use rand::{thread_rng, Rng};

use crate::tetris::t_block::Tblock;

#[derive(Clone)]
pub struct Bag{
    bag: Vec<Vec<usize>>,
    hold: usize
}

impl Bag{
    pub fn new() -> Self{
        Self {
            bag: vec![Self::spawn_7bag(), Self::spawn_7bag()],
            hold: 0
        }
    }

    fn spawn_7bag() -> Vec<usize>{
        let mut blocks: Vec<usize> = vec![];
        let mut block = 0;
        
        while blocks.len() != 7{
            let mut rng = thread_rng();
            block = rng.gen_range(1..8);
            if !blocks.contains(&block){
                blocks.push(block)
            }
        }
        // blocks.push(block);
        blocks
    }

    /// (before, after)
    pub fn next(&mut self) -> (usize, usize){
        let before: usize = self.bag[0].remove(0);
        if self.bag.len() < 2{
            self.bag.push(Self::spawn_7bag())
        }

        let after = if self.bag[0].len() == 0{
            self.bag.remove(0);
            self.bag[0][0]
        }else{
            self.bag[0][0]
        };
        
        (before, after)
    }
    pub fn hold(&mut self, block: usize) -> usize{
        if self.hold == 0{
            let a = self.next().0;
            self.hold = block;
            a
        }else{
            let a = self.hold.clone();
            self.hold = block;
            a
        }
    }
}


impl std::fmt::Debug for Bag{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut a = 0;
        let mut bags = String::new();
        let mut idx = 0;
        for i in &self.bag{
            for j in 0..i.len(){
                bags.push_str(
                    Tblock::chang_en(self.bag[idx][j])
                );
                bags.push_str(" ");
                a += 1;
                if a == 5{ break }
            }
            if a == 5{ break };
            idx += 1;
        }
        
        write!(f, "{} | {}", Tblock::chang_en(self.hold), bags)
    }
}