use std::fs::File;
use std::io;
use std::io::Read;

fn get_input(filename:&str) -> Result<Vec<i32>, io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let num_vec : Vec<i32> = contents.split_whitespace().map(|x| x.parse().unwrap()).collect();
    return Ok(num_vec);
}



fn steps_through_maze(mut maze : Vec<i32>) -> u32 {
    let mut counter = 0;
    let mut position = 0;
    while position < maze.len() {
        counter += 1;
        let new_position = (position as i32) + maze[position];
        if maze[position] < 3 {
            maze[position] += 1;
        }
        else {
            maze[position] -= 1;
        }
        position = new_position as usize;
    }
    return counter;
}

fn main() {
    //let input = vec![0,3,0,1,-3]
    let input = match get_input("Input") {
        Ok(v) => v,
        Err(_) => panic!("Problem when reading file."),
    };
    println!("{} steps.",steps_through_maze(input));
}
