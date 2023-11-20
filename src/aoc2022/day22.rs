use lib::aoc;
use lib::challenge::Challenge;

const EMPTY: char = ' ';
const OPEN: char = '.';
const BLOCK: char = '#';

pub struct Day22;

impl Challenge for Day22 {
    aoc!(year = 2022, day = 22);

    fn solve(input: String) -> (String, String) {
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let n = map.split_whitespace().map(|s| s.len()).min().unwrap();

        let lines = map.lines().collect::<Vec<_>>();
        let mut faces = (0..lines.len() / n)
            .map(|i| parse_row(&lines[i * n..(i + 1) * n], n))
            .collect::<Vec<_>>();

        let mut solver = PlaneSolver::new(&faces);
        solver.run(instructions);
        let res1 = score(solver.abs_pos().0, solver.abs_pos().1, solver.facing());

        let mut solver = CubeSolver::new(&mut faces);
        solver.run(instructions);
        let mut res2 = 0;
        'outer: for (i, row) in faces.iter_mut().enumerate() {
            for (j, maybe) in row.iter_mut().enumerate().filter(|(_, f)| f.is_some()) {
                let face = maybe.as_mut().unwrap();
                let mut pos = solver.current;
                let mut facing = solver.facing;

                for _ in 0..4 {
                    if face == solver.cube.get_front().unwrap() {
                        let final_pos = (i * n + pos.0, j * n + pos.1);
                        res2 = score(final_pos.0, final_pos.1, facing);
                        break 'outer;
                    }

                    Cube::rotate_face_90(Some(face));
                    Direction::rotate_left(&mut facing);
                    pos = (n - 1 - pos.1, pos.0);
                }
            }
        }

        (res1.to_string(), res2.to_string())
    }
}

fn first_index<T>(faces: &[Vec<Option<T>>]) -> Option<(usize, usize)> {
    for (i, row) in faces.iter().enumerate() {
        for (j, face) in row.iter().enumerate() {
            if face.is_some() {
                return Some((i, j));
            }
        }
    }

    None
}

fn score(row: usize, col: usize, dir: Direction) -> usize {
    1000 * (row + 1)
        + 4 * (col + 1)
        + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

type Face = Vec<Vec<char>>;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn rotate_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };
    }
}

trait Solver {
    fn rotate_right(&mut self);
    fn rotate_left(&mut self);
    fn forward(&mut self, count: usize);

    fn run(&mut self, instructions: &str) {
        let mut count = String::new();
        for char in instructions.chars() {
            if char.is_ascii_digit() {
                count.push(char);
                continue;
            }

            self.forward(count.parse::<usize>().unwrap());
            count.clear();

            match char {
                'R' => self.rotate_right(),
                'L' => self.rotate_left(),
                _ => unreachable!(),
            };
        }

        if let Ok(forward) = count.parse::<usize>() {
            self.forward(forward);
        }
    }
}

struct PlaneSolver<'a> {
    faces: &'a [Vec<Option<Face>>],
    size: usize,
    face_pos: (usize, usize),
    pos: (i32, i32),
    dir: Direction,
}

impl<'a> PlaneSolver<'a> {
    pub fn new(faces: &'a [Vec<Option<Face>>]) -> Self {
        let first_face = first_index(faces).unwrap();
        Self {
            faces,
            size: faces[first_face.0][first_face.1].as_ref().unwrap().len(),
            face_pos: first_face,
            pos: (0, 0),
            dir: Direction::Right,
        }
    }

    pub fn abs_pos(&self) -> (usize, usize) {
        let y = self.pos.0 as usize + (self.size * self.face_pos.0);
        let x = self.pos.1 as usize + (self.size * self.face_pos.1);
        (y, x)
    }

    pub fn facing(&self) -> Direction {
        self.dir
    }
}

