const REF_DATA_VALUE_MAX: u32 = 64;// (1/64) is minimum value

// reference from https://github.com/vha3/Hunter-Adams-RP2040-Demos/blob/master/Audio/g_Audio_FFT/fft.c
const NUM_SAMPLES: usize = 64;// (1/64) is minimum value
const LOG2_NUM_SAMPLES: u16 = 6;// 64 = 2^6
// Length of short (16 bits) minus log2 number of samples (6)
const SHIFT_AMOUNT: u16 = 16 - LOG2_NUM_SAMPLES;

const BALL_PULSE: [u16; NUM_SAMPLES] =
[
    64, 64, 64, 64, 64, 64, 64, 64, // first FULL: 8 pulse
    16, 16, 16, 16, // second 1/4: 4 pulse
    4, 4, 4, 4, // second 1/16: 4 pulse
    1, 1, 1, 1, // second 1/64: 4 pulse
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 833/25 = 33.32, 33 - 8 - 4 - 4 - 4 = 13. 33 * 2 = 66, =>
                                        // shrink 64, 32 - 8 - 4 - 4 - 4 = 12, 
    64, 64, 64, 64, 64, 64, 64, 64, // first FULL: 8 pulse
    16, 16, 16, 16, // second 1/4: 4 pulse
    4, 4, 4, 4, // second 1/16: 4 pulse
    1, 1, 1, 1, // second 1/64: 4 pulse
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 833/25 = 33.32, 33 - 8 - 4 - 4 - 4 = 13. 33 * 2 = 66, =>
                                        // shrink 64, 32 - 8 - 4 - 4 - 4 = 12, 
];

fn multfix15(a: i16, b: i16) -> i16 {
    ((a as i32 * b as i32) >> 15) as i16
}

fn fftfix(fr: &mut [i16; NUM_SAMPLES], fi: &mut [i16; NUM_SAMPLES], sinewave: &[i16; NUM_SAMPLES]) -> () {
    //bit order reverse
    for m in 1..(NUM_SAMPLES - 1) {
        // swap odd and even bits
        let mut mr = ((m >> 1) & 0x5555) | ((m & 0x5555) << 1);
        // swap consecutive pairs
        mr = ((mr >> 2) & 0x3333) | ((mr & 0x3333) << 2);
        // swap nibbles ... 
        mr = ((mr >> 4) & 0x0F0F) | ((mr & 0x0F0F) << 4);
        // swap bytes
        mr = ((mr >> 8) & 0x00FF) | ((mr & 0x00FF) << 8);
        // shift down mr
        mr >>= SHIFT_AMOUNT ;
        // don't swap that which has already been swapped
        if mr<=m { continue; }
        // swap the bit-reveresed indices
        let tr = fr[m] ;
        fr[m] = fr[mr] ;
        fr[mr] = tr ;
        let ti = fi[m] ;
        fi[m] = fi[mr] ;
        fi[mr] = ti ;
    }
    println!("{:?}", fr);
    // Adapted from code by:
    // Tom Roberts 11/8/89 and Malcolm Slaney 12/15/94 malcolm@interval.com
    // Length of the FFT's being combined (starts at 1)
    let mut l: usize = 1 ;
    // Log2 of number of samples, minus 1
    let mut k: u16 = LOG2_NUM_SAMPLES - 1 ;
    // While the length of the FFT's being combined is less than the number 
    // of gathered samples . . .
    while l < NUM_SAMPLES {
        // Determine the length of the FFT which will result from combining two FFT's
        let istep: usize = l << 1 ;
        // For each element in the FFT's that are being combined . . .
        for m in 0..l {
            let j = m << k;
            let mut wr: i16 =  sinewave[j + (NUM_SAMPLES / 4)] ; // cos(2pi m/N)
            let mut wi: i16 = -sinewave[j] ;                 // sin(2pi m/N)
            wr >>= 1 ;                          // divide by two
            wi >>= 1 ;                          // divide by two
            // i gets the index of one of the FFT elements being combined
            let mut i: usize = m;
            while i < NUM_SAMPLES {
                // j gets the index of the FFT element being combined with i
                let j = i + l ;
                // compute the trig terms (bottom half of the above matrix)
                let tr = multfix15(wr, fr[j]) - multfix15(wi, fi[j]) ;
                let ti = multfix15(wr, fi[j]) + multfix15(wi, fr[j]) ;
                // divide ith index elements by two (top half of above matrix)
                let qr = fr[i] >> 1 ;
                let qi = fi[i] >> 1 ;
                // compute the new values at each index
                fr[j] = qr - tr ;
                fi[j] = qi - ti ;
                fr[i] = qr + tr ;
                fi[i] = qi + ti ;

                i += istep;
            }
        }
        k = k - 1;
        l = istep;
    }
}

fn main() {
    // should calc once
    let mut sinewave: [i16; NUM_SAMPLES] = [0; NUM_SAMPLES];
    for i in 0..NUM_SAMPLES {
        sinewave[i] = ((6.283 * (i as f32 / NUM_SAMPLES as f32)).sin() as f32 * 32768.0 as f32) as i16; // float2fix15 //2^15
    }
    println!("{:?}", sinewave);

    let mut fr: [i16; NUM_SAMPLES] = [0; NUM_SAMPLES];
    let mut fi: [i16; NUM_SAMPLES] = [0; NUM_SAMPLES];
    for i in 0..NUM_SAMPLES {
        fr[i] = BALL_PULSE[i] as i16 * (i16::MAX / 64);
    }


    //display
    const NX: usize = NUM_SAMPLES + 1;
    const NY: u32 = REF_DATA_VALUE_MAX + 1; //(TERMINAL $LINES)

    let mut plot: [[bool; NY as usize]; NX as usize] =[[false; NY as usize]; NX as usize];

        /*
    for x in 0..NUM_SAMPLES {
        let y: usize = (fr[x] / (i16::MAX / 64)) as usize;
        plot[x][y] = true;
    }
        */

    // test sine
    let sample_interval = 1.0 / NUM_SAMPLES as f32;
    for i in 0..NUM_SAMPLES {
       fr[i] = ((2.0 * std::f32::consts::PI * 3.0 * sample_interval * i as f32).sin() * 64.0) as i16;
    }
    for x in 0..NUM_SAMPLES {
        println!("{}", fr[x]);
        let y: usize = ((fr[x] + 64) / 2) as usize;
        plot[x][y] = true;
    }

    println!("BALL_PULSE");

    for y in 0..NY {
        for x in 0..NX {
            match plot[x as usize][y as usize] {
                true => print!("*"),
                false => print!(" "),
            }
        }
        println!("");
    }

    //try fft
    fftfix(&mut fr, &mut fi, &sinewave);
    println!("{:?}", fr);
    println!("{:?}", fi);

    //display

    let mut plot: [[bool; NY as usize]; NX as usize] =[[false; NY as usize]; NX as usize];
    let mut max: i16 = 0;
    for x in 0..NUM_SAMPLES {
        fr[x] = fr[x].abs() + fi[x].abs();
        if fr[x] > max {
            max = fr[x];
        }
    }
    println!("max: {}", max);
    for x in 0..NUM_SAMPLES {
        //let y: usize = (fr[x] as f32 / max as f32 * REF_DATA_VALUE_MAX as f32) as usize; // need abs ?
        let y: usize = fr[x] as usize;
        //println!("{}", y);
        plot[x][y] = true;
    }

    println!("FFT result");

    for y in 0..NY {
        for x in 0..NX {
            match plot[x as usize][y as usize] {
                true => print!("*"),
                false => print!(" "),
            }
        }
        println!("");
    }

}
