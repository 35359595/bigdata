extern crate csv;

fn main() {
	let mut rdr = csv::Reader::from_file("./data/train.csv").unwrap();
	for row in rdr.byte_records().map(|r| r.unwrap()) {
    		println!("{:?}", row.y);
	}
}	
