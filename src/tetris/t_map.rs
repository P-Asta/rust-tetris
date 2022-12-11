
// 이건 생각해보니 t_block이라는 모듈이 이미 정의되어있어서 mod super::t_block으로 쓸 필요가 없었네요
// 그냥 지워주시면 됩니다 use만써주고
// mod는 새 모듈을 만들어주는키워든데 mod.rs에서 t_block모듈을 생성해줬으니까요

// 아하

use rand::{Rng, thread_rng};
use std::io::{stdout};
use crossterm::execute;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color};
use super::t_block::Tblock;


#[derive(Debug)]
pub struct Tmap {
    pub map: Vec<Vec<usize>>,
    pub blocks: Vec<Tblock>
}



/*
# 1
⬜⬜⬜⬜

# 2
⬛🟪
🟪🟪🟪

# 3
🟨🟨
🟨🟨

# 4
🟦
🟦🟦🟦

# 5
⬛⬛🟧
🟧🟧🟧

# 6
🟥🟥
⬛🟥🟥

# 7
⬛🟩🟩
🟩🟩
*/

impl Tmap {
    pub fn new() -> Self {
        let mut map: Vec<Vec<usize>> = vec![];
        let mut arr: Vec<usize>;
        
        let mut blocks: Vec<Tblock> = vec![];
        
        let mut rng = thread_rng();
        blocks.push(Tblock::new(rng.gen_range(1..8), None));
        for _ in 0..20{
            arr = vec![];
            for _ in 0..10{
                arr.push(0);
            }
            map.push(arr);
        }

        Self{
            map: map,
            blocks: blocks
        }
    }

    pub fn push_block(mut self){
        let mut rng = thread_rng();
        self.blocks.push(Tblock::new(rng.gen_range(1..8), None));
        // Self {
        //     blocks: self.blocks,
        //     map: self.map
        // }
    }

    pub fn get_block(&self, index: usize) -> &Tblock{
        &self.blocks[index]
    }

    pub fn encoding(&self){
        let mut map = self.map.clone();
        let blocks = self.blocks.clone(); // 여기에는 clone메서드가 없는데 어떻게 해야하나요?
        for block in &blocks{ // 요게 =을적으면 blocks변수에 값을 복사하라는뜻인데
            // vector는 복사가 안되거든요
            // 복제를하려면 .clone() 사용해야해요
            // 여게
            // 저 vector가 TBlock을 가지고있는데
            // Tblock이 clone이 불가능해서 나타나는 문제에요
            
            // 그러면 어떻게 고처야 할까요?
            for shape in &block.shape{
                map[shape[1]][shape[0]] = block.id;
                
            }
        }
        for _ in 0..12{
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );
        }
        print!("\n");
        for i in &map{
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );

            for j in i{
                let color = match j{
                    0 => Color::Rgb { r: 0, g: 0, b: 0 },
                    1 => Color::Rgb { r: 0, g: 240, b: 240 },
                    2 => Color::Rgb { r: 160, g: 0, b: 240 },
                    3 => Color::Rgb { r: 240, g: 240, b: 0 },
                    4 => Color::Rgb { r: 0, g: 0, b: 240 },
                    5 => Color::Rgb { r: 240, g: 160, b: 240 },
                    6 => Color::Rgb { r: 240, g: 0, b: 0 },
                    _ => Color::Rgb { r: 0, g: 240, b: 0 }
                };
                let b = Color::Rgb{ r: 0, g: 0, b: 0 };

                if color != b {
                    let _ = execute!(
                        stdout(),
                        SetForegroundColor(color),
                        SetBackgroundColor(color),
                        Print("ㅤ".to_string()),
                        ResetColor
                    );
                }else{
                    print!("ㅤ")
                }
                // let b = match j{
                //     0 => "⬛",
                //     1 => "⬜",
                //     2 => "🟪",
                //     3 => "🟨",
                //     4 => "🟦",
                //     5 => "🟧",
                //     6 => "🟥",
                //     _ => "🟩"
                // };
                // print!("{}", b)
            }
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );
            print!("\n");
        }
        for _ in 0..12{
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );
        }
    }
}