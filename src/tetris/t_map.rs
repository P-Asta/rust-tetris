
// 이건 생각해보니 t_block이라는 모듈이 이미 정의되어있어서 mod super::t_block으로 쓸 필요가 없었네요
// 그냥 지워주시면 됩니다 use만써주고
// mod는 새 모듈을 만들어주는키워든데 mod.rs에서 t_block모듈을 생성해줬으니까요

// 아하

use rand::{Rng, thread_rng};
use std::io::{stdout};
use crossterm::execute;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color};
use super::t_block::Tblock;


#[derive(Debug, Clone)]
pub struct Tmap {
    pub map: Vec<Vec<usize>>,
    pub block: Tblock
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
        
        
        // let mut rng = thread_rng();
        // let block: Tblock = Tblock::new(rng.gen_range(1..8), None, 0);
        let block: Tblock = Tblock::new(2, None, 3);

        for _ in 0..20{
            arr = vec![];
            for _ in 0..10{
                arr.push(0);
            }
            map.push(arr);
        }

        Self{
            map: map,
            block: block
        }
    }

    pub fn get_block(&mut self) -> &mut Tblock{
        &mut self.block
    }

    pub fn encoding(&self){
        let mut map = self.map.clone();
        let block = self.block.clone(); // 여기에는 clone메서드가 없는데 어떻게 해야하나요?

        for shape in &block.shape{
            map[shape[1]][shape[0]] = block.id;
            
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