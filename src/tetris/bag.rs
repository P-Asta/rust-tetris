use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Bag{
    bag: Vec<Vec<usize>>
}

impl Bag{
    pub fn new() -> Self{
        Self {
            bag: vec![Self::spawn_7bag(), Self::spawn_7bag()]
        }
    }

    fn spawn_7bag() -> Vec<usize>{
        let mut blocks: Vec<usize> = vec![];
        while blocks.len() != 7{
            let mut rng = thread_rng();
            let block = rng.gen_range(1..8);
            if !blocks.contains(&block){
                blocks.push(block)
            }
        }
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
}


impl std::fmt::Debug for Bag{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}| {}", self.bag, self.bag[0].len())
    }
}