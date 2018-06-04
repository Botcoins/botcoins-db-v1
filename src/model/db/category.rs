#[derive(Clone, Copy)]
pub enum Category {
	Reference,
	LiveData,
	History,
}

impl Category {
	pub fn ident(&self) -> u8 {
		match *self {
			Category::Reference => 0,
			Category::LiveData => 1,
			Category::History => 2
		}
	}
}