use ::chrono::Utc;
use ::getrandom;

pub fn uu64() -> u64 {
    let millis = Utc::now().timestamp_millis() as u64 & 0x0000_FFFF_FFFF_FFFF;

    let mut rnd = [0u8; 2];
    getrandom::fill(&mut rnd).expect("randomizer failed");
    let rand16 = u16::from_be_bytes(rnd) as u64;

    (millis << 16) | rand16
}

#[macro_export]
macro_rules! safe_nanoid {
    () => {
        ::shared::nanoid::nanoid!(
            20,
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            ]
        )
    };
    ($size:expr) => {
        ::shared::nanoid::nanoid!(
            $size,
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            ]
        )
    };
}
