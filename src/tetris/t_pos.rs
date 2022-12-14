#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}
/* 
clone은 데이터 안에 들어가서 하나하나 복사하는건데
copy는 그냥 겉부분만 복사한다는뜻이에요
이건 구조체라 겉부분만 복사가 안되는데
123 같은건 그냥 값을 옮길수있으니까 그걸 copy한다고해요
vector는 값을 못옮기구요
*/