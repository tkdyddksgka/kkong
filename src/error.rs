pub enum Error {
    CommandNotFound(char),
    NumberParseError,
    PointerOutOfBounds,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::CommandNotFound(ref command) => {
                write!(f, "`{}` 라는 명령어를 찾을 수 없어요.", command)
            }
            Error::NumberParseError => {
                write!(f, "숫자만 입력해요.")
            }
            Error::PointerOutOfBounds => {
                write!(f, "포인터가 범위를 벗어났어요.")
            }
        }
    }
}

pub fn error(t: Error, col: usize) {
    println!("{}: {}", col, t);
}
