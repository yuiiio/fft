const PI_2_32: f32 = 2.0 as f32 * std::f32::consts::PI;

const SAMPLE_SIZE_POWER_OF_TWO: u32 = 6;// 64 = 2^6
const SAMPLE_SIZE: u32 = 2u32.pow(SAMPLE_SIZE_POWER_OF_TWO);

const ADC_BIT: u32 = 12; // 10 or 12 bit adc
const ADC_BIT_MAX: u32 = 2u32.pow(ADC_BIT);

const REF_DATA_VALUE_MAX: u32 = 64;// (1/64) is minimum value
const BALL_PULSE: [u32; SAMPLE_SIZE as usize] =
[
    0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, // first FULL: 8 pulse
    0, 16, 0, 16, 0, 16, 0, 16, // second 1/4: 4 pulse
    0, 4, 0, 4, 0, 4, 0, 4, // second 1/16: 4 pulse
    0, 1, 0, 1, 0, 1, 0, 1, // second 1/64: 4 pulse
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 64 - ((8+4+4+4)*2) = 24
]; // need revert for sensor input value ?
   // and need 10bit (1024) or 12bit (4096) scale is need ?
   // or only wave signal is important ?

//should calc only onece on.
fn bit_reverse_order() -> [u32; SAMPLE_SIZE as usize] {
    let mut default_order: [u32; SAMPLE_SIZE as usize] = [0; SAMPLE_SIZE as usize];
    for i in 0..SAMPLE_SIZE {
        default_order[i as usize] = i
    }

    let mut reverse_order: [u32; SAMPLE_SIZE as usize] = [0; SAMPLE_SIZE as usize];

    let bits_size: usize = SAMPLE_SIZE_POWER_OF_TWO as usize;

    // bit num use u32
    let shift_bit: usize = 32 - bits_size;

    for i in 0..SAMPLE_SIZE {
        reverse_order[i as usize] = default_order[i as usize].reverse_bits() >> shift_bit;
    }

    return reverse_order;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct complex_t {
    x: f32,
    i: f32,
}

impl complex_t {
    pub fn new(x: f32, i: f32) -> Self {
        complex_t { x, i }
    }

    pub fn add(&self, a: complex_t) -> Self {
        let bx: f32 = self.x + a.x;
        let bi: f32 = self.i + a.i;
        complex_t {x: bx, i: bi}
    }

    pub fn sub(&self, a: complex_t) -> Self {
        let bx: f32 = self.x - a.x;
        let bi: f32 = self.i - a.i;
        complex_t {x: bx, i: bi}
    }

    pub fn mul(&self, a: complex_t) -> Self {
        let bx: f32 = (self.x * a.x) - (self.i * a.i);
        let bi: f32 = (self.x * a.i) + (self.i * a.x);
        complex_t {x: bx, i: bi}
    }
}

// should calc only onece.
// should make rotate table at startup.
fn rotate_w_matrix() -> [[complex_t; SAMPLE_SIZE as usize]; SAMPLE_SIZE_POWER_OF_TWO as usize] {
    let mut rotate_w_matrix:[[complex_t; SAMPLE_SIZE as usize]; SAMPLE_SIZE_POWER_OF_TWO as usize] =
        [[complex_t {x: 0.0, i: 0.0}.clone(); SAMPLE_SIZE as usize]; SAMPLE_SIZE_POWER_OF_TWO as usize];

    for i in (0..SAMPLE_SIZE_POWER_OF_TWO).rev() {
        let block_num: u32 = 2u32.pow(i);
        let block_size: u32 = SAMPLE_SIZE / block_num;

        let n: f32 = (2.0 as f32).powi((SAMPLE_SIZE_POWER_OF_TWO - i) as i32);
        let dir: f32 =  PI_2_32 / n;
        let rotate_w_first: complex_t = complex_t { x: dir.cos(), i: dir.sin() };

        let mut rotate_w_vec: Vec<complex_t> = vec!();
        let mut rotate_w: complex_t = complex_t {x: 1.0, i: 0.0};
        for _j in 0..block_size {
            rotate_w_vec.push(rotate_w);
            rotate_w = rotate_w.mul(rotate_w_first);
        }

        for j in 0..SAMPLE_SIZE {
            let wk: u32 = j % block_size;
            rotate_w_matrix[((SAMPLE_SIZE_POWER_OF_TWO - 1) - i) as usize][j as usize] = rotate_w_vec[wk as usize];
        }
    }
    return rotate_w_matrix;
}

// input u32 vector and output f32 vector.
fn fft(input_vec: &[u32; SAMPLE_SIZE as usize],
       reverse_order: &[u32; SAMPLE_SIZE as usize],
       rotate_w_matrix: &[[complex_t; SAMPLE_SIZE as usize]; SAMPLE_SIZE_POWER_OF_TWO as usize]) -> [f32; SAMPLE_SIZE as usize] {

    let mut input_reverse_order: [complex_t; SAMPLE_SIZE as usize] = [complex_t{ x: 0.0, i: 0.0 }; SAMPLE_SIZE as usize];

    //bit order reverse
    //and set input data to complex_t x: (and convert to f32 for below calc)
    for i in 0..SAMPLE_SIZE {
        input_reverse_order[i as usize].x = input_vec[reverse_order[i as usize] as usize] as f32;
    }

    let mut output_vec: [complex_t; SAMPLE_SIZE as usize] = input_reverse_order;
    for i in (0..SAMPLE_SIZE_POWER_OF_TWO).rev() {
        let middle_vec: [complex_t; SAMPLE_SIZE as usize] = output_vec;

        let block_num: u32 = 2u32.pow(i);
        let block_size: u32 = SAMPLE_SIZE / block_num;

        for j in 0..block_num {
            let block_start_point: u32 = j * block_size; // block_size is 2 ~ SAMPLE_SIZE
            for k in 0..(block_size/2) {
                let point1: u32 = block_start_point + k;
                let point2: u32 = block_start_point + (k * 2);
                output_vec[point1 as usize] = middle_vec[point1 as usize].add( rotate_w_matrix[i as usize][point1 as usize].mul(middle_vec[point2 as usize]) );
            }
            for k in 0..(block_size/2) {
                let point1: u32 = block_start_point + k;
                let point2: u32 = block_start_point + (k * 2);
                output_vec[(point1 + (block_size/2)) as usize] = middle_vec[point1 as usize].add( rotate_w_matrix[i as usize][(point1 + (block_size/2)) as usize].mul(middle_vec[point2 as usize]) );
            }
        }
    }

    let mut output_x_vec: [f32; SAMPLE_SIZE as usize] = [0.0; SAMPLE_SIZE as usize];
    for i in 0..SAMPLE_SIZE {
        output_x_vec[i as usize] = output_vec[i as usize].x;
    }
    return output_x_vec;
}

fn main() {
    let mut sensor_val: [u32; SAMPLE_SIZE as usize] = [0; SAMPLE_SIZE as usize]; //10 or 12 bit adc < 32 bit

    // set sample data scale and fit.
    for i in 0..SAMPLE_SIZE {
        sensor_val[i as usize] = ADC_BIT_MAX - ((ADC_BIT_MAX / REF_DATA_VALUE_MAX) * BALL_PULSE[i as usize]);
    }

    //sin wave for test
    for i in 0..SAMPLE_SIZE {
        let x: f32 = PI_2_32 * (i as f32 / SAMPLE_SIZE as f32);
        sensor_val[i as usize] = ((ADC_BIT_MAX as f32 * 1.0/2.0) + ((ADC_BIT_MAX as f32 * x.sin()) * 1.0/2.0)) as u32;
    }

    //display
    const NX: u32 = SAMPLE_SIZE;
    const NY: u32 = REF_DATA_VALUE_MAX;//40;//(TERMINAL $LINES)

    let mut plot: [[bool; NY as usize]; NX as usize] =[[false; NY as usize]; NX as usize];

    for x in 0..SAMPLE_SIZE {
        let y: usize = (sensor_val[x as usize] as f32 * ((NY - 1) as f32 / ADC_BIT_MAX as f32) as f32) as usize;
        plot[x as usize][y as usize] = true;
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

    // reverse bit order and w rotate matrix should calc only onece.
    let reverse_order: [u32; SAMPLE_SIZE as usize] = bit_reverse_order();
    //println!("{:?}", reverse_order);
    let rotate_w_matrix: [[complex_t; SAMPLE_SIZE as usize]; SAMPLE_SIZE_POWER_OF_TWO as usize] = rotate_w_matrix();
    //println!("{:?}", rotate_w_matrix);

    //try fft
    let fft_result: [f32; SAMPLE_SIZE as usize] = fft(&sensor_val, &reverse_order, &rotate_w_matrix);
    //println!("{:?}", fft_result);

    //display

    let mut plot: [[bool; NY as usize]; NX as usize] =[[false; NY as usize]; NX as usize];
    for x in 0..SAMPLE_SIZE {
        let y: f32 = NY as f32 - ((fft_result[x as usize] / (ADC_BIT_MAX * SAMPLE_SIZE) as f32) as f32) - 1.0;
        let y: usize = y as usize;
        plot[x as usize][y as usize] = true;
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
