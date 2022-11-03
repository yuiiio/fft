const SAMPLE_SIZE_POWER_OF_FOUR: u32 = 3; // 4^3=64
const SAMPLE_SIZE: usize = 4usize.pow(SAMPLE_SIZE_POWER_OF_FOUR);//(2^6)==(4^3)
const VALUE_MAX: usize = 64;// (1/64) is minimum value

const BALL_PULSE: [u8; SAMPLE_SIZE] = //max_value=64 < u8_max(256)
[
    0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, // first FULL: 8 pulse
    0, 16, 0, 16, 0, 16, 0, 16, // second 1/4: 4 pulse
    0, 4, 0, 4, 0, 4, 0, 4, // second 1/16: 4 pulse
    0, 1, 0, 1, 0, 1, 0, 1, // second 1/64: 4 pulse
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 64 - ((8+4+4+4)*2) = 24
]; // need revert for sensor input value ?
   // and need 10bit (1023) scale is need ?
   // or only wave signal is important ?

//should calc only onece on.
fn bit_reverse_order() -> [usize; SAMPLE_SIZE] {
    let mut default_order: [usize; SAMPLE_SIZE] = [0; SAMPLE_SIZE];
    for i in 0..SAMPLE_SIZE {
        default_order[i] = i
    }

    let mut reverse_order: [usize; SAMPLE_SIZE] = [0; SAMPLE_SIZE];

    let bits_size: usize = SAMPLE_SIZE_POWER_OF_FOUR as usize * 2;

    // 64-bit target usize = 8 bytes = 64 bit;
    let shift_bit: usize = (64 - bits_size) / 2;

    for i in 0..SAMPLE_SIZE {
        reverse_order[i] = ((default_order[i] << shift_bit).reverse_bits() >> shift_bit);
    }

    return reverse_order;
}

fn quad_fft(input_vec: [u8; SAMPLE_SIZE], reverse_order: [usize; SAMPLE_SIZE]) -> [u8; SAMPLE_SIZE] {
    let mut output_vec: [u8; SAMPLE_SIZE] = [0; SAMPLE_SIZE];
    for i in 0..SAMPLE_SIZE_POWER_OF_FOUR {
        //Q4-FFT
        //first bit reverse
        //
        let pdeg: usize = SAMPLE_SIZE / (i as usize + 1);
        for j in 0..(4^(i+1)) {
        }
    }

    return output_vec;
}

fn main() {
    //display
    const NX: usize = SAMPLE_SIZE + 1;
    const NY: usize = VALUE_MAX + 1;//40;//(TERMINAL $LINES)

    let mut plot: [[bool; NY]; NX] =[[false; NY]; NX];

    for x in 0..SAMPLE_SIZE {
        let y: usize = (64 - BALL_PULSE[x]) as usize;//BALL_PULSE[x] as usize;
                                                     // revert for dislpay
        plot[x][y] = true;
    }

    println!("BALL_PULSE");

    for y in 0..NY {
        for x in 0..NX {
            match plot[x][y] {
                true => print!("*"),
                false => print!(" "),
            }
        }
        println!("");
    }

    let reverse_order: [usize; SAMPLE_SIZE] = bit_reverse_order();
    println!("{:?}", reverse_order);
}
