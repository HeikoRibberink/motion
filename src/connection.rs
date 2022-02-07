use bincode::{deserialize, serialize};
use chrono::prelude::*;
use nalgebra::{
	geometry::{Quaternion, UnitQuaternion},
	Vector3,
};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub enum ChannelMessage {
	Data(ChannelData),
	End,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelData {
	pub pos: Vector3<f32>,
	pub rot: UnitQuaternion<f32>,
}

// #[derive(Serialize, Deserialize)]
// struct TcpData {
// 	pos: [f32; 3],
// 	rot: [f32; 4],
// }

pub fn spawn_new() -> (thread::JoinHandle<()>, Sender<ChannelMessage>) {
	let (tx, rx) = mpsc::channel();
	let handle = thread::spawn(|| receiver(rx));
	(handle, tx)
}

const DELAY_MS: i64 = 100000;
const ADDRESS: &str = "192.168.2.211:8080";

fn receiver(rx: Receiver<ChannelMessage>) {
	use std::io::{Read, Write};
	use std::net::TcpStream;
	let mut stream = TcpStream::connect(ADDRESS).unwrap();

	let mut last = Utc::now();

	loop {
		let msg = rx.recv().unwrap();
		match msg {
			ChannelMessage::End => break,
			ChannelMessage::Data(data) => {
				let now = Utc::now();
				if now.signed_duration_since(last).num_microseconds().unwrap() > DELAY_MS {
					let serialized = serialize(&data).unwrap();
					stream.write(&serialized).unwrap();
					last = now;
				}
			}
		}
	}
}
