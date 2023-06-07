mod tetris;
use std::io::{stdout, BufReader};
use std::{thread, time};

use crossterm::ExecutableCommand;
use tetris::t_built_in::built_in;
use tetris::t_map::Tmap;
use tetris::t_move::Move;

use crossterm::event::{read, Event::Key};
use crossterm::event::KeyCode;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source};

/*
TODO: 버그해결

FIXME: 

*/
fn main() {
    let map = std::sync::Mutex::new(Tmap::new());
    // return;
    thread::scope(|scope|{
        let main_th = scope.spawn(|| {
            loop {
                {
                    disable_raw_mode().unwrap();
                    let checker = map.lock();
                    match checker {
                        Ok(x) => {
                            if !x.stop{
                                built_in::cls();
                                x.encoding();
                                // File::open()
                                x.print_points();
                                if x.block.is_none(){
                                    println!("GAME OVER!");
                                    return;
                                }
                            }
                        }
                        Err(e) => {println!("{e:?}")}
                    }
                    enable_raw_mode().unwrap();
                }
                match read().unwrap() {
                    Key(key) => {
                        {
                            let mut map_writer = map.lock().unwrap();
                            match key.code{
                                KeyCode::Up => {
                                    map_writer.spin_block();
                                },
                                KeyCode::Down => {
                                    map_writer.down_block();
                                },
            
                                KeyCode::Left => {
                                    map_writer.move_block(Move::Left);
                                },
            
                                KeyCode::Right => {
                                    map_writer.move_block(Move::Right);
                                },
            
                                _code => {}
                            }
                        }
                    }
                    _ => { }
                };
            }
        });

        let down_th = scope.spawn(|| {
            loop {
                disable_raw_mode();
                {
                    let checker = map.lock();
                    match checker {
                        Ok(x) => {
                            built_in::cls();
                            let mut map_writer = x;
                            map_writer.down_block();
                            map_writer.encoding();
                            
                            map_writer.print_points();

                            if map_writer.block.is_none(){
                                println!("GAME OVER!");
                                return;
                            }
                        }
                        Err(e) => {println!("{e:?}")}
                    }
                }
                enable_raw_mode();
                thread::sleep(time::Duration::from_millis(1500))
            }
        });

        let bgm_th = scope.spawn(||{
            loop{
                built_in::play_sound("bgm")
            }
        });
        
        bgm_th.join().unwrap();
        down_th.join().unwrap();
        main_th.join().unwrap();
    });
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
