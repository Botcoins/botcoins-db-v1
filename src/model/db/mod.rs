extern crate lmdb_rs as lmdb;

use botcoins_objects::*;
pub use self::category::*;
use self::lmdb::{Database, DbFlags, EnvBuilder, Environment};
pub use self::value::*;
use serde_json;
use std::fs;

mod value;
mod category;

pub struct DB {
	index: Vec<PartialCoin>,
	env: Environment,
}

impl DB {
	pub fn load() -> DB {
		DB {
			index: load_index(),
			env: EnvBuilder::new()
				.map_size(1024 * 1024 * 1024 * 45) // 45GiB
				.open("botcoins-db.lmdb", 0o777)
				.unwrap(),
		}
	}

	pub fn coin_id(&mut self, coin: &PartialCoin) -> u16 {
		for i in 0..self.index.len() {
			if *coin == self.index[i] {
				return i as u16;
			}
		}

		self.index.push(coin.clone());
		write_index(&self.index);

		return (self.index.len() - 1) as u16;
	}

	pub fn id(&self, category: Category, source: Provider, coin_id: u16, time: u32) -> u64 {
		let mut buf = 0u64;
		buf += category.ident() as u64;
		buf <<= 8;
		buf += source.ident() as u64;
		buf <<= 16;
		buf += coin_id as u64;
		buf <<= 32;
		buf + time as u64
	}

	pub fn find_coin(&self, coin_id: u16, source: Provider) -> Option<Coin> {
		self.reader(move |db| {
			let id = self.id(Category::LiveData, source, coin_id, 0);
			if let Ok(res) = db.get(&id) {
				if let DBValue::LiveData(coin) = DBValue::deserialize(res) {
					return Some(coin);
				}
			}

			return None;
		})
	}

	pub fn coins<'a>(&'a self) -> &'a Vec<PartialCoin> {
		&self.index
	}

	pub fn writer<F: FnOnce(&Database) + Sized>(&self, func: F) -> bool {
		let db_handle = self.env.get_default_db(DbFlags::empty()).unwrap();
		let txn = self.env.new_transaction().unwrap();
		func(&txn.bind(&db_handle));
		if let Err(err) = txn.commit() {
			error!("LMDB Error while writing: {:?}", err);
			false
		} else {
			true
		}
	}

	pub fn reader<T, F: FnOnce(&Database) -> T + Sized>(&self, func: F) -> T {
		let db_handle = self.env.get_default_db(DbFlags::empty()).unwrap();
		let reader = self.env.get_reader().unwrap();
		func(&reader.bind(&db_handle))
	}
}

fn load_index() -> Vec<PartialCoin> {
	let open = fs::OpenOptions::new().read(true).create(false).open("index.json");
	if let Ok(open) = open {
		if let Ok(tree) = serde_json::from_reader(open) {
			return tree;
		}
	}

	vec![]
}

fn write_index(map: &Vec<PartialCoin>) {
	let open = fs::OpenOptions::new().write(true).create(true).open("index.json");
	if let Ok(open) = open {
		let _ = serde_json::to_writer(open, map);
	}
}
