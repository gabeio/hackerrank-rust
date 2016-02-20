use std::io;
use std::io::Write; // for stderr

fn main() {
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap(); // read line into s
	let n:Vec<i32> = s.split_whitespace().
		map(|x| x.trim().parse().unwrap()).
		collect();
	writeln!(&mut std::io::stderr(), "n|{}", n[0]).unwrap();
	let mut stk:Vec<i32> = vec!();
	let mut maxele:i32 = -1;
	for _ in 0..n[0] {
		s.clear();
		io::stdin().read_line(&mut s).unwrap(); // read line into s
		let call:Vec<i32> = s.split_whitespace(). // split on space
			map(|x| x.trim().parse().unwrap()). // each do (trim.parse.unwrap)
			collect(); // collection
		// writeln!(&mut std::io::stderr(), "call[0]|{}", call[0]).unwrap();
		match call[0] {
			1 => {
				stk.push(call[1]);
				if call[1] > maxele {
					maxele = call[1];
				}
			},
			2 => {
				// assure max is updated
				if stk.len() > 0 {
					stk.pop();
					maxele = -1;
					for x in stk.iter() {
						// writeln!(&mut std::io::stderr(), "x|{}", x).unwrap();
						if *x > maxele {
							maxele = *x;
						}
					}
				} else {
					maxele = -1;
				}
			},
			3 => {
				if stk.len() > 0 {
					println!("{}", maxele)
				}
			},
			_ => panic!("idk what this call is")
		}
	}
}
