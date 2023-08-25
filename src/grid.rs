use rand::Rng;

pub struct Grid {
    grid: Vec<i8>,
    shape: (usize, usize),
}

impl Grid {

    fn panic_if_too_many_mines(number_of_mines: usize, shape: (usize, usize)) {
        if number_of_mines > shape.0 * shape.1 {
            panic!("Too many mines for the grid!");
        }
    }

    fn set(&mut self, x: usize, y: usize, value: i8) {
        self.grid[y * self.shape.0 + x] = value;
    }

    fn increment_cell(&mut self, x: usize, y: usize) {
        if self.get(x, y) != -1 {
            self.set(x, y, self.get(x, y) + 1);
        }
    }

    fn set_mines(&mut self, number_of_mines: usize) {
        let mut rng = rand::thread_rng();
        let mut mines = 0;
        while mines < number_of_mines {
            let x = rng.gen_range(0..self.shape.0);
            let y = rng.gen_range(0..self.shape.1);
            if self.get(x, y) != -1 {
                self.set(x, y, -1);

                if x > 0 {
                    self.increment_cell(x - 1, y);
                }
                if x > 0 && y < self.shape.1 - 1 {
                    self.increment_cell(x, y + 1);
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


    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Grid {
        Grid::panic_if_too_many_mines(number_of_mines, shape);
        let mut grid = Grid {
            grid: vec![0; shape.0 * shape.1],
            shape: shape,
        };
        grid.init(number_of_mines);
        return grid
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        self.grid[y * self.shape.0 + x]
    }

    pub fn get_shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn reshape(&mut self, shape: (usize, usize), number_of_mines: usize) {
        Grid::panic_if_too_many_mines(number_of_mines, shape);
        self.shape = shape;
        self.grid = vec![0; shape.0 * shape.1];
        self.init(number_of_mines);
    }

    pub fn print(&self) {
        for y in 0..self.shape.1 {
            for x in 0..self.shape.0 {
                print!("{}\t", self.get(x, y));
            }
            println!();
        }
    }
}
