use bincode;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use std::io::Cursor;

pub struct ResponseList<'r, T: Responder<'r> + Serialize> {
	buffer: Vec<T>,
	_lifetime_r: &'r (),
}

impl<'r, T: Responder<'r> + Serialize> ResponseList<'r, T> {
	pub fn serialize(&self) -> Vec<u8> {
		bincode::serialize(&self.buffer).unwrap()
	}
}

impl<'r, T: Responder<'r> + Serialize> From<Vec<T>> for ResponseList<'r, T> {
	fn from(buffer: Vec<T>) -> ResponseList<'r, T> {
		ResponseList { buffer, _lifetime_r: &() }
	}
}

impl<'r, T: Responder<'r> + Serialize> Responder<'r> for ResponseList<'r, T> {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		Response::build()
			.sized_body(Cursor::new(self.serialize()))
			.header(ContentType::new("application", "x-bincode-vec"))
			.ok()
	}
}