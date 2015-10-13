//9, 14, 24-61

extern crate csv;

fn main() {
	let mut y0: Vec<Vec<String>> = vec![];
	let mut y1: Vec<Vec<String>> = vec![];
	let mut y2: Vec<Vec<String>> = vec![];
	let mut y3: Vec<Vec<String>> = vec![];
	let mut y4: Vec<Vec<String>> = vec![];
	let mut y5: Vec<Vec<String>> = vec![];
	let mut y6: Vec<Vec<String>> = vec![];
	let mut els: Vec<Vec<String>> = vec![];
	
	let mut p1: Vec<String> = vec![];

	let mut train = csv::Reader::from_file("./data/train.csv").unwrap();
	for train_record in train.decode() {
		let s1: Vec<String> = train_record.unwrap();

		if s1[62] == "0" { &y0.push(s1); }
		else if s1[62] == "1" { &y1.push(s1); }
		else if s1[62] == "2" { &y2.push(s1); }
		else if s1[62] == "3" { &y3.push(s1); }
		else if s1[62] == "4" { &y4.push(s1); }
		else if s1[62] == "5" { &y5.push(s1); }
		else if s1[62] == "6" { &y6.push(s1); }
		else { &els.push(s1); }
	}

	println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, else: {}", y0.len(), y1.len(), y2.len(), y3.len(), y4.len(), y5.len(), y6.len(), els.len());
	
	p1 = find_uniq(y0, 2);
	for p in p1 { println!("Uniq in a row {}", p); }
	
	//interesting part
	if els.len() != 0 {panic!("Parse went wrong! els is not epmty!")} //if parse of test was not complete as expected
	
	//writing results
//	let mut writer = csv::Writer::from_file("./data/y0.csv").unwrap();
//		for y00 in y0 {
//			writer.encode(y00).ok().expect("CSV Writer error.");
//		}

//	let mut test = csv::Reader::from_file("./data/test.csv").unwrap(); //loadint work data
	
}

fn find_uniq(v: Vec<Vec<String>>, i: usize) -> Vec<String> {
	let mut ret: Vec<String> = vec![];
	for var in &v {
		let mut check = false;
		let st1: &String = &var[i];
		for r in &ret {
			if r == st1 { check = true; }
		}
		if ! check { &ret.push(st1.to_string()); }
	}
	return ret
}
