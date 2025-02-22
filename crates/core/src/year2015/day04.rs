use crate::Input;
use md5::digest::generic_array::arr;
use md5::digest::FixedOutput;
use md5::{Digest, Md5};

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const MAX_INDEX: i32 = 100_000_000;

    let secret_key = input.text.as_bytes();

    let mut hasher = Md5::new();
    let mut output = arr![u8; 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 , 0, 0];

    for index in 0..MAX_INDEX {
        hasher.update(secret_key);
        hasher.update(index.to_string().as_bytes());

        hasher.finalize_into_reset(&mut output);

        // Check if hash starts with five/six zeros without converting it to a string:
        if output[..2] == [0, 0] && output[2] <= input.part_values(0x0F, 0) {
            return Ok(index as u32);
        }
    }

    return Err(format!("Aborting after {} iterations", MAX_INDEX));
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day04_input.txt");
    test_part_one!(real_input => 117_946);
    test_part_two!(real_input => 3_938_038);
}
