pub fn solve() {
    let grid = Grid::from_str(include_str!("../input/day11"));

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

fn part1(grid: &Grid) -> u32 {
    let mut grid = grid.clone();

    for _ in 0..100 {
        grid.step_part1();
    }

    grid.count_occupied()
}

fn part2(grid: &Grid) -> u32 {
    let mut grid = grid.clone();

    for _ in 0..100 {
        grid.step_part2();
    }

    grid.count_occupied()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Square {
    Floor,
    Empty,
    Occupied,
}

impl Square {
    fn from_char(c: char) -> Option<Square> {
        match c {
            '.' => Some(Square::Floor),
            'L' => Some(Square::Empty),
            '#' => Some(Square::Occupied),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    squares: Vec<Square>,
}

impl Grid {
    fn from_str(s: &str) -> Grid {
        let height = s.lines().count();
        let width = s.lines().nth(0).unwrap().len();
        let squares: Vec<Square> = s.chars().filter_map(Square::from_char).collect();

        Grid {
            width,
            height,
            squares,
        }
    }

    fn at(&self, x: isize, y: isize) -> Square {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.squares[x as usize + y as usize * self.width]
        } else {
            Square::Floor
        }
    }

    fn adjacent_squares(&self, x: usize, y: usize) -> [Square; 8] {
        let x = x as isize;
        let y = y as isize;

        [
            self.at(x - 1, y - 1),
            self.at(x, y - 1),
            self.at(x + 1, y - 1),
            self.at(x + 1, y),
            self.at(x + 1, y + 1),
            self.at(x, y + 1),
            self.at(x - 1, y + 1),
            self.at(x - 1, y),
        ]
    }

    fn visible_seat_in_dir(&self, x: usize, y: usize, dx: isize, dy: isize) -> Square {
        let mut x = x as isize;
        let mut y = y as isize;

        loop {
            x += dx;
            y += dy;

            if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
                return Square::Floor;
            }

            let s = self.at(x, y);

            if s != Square::Floor {
                return s;
            }
        }
    }

    fn visible_seats(&self, x: usize, y: usize) -> [Square; 8] {
        [
            self.visible_seat_in_dir(x, y, -1, -1),
            self.visible_seat_in_dir(x, y, 0, -1),
            self.visible_seat_in_dir(x, y, 1, -1),
            self.visible_seat_in_dir(x, y, 1, 0),
            self.visible_seat_in_dir(x, y, 1, 1),
            self.visible_seat_in_dir(x, y, 0, 1),
            self.visible_seat_in_dir(x, y, -1, 1),
            self.visible_seat_in_dir(x, y, -1, 0),
        ]
    }

    fn next_square_part1(&self, x: usize, y: usize) -> Square {
        let current_square = self.at(x as isize, y as isize);

        if current_square == Square::Floor {
            return Square::Floor;
        }

        let mut count = 0;

        for &s in &self.adjacent_squares(x, y) {
            if s == Square::Occupied {
                count += 1;
            }
        }

        if count == 0 {
            Square::Occupied
        } else if count >= 4 {
            Square::Empty
        } else {
            current_square
        }
    }

    fn next_square_part2(&self, x: usize, y: usize) -> Square {
        let current_square = self.at(x as isize, y as isize);

        if current_square == Square::Floor {
            return Square::Floor;
        }

        let mut count = 0;

        for &s in &self.visible_seats(x, y) {
            if s == Square::Occupied {
                count += 1;
            }
        }

        if count == 0 {
            Square::Occupied
        } else if count >= 5 {
            Square::Empty
        } else {
            current_square
        }
    }

    fn count_occupied(&self) -> u32 {
        let mut count = 0;

        for &s in &self.squares {
            if s == Square::Occupied {
                count += 1;
            }
        }

        count
    }

    fn step_part1(&mut self) {
        let mut new_squares = self.squares.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                new_squares[x + y * self.width] = self.next_square_part1(x, y);
            }
        }

        self.squares = new_squares;
    }

    fn step_part2(&mut self) {
        let mut new_squares = self.squares.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                new_squares[x + y * self.width] = self.next_square_part2(x, y);
            }
        }

        self.squares = new_squares;
    }
}
