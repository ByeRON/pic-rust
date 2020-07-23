pub fn hello_world<'a>() -> &'a str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hello_world() {
		assert_eq!("Hello, world!", hello_world());
	}
}
