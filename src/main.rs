#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate bincode;
extern crate botcoins_objects;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate rocket;
extern crate scheduled_thread_pool;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use botcoins_objects::PartialCoin;
use model::db::DB;
use model::export::list::ResponseList;
use scheduled_thread_pool::*;
use std::sync::Mutex;
use std::time::Duration;

mod model;
mod updater;

lazy_static! {
	static ref DATABASE: Mutex<DB> = Mutex::new(DB::load());
}

#[get("/list")]
fn list<'a>() -> ResponseList<'a, PartialCoin> {
	DATABASE.lock().unwrap().coins().clone().into()
}

fn main() {
	let tp = ScheduledThreadPool::new(2);
	tp.execute_with_fixed_delay(Duration::from_secs(0), Duration::from_secs(120), updater::coinmarketcap);

	rocket::ignite().mount("/api/", routes![list]).launch();
}
