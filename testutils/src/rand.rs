use rand::Rng;

static CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn i32(lower: i32, upper: i32) -> i32 {
    rand::thread_rng().gen_range(lower..upper)
}

pub fn bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..CHARSET.len()) % 2 == 0
}

pub fn string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let ret: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    ret
}

pub fn port() -> i32 {
    rand::thread_rng().gen_range(0..65536)
}

pub fn ip() -> String {
    format!(
        "{}.{}.{}.{}:{}",
        i32(0, 255),
        i32(0, 255),
        i32(0, 255),
        i32(0, 255),
        port()
    )
}

pub fn url() -> String {
    format!("{}://{}.org:{}", string(5), string(10), port())
}
