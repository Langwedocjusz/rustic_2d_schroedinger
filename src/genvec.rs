//Small utility to generate a 2d vector based on a function

pub fn generate_vec2d<T, F>(size_x: usize, size_y: usize, f: F) -> Vec<Vec<T>> where
    F: Fn(usize, usize) -> T {
    (0..size_x).map(|i| {(0..size_y).map(|j| {f(i,j)}).collect()}).collect()
}