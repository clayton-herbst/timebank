use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn generate_hash(id: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&id);
    hasher.result_str()
}
