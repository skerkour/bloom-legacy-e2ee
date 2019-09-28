use crypto42::rand as crypto42rand;
use rand::Rng;

pub fn random_digit_string(n: usize) -> String {
    let mut rng = rand::thread_rng();

    return (0..n)
        .map(|_| rng.gen_range(0, 10))
        .map(|x| x.to_string())
        .collect();
}

pub fn random_hex_string(n: usize) -> String {
    let bytes = crypto42rand::bytes(n);
    return hex::encode(bytes);
}

pub fn random_base64_string(n: usize) -> String {
    let bytes = crypto42rand::bytes(n);
    return base64::encode(&bytes);
}

pub fn encode_session(session_id: &str, token: &str) -> String {
    return base64::encode(format!("{}:{}", session_id, token).as_str());
}
