pub struct Life {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        Life {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn randomize(&mut self, p_alive: f64) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for cell in self.cells.iter_mut() {
            *cell = rng.gen_bool(p_alive);
        }
    }

    #[inline]
    fn is_alive(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            false
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.cells[idx]
        }
    }

    fn live_neighbors(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                if self.is_alive(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn step(&mut self) {
        let mut next = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let alive = self.cells[idx];
                let n = self.live_neighbors(x, y);
                next[idx] = match (alive, n) {
                    (true, 2) | (true, 3) => true,  // sobrevive
                    (false, 3) => true,             // nace
                    _ => false,                     // muere o sigue muerto
                };
            }
        }
        self.cells = next;
    }

    pub fn set_cells_alive(&mut self, coords: &[(usize, usize)]) {
        for &(x, y) in coords {
            if x < self.width && y < self.height {
                let idx = y * self.width + x;
                self.cells[idx] = true;
            }
        }
    }

}
