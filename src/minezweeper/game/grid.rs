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
    number_of_mines: usize,
    number_of_cleared: usize,
    initialized: bool
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

    fn init(&mut self, first_cell: (usize, usize)) {
        let mut rng = rand::thread_rng();
        let mut mines = 0;
        while mines < self.number_of_mines {
            let x = rng.gen_range(0..self.shape.0);
            let y = rng.gen_range(0..self.shape.1);
            if first_cell.0 <= 1 + x && first_cell.0 + 1 >= x && first_cell.1 <= 1 + y && first_cell.1 + 1 >= y {
                continue;
            }
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

    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Self {
        Self::panic_if_too_many_mines(number_of_mines, shape);
        Grid {
            grid: vec![Cell::new(0); shape.0 * shape.1],
            shape: shape,
            number_of_mines,
            number_of_cleared: 0,
            initialized: false
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.grid[y * self.shape.0 + x]
    }

    pub fn all_cleared(&self) -> bool {
        self.number_of_cleared == self.shape.0 * self.shape.1 - self.number_of_mines
    }

    fn surrounding_flags(&self, x: usize, y: usize) -> i8 {
        let mut number_of_flagged: i8 = 0;
        if x > 0 {
            number_of_flagged += self.get(x - 1, y).flagged as i8;
        }
        if x > 0 && y < self.shape.1 - 1 {
            number_of_flagged += self.get(x - 1, y + 1).flagged as i8;
        }
        if y > 0 {
            number_of_flagged += self.get(x, y - 1).flagged as i8;
        }
        if y > 0 && x < self.shape.0 - 1 {
            number_of_flagged += self.get(x + 1, y - 1).flagged as i8;
        }
        if x > 0 && y > 0 {
            number_of_flagged += self.get(x - 1, y - 1).flagged as i8;
        }
        if x < self.shape.0 - 1 {
            number_of_flagged += self.get(x + 1, y).flagged as i8;
        }
        if y < self.shape.1 - 1 {
            number_of_flagged += self.get(x, y + 1).flagged as i8;
        }
        if x < self.shape.0 - 1 && y < self.shape.1 - 1 {
            number_of_flagged += self.get(x + 1, y + 1).flagged as i8;
        }
        
        number_of_flagged
    }

    pub fn clear_adjacent(&mut self, x: usize, y: usize) -> Result<(), ()> {
        let cell = self.get(x, y);
        if !cell.cleared {
            return Ok(());
        }
        if self.surrounding_flags(x, y) != cell.value {
            return Ok(());
        }

        if x > 0 {
            self.set_cleared(x - 1, y)?;
        }
        if x > 0 && y < self.shape.1 - 1 {
            self.set_cleared(x - 1, y + 1)?;
        }
        if y > 0 {
            self.set_cleared(x, y - 1)?;
        }
        if y > 0 && x < self.shape.0 - 1 {
            self.set_cleared(x + 1, y - 1)?;
        }
        if x > 0 && y > 0 {
            self.set_cleared(x - 1, y - 1)?;
        }
        if x < self.shape.0 - 1 {
            self.set_cleared(x + 1, y)?;
        }
        if y < self.shape.1 - 1 {
            self.set_cleared(x, y + 1)?;
        }
        if x < self.shape.0 - 1 && y < self.shape.1 - 1 {
            self.set_cleared(x + 1, y + 1)?;
        }
        Ok(())
    }

    pub fn set_cleared(&mut self, x: usize, y: usize) -> Result<(), ()> {
        if !self.initialized {
            self.init((x, y));
            self.initialized = true;
        }
        let cell = &mut self.grid[y * self.shape.0 + x];
        if cell.cleared || cell.flagged {
            return Ok(());
        }
        cell.cleared = true;
        if cell.value == -1 {
            return Err(());
        }
        self.number_of_cleared += 1;
        println!(
            "number of remaining uncleared {}",
            self.shape.0 * self.shape.1 - self.number_of_cleared - self.number_of_mines
        );
        if cell.value == 0 {
            if x > 0 {
                self.set_cleared(x - 1, y)?;
            }
            if x > 0 && y < self.shape.1 - 1 {
                self.set_cleared(x - 1, y + 1)?;
            }
            if y > 0 {
                self.set_cleared(x, y - 1)?;
            }
            if y > 0 && x < self.shape.0 - 1 {
                self.set_cleared(x + 1, y - 1)?;
            }
            if x > 0 && y > 0 {
                self.set_cleared(x - 1, y - 1)?;
            }
            if x < self.shape.0 - 1 {
                self.set_cleared(x + 1, y)?;
            }
            if y < self.shape.1 - 1 {
                self.set_cleared(x, y + 1)?;
            }
            if x < self.shape.0 - 1 && y < self.shape.1 - 1 {
                self.set_cleared(x + 1, y + 1)?;
            }
        }
        Ok(())
    }

    pub fn toggle_flagged(&mut self, x: usize, y: usize) {
        if !self.grid[y * self.shape.0 + x].cleared {
            self.grid[y * self.shape.0 + x].flagged = !(self.grid[y * self.shape.0 + x].flagged);
        }
    }

    pub fn toggle_question_marked(&mut self, x: usize, y: usize) {
        if !self.grid[y * self.shape.0 + x].cleared {
            self.grid[y * self.shape.0 + x].question_marked = !(self.grid[y * self.shape.0 + x].question_marked);
        }
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
