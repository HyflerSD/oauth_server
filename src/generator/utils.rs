static ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static NUMERIC: &str = "1234567890";
static ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
static ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const PREFIX: &str = "1000";

pub fn generate_client_id() -> String {
    //just temp code bro
    generate(12, ALPHANUMERIC)
} 


pub fn generate<S: AsRef<str>>(length: usize, charset: S) -> String {
    let charset_str = charset.as_ref();
    assert_eq!(charset_str.is_empty(), false);
    
    let chars: Vec<char> = charset_str.chars().collect();
    let mut res = String::with_capacity(length);

    unsafe {
        for _ in 0..length {
            res.push(
                *chars.get_unchecked(fastrand::usize(0..chars.len()))
            );
        }
    }
    res
}
