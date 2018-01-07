#[derive(PartialEq)]
struct MemoryBanks {
    banks : Vec<u32>,
}

impl MemoryBanks {
    fn get_max(&self) -> usize {
        let mut current_max = 0;
        let mut current_max_ind = 0;
        for (i,&item) in self.banks.iter().enumerate() {
            if item > current_max {
                current_max = item;
                current_max_ind = i;
            }
        }
        return current_max_ind;
    }

    fn update(&mut self, ind : usize) {
        let to_distribute = self.banks[ind];
        self.banks[ind] = 0;
        self.banks = self.banks.iter().map(|x| x+(to_distribute / (self.banks.len() as u32))).collect();
        let remainder = (to_distribute as usize) % self.banks.len();
        if ind +1 + remainder < self.banks.len() {
            for i in ind+1..ind+1 + ((to_distribute as usize) % self.banks.len()) {
                self.banks[i] += 1;
            }
            //self.banks[ind+1..ind+1+((to_distribute as usize) % self.banks.len())] += 1;
        }
        else {
            for i in ind+1..self.banks.len() {
                self.banks[i] += 1;
            }
            for i in 0..remainder + ind+1 - self.banks.len() {
                self.banks[i] += 1;
            }
        }
    }
}

fn is_in_vec(entry:&Vec<u32>,v:&Vec<Vec<u32>>) -> bool {
    for e in v.iter() {
        if e == entry {
            return true;
        }
    }
    return false;
}

fn is_where_in_vec(entry:&Vec<u32>,v:&Vec<Vec<u32>>) -> usize {
    for (i,e) in v.iter().enumerate() {
        if e == entry {
            return i;
        }
    }
    return v.len();
}

fn times_until_loop(mem: &mut MemoryBanks) -> u32 {
    //let mut counter = 0;
    let mut occurred : Vec<Vec<u32>>= Vec::new();
    while !(is_in_vec(&mem.banks,&occurred)) {
        occurred.push(mem.banks.clone());
        let m = mem.get_max();
        mem.update(m);
        //counter += 1;
    }
    return occurred.len() as u32;
}

fn times_until_loop_and_length(mem: &mut MemoryBanks) -> (u32,u32) {
    //let mut counter = 0;
    let mut occurred : Vec<Vec<u32>>= Vec::new();
    while !(is_in_vec(&mem.banks,&occurred)) {
        occurred.push(mem.banks.clone());
        let m = mem.get_max();
        mem.update(m);
        //counter += 1;
    }
    return (occurred.len() as u32,(occurred.len()-is_where_in_vec(&mem.banks,&occurred)) as u32);
}

//fn times_until_loop_and_length(mem : &mut MemoryBanks) -> (u32,u32) {
    //let mut counter =
//}


fn main() {
    let input = vec![0,5,10,0,11,14,13,4,11,8,8,7,1,4,12,11];
    //let input = vec![0,2,7,0];
    let mut mem = MemoryBanks{banks:input};
    let (iterations,length) = times_until_loop_and_length(&mut mem);
    println!("Loop occurs after {} iterations and has a length of {}",iterations,length);

}
