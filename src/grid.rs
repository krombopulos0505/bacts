pub struct Grid<T> {
    w: usize,
    h: usize,
    cells: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            cells: vec![T::default(); (w*h)],
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        self.w * y + x
    }

    pub get(&self, x: usize, y: usize) -> T {
        self.cells[self.idx(x, y)]
    }

    pub get_mut(&self, x: usize, y; usize) -> T {
        &self.cells[self.idx(x, y)]
    }
}
