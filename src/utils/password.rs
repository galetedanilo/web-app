use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn helper_hash_password(password: &str) -> String {

    let mut hasher = Sha256::new();

    hasher.input_str(password);

    hasher.result_str()
}