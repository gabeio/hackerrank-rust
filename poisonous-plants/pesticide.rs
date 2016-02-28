use std::io;

fn main() {
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap(); // read line into s
	s.clear(); // clear s (we don't care about how many in this language :D )
	io::stdin().read_line(&mut s).unwrap(); // read line into s
	let arr: Vec<u32> = s.trim().split(' '). // split per space
		map(|x| x.parse().unwrap()). // each do (trim.parse.unwrap)
		collect(); // collection
	println!("{}",max_days_alive(arr));
}

fn max_days_alive(arr: Vec<u32>) ->u32 {
	let mut stack: Vec<Plant> = vec!();
	let mut max_days_alive: u32 = 0;
	for pesticide in arr {
		let mut days_alive: u32 = 0;
		loop {
			// while the stack is not empty and:
			//       current pesticide is lessthan or equal to the last pesticide
			if !stack.is_empty() && pesticide <= stack[stack.len()-1].pesticide {
				let spd = stack.pop().unwrap().days;
				if spd > days_alive {
					days_alive = spd;
				}
			} else { break; }
		}
		if stack.is_empty() {
			days_alive = 0;
		} else {
			days_alive+=1;
		}
		if days_alive > max_days_alive {
			max_days_alive = days_alive;
		}
		let new1: Plant = Plant { pesticide: pesticide, days: days_alive };
		stack.push(new1);
	}
	max_days_alive
}

struct Plant {
	pesticide: u32,
	days: u32,
}
