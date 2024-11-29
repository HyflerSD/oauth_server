static ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static NUMERIC: &str = "1234567890";
static ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
static ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static PREFIX: &str = "1000.";
static SEGMENT_LEN: usize = 24;
static SEPERATORS: usize = 2;

pub fn generate_client_id() -> String {
    //just temp code bro
    generate(ALPHA_LOWER)
} 


pub fn generate<S: AsRef<str>>(charset: S) -> String {
    let charset_str = charset.as_ref();
    assert_eq!(charset_str.is_empty(), false);

    let chars: Vec<char> = charset_str.chars().collect();
    let mut res = String::with_capacity((SEGMENT_LEN * SEPERATORS) +  SEPERATORS - 1);
    unsafe {
        for c in 1..=SEPERATORS {
            for i in 0..SEGMENT_LEN {
                res.push(
                    *chars.get_unchecked(fastrand::usize(0..chars.len()))
                );
            }

            if c < SEPERATORS {
                res.push('.');
            }
        }
    }
    PREFIX.to_owned() + &res
}
