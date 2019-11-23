fn main() {
    test_1d();
    test_2d();
}

fn test_1d() {
    let mut simple_vector = vec![0; 9];

    simple_vector[0] = 1;
    simple_vector[8] = 9;

    println!("[0] is {}", simple_vector[0]);
    println!("[8] is {}", simple_vector[8]);
}

fn test_2d() {
    let width: usize = 10;
    let height: usize = 8;
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    set_2d(grid);

    println!("[0:0] is {}", grid[0][0]);
    println!("[1:9] is {}", grid[1][9]);
    println!("[7:9] is {}", grid[7][9]);
}

fn set_2d(grid: &mut [&mut [usize]]) {
    grid[0][0] = 1;
    grid[1][9] = 19;
    grid[7][9] = 79;
}
