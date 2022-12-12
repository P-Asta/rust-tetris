mod tetris;
use std::io::stdout;
use std::{thread, time};

use crossterm::ExecutableCommand;
use tetris::t_map::Tmap;
use tetris::t_move::Move;

use crossterm::event::{read, Event::Key};
use crossterm::event::KeyCode;


fn main() {
    let mut map = Tmap::new();
    // map.push_block();
    // map.blocks[0].t_move(Move::Left);
    // print!("{:?}", Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::all(), kind: KeyEventKind::Press, state: KeyEventState::all() }));
    // assert!( std::process::Command::new("cls") );
    loop {
        stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        println!("");
        map.encoding();
        map.block.t_spin(); // 이거에요
        // 잘 되네요
        // 문제가 여긴가요?
        // 지금은 mut이 안된다고 에러가 바뀌었네요
        // 이것처럼 비슷하게
        // 어제 copy써준부분에서도 &을 써주면 copy를 지울 수 있어요
        // 넹 수구하세용
        // 그리구 이거 출력보면 버벅거리는데 아마
        // 흰색부분은 놔두고 블럭부분만 스페이스바로 칠하면 좋을것같아요
        stdout().execute(crossterm::cursor::MoveTo(2, 0)).unwrap(); //이런방식으로 커서 이동할 수 있는데
        // 이동하고 println!(" "); 출력하면 한글자씩 지워질거에요
        // 여러개 지울땐 println!("         ");등으로 하면 되구요
        // 안녕히주무세요
        // 안녕히주무세요 :)

        // 감사합니다! 저는 이만 자러 가보겠습니다 :)
        thread::sleep(time::Duration::from_millis(150))
        // print!("a");
        // match read().unwrap() {
        //     Key(key) => {
        //         println!("{:?}", key.code);
        //     }
        //     _ => {
        //         println!("asdf")
        //     }
        // };
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