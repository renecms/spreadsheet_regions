use crate::cell::Cell;

pub struct Region {
    pub row_start: usize,
    pub column_start: usize,
    pub row_end: usize,
    pub column_end: usize,
    pub value: String,
}

impl Region {
    pub fn contains(&self, cell: &Cell) -> bool {
        self.row_start <= cell.row_index
            && self.row_end >= cell.row_index
            && self.column_start <= cell.column_index
            && self.column_end >= cell.column_index
    }
}

#[cfg(test)]
mod tests {
        use crate::{cell::Cell, region::Region, sheet::Sheet};

    #[test]
    fn test_region_contains_cells() {
        let sheet = Sheet::new(10, 10);
        let cell_in_region = Cell {
            sheet: sheet.clone(),
            row_index: 1,
            column_index: 2,
            value: Some("3".to_string())
        };
        let cell_not_in_region = Cell {
            sheet: sheet.clone(),
            row_index: 3,
            column_index: 6,
            value: Some("9".to_string())
        };
        let region = Region {
            row_start: 1,
            column_start: 1,
            row_end: 5,
            column_end: 5,
            value: "Region Value".to_string(),
        };

        assert!(region.contains(&cell_in_region));
        assert!(!region.contains(&cell_not_in_region));
    }
}
