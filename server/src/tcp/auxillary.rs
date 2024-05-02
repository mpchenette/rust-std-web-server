// checks if a given 'untyped' vector is valid ASCII
pub fn is_vec_u8_ascii(bytes: Vec<u8>) -> bool {
	bytes.iter().all(|&byte| byte.is_ascii())
}