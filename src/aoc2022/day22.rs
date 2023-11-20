use std::fs;

const EMPTY: char = ' ';
const OPEN: char = '.';

type Face = Vec<Vec<char>>;

#[derive(Copy, Clone)]
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

    fn transform_face<F>(face: &mut Face, transform: F) where F: Fn(usize, usize) -> (usize, usize) {
        let n = face.len();
        let mut new_face = vec![vec![OPEN; n]; n];

        for i in 0..n {
            for j in 0..n {
                let (ti, tj) = transform(i, j);
                new_face[i][j] = face[ti][tj];
            }
        }

        *face = new_face;
    }
}

struct Solver {
    cube: Cube,
    current: (usize, usize),
    facing: Direction,
}

impl Solver {
    fn new(cube: Cube) -> Self {
        Self {
            cube,
            current: (0, 0),
            facing: Direction::Right,
        }
    }

    fn move_forward(&mut self, count: usize) {
        let n = self.cube.size;
        let mut pos = self.current;

        let is_valid: Box<dyn Fn((usize, usize)) -> bool> = match self.facing {
            Direction::Up    => Box::new(|(i, _): (usize, usize)| i > 0),
            Direction::Down  => Box::new(|(i, _): (usize, usize)| i < n - 1),
            Direction::Right => Box::new(|(_, j): (usize, usize)| j < n - 1),
            Direction::Left  => Box::new(|(_, j): (usize, usize)| j > 0),
        };

        let after_permute: Box<dyn Fn((usize, usize)) -> (usize, usize)> = match self.facing {
            Direction::Up    => Box::new(|(_, j): (usize, usize)| (n - 1, j)),
            Direction::Down  => Box::new(|(_, j): (usize, usize)| (0, j)),
            Direction::Right => Box::new(|(i, _): (usize, usize)| (i, 0)),
            Direction::Left  => Box::new(|(i, _): (usize, usize)| (i, n - 1)),
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

fn fold_cube(faces: &mut Vec<Vec<Option<Face>>>) -> Cube {
    let first = faces[0].iter().position(|f| f.is_some()).unwrap();
    let mut cube = Cube::new();
    fold_cube_dfs(&mut faces.clone(), (0, first), &mut cube);
    cube
}

fn fold_cube_dfs(faces: &mut Vec<Vec<Option<Face>>>, pos: (usize, usize), cube: &mut Cube) {
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

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let (map, instructions) = content.split_once("\n\n").unwrap();
    let n = map.split_whitespace().map(|s| s.len()).min().unwrap();

    let lines = map.lines().collect::<Vec<_>>();
    let mut faces = (0..lines.len() / n)
        .map(|i| parse_row(&lines[i * n..(i + 1) * n], n))
        .collect::<Vec<_>>();

    let mut solver = Solver::new(fold_cube(&mut faces));

    let mut stack = Vec::new();
    for char in instructions.trim().chars() {
        if '0' <= char && char <= '9' {
            stack.push(char);
            continue;
        }

        let forward = String::from_iter(stack.drain(..)).parse::<usize>().unwrap();
        solver.move_forward(forward);

        match char {
            'R' => solver.rotate_right(),
            'L' => solver.rotate_left(),
            _ => unreachable!(),
        }
    }

    if let Ok(forward) = String::from_iter(stack.drain(..)).parse::<usize>() {
        solver.move_forward(forward);
    }

    'outer: for (i, row) in faces.iter_mut().enumerate() {
        for (j, maybe) in row.iter_mut().enumerate().filter(|(_, f)| f.is_some()) {
            let face = maybe.as_mut().unwrap();
            let mut pos = solver.current;
            let mut facing = solver.facing;

            for _ in 0..4 {
                if face == solver.cube.get_front().unwrap() {
                    let final_pos = (i * n + pos.0 + 1, j * n + pos.1 + 1);

                    let facing_res = match facing {
                        Direction::Right => 0,
                        Direction::Down => 1,
                        Direction::Left => 2,
                        Direction::Up => 3,
                    };

                    println!("{}", 1000 * final_pos.0 + 4 * final_pos.1 + facing_res);
                    break 'outer;
                }

                Cube::rotate_face_90(Some(face));
                Direction::rotate_left(&mut facing);
                pos = (n - 1 - pos.1, pos.0);
            }
        }
    }
}
