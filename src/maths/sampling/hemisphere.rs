use std::num::{ Float, sqrt };
use rand::random;

use maths::vec::Vec3;
use maths::angle::{ Angle, Rad };
use maths::sampling::disk;

// sample the unit hemisphere uniformly
pub fn sample() -> Vec3 {
	let cos_theta = random::< f64 >();
	let phi = Rad { r : 2.0 * Float::pi() * random::< f64 >() };

	let sin_theta = sqrt( 1.0 - cos_theta * cos_theta );
	let ( sin_phi, cos_phi ) = phi.sin_cos();

	return Vec3 {
		x : sin_theta * cos_phi,
		y : sin_theta * sin_phi,
		z : cos_theta,
	};
}

// sample the unit hemisphere weighted by cos( theta )
pub fn sample_cos() -> Vec3 {
	let p = disk::sample();

	return Vec3 {
		x : p.x,
		y : p.y,
		z : sqrt( 1.0 - ( p.x * p.x + p.y * p.y ) ),
	};
}
