use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn generate_string<T>(len: usize, chars: &[T]) -> String
where
    T: Into<char> + Copy,
    char: From<T>,
{
    let mut result = String::with_capacity(len);
    append_generate_string(&mut result, len, chars);

    result
}

pub fn append_generate_string<T>(str: &mut String, len: usize, chars: &[T])
where
    T: Into<char> + Copy,
    char: From<T>,
{
    let mut rng = ChaCha20Rng::from_entropy();

    for _ in 0..len {
        str.push(char::from(*chars.choose(&mut rng).unwrap()));
    }
}
