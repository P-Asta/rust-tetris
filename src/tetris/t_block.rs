use super::t_pos::Pos;
use crate::tetris::t_built_in::built_in::make_shape;
use super::t_move::Move;

#[derive(Debug, Clone)]
pub struct Tblock {
    pub shape: Vec<Vec<usize>>,
    pub pos: Pos,
    pub id: usize,
    pub deg: usize
}
// struct도 public 인가요?
// 넹
// 멤버도 pub필요한건 써줘야해요 안그러면 접근못해요

// Clone이라는게 있군요
// 요데 저기도 Pos가 clone이없어서 posㄷ clone적어줘야해요
impl Tblock {
    pub fn new(id: usize, pos: Option<Pos>, deg:usize) -> Self{
        let pos = pos.unwrap_or_else(||Pos{x: 3, y: 0});
        let shape = make_shape(id, pos, deg);

        Self{
            shape: shape,
            pos: pos,
            id: id,
            deg: deg
        }

        /*
        use of moved value: `pos`
        value used here after moverustcClick for full compiler diagnostic
        t_block.rs(20, 36): value moved here
        t_block.rs(19, 13): move occurs because `pos` has type `Pos`, which does not implement the `Copy` trait
        저건 위에 make_shape할때 pos를 써주잖아요?
        근데 그떄 pos값이 사라져서그래요
        사라지지않게하려면 &pos를 적어야해요
        */
        // Copy붙이고 올게요 아하
        // 그래두되구요
        // 저건 &값을 사용못하는함수로 설정되어있어서 그런거에요
    }

    pub fn t_move(&mut self, direction: Move){
        if direction == Move::Left{
            self.pos.x -= 1;
        }else{
            self.pos.x += 1;
        }
        self.shape = make_shape(self.id, self.pos, self.deg)
    }

    // 여기가 문제인데
    // 여기가 mut self를 받잖아요?
    // 근데 이렇게 선언하면 이 함수를 타고 값이 사라져요 구조체가 사라져요
    // 그래서 & 을 써주면 값이 사라지지 않고 포인터만 주게 할 수 있어요
    //아하
    pub fn t_spin(&mut self){
        if self.deg + 1 >= 4{
            self.deg = 0;
        }else{
            self.deg += 1
        }
        self.shape = make_shape(self.id, self.pos, self.deg)
    }
}