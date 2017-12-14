use std::i64;

mod knot;

fn main() {
    let input = "jxqlasbh";
    let grid = generate_grid(input);
    println!("Part one: {}", grid.iter().filter(|&c| *c == '1').count());
    println!("Part two: {}", count_regions(grid.clone()));
}

fn generate_grid(input: &str) -> Vec<char> {
    (0..128)
        .map(|i| format!("{}-{}", input, i))
        .map(|k| knot::hash(&k))
        .map(|h| {
            h.chars()
                .map(|c| c.to_string())
                .filter_map(|c| i64::from_str_radix(&c, 16).ok())
                .map(|i| format!("{:04b}", i))
                .fold(String::new(), |acc, s| format!("{}{}", &acc, &s))
        })
        .fold(String::new(), |grid, row| format!("{}{}", &grid, &row))
        .chars()
        .collect()
}

fn count_regions(mut grid: Vec<char>) -> usize {
    let mut groups: usize = 0;
    for i in 0..grid.len() {
        if grid[i] == '1' {
            groups += 1;
            let mut frontier = vec![i];
            while !frontier.is_empty() {
                let pos = frontier.pop().unwrap();
                grid[pos] = ' ';
                for nb_pos in neighbour_coords(pos, 128) {
                    if nb_pos < grid.len() && grid[nb_pos] == '1' {
                        frontier.push(nb_pos);
                    }
                }
            }
        }
    }
    groups
}

fn neighbour_coords(pos: usize, grid_width: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![pos + grid_width];
    if pos > 0 && pos % grid_width != 0 {
        vec.push(pos - 1);
    }
    if (pos + 1) % grid_width != 0 {
        vec.push(pos + 1);
    }
    if pos >= grid_width {
        vec.push(pos - grid_width);
    }
    vec
}
