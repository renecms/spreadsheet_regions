use std::{cell::RefCell, rc::Rc};

use crate::{cell::Cell, region::Region};

pub struct Sheet {
    pub rows: usize,
    pub columns: usize,
    pub regions: Vec<Region>,
    pub cells: RefCell<Vec<Cell>>,
}

impl Sheet {
    pub fn new(rows: usize, columns: usize) -> Rc<RefCell<Sheet>> {
        let sheet = Sheet {
            rows,
            columns,
            regions: Sheet::create_regions(rows, columns),
            cells: RefCell::new(Vec::new()),
        };
        let sheet_reference = Rc::new(RefCell::new(sheet));
        sheet_reference.borrow_mut().cells =
            Sheet::create_cells(sheet_reference.clone(), rows, columns);
        return sheet_reference;
    }

    pub fn set_regions(&mut self, regions: Vec<Region>) {
        self.regions = regions;
    }

    fn create_cells(sheet: Rc<RefCell<Sheet>>, rows: usize, columns: usize) -> RefCell<Vec<Cell>> {
        let mut cells = Vec::with_capacity(rows * columns);
        for row_index in 0..rows {
            for column_index in 0..columns {
                cells.push(Cell {
                    sheet: sheet.clone(),
                    row_index,
                    column_index,
                    value: None,
                })
            }
        }
        return RefCell::new(cells);
    }

    fn create_regions(rows: usize, columns: usize) -> Vec<Region> {
        let mut regions: Vec<Region> = Vec::with_capacity(rows * columns);
        for row_index in 0..rows {
            for column_index in 0..columns {
                let region = Region {
                    row_start: row_index,
                    column_start: column_index,
                    row_end: row_index,
                    column_end: column_index,
                    value: row_index.to_string() + "_" + &column_index.to_string(),
                };
                regions.push(region);
            }
        }
        return regions;
    }
}
