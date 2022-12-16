// 여기는 crate::tetris::t_built_in

pub mod built_in{
    use std::{io::{stdout, BufReader}, thread, time};
    use crossterm::ExecutableCommand;

    // 여기는 crate::tetris::t_built_in::built_in
    use super::super::t_pos::Pos;
    #[allow(unused_assignments)]
    pub fn make_shape(id: usize, pos: Pos, deg: usize) -> Result<Vec<Vec<usize>>, ()>{
        let mut block: Vec<Vec<Vec<usize>>> = match id {
            1 => {
                vec![
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![3 , 1]
                    ],
                    vec![
                        vec![2 , 0], vec![2 , 1], vec![2 , 2], vec![2 , 3]
                    ],
                    vec![
                        vec![0 , 2], vec![1 , 2], vec![2 , 2], vec![3 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![1 , 2], vec![1 , 3]
                    ]
                ]
            }
            2 => {
                vec![
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![0, 1], vec![2 , 1]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![2, 1], vec![1 , 2]                  
                    ],
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![1 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![0, 1], vec![1 , 2]
                    ]
                ]
            }
            3 => {
                vec![
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![1 , 1], vec![2 , 1]
                    ],
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![1 , 1], vec![2 , 1]
                    ],
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![1 , 1], vec![2 , 1]
                    ],
                    vec![
                        vec![1 , 0], vec![2 , 0], vec![1 , 1], vec![2 , 1]
                    ]
                ]
            }
            4 => {
                vec![
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![0 , 0]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![1 , 2], vec![2 , 0]
                    ],
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![2 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![1 , 2], vec![0 , 2]
                    ]
                ]
            }
            5 => {
                vec![
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![2 , 0]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![1 , 2], vec![2 , 2]
                    ],
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![2 , 1], vec![0 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![1 , 2], vec![0 , 0]
                    ]
                ]
            }
            6 => {
                vec![   
                    vec![
                        vec![0 , 0], vec![1 , 1], vec![1 , 0], vec![2 , 1]
                    ],
                    vec![
                        vec![1 , 2], vec![1 , 1], vec![2 , 1], vec![2 , 0]
                    ],
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![1 , 2], vec![2 , 2]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![0 , 1], vec![0 , 2]
                    ]
                ]
            }
            7 => {
                vec![
                    vec![
                        vec![0 , 2], vec![1 , 1], vec![1 , 2], vec![2 , 1]
                    ],
                    vec![
                        vec![0 , 0], vec![1 , 1], vec![0 , 1], vec![1 , 2]
                    ],
                    vec![
                        vec![0 , 1], vec![1 , 1], vec![1 , 0], vec![2 , 0]
                    ],
                    vec![
                        vec![1 , 0], vec![1 , 1], vec![2 , 1], vec![2 , 2]
                    ]
                ]
            }
            _ => {
                vec![vec![vec![0; 2]; 4]; 4]
            }
        };
        let block_clone = block.clone();
        let mut i: usize = 0;

        let mut ok = true;

        let mut x = 0;
        let mut y = 0;

        for _ in &block_clone[deg]{
            x = block[deg][i][0] as isize;
            y = block[deg][i][1] as isize;
            x += pos.x as isize;
            y += pos.y as isize;
            
            i += 1;
            if id == 1{
                if y - 1 < 0{
                    
                }else{
                    y -= 1;
                }
            }
            if x >= 10 || x < 0 || y >= 20{
                ok = false;
                break;
            }
        }

        i = 0;
        if ok{
            for _ in &block_clone[deg]{
                if pos.x < 0{
                    block[deg][i][0] = ((block[deg][i][0] as isize) + pos.x) as usize;
                }else{
                    block[deg][i][0] += pos.x as usize;
                }

                if pos.y < 0{
                    block[deg][i][1] = ((block[deg][i][1] as isize) + pos.y) as usize;
                }else {
                    block[deg][i][1] += pos.y as usize;
                }
                if id == 1{
                    if (block[deg][i][1] as isize) - 1 < 0{
                        drop(block[deg][i][1])
                    }else{
                        block[deg][i][1] -= 1;
                    }
                }
                i += 1;
            }
            Ok(block[deg].clone())
        }else{
            Err(())
        }
        
    }

    pub fn play_sound(name: &str){
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let root = std::env::current_dir().unwrap();
        let path = root.join(format!("src/sounds/{name}.mp3"))
            .to_str().unwrap()
            .replace("\\", "/");
        let file = std::fs::File::open(path);
        sink.append(rodio::Decoder::new(BufReader::new(file.unwrap())).unwrap());

        sink.sleep_until_end();
        thread::sleep(time::Duration::from_millis(10))
    }

    pub fn cls(){
        stdout().execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
        stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    }

}