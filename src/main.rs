extern crate csv;
extern crate rustc_serialize;

use rustc_serialize::serialize::Decodable;

struct TrainAbon {
	x0: String,
	x1: String,
	x2: String,
	x3: String,
	x4: String,
	x5: String,
	x6: String,
	x7: String,
	x8: String,
	x9: String,
	x10: String,
	x11: String,
	x12: String,
	x13: String,
	x14: String,
	x15: String,
	x16: String,
	x17: String,
	x18: String,
	x19: String,
	x20: String,
	x21: String,
	x22: String,
	x23: String,
	x24: String,
	x25: String,
	x26: String,
	x27: String,
	x28: String,
	x29: String,
	x30: String,
	x31: String,
	x32: String,
	x33: String,
	x34: String,
	x35: String,
	x36: String,
	x37: String,
	x38: String,
	x39: String,
	x40: String,
	x41: String,
	x42: String,
	x43: String,
	x44: String,
	x45: String,
	x46: String,
	x47: String,
	x48: String,
	x49: String,
	x50: String,
	x51: String,
	x52: String,
	x53: String,
	x54: String,
	x55: String,
	x56: String,
	x57: String,
	x58: String,
	x59: String,
	x60: String,
	x61: String,
	y: i32,
}

struct TestAbon {
	ID: i32,
	x0: String,
	x1: String,
	x2: String,
	x3: String,
	x4: String,
	x5: String,
	x6: String,
	x7: String,
	x8: String,
	x9: String,
	x10: String,
	x11: String,
	x12: String,
	x13: String,
	x14: String,
	x15: String,
	x16: String,
	x17: String,
	x18: String,
	x19: String,
	x20: String,
	x21: String,
	x22: String,
	x23: String,
	x24: String,
	x25: String,
	x26: String,
	x27: String,
	x28: String,
	x29: String,
	x30: String,
	x31: String,
	x32: String,
	x33: String,
	x34: String,
	x35: String,
	x36: String,
	x37: String,
	x38: String,
	x39: String,
	x40: String,
	x41: String,
	x42: String,
	x43: String,
	x44: String,
	x45: String,
	x46: String,
	x47: String,
	x48: String,
	x49: String,
	x50: String,
	x51: String,
	x52: String,
	x53: String,
	x54: String,
	x55: String,
	x56: String,
	x57: String,
	x58: String,
	x59: String,
	x60: String,
	x61: String,
}

impl Decodable for TrainAbon {
  fn decode<D: rustc_serialize::Decoder>(decoder: &mut D) -> Result<TrainAbon, D::Error> {
    // stuff
  }
}

impl Decodable for TestAbon {
  fn decode<D: rustc_serialize::Decoder>(decoder: &mut D) -> Result<TestAbon, D::Error> {
    // stuff
  }
}

fn main() {
	let mut train = csv::Reader::from_file("./data/train.csv").unwrap();
	let mut test = csv::Reader::from_file("./data/test.csv").unwrap();
	for record in train.decode() {
		let train_abon: TrainAbon = record.unwrap();
		for srecord in test.decode() {
			let test_abon: TestAbon = srecord.unwrap();
			
				println!("{} = {}", test_abon.ID, train_abon.y);
		}
	}
}
