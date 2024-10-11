mod log;
mod statics;

const INVITE_CODE_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const INVITE_CODE_LENGTH: usize = 6;
pub fn random_invite_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let invite_code: String = (0..INVITE_CODE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..INVITE_CODE_ALPHABET.len());
            INVITE_CODE_ALPHABET.chars().nth(idx).unwrap()
        })
        .collect();
    invite_code
}
