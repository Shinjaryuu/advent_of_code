use std::cmp;
use std::io::{Error,ErrorKind};

#[derive(PartialEq)]
struct Coordinates {
    x:i32,
    y:i32,
}

impl Coordinates {
    fn get_neighbours(&self) -> Vec<Coordinates> {
        let n = cmp::max(self.x.abs(),self.y.abs());
        if n == 0 {
            return Vec::new();
        }
        let left = Coordinates{x:self.x-1,y:self.y};
        let left_down = Coordinates{x:self.x-1,y:self.y-1};
        let down = Coordinates{x:self.x,y:self.y-1};
        let right_down = Coordinates{x:self.x+1,y:self.y-1};
        let right = Coordinates{x:self.x+1,y:self.y};
        let right_up = Coordinates{x:self.x+1,y:self.y+1};
        let up = Coordinates{x:self.x,y:self.y+1};
        let left_up = Coordinates{x:self.x-1,y:self.y+1};
        return vec![left,left_down,down,right_down,right,right_up,up,left_up];
    }

    fn manahatten_distance(&self) -> u32 {
        return (self.x.abs() + self.y.abs()) as u32;
    }
}

struct Point {
    coords : Coordinates,
    value : i32,
}

struct SumSpiral {
    points : Vec<Point>,
}

impl SumSpiral {
    fn constr() -> SumSpiral {
        let p = vec![Point{coords:Coordinates{x:0,y:0}, value:1}];
        return SumSpiral{points:p};
    }

    fn next_coordinates(c : &Coordinates) -> Coordinates {
        let n = cmp::max(c.x.abs(),c.y.abs());
        if c.x == n {
            if c.y == -n {
                return Coordinates{x:c.x+1, y: c.y};
            }
            if c.y == n {
                return Coordinates{x: c.x-1, y: c.y};
            }
            return Coordinates{x: c.x, y : c.y+1};
        }
        if c.y == n {
            if c.x == -n {
                return Coordinates{x:c.x, y: c.y-1};
            }
            return Coordinates{x:c.x-1, y:c.y};
        }
        if c.x == -n {
            if c.y == -n {
                return Coordinates{x: c.x+1, y:c.y};
            }
            return Coordinates{x:c.x, y:c.y-1};
        }
        return Coordinates{x:c.x+1, y:c.y};
    }

    fn get_value(&self, c:&Coordinates) -> Result<i32,Error> {
        for p in self.points.iter() {
            if p.coords == *c {
                return Ok(p.value);
            }
        }
        return Err(Error::new(ErrorKind::Other,"Point not found!"));
    }

    fn get_next_point(&mut self) -> () {
        //let current = &self.points[self.points.len()-1];
        let next_coords = SumSpiral::next_coordinates(&self.points[self.points.len()-1].coords);
        let neighbours = next_coords.get_neighbours();
        let mut next_value = 0;
        for neighbour in neighbours.iter() {
            next_value += match self.get_value(neighbour) {
                Ok(v) => v,
                Err(_) => 0,
            }
        }
        self.points.push(Point{coords:next_coords, value:next_value});
    }
}

struct Spiral {
    points : Vec<Point>,
}

impl Spiral {
    fn constr() -> Spiral {
        let p = vec![Point{coords:Coordinates{x:0,y:0}, value:1}];
        return Spiral{points:p};
    }

    fn next_coordinates(c : &Coordinates) -> Coordinates {
        let n = cmp::max(c.x.abs(),c.y.abs());
        if c.x == n {
            if c.y == -n {
                return Coordinates{x:c.x+1, y: c.y};
            }
            if c.y == n {
                return Coordinates{x: c.x-1, y: c.y};
            }
            return Coordinates{x: c.x, y : c.y+1};
        }
        if c.y == n {
            if c.x == -n {
                return Coordinates{x:c.x, y: c.y-1};
            }
            return Coordinates{x:c.x-1, y:c.y};
        }
        if c.x == -n {
            if c.y == -n {
                return Coordinates{x: c.x+1, y:c.y};
            }
            return Coordinates{x:c.x, y:c.y-1};
        }
        return Coordinates{x:c.x+1, y:c.y};
    }

    fn get_next_point(&mut self) -> () {
        //let current = &self.points[self.points.len()-1];
        let next_coords = Spiral::next_coordinates(&self.points[self.points.len()-1].coords);
        let next_value = self.points[self.points.len()-1].value + 1;
        self.points.push(Point{coords:next_coords, value:next_value});
    }
}

fn find_next_higher_value(v : i32) -> i32 {
    let mut spiral = SumSpiral::constr();
    while spiral.points[spiral.points.len()-1].value < v {
        spiral.get_next_point();
    }
    return spiral.points[spiral.points.len()-1].value;
}

fn find_manhatten_distance(v : i32) -> u32 {
    let mut spiral = Spiral::constr();
    while spiral.points[spiral.points.len()-1].value < v {
        spiral.get_next_point();
    }
    return spiral.points[spiral.points.len()-1].coords.manahatten_distance();
}

fn main() {
    let input = 347991;
    println!("Distance from center in normal spiral of {} is {}.",input,find_manhatten_distance(input));
    println!("Next highest value in sum-spiral after {} is {}.",input,find_next_higher_value(input));
}
