//An extension that allows for rendering of 2D grids to the screen.
pub struct Grid {
    pub mem_idx: u8,
    pub size: (u8, u8),
}

pub fn setup() -> Grid {
    return Grid::new(64, 64);
}
