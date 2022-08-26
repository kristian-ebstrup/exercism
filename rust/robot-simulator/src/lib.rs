// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, dir: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        // get new direction
        match self.dir {
            Direction::North => Self {dir: Direction::East, ..self},
            Direction::East => Self {dir: Direction::South, ..self},
            Direction::South => Self {dir: Direction::West, ..self},
            Direction::West => Self {dir: Direction::North, ..self},
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        // get new direction
        match self.dir {
            Direction::North => Self {dir: Direction::West, ..self},
            Direction::East => Self {dir: Direction::North, ..self},
            Direction::South => Self {dir: Direction::East, ..self},
            Direction::West => Self {dir: Direction::South, ..self},
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        // get new position based on direction
        match self.dir {
            Direction::North => Self { y: self.y + 1, ..self },
            Direction::East => Self { x: self.x + 1, ..self },
            Direction::South => Self { y: self.y - 1, ..self },
            Direction::West => Self { x: self.x - 1, ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // extract "current" information
        let (x, y): (i32, i32) = self.position();
        let dir: Direction = self.dir;

        // create mutable instance of Robot
        let mut robot: Robot = Robot::new(x, y, dir);

        // Run instructions
        for c in instructions.to_lowercase().chars() {
            // Three possible instructions:
            //  r: Turn right
            //  l: Turn left
            //  a: Advance
            robot = match c {
                'r' => robot.turn_right(),
                'l' => robot.turn_left(),
                'a' => robot.advance(),
                _ => panic!("Unrecognized Instruction: {}.", c),
            };
        }

        return robot;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
