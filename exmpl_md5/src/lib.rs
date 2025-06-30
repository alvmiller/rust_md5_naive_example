use std::convert::TryInto;

//----------------------------------------------------//
const VALS_INIT: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

const RCNT: [u32; 64] = [
    // round 1
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    // round 2
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    // round 3
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    // round 4
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];
//----------------------------------------------------//

//----------------------------------------------------//
#[inline(always)]
fn bit_padding(input: &str) -> Vec<u8> {
    let mut input_vector: Vec<u8> = convert_str_to_vec(input);
    let bit_length: u64 = (input.len() as u64) * 8u64; // todo - add support for > 2^64 bit size

    input_vector.push(128_u8);
    while (input_vector.len() * 8) % 512 != 448 {
        input_vector.push(0_u8);
    }

    let length_bits_as_u8_array = split_u64_to_u8_array(bit_length);
    input_vector.extend(length_bits_as_u8_array);

    return input_vector;
}

#[inline(always)]
fn split_u64_to_u8_array(s: u64) -> [u8; 8] {
    let u8_array = [
        s as u8,
        (s >> 8) as u8,
        (s >> 16) as u8,
        (s >> 24) as u8,
        (s >> 32) as u8,
        (s >> 40) as u8,
        (s >> 48) as u8,
        (s >> 56) as u8,
    ];
    return u8_array;
}

#[inline(always)]
fn convert_str_to_vec(input: &str) -> Vec<u8> {
    let mut byte_vec: Vec<u8> = Vec::new();
    byte_vec.extend(input.as_bytes());
    return byte_vec;
}

#[inline(always)]
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|_v: Vec<T>| panic!("error converting vector to array - sizes don't match"))
}

#[inline(always)]
fn convert_u8_chunk_to_u32(chunk: &mut [u8]) -> Vec<u32> {
    let mut x: Vec<u32> = Vec::new();

    let mut count = 0;
    let mut temporary_vec: Vec<u8> = Vec::new();

    for i in 0..chunk.len() {
        temporary_vec.push(chunk[i]);
        count += 1;
        if count == 4 {
            let temp_arr: [u8; 4] = vec_to_array(temporary_vec.clone());
            let value = u32::from_ne_bytes(temp_arr);
            x.push(value);
            count = 0;
            temporary_vec.clear();
        }
    }
    return x;
}

#[inline(always)]
fn op_f(w: u32, x: u32, y: u32, z: u32, m: u32, c: u32, s: u32) -> u32 {
    ((x & y) | (!x & z))
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(c)
        .rotate_left(s)
        .wrapping_add(x)
}
#[inline(always)]
fn op_g(w: u32, x: u32, y: u32, z: u32, m: u32, c: u32, s: u32) -> u32 {
    ((x & z) | (y & !z))
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(c)
        .rotate_left(s)
        .wrapping_add(x)
}

#[inline(always)]
fn op_h(w: u32, x: u32, y: u32, z: u32, m: u32, c: u32, s: u32) -> u32 {
    (x ^ y ^ z)
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(c)
        .rotate_left(s)
        .wrapping_add(x)
}

#[inline(always)]
fn op_i(w: u32, x: u32, y: u32, z: u32, m: u32, c: u32, s: u32) -> u32 {
    (y ^ (x | !z))
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(c)
        .rotate_left(s)
        .wrapping_add(x)
}

