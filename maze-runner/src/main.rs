mod maze_generator;

fn main() {
    let rows = 10;
    let cols = 10;

    let maze = maze_generator::generate_maze(rows, cols);

    for row in &maze {
        let line: String = row.iter().map(|&cell| if cell { ' ' } else { '#' }).collect();
        println!("{}", line);
    }
}
