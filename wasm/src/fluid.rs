use crate::programs::Cell;
use web_sys::WebGlRenderingContext as GL;

pub struct CellMap {
    cells: Vec<Cell>,
    divisions: usize,
}

impl CellMap {
    pub fn new(gl: &GL, divisions: usize) -> CellMap {
        let cell_size = 2.0 / (divisions as f32);
        let mut cells: Vec<Cell> = Vec::with_capacity(divisions * divisions);

        for x in 0..divisions {
            for y in 0..divisions {
                cells.push(Cell::new(
                    &gl,
                    -1.0 + cell_size * x as f32,
                    -1.0 + cell_size * x as f32 + cell_size,
                    -1.0 + cell_size * y as f32,
                    -1.0 + cell_size * y as f32 + cell_size,
                ))
            }
        }

        CellMap { cells, divisions }
    }

    pub fn render(&self, gl: &GL, proj: &[f32]) {
      for x in 0..self.divisions {
        for y in 0..self.divisions  {
            self.cells[y * self.divisions + x ].render(gl, &proj, x as f32 / self.divisions as f32, y as f32 / self.divisions as f32, 1.0 - x as f32 / self.divisions as f32 )
        }
    }}

}
