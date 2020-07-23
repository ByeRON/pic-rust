struct Vector3d<T> 
{
	x: T,
	y: T,
	z: T,
}

impl<T> Vector3d<T> {
	fn new(x: T, y: T, z: T) -> Self {
		Vector3d {x: x, y: y, z: z}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let actual = Vector3d::<i32>::new(0, 1, 2);
		assert_eq!(0, actual.x);
		assert_eq!(1, actual.y);
		assert_eq!(2, actual.z);

		let actual = Vector3d::<f64>::new(0.0, 1.0, 2.0);
		assert_eq!(0.0, actual.x);
		assert_eq!(1.0, actual.y);
		assert_eq!(2.0, actual.z);
	}
}
