mod tetris;
use std::io::{stdout, BufReader};
use std::{thread, time};

use crossterm::ExecutableCommand;
use tetris::t_map::Tmap;
use tetris::t_move::Move;

use crossterm::event::{read, Event::Key};
use crossterm::event::KeyCode;

use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source};
/*
TODO: 버그해결

*/
fn main() {
    let map = std::sync::Mutex::new(Tmap::new());
    
    thread::scope(|scope|{
        let main_th = scope.spawn(|| {
            loop {
                {
                    let checker = map.lock();
                    match checker {
                        Ok(x) => {
                            cls();
                            x.encoding();
                            println!("\npoint: {}", x.point);
                            if x.block.is_none(){
                                println!("GAME OVER!");
                                break;
                            }
                        }
                        Err(e) => {println!("{e:?}")}
                    }
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
            
                                _code => { }
                            }
                        }
                    }
                    _ => { }
                };
            }
        });

        let down_th = scope.spawn(|| {
            loop {
                {
                    let checker = map.lock();
                    match checker {
                        Ok(x) => {
                            cls();
                            let mut map_writer = x;
                            map_writer.down_block();
                            map_writer.encoding();
        
                            println!("\npoint: {}", map_writer.point);
                            if map_writer.block.is_none(){
                                println!("GAME OVER!");
                                break;
                            }
                        }
                        Err(e) => {println!("{e:?}")}
                    }
                }
                thread::sleep(time::Duration::from_millis(1500))
            }
        });

        let bgm_th = scope.spawn(||{
            loop{
                let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                let sink = rodio::Sink::try_new(&handle).unwrap();
    
                let file = std::fs::File::open("C:/Users/smile/OneDrive/문서/GitHub/rust-tetris/src/sounds/bgm.mp3").unwrap();
                sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    
                sink.sleep_until_end();
            }
        });
        
        bgm_th.join().unwrap();
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