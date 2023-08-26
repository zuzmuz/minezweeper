use ggez::input::keyboard::KeyCode;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Action {
    Move(Direction),
    Clear,
    Flag,
    QuestionMark,
    ClearAdjacent,
    None,
}

pub struct Controls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub clear: KeyCode,
    pub flag: KeyCode,
    pub question_mark: KeyCode,
    pub clear_adjacent: KeyCode,
}

impl Controls {
    pub fn default() -> Self {
        Controls {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
            clear: KeyCode::Space,
            flag: KeyCode::C,
            question_mark: KeyCode::Z,
            clear_adjacent: KeyCode::X,
        }
    }

    pub fn handle(&self, keycode: KeyCode) -> Action {
        match keycode {
            _ if keycode == self.up => Action::Move(Direction::Up),
            _ if keycode == self.down => Action::Move(Direction::Down),
            _ if keycode == self.left => Action::Move(Direction::Left),
            _ if keycode == self.right => Action::Move(Direction::Right),
            _ if keycode == self.clear => Action::Clear,
            _ if keycode == self.flag => Action::Flag,
            _ if keycode == self.question_mark => Action::QuestionMark,
            _ if keycode == self.clear_adjacent => Action::ClearAdjacent,
            _ => Action::None,
        }
    } 
}