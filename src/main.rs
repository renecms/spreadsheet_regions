use spreadsheet_regions::{get_values_from_regions, getting_data_directly_from_cell, sheet::Sheet};

fn main() {
    let sheet = Sheet::new(100, 10);

    get_values_from_regions(&sheet.borrow());

    getting_data_directly_from_cell(&sheet.borrow());
}
