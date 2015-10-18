//parser created by Ivan Temchenko <ivan.temchenko@yandex.ua>, 2015

extern crate csv;

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};
//use std::sync::mpsc::channel;

struct All {
	y0: Vec<Vec<String>>,
	y1: Vec<Vec<String>>,
	y2: Vec<Vec<String>>,
	y3: Vec<Vec<String>>,
	y4: Vec<Vec<String>>,
	y5: Vec<Vec<String>>,
	y6: Vec<Vec<String>>,
	els: Vec<Vec<String>>,
}
#[derive(Clone)]
struct Uniq {
	p0: Vec<String>,
	p1: Vec<String>,
	p2: Vec<String>,
	p3: Vec<String>,
	p4: Vec<String>,
	p5: Vec<String>,
	p6: Vec<String>,
}

struct RCounter {
	c0: i32,
	c1: i32,
	c2: i32,
	c3: i32,
	c4: i32,
	c5: i32,
	c6: i32,
}

impl Uniq {
	fn seek0(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p0.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}
	fn seek1(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p1.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}
	fn seek2(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p2.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}
	fn seek3(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p3.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}
	fn seek4(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p4.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}
	fn seek5(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p5.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}
	fn seek6(self, input: &String) -> i32 {
		let mut count: i32 = 0;
		let thread: Vec<_> = self.p6.into_iter().map(|p| {
			let input = input.clone();
			let mut count = count.clone();
			thread::spawn(move || {
				if p == input {
					count += 1;
				}
				return count
			})
		}).collect();

		for t in thread { count += t.join().unwrap() }

		return count
	}

}

fn main() {
	let mut all = All {y0: vec![], y1: vec![], y2: vec![], y3: vec![], y4: vec![], y5: vec![], y6: vec![], els: vec![]};
	let mut uniq = Uniq {p0: vec![], p1: vec![], p2: vec![], p3: vec![], p4: vec![], p5: vec![], p6: vec![]};

	let mut train = csv::Reader::from_file("./data/train.csv").unwrap();
	for train_record in train.decode() {
		let s1: Vec<String> = train_record.unwrap();
			if s1[62] == "0" { &all.y0.push(s1); }
			else if s1[62] == "1" { &all.y1.push(s1); }
			else if s1[62] == "2" { &all.y2.push(s1); }
			else if s1[62] == "3" { &all.y3.push(s1); }
			else if s1[62] == "4" { &all.y4.push(s1); }
			else if s1[62] == "5" { &all.y5.push(s1); }
			else if s1[62] == "6" { &all.y6.push(s1); }
			else { &all.els.push(s1); }

	}

	println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, else: {}", all.y0.len(), all.y1.len(), all.y2.len(), all.y3.len(), all.y4.len(), all.y5.len(), all.y6.len(), all.els.len());

	uniq.p0 = find_uniq(all.y0);
	println!("15% done..");
	uniq.p1 = find_uniq(all.y1);
	println!("30% done..");
	uniq.p2 = find_uniq(all.y2);
	println!("45% done..");
	uniq.p3 = find_uniq(all.y3);
	println!("60% done..");
	uniq.p4 = find_uniq(all.y4);
	println!("75% done..");
	uniq.p5 = find_uniq(all.y5);
	println!("90% done..");
	uniq.p6 = find_uniq(all.y6);
	println!("100% uniq search done!\n");

	if all.els.len() != 0 {panic!("Parse went wrong! els is not epmty!")} //if parse of test was not complete as expected

	//populating results
	build_rating(&uniq);

}

fn find_uniq(v: Vec<Vec<String>>) -> Vec<String> {
	v.into_iter().flat_map(|v| v).collect::<HashSet<String>>().into_iter().collect()
}

fn write_to_file(s: String) {
	let path = Path::new("./data/sol.csv");
	let mut options = OpenOptions::new();
		options.write(true).append(true);
	let file = match options.open(&path) {
  		Ok(file) => file,
    		Err(e) => panic!("Error: {}", e),
	};
	let mut writer = BufWriter::new(&file);
		writer.write(s.as_bytes());



}

fn build_rating(uniq_in: &Uniq) {
	println!("Building ratings and printing results:\n");
	println!("ID,y");
	let mut test = csv::Reader::from_file("./data/test.csv").unwrap();
		for test_rows in test.decode() {
			let mut rate = RCounter {c0: 0,c1: 0,c2: 0,c3: 0,c4: 0,c5: 0,c6: 0};
			let test_row: Vec<String> = test_rows.unwrap();
				for test_string0 in &test_row {
					let test_string = &test_string0;
					let uniq0 = uniq_in.clone();
					let uniq1 = uniq_in.clone();
					let uniq2 = uniq_in.clone();
					let uniq3 = uniq_in.clone();
					let uniq4 = uniq_in.clone();
					let uniq5 = uniq_in.clone();
					let uniq6 = uniq_in.clone();

					rate.c0 += uniq0.seek0(&test_string);
					rate.c1 += uniq1.seek1(&test_string);
					rate.c2 += uniq2.seek2(&test_string);
					rate.c3 += uniq3.seek3(&test_string);
					rate.c4 += uniq4.seek4(&test_string);
					rate.c5 += uniq5.seek5(&test_string);
					rate.c6 += uniq6.seek6(&test_string);

					//println!("{}", &rate.c0);
				}
				println!("rate is: 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}", &rate.c0, &rate.c1, &rate.c2, &rate.c3, &rate.c4, &rate.c5, &rate.c6);
			let mut decision = 0;
			if rate.c0 > rate.c1 { decision = 0 }
			else if rate.c1 > rate.c2 { decision = 1 }
			else if rate.c2 > rate.c3 { decision = 2 }
			else if rate.c3 > rate.c4 { decision = 3 }
			else if rate.c4 > rate.c5 { decision = 4 }
			else if rate.c5 > rate.c6 { decision = 5 }
			else { decision = 6 }
			let sting: &String = &test_row[0].to_string();
			println!("{}, {}", &sting, &decision);
		let mut out = String::new();
		out.push_str("\n");
		out.push_str(&sting.to_string());
		out.push_str(",");
		out.push_str(&decision.to_string());
		write_to_file(out);
		}
}
