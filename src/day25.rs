pub fn solution() -> [u64; 2] {
    let card_public_key = 2_959_251;
    let door_public_key = 4_542_595;

    let card_loop_size = find_loop_size(card_public_key, 7);
    let encryption_key = generate_encryption_key(door_public_key, card_loop_size, 0, 1);

    [encryption_key, 0]
}

fn generate_encryption_key(
    subject_number: u64,
    loop_size: u64,
    last_loop_size: u64,
    encryption_key: u64,
) -> u64 {
    let mut encryption_key = encryption_key;
    for _ in last_loop_size..loop_size {
        encryption_key = (encryption_key * subject_number) % 20_201_227
    }
    encryption_key
}

fn find_loop_size(public_key: u64, subject_number: u64) -> u64 {
    let mut loop_size = 0;
    let mut last_encryption_key = 1;

    while last_encryption_key != public_key {
        loop_size += 1;
        last_encryption_key = generate_encryption_key(
            subject_number,
            loop_size,
            loop_size - 1,
            last_encryption_key,
        );
    }

    loop_size
}
