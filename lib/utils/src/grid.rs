use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

pub struct Grid<T> {
    inner: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(inner: Vec<Vec<T>>) -> Grid<T> {
        let height = inner.len();
        let width = if height != 0 { inner[0].len() } else { 0 };

        Grid {
            inner,
            height,
            width,
        }
    }

    pub fn adjacent_coords(&self, coords: Coords) -> Vec<Coords> {
        let mut adjacent = vec![];

        for dx in [-1isize, 0, 1] {
            for dy in [-1isize, 0, 1] {
                if dy == 0 && dx == 0 {
                    continue;
                }

                let x = coords.x as isize + dx;
                let y = coords.y as isize + dy;

                if y < 0 || y >= self.height as isize {
                    continue;
                }

                let y = y as usize;

                if x < 0 || x >= self.width as isize {
                    continue;
                }

                let x = x as usize;

                adjacent.push(Coords { x, y })
            }
        }

        adjacent
    }

    pub fn rows(&self) -> impl Iterator<Item = (usize, &Vec<T>)> {
        self.inner.iter().enumerate()
    }
}

impl<T> Index<Coords> for Grid<T> {
    type Output = T;

    fn index(&self, coords: Coords) -> &Self::Output {
        &self.inner[coords.y][coords.x]
    }
}
