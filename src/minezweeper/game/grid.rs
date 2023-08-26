use rand::Rng;
#[derive(Clone)]
pub struct Cell {
    value: i8,
    pub cleared: bool,
    pub flagged: bool,
    pub question_marked: bool,
    pub hovered: bool,
    pub clicked: bool,
}

impl Cell {
    fn new(value: i8) -> Self {
        Cell {
            value: value,
            cleared: false,
            flagged: false,
            question_marked: false,
            hovered: false,
            clicked: false,
        }
    }

    pub fn get_value(&self) -> i8 {
        self.value
    }
}

pub struct Grid {
    grid: Vec<Cell>,
    shape: (usize, usize),
}

impl Grid {
    fn panic_if_too_many_mines(number_of_mines: usize, shape: (usize, usize)) {
        if number_of_mines > shape.0 * shape.1 {
            panic!("Too many mines for the grid!");
        }
    }

    fn set(&mut self, x: usize, y: usize, value: i8) {
        self.grid[y * self.shape.0 + x].value = value;
    }

    fn increment_cell(&mut self, x: usize, y: usize) {
        if self.get(x, y).value != -1 {
            self.set(x, y, self.get(x, y).value + 1);
        }
    }

    fn set_mines(&mut self, number_of_mines: usize) {
        let mut rng = rand::thread_rng();
        let mut mines = 0;
        while mines < number_of_mines {
            let x = rng.gen_range(0..self.shape.0);
            let y = rng.gen_range(0..self.shape.1);
            if self.get(x, y).value != -1 {
                self.set(x, y, -1);

                if x > 0 {
                    self.increment_cell(x - 1, y);
                }
                if x > 0 && y < self.shape.1 - 1 {
                    self.increment_cell(x - 1, y + 1);
                }
                if y > 0 {
                    self.increment_cell(x, y - 1);
                }
                if y > 0 && x < self.shape.0 - 1 {
                    self.increment_cell(x + 1, y - 1);
                }
                if x > 0 && y > 0 {
                    self.increment_cell(x - 1, y - 1);
                }
                if x < self.shape.0 - 1 {
                    self.increment_cell(x + 1, y);
                }
                if y < self.shape.1 - 1 {
                    self.increment_cell(x, y + 1);
                }
                if x < self.shape.0 - 1 && y < self.shape.1 - 1 {
                    self.increment_cell(x + 1, y + 1);
                }
                mines += 1;
            }
        }
    }

    fn init(&mut self, number_of_mines: usize) {
        self.set_mines(number_of_mines);
    }

    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Self {
        Self::panic_if_too_many_mines(number_of_mines, shape);
        let mut grid = Grid {
            grid: vec![
                Cell::new(0);
                shape.0 * shape.1
            ],
            shape: shape,
        };
        grid.init(number_of_mines);
        return grid;
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.grid[y * self.shape.0 + x]
    }

    pub fn set_cleared(&mut self, x: usize, y: usize, cleared: bool) {
        self.grid[y * self.shape.0 + x].cleared = cleared;
    }

    pub fn set_flagged(&mut self, x: usize, y: usize, flagged: bool) {
        self.grid[y * self.shape.0 + x].flagged = flagged;
    }

    pub fn set_question_marked(&mut self, x: usize, y: usize, question_marked: bool) {
        self.grid[y * self.shape.0 + x].question_marked = question_marked;
    }

    pub fn set_hovered(&mut self, x: usize, y: usize, hovered: bool) {
        self.grid[y * self.shape.0 + x].hovered = hovered;
    }

    pub fn set_clicked(&mut self, x: usize, y: usize, clicked: bool) {
        self.grid[y * self.shape.0 + x].clicked = clicked;
    }

    pub fn get_shape(&self) -> (usize, usize) {
        self.shape
    }

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..self.shape.1 {
            for x in 0..self.shape.0 {
                print!("{}\t", self.get(x, y).value);
            }
            println!();
        }
    }
}
