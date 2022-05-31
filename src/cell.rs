use std::{cell::RefCell, rc::Rc};

use crate::sheet::Sheet;

pub struct Cell {
    pub sheet: Rc<RefCell<Sheet>>,
    pub row_index: usize,
    pub column_index: usize,
    pub value: Option<String>,
}

impl Cell {
    pub fn get_region_value(&self) -> Option<String> {
        self.sheet
            .borrow()
            .regions
            .iter()
            .find(|region| region.contains(self))
            .map(|region| region.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, region::Region, sheet::Sheet};

    #[test]
    fn test_cells_get_region_value() {
        let region = Region {
            row_start: 1,
            column_start: 1,
            row_end: 5,
            column_end: 5,
            value: "Region Value".to_string(),
        };
        let sheet = Sheet::new(10, 10);
        sheet.borrow_mut().set_regions(vec![region]);
        let cell_in_region = Cell {
            sheet: sheet.clone(),
            row_index: 1,
            column_index: 2,
            value: Some("3".to_string()),
        };
        let cell_not_in_region = Cell {
            sheet: sheet.clone(),
            row_index: 3,
            column_index: 6,
            value: Some("9".to_string()),
        };

        assert_eq!(
            "Region Value".to_string(),
            cell_in_region.get_region_value().unwrap()
        );
        assert_eq!(None, cell_not_in_region.get_region_value());
    }
}
