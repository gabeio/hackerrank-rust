use std::io;
use std::io::Write;
use std::collections::HashMap;

fn fib(mut v:&mut HashMap<i64, i64>, i:i64) ->i64 {//(&mut HashMap<i64, i64>,)
	// let (&mut v, i) = input;
	if i < 2 {
		return i;//(&mut v,i);
	}
	if v.contains_key(&i) {
		return v[&i];//(&mut v,v[&i]);
	}
	let i1 = fib(&mut v,i-1);
	(*v).insert(i-1, i1);//(i1));
	let i2 = fib(&mut v,i-2);
	(*v).insert(i-2, i2);//i1+i2);
	// let i3 = i1+i2;
	(*v).insert(i, i1+i2);
	return v[&i];//(v,v[&i]);
}

fn main(){
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap(); // read line into s
	let n:Vec<i64> = s.
		// divide on the whitespace
		split_whitespace().
		// for each:
		// - remove whitespace
		// - parse int
		// - assume ok
		map(|x| x.trim().parse().unwrap()).
		// convert to vec
		collect();
	let mut hm:HashMap<i64,i64> = HashMap::new();
	// let (a,b) = fib(&mut hm,n[0]));
	println!("{}", fib(&mut hm,n[0]));
	for (key, val) in hm.iter() {
		writeln!(&mut std::io::stderr(), "{:3}| {}", key, val).unwrap();
	}
}
