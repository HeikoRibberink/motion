use chrono::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use linux_embedded_hal::Delay;
use mpu6050::Mpu6050;
use rppal::i2c::I2c;
use terminal;
use tracker::Tracker;

mod tracker;

type MpuErr = mpu6050::Mpu6050Error<rppal::i2c::Error>;

fn main() -> Result<(), MpuErr> {
    //Setup i2c
    let i2c = I2c::new().unwrap();

    let mut mpu = Mpu6050::new(i2c);

    //Accelerometer offsets of Mpu6050
    mpu.acc_offsets.x = -0.022777887;
    mpu.acc_offsets.y = -0.01322483;
    mpu.acc_offsets.z = 0.04565797 - 0.0;
    //Gyroscope offsets of Mpu6050
    mpu.gyro_offsets.x = 0.046678342;
    mpu.gyro_offsets.y = -0.0048800064;
    mpu.gyro_offsets.z = 0.037488528;

    let mut tracker = Tracker::new();
    let mut delay = Delay;

    let t = terminal::stdout();

    let mut last = Utc::now();
    let mut counter = 0;
    const PC: u32 = 40;

    loop {
        let acc = mpu.get_acc()?;
        let gyro = mpu.get_gyro()?;

        let now = Utc::now();
        let delta = now.signed_duration_since(last);
        let delta = delta.num_microseconds().unwrap() as f32 / 1_000_000.0;
        tracker.update(acc, gyro, delta);
        last = now;

        if counter > PC {
            t.act(terminal::Action::ClearTerminal(terminal::Clear::All))
                .unwrap();
            println!("{:?}\n{:?}", acc, gyro);
            println!("{:#?}", tracker);
            println!("delta: {}", delta);
            counter = 0;
        }
        counter += 1;
        delay.delay_ms(5u16);
    }

    // Ok(())
}