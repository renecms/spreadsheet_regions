use std::time::Instant;

use sheet::Sheet;

pub mod cell;
pub mod region;
pub mod sheet;

pub fn get_values_from_regions(sheet_borrow: &Sheet) {
    let mut vec_result = Vec::with_capacity(sheet_borrow.rows);
    let now = Instant::now();
    for cell in sheet_borrow.cells.borrow().iter() {
        vec_result.push(cell.get_region_value().unwrap_or_default());
    }
    let elapsed = now.elapsed();
    println!(
        "Elapsed time when getting data from region: {:.2?}",
        elapsed
    );
    println!("{:?}", vec_result);
}

pub fn getting_data_directly_from_cell(sheet_borrow: &Sheet) {
    let initial_time = Instant::now();
    copy_values_from_regions_to_cells(&sheet_borrow);
    let copy_elapsed = initial_time.elapsed();

    let get_initial_time = Instant::now();
    let mut vec_result = Vec::with_capacity(sheet_borrow.rows);
    for cell in sheet_borrow.cells.borrow().iter() {
        vec_result.push(cell.value.clone().unwrap_or("".to_string()).clone());
    }
    let get_elapsed = get_initial_time.elapsed();

    println!(
        "Elapsed time when copying data from region to cells: {:.2?}",
        copy_elapsed
    );
    println!(
        "Elapsed time when getting data from cells: {:.2?}",
        get_elapsed
    );
    println!("{:?}", vec_result);

    println!(
        "Total time when getting data from cells: {:.2?}",
        copy_elapsed + get_elapsed
    );
}

pub fn copy_values_from_regions_to_cells(sheet_borrow: &Sheet) {
    let mut cells = sheet_borrow.cells.borrow_mut();
    for region in &sheet_borrow.regions {
        for row in region.row_start..=region.row_end {
            for column in region.column_start..=region.column_end {
                cells
                    .get_mut(row * sheet_borrow.columns + column)
                    .map(|cell| cell.value = Some(region.value.clone()));
            }
        }
    }
}
