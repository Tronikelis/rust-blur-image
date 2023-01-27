use eyre::{eyre, Result};

pub type Dimension = (i32, i32);

pub fn calc_dimensions(pixels: &i32) -> Result<Vec<Dimension>> {
    if *pixels < 0 {
        return Err(eyre!("[pixels] can't be negative, got {pixels}"));
    }

    let base_x = 3;
    let base_y = 3;

    let mut dimensions: Vec<Dimension> = vec![];

    let range_x = base_x + pixels;
    let range_y = base_y + pixels;

    // generates dimensions from the bottom left
    for x in 0..range_x {
        for y in 0..range_y {
            dimensions.push((x, y));
        }
    }

    let back_x = range_x - 2;
    let back_y = range_x - 2;

    // move back the dimensions to the middle
    // filter out the (0, 0), it is the start point
    dimensions = dimensions
        .iter()
        .map(|(x, y)| (x - back_x, y - back_y))
        .filter(|(x, y)| !(*x == 0 && *y == 0))
        .collect();

    Ok(dimensions)
}
