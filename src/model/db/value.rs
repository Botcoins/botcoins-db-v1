use bincode;
use botcoins_objects::*;
use model::db::Category;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DBValue {
	LiveData(Coin),
	History(PriceHistory),
}

impl DBValue {
	pub fn serialize(&self) -> Vec<u8> {
		bincode::serialize(self).unwrap()
	}

	pub fn deserialize(bytes: Vec<u8>) -> Self {
		bincode::deserialize(&bytes[..]).unwrap()
	}
}

impl Into<Category> for DBValue {
	fn into(self) -> Category {
		match self {
			DBValue::LiveData(_) => Category::LiveData,
			DBValue::History(_) => Category::History
		}
	}
}