impl Solver for PlaneSolver<'_> {
    fn rotate_right(&mut self) {
        self.dir.rotate_right()
    }

    fn rotate_left(&mut self) {
        self.dir.rotate_left()
    }

    fn forward(&mut self, count: usize) {
        if count == 0 {
            return;
        }

        let mut pos = self.pos;
        let mut face_pos = self.face_pos;

        if self.dir == Direction::Up {
            if pos.0 == 0 {
                loop {
                    face_pos = (
                        face_pos.0.checked_sub(1).unwrap_or(self.faces.len() - 1),
                        face_pos.1,
                    );
                    if self
                        .faces
                        .get(face_pos.0)
                        .and_then(|face| face.get(face_pos.1))
                        .is_some_and(|f| f.is_some())
                    {
                        pos = (self.size as i32 - 1, pos.1);
                        break;
                    }
                }
            } else {
                pos = (pos.0 - 1, pos.1);
            }
        }

        if self.dir == Direction::Right {
            if pos.1 as usize == self.size - 1 {
                loop {
                    face_pos = (face_pos.0, (face_pos.1 + 1) % self.faces[face_pos.0].len());
                    if self
                        .faces
                        .get(face_pos.0)
                        .and_then(|face| face.get(face_pos.1))
                        .is_some_and(|f| f.is_some())
                    {
                        pos = (pos.0, 0);
                        break;
                    }
                }
            } else {
                pos = (pos.0, pos.1 + 1);
            }
        }

        if self.dir == Direction::Down {
            if pos.0 as usize == self.size - 1 {
                loop {
                    face_pos = ((face_pos.0 + 1) % self.faces.len(), face_pos.1);
                    if self
                        .faces
                        .get(face_pos.0)
                        .and_then(|face| face.get(face_pos.1))
                        .is_some_and(|f| f.is_some())
                    {
                        pos = (0, pos.1);
                        break;
                    }
                }
            } else {
                pos = (pos.0 + 1, pos.1);
            }
        }

        if self.dir == Direction::Left {
            if pos.1 == 0 {
                loop {
                    face_pos = (
                        face_pos.0,
                        face_pos
                            .1
                            .checked_sub(1)
                            .unwrap_or(self.faces[face_pos.0].len() - 1),
                    );
                    if self
                        .faces
                        .get(face_pos.0)
                        .and_then(|face| face.get(face_pos.1))
                        .is_some_and(|f| f.is_some())
                    {
                        pos = (pos.0, self.size as i32 - 1);
                        break;
                    }
                }
            } else {
                pos = (pos.0, pos.1 - 1);
            }
        }

        let face = self.faces[face_pos.0][face_pos.1].as_ref().unwrap();

        if face[pos.0 as usize][pos.1 as usize] != BLOCK {
            self.pos = pos;
            self.face_pos = face_pos;
            self.forward(count - 1)
        }
    }
}

enum FaceType {
    Front = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Left = 4,
    Back = 5,
}

struct Cube {
    faces: [Option<Face>; 6],
    size: usize,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            faces: [None, None, None, None, None, None],
            size: 0,
        }
    }

    fn get_front(&self) -> Option<&Face> {
        self.faces[FaceType::Front as usize].as_ref()
    }

    fn is_open(&self, pos: (usize, usize)) -> bool {
        self.faces[FaceType::Front as usize].as_ref().unwrap()[pos.0][pos.1] == OPEN
    }

    fn set_front(&mut self, face: Face) {
        self.size = face.len();
        self.faces[FaceType::Front as usize] = Some(face);
    }

    fn permute(&mut self, dir: Direction) {
        let front = self.faces[FaceType::Front as usize].take();

        let mut top = self.faces[FaceType::Top as usize].take();
        let mut right = self.faces[FaceType::Right as usize].take();
        let mut bottom = self.faces[FaceType::Bottom as usize].take();
        let mut left = self.faces[FaceType::Left as usize].take();
        let mut back = self.faces[FaceType::Back as usize].take();

        match dir {
            Direction::Up => {
                Cube::rotate_face_270(right.as_mut());
                Cube::rotate_face_90(left.as_mut());

                Cube::rotate_face_180(back.as_mut());
                Cube::rotate_face_180(bottom.as_mut());

                self.faces = [top, back, right, front, left, bottom];
            }

            Direction::Right => {
                Cube::rotate_face_90(top.as_mut());
                Cube::rotate_face_270(bottom.as_mut());

                self.faces = [right, top, back, bottom, front, left];
            }

            Direction::Down => {
                Cube::rotate_face_90(right.as_mut());
                Cube::rotate_face_270(left.as_mut());

                Cube::rotate_face_180(back.as_mut());
                Cube::rotate_face_180(top.as_mut());

                self.faces = [bottom, front, right, back, left, top];
            }

            Direction::Left => {
                Cube::rotate_face_270(top.as_mut());
                Cube::rotate_face_90(bottom.as_mut());

                self.faces = [left, top, front, bottom, back, right];
            }
        }
    }

    fn permute_back(&mut self, dir: Direction) {
        self.permute(match dir {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        });
    }

    fn rotate_face_90(face: Option<&mut Face>) {
        if let Some(face) = face {
            let n = face.len() - 1;
            Self::transform_face(face, |i, j| (n - j, i));
        }
    }

    fn rotate_face_180(face: Option<&mut Face>) {
        if let Some(face) = face {
            let n = face.len() - 1;
            Self::transform_face(face, |i, j| (n - i, n - j));
        }
    }

    fn rotate_face_270(face: Option<&mut Face>) {
        if let Some(face) = face {
            let n = face.len() - 1;
            Self::transform_face(face, |i, j| (j, n - i));
        }
    }

    fn transform_face<F>(face: &mut Face, transform: F)
    where
        F: Fn(usize, usize) -> (usize, usize),
    {
        let n = face.len();
        let mut new_face = vec![vec![OPEN; n]; n];

        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..n {
                let (ti, tj) = transform(i, j);
                new_face[i][j] = face[ti][tj];
            }
        }

        *face = new_face;
    }
}

