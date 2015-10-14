extern crate csv;

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
	uniq.p1 = find_uniq(all.y1);
	uniq.p2 = find_uniq(all.y2);
	uniq.p3 = find_uniq(all.y3);
	uniq.p4 = find_uniq(all.y4);
	uniq.p5 = find_uniq(all.y5);
	uniq.p6 = find_uniq(all.y6);
//	for p in uniq.p0 { println!("Uniq 0: {}", p); }

	//interesting part
	if all.els.len() != 0 {panic!("Parse went wrong! els is not epmty!")} //if parse of test was not complete as expected
	
	//writing results
//	let mut writer = csv::Writer::from_file("./data/y0.csv").unwrap();
//		for y00 in y0 {
//			writer.encode(y00).ok().expect("CSV Writer error.");
//		}

//	let mut test = csv::Reader::from_file("./data/test.csv").unwrap(); //loadint work data
	
}

fn find_uniq(v: Vec<Vec<String>>) -> Vec<String> {
	let mut ret: Vec<String> = vec![];
	for i in 0..62 {
	for var in &v {
		let mut check = false;
		let st1: &String = &var[i];
		for r in &ret {
			if r == st1 { check = true; }
		}
		if ! check { &ret.push(st1.to_string()); }
	}
	}
	return ret
}

fn find_same(v1: &Vec<String>, v2: &Vec<String>) -> Vec<String> {
	let mut ret: Vec<String> = vec![];
	for var1 in v1 {
		for var2 in v2 {
			if var1 == var2 { &ret.push(var1.to_string()); }
		}
	}
	return ret
}	
