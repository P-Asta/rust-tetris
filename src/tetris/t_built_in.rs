// 여기는 crate::tetris::t_built_in

pub mod built_in{
    // 여기는 crate::tetris::t_built_in::built_in
    use super::super::t_pos::Pos;
    pub fn make_shape(id: usize, pos: Pos, deg: usize) -> Vec<Vec<usize>>{
        let mut block = match id {
            1 => {
                vec![
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![3 , 0],
                    ],
                    vec![
                        vec![1 , 1], vec![1 , 0], vec![1 , 1], vec![1 , 2],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![3 , 0],
                    ],
                    vec![
                        vec![0 , 1], vec![0 , 0], vec![0 , 1], vec![0 , 2],
                    ]
                ]
            }
            2 => {
                vec![
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![1 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![0, 1], vec![1 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![0, 1], vec![2 , 1]
                        // vec![1 , 0], vec![1 , 1], vec![2 , 1], vec![1 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![2, 1], vec![1 , 2]                  
                    ]
                ]
            }
            3 => {
                vec![
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![0 , 1], vec![1 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![0 , 1], vec![1 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![0 , 1], vec![1 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![0 , 1], vec![1 , 1],
                    ]
                ]
            }
            4 => {
                vec![
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![2 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![2 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![2 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![2 , 1],
                    ]
                ]
            }
            5 => {
                vec![
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![0 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![0 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![0 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![2 , 0], vec![0 , 1],
                    ]
                ]
            }
            6 => {
                vec![
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![1 , 1], vec![2 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![1 , 1], vec![2 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![1 , 1], vec![2 , 1],
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 0], vec![1 , 1], vec![2 , 1],
                    ]
                ]
            }
            _ => {
                vec![
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![0 , 1], vec![1 , 1],
                    ],
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![0 , 1], vec![1 , 1],
                    ],
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![0 , 1], vec![1 , 1],
                    ],
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![0 , 1], vec![1 , 1],
                    ]
                ]
            }
        };
        let block_clone = block.clone();
        let mut i: usize = 0;
        for _ in &block_clone[deg]{
            block[deg][i][0] += pos.x;
            block[deg][i][1] += pos.y;
            i += 1
        }
        block[deg].clone()
    }
}