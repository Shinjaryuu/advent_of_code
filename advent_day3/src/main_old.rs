use std::cmp;
use std::io::{Error,ErrorKind};
//use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
struct Coordinates{
    x : i32,
    y : i32,
}

impl Coordinates {
    fn next(self) -> Coordinates {
        let n = cmp::max(self.x.abs(),self.y.abs());
        if self.x == n {
            if self.y == -n {
                return Coordinates{x:self.x+1, y: self.y};
            }
            if self.y == n {
                return Coordinates{x: self.x-1, y: self.y};
            }
            return Coordinates{x: self.x, y : self.y+1};
        }
        if self.y == n {
            if self.x == -n {
                return Coordinates{x:self.x, y: self.y-1};
            }
            return Coordinates{x:self.x-1, y:self.y};
        }
        if self.x == -n {
            if self.y == -n {
                return Coordinates{x: self.x+1, y:self.y};
            }
            return Coordinates{x:self.x, y:self.y-1};
        }
        return Coordinates{x:self.x+1, y:self.y};
    }

    fn get_previous_neighbours(&self) -> Vec<Coordinates>{
        let n = cmp::max(self.x.abs(),self.y.abs());
        if n == 0 {
            return Vec::new();
        }
        let left = Coordinates{x:self.x-1,y:self.y};
        let left_down = Coordinates{x:self.x-1,y:self.y-1};
        let down = Coordinates{x:self.x,y:self.y-1};
        let right_down = Coordinates{x:self.x+1,y:self.y-1};
        let right = Coordinates{x:self.x+11,y:self.y};
        let right_up = Coordinates{x:self.x+1,y:self.y+1};
        let up = Coordinates{x:self.x,y:self.y+1};
        let left_up = Coordinates{x:self.x-1,y:self.y+1};
        let possible_neighbours = vec![left,left_down,down,right_down,right,right_up,up,left_up];
        for neihbour in possible_neighbours {
        }
        //if self.x == n {
        //    if self.y == -n {
        //        return vec![left,left_up,up];
        //    }
        //    if self.y == n {
        //        return vec![left_down,down];
        //    }
        //    if self.y == n-1 {
        //        return vec![left,left_down,down];   //Problems here still with when left up and right up exist
        //    }
        //    return vec![left_up,left,left_down,down];
        //}
        //if self.y == n{
        //    if self.x == -n {
        //        return vec![right_down,right];
        //    }
        //    return vec![left_down,down,right_down,right];
        //}
        //if self.x == -n {
        //    if self.y == -n {
        //        return vec![right_up,up];
        //    }
        //    return vec![right_down,right,right_up,up];
        //}
        //return vec![right_up,up,left_up,left];
    }
}

fn get_point(c: &Coordinates, g : &Vec<Point>) -> Result<Point,Error> {
    for p in g {
        if p.coords == *c {
            return Ok(p.clone());
        }
    }
    return Err(Error::new(ErrorKind::Other,"Point not found!"));

}


fn find_max_sum(max_value:i32)->i32 {
    let mut g : Vec<Point> = Vec::new();
    let mut current_coords = Coordinates{x:0,y:0};
    let mut current_value = 1;
    while current_value < max_value{
        println!("Coords: {:?}, Value: {}",current_coords,current_value);
        g.push(Point{coords:current_coords.clone(), value:current_value});
        current_coords = current_coords.next();
        println!("Next coords: {:?}",current_coords);
        let neighbours = current_coords.get_previous_neighbours();
        println!("{:?}",neighbours);
        current_value = neighbours.iter().map(|x| get_point(x,&g).value).sum();
    }
    return current_value;
}

#[derive(Clone)]
struct Point{
    coords : Coordinates,
    value : i32,
}

impl Point{
    fn distance_from_center(&self) -> i32{
        return self.coords.x.abs()+self.coords.y.abs();
    }
}

struct Square{
    n : i32,
    points : Vec<Point>,
    side_length : i32,
}

impl Square{
    fn constr(n:i32) -> Square{
        if n==0 {
            let point = Point{coords:Coordinates{x:0,y:0},value:1};
            return Square{n:0, points:vec![point],side_length:1};
        }
        let side_length = 2*n+1;
        //The lower right corner of the nth square is given by (2n+1)², where the '1' in the center
        // is counted as the 0th square. This is the largest number in that square.
        //The other corners are always a side_length away
        let lower_right_corner = side_length.pow(2);
        let lower_left_corner = lower_right_corner - side_length + 1;
        let upper_left_corner = lower_left_corner - side_length + 1;
        let upper_right_corner = upper_left_corner - side_length + 1;
        let mut value = (2*n-1).pow(2) + 1; //lowest number in the square
        let mut coords = Coordinates{ x : i32::from(n), y : -i32::from(n-1)}; //coordinates of lowest number
        let mut points : Vec<Point> = Vec::new();
        while value < upper_right_corner {
            let p = Point{coords:coords.clone(), value:value};
            points.push(p);
            value = value + 1;
            coords.y = coords.y + 1;
        }
        while value < upper_left_corner {
            let p = Point{coords:coords.clone(), value:value};
            points.push(p);
            value = value + 1;
            coords.x = coords.x - 1;
        }
        while value < lower_left_corner {
            let p = Point{coords:coords.clone(), value:value};
            points.push(p);
            value = value + 1;
            coords.y = coords.y - 1;
        }
        while value < lower_right_corner {
            let p = Point{coords:coords.clone(), value:value};
            points.push(p);
            value = value + 1;
            coords.x = coords.x + 1;
        }
        points.push(Point{coords:coords.clone(), value:value});
        return Square{n:n, points:points, side_length:side_length};
    }

    fn find_point(&self,n:i32) -> Option<&Point> {
        for p in self.points.iter(){
            if p.value == n {
                return Some(p);
            }
        }
        return None;
    }
}

fn main() {
    let input = 347991;
    //let input = 1;
    let mut n :i32 = 0;

    while (2*n+1).pow(2) < input{
        n += 1;
    };

    let s = Square::constr(n);
    let p = match s.find_point(input) {
        Some(point) => point,
        None => panic!("Point not found in square!"),
    };
    println!("Distance of {} from center: {:?}",
                input,p.distance_from_center());
    println!("Highest sum-value printed after {} is {:?}.",input,find_max_sum(input));



    //The lower right corner of the nth square is at coordinates (n,n)


}
