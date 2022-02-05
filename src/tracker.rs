use nalgebra::{base::Vector3, Quaternion, Unit, UnitQuaternion};

#[derive(Debug)]
pub struct Tracker {
	pos: Vector3<f32>,
	vel: Vector3<f32>,
	rot: UnitQuaternion<f32>,
}

impl Tracker {
	pub fn new() -> Self {
		Tracker {
			pos: Default::default(),
			vel: Default::default(),
			rot: Unit::from_quaternion(Quaternion::from([1.0, 0.0, 0.0, 0.0])),
		}
	}

	pub fn update(&mut self, mut pos_acc: Vector3<f32>, mut rot_vel: Vector3<f32>, delta_time: f32) {
		//Rotate self.rot by rot_vel
		rot_vel *= delta_time;
		let q = UnitQuaternion::from_euler_angles(rot_vel.x, rot_vel.y, rot_vel.z);
		self.rot *= q;

		//Add the acceleration to the velocity, after rotating it by the local orientation.
		pos_acc = self.rot.transform_vector(&pos_acc);
		pos_acc.z += 1.0;
		pos_acc *= delta_time;
		self.vel += pos_acc;
		//The position is automatically corrected for local orientation of the mpu, because the velocity already is.
		self.pos += self.vel * delta_time;
	}

	pub fn pos(&self) -> Vector3<f32> {
		self.pos
	}

	pub fn vel(&self) -> Vector3<f32> {
		self.vel
	}

	pub fn rot(&self) -> UnitQuaternion<f32> {
		self.rot
	}

	pub fn pos_mut(&mut self) -> &mut Vector3<f32> {
		&mut self.pos
	}

	pub fn vel_mut(&mut self) -> &mut Vector3<f32> {
		&mut self.vel
	}

	pub fn rot_mut(&mut self) -> &mut UnitQuaternion<f32> {
		&mut self.rot
	}
}
