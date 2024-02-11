use crypto::{digest::Digest, sha1::Sha1};

pub fn generate_hash(s: &str) -> String {
  let mut hasher = Sha1::new();
  hasher.input_str(s);
  hasher.result_str()
}