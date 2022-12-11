// 여기는 crate::tetris::t_built_in

pub mod built_in{
    // 여기는 crate::tetris::t_built_in::built_in
    use super::super::t_pos::Pos;
    pub fn make_shape(id: usize, pos: Pos) -> Vec<Vec<usize>>{
        match id {
            1 => {
                vec![
                    vec![0 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 0 + pos.y],
                    vec![2 + pos.x , 0 + pos.y],
                    vec![3 + pos.x , 0 + pos.y],
                ]
            }
            2 => {
                vec![
                    vec![0 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 0 + pos.y],
                    vec![2 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 1 + pos.y],
                ]
            }
            3 => {
                vec![
                    vec![0 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 0 + pos.y],
                    vec![0 + pos.x , 1 + pos.y],
                    vec![1 + pos.x , 1 + pos.y],
                ]
            }
            4 => {
                vec![
                    vec![0 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 0 + pos.y],
                    vec![2 + pos.x , 0 + pos.y],
                    vec![2 + pos.x , 1 + pos.y],
                ]
            }
            5 => {
                vec![
                    vec![0 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 0 + pos.y],
                    vec![2 + pos.x , 0 + pos.y],
                    vec![0 + pos.x , 1 + pos.y],
                ]
            }
            6 => {
                vec![
                    vec![0 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 0 + pos.y],
                    vec![1 + pos.x , 1 + pos.y],
                    vec![2 + pos.x , 1 + pos.y],
                ]
            }
            _ => {
                vec![
                    vec![1 + pos.x , 0 + pos.y],
                    vec![2 + pos.x , 0 + pos.y],
                    vec![0 + pos.x , 1 + pos.y],
                    vec![1 + pos.x , 1 + pos.y],
                ]
            }
        }
    }
}