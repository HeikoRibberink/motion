use chrono::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use linux_embedded_hal::Delay;
use mpu6050::Mpu6050;
use rppal::i2c::I2c;
use terminal;
use tracker::Tracker;
use connection::{ChannelMessage, ChannelData};

mod connection;
mod tracker;

type MpuErr = mpu6050::Mpu6050Error<rppal::i2c::Error>;

fn main() -> Result<(), MpuErr> {
	//Setup i2c
	let i2c = I2c::new().unwrap();

	let mut mpu = Mpu6050::new(i2c);
	let mut delay = Delay;

	println!(
		"Please hold still for {} seconds.",
		(ITERS * DELAY) as f32 / 1000.0
	);
	offset(&mut mpu, &mut delay).unwrap();
	println!("Done!");

	// //Accelerometer offsets of Mpu6050
	// mpu.acc_offsets.x = -0.022777887;
	// mpu.acc_offsets.y = -0.01322483;
	// mpu.acc_offsets.z = 0.04565797;
	// //Gyroscope offsets of Mpu6050
	// mpu.gyro_offsets.x = 0.046678342;
	// mpu.gyro_offsets.y = -0.0048800064;
	// mpu.gyro_offsets.z = 0.037488528;

	let (handle, tx) = connection::spawn_new();

	let mut tracker = Tracker::new();

	let mut last = Utc::now();

	// let t = terminal::stdout();
	// let mut counter = 0;
	// const PC: u32 = 40;

	loop {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;

		let now = Utc::now();
		let delta = now.signed_duration_since(last);
		let delta = delta.num_microseconds().unwrap() as f32 / 1_000_000.0;
		tracker.update(acc, gyro, delta);
		last = now;

		{
			// if counter > PC {
			// 	t.act(terminal::Action::ClearTerminal(terminal::Clear::All))
			// 		.unwrap();
			// 	println!("{:?}\n{:?}", acc, gyro);
			// 	println!("{:#?}", tracker);
			// 	println!("delta: {}", delta);
			// 	counter = 0;
			// }
			// counter += 1;
		}

		tx.send(ChannelMessage::Data(ChannelData {pos: tracker.pos(), rot: tracker.rot()})).unwrap();

		delay.delay_ms(5u16);
	}

	// Ok(())
}

const ITERS: u32 = 100;
const SPEED: f32 = 0.2;
const DELAY: u32 = 10;
const ACC_ZERO: [f32; 3] = [0.0, 0.0, 1.0];
const GYRO_ZERO: [f32; 3] = [0.0, 0.0, 0.0];

fn offset(mpu: &mut Mpu6050<I2c>, delay: &mut Delay) -> Result<(), MpuErr> {
	for _ in 0..ITERS {
		let acc = mpu.get_acc()?;
		let gyro = mpu.get_gyro()?;
		let delta = ACC_ZERO[0] - acc.x;
		mpu.acc_offsets.x += delta * SPEED;
		let delta = ACC_ZERO[1] - acc.y;
		mpu.acc_offsets.y += delta * SPEED;
		let delta = ACC_ZERO[2] - acc.z;
		mpu.acc_offsets.z += delta * SPEED;

		let delta = GYRO_ZERO[0] - gyro.x;
		mpu.gyro_offsets.x += delta * SPEED;
		let delta = GYRO_ZERO[1] - gyro.y;
		mpu.gyro_offsets.y += delta * SPEED;
		let delta = GYRO_ZERO[2] - gyro.z;
		mpu.gyro_offsets.z += delta * SPEED;
		delay.delay_ms(DELAY);
	}
	Ok(())
}