struct CubeSolver {
    cube: Cube,
    current: (usize, usize),
    facing: Direction,
}

type ValidityChecker = Box<dyn Fn((usize, usize)) -> bool>;
type AfterPermuter = Box<dyn Fn((usize, usize)) -> (usize, usize)>;
impl CubeSolver {
    fn new(faces: &mut [Vec<Option<Face>>]) -> Self {
        Self {
            cube: fold_cube(faces),
            current: (0, 0),
            facing: Direction::Right,
        }
    }

    fn move_forward(&mut self, count: usize) {
        let n = self.cube.size;
        let mut pos = self.current;

        let is_valid: ValidityChecker = match self.facing {
            Direction::Up => Box::new(move |(i, _): (usize, usize)| i > 0),
            Direction::Down => Box::new(move |(i, _): (usize, usize)| i < n - 1),
            Direction::Right => Box::new(move |(_, j): (usize, usize)| j < n - 1),
            Direction::Left => Box::new(move |(_, j): (usize, usize)| j > 0),
        };

        let after_permute: AfterPermuter = match self.facing {
            Direction::Up => Box::new(move |(_, j): (usize, usize)| (n - 1, j)),
            Direction::Down => Box::new(move |(_, j): (usize, usize)| (0, j)),
            Direction::Right => Box::new(move |(i, _): (usize, usize)| (i, 0)),
            Direction::Left => Box::new(move |(i, _): (usize, usize)| (i, n - 1)),
        };

        for _ in 0..count {
            if is_valid(pos) {
                let next_pos = match self.facing {
                    Direction::Up => (pos.0 - 1, pos.1),
                    Direction::Down => (pos.0 + 1, pos.1),
                    Direction::Left => (pos.0, pos.1 - 1),
                    Direction::Right => (pos.0, pos.1 + 1),
                };

                if !self.cube.is_open(next_pos) {
                    break;
                }

                pos = next_pos;
            } else {
                self.cube.permute(self.facing);
                let next_pos = after_permute(pos);

                if !self.cube.is_open(next_pos) {
                    self.cube.permute_back(self.facing);
                    break;
                }

                pos = next_pos;
            }
        }

        self.current = pos;
    }

    fn rotate_right(&mut self) {
        Direction::rotate_right(&mut self.facing);
    }

    fn rotate_left(&mut self) {
        Direction::rotate_left(&mut self.facing);
    }
}

impl Solver for CubeSolver {
    fn rotate_right(&mut self) {
        self.rotate_right()
    }

    fn rotate_left(&mut self) {
        self.rotate_left()
    }

    fn forward(&mut self, count: usize) {
        self.move_forward(count)
    }
}

fn parse_face(face: &[&str]) -> Option<Face> {
    let face: Face = face.iter().map(|&s| s.chars().collect()).collect();
    if face[0][0] == EMPTY {
        None
    } else {
        Some(face)
    }
}

fn parse_row(row: &[&str], n: usize) -> Vec<Option<Face>> {
    (0..row[0].len() / n)
        .map(|i| {
            parse_face(
                &row.iter()
                    .map(|r| &r[i * n..(i + 1) * n])
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn fold_cube(faces: &mut [Vec<Option<Face>>]) -> Cube {
    let first = faces[0].iter().position(|f| f.is_some()).unwrap();
    let mut cube = Cube::new();
    fold_cube_dfs(&mut faces.to_owned(), (0, first), &mut cube);
    cube
}

fn fold_cube_dfs(faces: &mut [Vec<Option<Face>>], pos: (usize, usize), cube: &mut Cube) {
    if let Some(face) = faces[pos.0][pos.1].take() {
        cube.set_front(face);

        if pos.0 > 0 && pos.1 < faces[pos.0 - 1].len() {
            cube.permute(Direction::Up);
            fold_cube_dfs(faces, (pos.0 - 1, pos.1), cube);
            cube.permute_back(Direction::Up);
        }

        if pos.0 < faces.len() - 1 && pos.1 < faces[pos.0 + 1].len() {
            cube.permute(Direction::Down);
            fold_cube_dfs(faces, (pos.0 + 1, pos.1), cube);
            cube.permute_back(Direction::Down);
        }

        if pos.1 > 0 && faces[pos.0][pos.1 - 1].is_some() {
            cube.permute(Direction::Left);
            fold_cube_dfs(faces, (pos.0, pos.1 - 1), cube);
            cube.permute_back(Direction::Left);
        }

        if pos.1 < faces[pos.0].len() - 1 && faces[pos.0][pos.1 + 1].is_some() {
            cube.permute(Direction::Right);
            fold_cube_dfs(faces, (pos.0, pos.1 + 1), cube);
            cube.permute_back(Direction::Right);
        }
    }
}
