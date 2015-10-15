extern crate csv;

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

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
	
	//All must be mutable to remove matches...

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

	//interesting part
	if all.els.len() != 0 {panic!("Parse went wrong! els is not epmty!")} //if parse of test was not complete as expected
	
	//populating results
	build_rating(&uniq);
	
}

fn find_uniq(v: Vec<Vec<String>>) -> Vec<String> {
	v.into_iter().flat_map(|v| v).collect::<HashSet<String>>().into_iter().collect()
}

fn write_to_file(s: &str) {
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

fn build_rating(uniq: &Uniq) {
	println!("Building ratings and printing results:\n");
	println!("ID, y");
	let mut test = csv::Reader::from_file("./data/test.csv").unwrap();
		for test_rows in test.decode() {
			let mut rate = RCounter {c0: 0,c1: 0,c2: 0,c3: 0,c4: 0,c5: 0,c6: 0};
			let test_row: Vec<String> = test_rows.unwrap();
			for test_string in &test_row {
				for u in &uniq.p0 {
					if test_string == &u.to_string() {
						rate.c0 += 1;
					}
				}
				for u in &uniq.p1 {
					if test_string == &u.to_string() {
						rate.c1 += 1;
					}
				}
				for u in &uniq.p2 {
					if test_string == &u.to_string() {
						rate.c2 += 1;
					}
				}
				for u in &uniq.p3 {
					if test_string == &u.to_string() {
						rate.c3 += 1;
					}
				}
				for u in &uniq.p4 {
					if test_string == &u.to_string() {
						rate.c4 += 1;
					}
				}
				for u in &uniq.p5 {
					if test_string == &u.to_string() {
						rate.c5 += 1;
					}
				}
				for u in &uniq.p6 {
					if test_string == &u.to_string() {
						rate.c6 += 1;
					}
				}
			}
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
		out.push_str(&sting.to_string());
		out.push_str(",");
		out.push_str(&decision.to_string());		
		println!("{}", &out);
		write_to_file(out);
		}
}
