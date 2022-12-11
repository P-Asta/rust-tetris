mod tetris;
use tetris::t_map::Tmap;
use tetris::t_move::Move;

use crossterm::event::{read, Event::Key};
use crossterm::event::KeyCode;


fn main() {
    let map = Tmap::new();
    // map.push_block();
    // map.blocks[0].t_move(Move::Left);
    print!("{:?}", map.encoding());
    // print!("{:?}", Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::all(), kind: KeyEventKind::Press, state: KeyEventState::all() }));
    while true {
        print!("a");
        match read().unwrap() {
            Key(key) => {
                println!("{:?}", key.code);
            }
            _ => {
                println!("asdf")
            }
        };
    }
    // println!("{:?}", Color::Rgb { r: 50, g: 60, b: 70 })
}


// fn main() -> Result<()> {
//     let color = Color::Rgb { r: 100, g: 100, b: 100 };
//     execute!(
//         stdout(),
//         SetForegroundColor(color),
//         SetBackgroundColor(color),
//         Print("ㅤ".to_string()),
//         ResetColor
//     );
// }