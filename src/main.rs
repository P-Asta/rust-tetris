mod tetris;
use std::io::stdout;
use std::{thread, time};

use crossterm::ExecutableCommand;
use tetris::t_map::Tmap;
use tetris::t_move::Move;

use crossterm::event::{read, Event::Key};
use crossterm::event::KeyCode;


fn main() {
    let map = std::sync::Mutex::new(Tmap::new());
    
    thread::scope(|scope|{
        let main_th = scope.spawn(|| {
            loop {
                cls();
                {
                    let mapwriter = map.lock().unwrap();
                    mapwriter.encoding();
                }
                
                match read().unwrap() {
                    Key(key) => {
                        let mut map_writer = map.lock().unwrap();
                        match key.code{
                            KeyCode::Up => {
                                map_writer.block.t_spin();
                            },
                            KeyCode::Down => {
                                map_writer.block_down();
                            },
        
                            KeyCode::Left => {
                                map_writer.block.t_move(Move::Left);
                            },
        
                            KeyCode::Right => {
                                map_writer.block.t_move(Move::Right);
                            },
        
                            _code => { }
                        }
                    }
                    _ => { }
                };
            }
        });

        let down_th = scope.spawn(|| {
            loop {
                cls();
                {
                    let mut map_writer = map.lock().unwrap();
                    map_writer.block_down();
                    map_writer.encoding();
                }
                thread::sleep(time::Duration::from_millis(500))
            }
        });

        down_th.join().unwrap();
        main_th.join().unwrap();
    });
}


fn cls(){
    stdout().execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
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