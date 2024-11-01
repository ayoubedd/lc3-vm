// uint16_t sign_extend(uint16_t x, int bit_count)
// {
//     if ((x >> (bit_count - 1)) & 1) {
//         x |= (0xFFFF << bit_count);
//     }
//     return x;
// }

// pub fn sign_extend(mut value: u16, width: u16) -> u16 {
//     let last_bit = 1 << (16 - width - 1);
//
//     if value & last_bit != 0 {
//         value |= 0xFFFF << 16 - width;
//     }
//
//     value
// }

pub fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }

    x
}