fn compute_md5_digest_ext(mut v: Vec<u8>) -> String {
    let mut wia = VALS_INIT[0];
    let mut wib = VALS_INIT[1];
    let mut wic = VALS_INIT[2];
    let mut wid = VALS_INIT[3];

    for chunk in v.chunks_exact_mut(64) {
        let data = convert_u8_chunk_to_u32(chunk); // -> [u8; 4]

        let mut a = wia;
        let mut b = wib;
        let mut c = wic;
        let mut d = wid;

        // round 1
        a = op_f(a, b, c, d, data[0], RCNT[0], 7);
        d = op_f(d, a, b, c, data[1], RCNT[1], 12);
        c = op_f(c, d, a, b, data[2], RCNT[2], 17);
        b = op_f(b, c, d, a, data[3], RCNT[3], 22);

        a = op_f(a, b, c, d, data[4], RCNT[4], 7);
        d = op_f(d, a, b, c, data[5], RCNT[5], 12);
        c = op_f(c, d, a, b, data[6], RCNT[6], 17);
        b = op_f(b, c, d, a, data[7], RCNT[7], 22);

        a = op_f(a, b, c, d, data[8], RCNT[8], 7);
        d = op_f(d, a, b, c, data[9], RCNT[9], 12);
        c = op_f(c, d, a, b, data[10], RCNT[10], 17);
        b = op_f(b, c, d, a, data[11], RCNT[11], 22);

        a = op_f(a, b, c, d, data[12], RCNT[12], 7);
        d = op_f(d, a, b, c, data[13], RCNT[13], 12);
        c = op_f(c, d, a, b, data[14], RCNT[14], 17);
        b = op_f(b, c, d, a, data[15], RCNT[15], 22);

        // round 2
        a = op_g(a, b, c, d, data[1], RCNT[16], 5);
        d = op_g(d, a, b, c, data[6], RCNT[17], 9);
        c = op_g(c, d, a, b, data[11], RCNT[18], 14);
        b = op_g(b, c, d, a, data[0], RCNT[19], 20);

        a = op_g(a, b, c, d, data[5], RCNT[20], 5);
        d = op_g(d, a, b, c, data[10], RCNT[21], 9);
        c = op_g(c, d, a, b, data[15], RCNT[22], 14);
        b = op_g(b, c, d, a, data[4], RCNT[23], 20);

        a = op_g(a, b, c, d, data[9], RCNT[24], 5);
        d = op_g(d, a, b, c, data[14], RCNT[25], 9);
        c = op_g(c, d, a, b, data[3], RCNT[26], 14);
        b = op_g(b, c, d, a, data[8], RCNT[27], 20);

        a = op_g(a, b, c, d, data[13], RCNT[28], 5);
        d = op_g(d, a, b, c, data[2], RCNT[29], 9);
        c = op_g(c, d, a, b, data[7], RCNT[30], 14);
        b = op_g(b, c, d, a, data[12], RCNT[31], 20);

        // round 3
        a = op_h(a, b, c, d, data[5], RCNT[32], 4);
        d = op_h(d, a, b, c, data[8], RCNT[33], 11);
        c = op_h(c, d, a, b, data[11], RCNT[34], 16);
        b = op_h(b, c, d, a, data[14], RCNT[35], 23);

        a = op_h(a, b, c, d, data[1], RCNT[36], 4);
        d = op_h(d, a, b, c, data[4], RCNT[37], 11);
        c = op_h(c, d, a, b, data[7], RCNT[38], 16);
        b = op_h(b, c, d, a, data[10], RCNT[39], 23);

        a = op_h(a, b, c, d, data[13], RCNT[40], 4);
        d = op_h(d, a, b, c, data[0], RCNT[41], 11);
        c = op_h(c, d, a, b, data[3], RCNT[42], 16);
        b = op_h(b, c, d, a, data[6], RCNT[43], 23);

        a = op_h(a, b, c, d, data[9], RCNT[44], 4);
        d = op_h(d, a, b, c, data[12], RCNT[45], 11);
        c = op_h(c, d, a, b, data[15], RCNT[46], 16);
        b = op_h(b, c, d, a, data[2], RCNT[47], 23);

        // round 4
        a = op_i(a, b, c, d, data[0], RCNT[48], 6);
        d = op_i(d, a, b, c, data[7], RCNT[49], 10);
        c = op_i(c, d, a, b, data[14], RCNT[50], 15);
        b = op_i(b, c, d, a, data[5], RCNT[51], 21);

        a = op_i(a, b, c, d, data[12], RCNT[52], 6);
        d = op_i(d, a, b, c, data[3], RCNT[53], 10);
        c = op_i(c, d, a, b, data[10], RCNT[54], 15);
        b = op_i(b, c, d, a, data[1], RCNT[55], 21);

        a = op_i(a, b, c, d, data[8], RCNT[56], 6);
        d = op_i(d, a, b, c, data[15], RCNT[57], 10);
        c = op_i(c, d, a, b, data[6], RCNT[58], 15);
        b = op_i(b, c, d, a, data[13], RCNT[59], 21);

        a = op_i(a, b, c, d, data[4], RCNT[60], 6);
        d = op_i(d, a, b, c, data[11], RCNT[61], 10);
        c = op_i(c, d, a, b, data[2], RCNT[62], 15);
        b = op_i(b, c, d, a, data[9], RCNT[63], 21);

        wia = wia.wrapping_add(a);
        wib = wib.wrapping_add(b);
        wic = wic.wrapping_add(c);
        wid = wid.wrapping_add(d);
    }

    let wra = wia.swap_bytes();
    let wrb = wib.swap_bytes();
    let wrc = wic.swap_bytes();
    let wrd = wid.swap_bytes();
    let message_digest = format!("{:08x}{:08x}{:08x}{:08x}", wra, wrb, wrc, wrd);

    return message_digest;
}

pub fn md5_ext(input: &str) -> String {
    let input_vec = bit_padding(input);
    return compute_md5_digest_ext(input_vec);
}
//----------------------------------------------------//

//----------------------------------------------------//
pub fn md5(input: &str) -> String {
    let ext_res = md5_ext(input);
    println!("md5_ext() result: {}", ext_res);
    return ext_res;
}
//----------------------------------------------------//
