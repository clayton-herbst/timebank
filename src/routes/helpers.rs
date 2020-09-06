use crypto::sha1::Sha1;
use crypto::digest::Digest;

pub fn generate_hash(id: &str) -> String {
	let mut hasher = Sha1::new();
	hasher.input_str(&id);
	hasher.result_str()
}