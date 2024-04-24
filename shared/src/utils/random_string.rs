use rand::{distributions::Alphanumeric, Rng};

// pub fn generate(len: usize) -> String {
//     rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(len)
//         .map(char::from)
//         .collect()
// }

pub fn generate(size: usize) -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
    bytes.iter().map(|&byte| format!("{:02x}", byte)).collect()
}
