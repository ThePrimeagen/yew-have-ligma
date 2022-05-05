use rand::Rng;

pub fn random_buffer(size: usize) -> Vec<u8> {
    let mut out = vec![0u8; size];
    rand::thread_rng().fill(&mut out[..]); // <---

    return out;
}
