use std::ops::{Index, IndexMut};
use vek::*;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    size: Vec2<i32>,
}

impl<T> Grid<T> {
    pub fn populate_with(size: Vec2<i32>, mut f: impl FnMut(Vec2<i32>) -> T) -> Self {
        Self {
            cells: (0..size.y)
                .map(|y| (0..size.x).map(move |x| Vec2::new(x, y)))
                .flatten()
                .map(&mut f)
                .collect(),
            size,
        }
    }

    pub fn new(size: Vec2<i32>, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            cells: vec![default; size.product() as usize],
            size,
        }
    }

    fn idx(&self, pos: Vec2<i32>) -> Option<usize> {
        if pos.map2(self.size, |e, sz| e >= 0 && e < sz).reduce_and() {
            Some((pos.y * self.size.x + pos.x) as usize)
        } else {
            None
        }
    }

    pub fn size(&self) -> Vec2<i32> {
        self.size
    }
    pub fn get(&self, pos: Vec2<i32>) -> Option<&T> {
        self.cells.get(self.idx(pos)?)
    }
    pub fn get_cloned(&self, pos: Vec2<i32>) -> Option<T>
    where
        T: Clone,
    {
        match self.get(pos) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
    pub fn get_mut(&mut self, pos: Vec2<i32>) -> Option<&mut T> {
        let idx = self.idx(pos)?;
        self.cells.get_mut(idx)
    }

    pub fn set(&mut self, pos: Vec2<i32>, cell: T) -> Option<T> {
        let idx = self.idx(pos)?;
        self.cells.get_mut(idx).map(|c| core::mem::replace(c, cell))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2<i32>, &T)> + '_ {
        let w = self.size.x;
        self.cells
            .iter()
            .enumerate()
            .map(move |(i, cell)| (Vec2::new(i as i32 % w, i as i32 / w), cell))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Vec2<i32>, &mut T)> + '_ {
        let w = self.size.x;
        self.cells
            .iter_mut()
            .enumerate()
            .map(move |(i, cell)| (Vec2::new(i as i32 % w, i as i32 / w), cell))
    }
}

impl<T> Index<Vec2<i32>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2<i32>) -> &Self::Output {
        self.get(index).unwrap_or_else(|| {
            panic!(
                "Attempted to index grid of size {:?} with index {:?}",
                self.size(),
                index
            )
        })
    }
}

impl<T> IndexMut<Vec2<i32>> for Grid<T> {
    fn index_mut(&mut self, index: Vec2<i32>) -> &mut Self::Output {
        let size = self.size();
        self.get_mut(index).unwrap_or_else(|| {
            panic!(
                "Attempted to index grid of size {:?} with index {:?}",
                size, index
            )
        })
    }
}
