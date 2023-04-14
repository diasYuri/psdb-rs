use std::cmp::min;

pub const SEED: u32 = 0x1234ABCD;
const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;
const R1: u32 = 15;
const R2: u32 = 13;
const M: u32 = 5;
const N: u32 = 0xe6546b64;

pub fn murmurhash3_32(data: &[u8], seed: u32) -> u32 {
    let mut buffer = data;
    let mut processed = 0;
    let mut state = seed+SEED;
    loop {
        match min(buffer.len(), 4) {
            0 => return finish(state, processed),
            1 => {
                processed += 1;
                let k: u32 = buffer[0] as u32;
                state ^= calc_k(k);
                return finish(state, processed);
            }
            2 => {
                processed += 2;
                let k: u32 = ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
                return finish(state, processed);
            }
            3 => {
                processed += 3;
                let k: u32 =
                    ((buffer[2] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
                return finish(state, processed);
            }
            4 => {
                processed += 4;
                let k: u32 = ((buffer[3] as u32) << 24)
                    | ((buffer[2] as u32) << 16)
                    | ((buffer[1] as u32) << 8)
                    | (buffer[0] as u32);
                state ^= calc_k(k);
                state = state.rotate_left(R2);
                state = (state.wrapping_mul(M)).wrapping_add(N);
                buffer = &buffer[4..];
            }
            _ => unreachable!(),
        };
    }
}
fn finish(state: u32, processed: u32) -> u32 {
    let mut hash = state;
    hash ^= processed;
    hash ^= hash.wrapping_shr(R1);
    hash = hash.wrapping_mul(C1);
    hash ^= hash.wrapping_shr(R2);
    hash = hash.wrapping_mul(C2);
    hash ^= hash.wrapping_shr(R1);
    hash
}

fn calc_k(k: u32) -> u32 {
    k.wrapping_mul(C1).rotate_left(R1).wrapping_mul(C2)
}
