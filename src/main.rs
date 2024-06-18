fn main() {
    let mut grid = [0.0 as f32; 16];

    let gen = perlin::Perlin2D::new();

    for y in 0..4 {
        for x in 0..4 {
            grid[y * 4 + x] = gen.perlin(x as f32 * 0.1, y as f32 * 0.1);
        }
    }

    for y in 0..4 {
        println!(
            "{}, {}, {}, {}",
            grid[y * 4],
            grid[y * 4 + 1],
            grid[y * 4 + 2],
            grid[y * 4 + 3]
        );
    }
}
