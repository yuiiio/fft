fn main() {
    const SAMPLE_SIZE: usize = 64;//(2^6)
    const VALUE_MAX: usize = 64;// (1/64) is minimum value

    const BALL_PULSE: [u8; SAMPLE_SIZE] =
        [
        0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, 0, 64, // first FULL: 8 pulse
        0, 16, 0, 16, 0, 16, 0, 16, // second 1/4: 4 pulse
        0, 4, 0, 4, 0, 4, 0, 4, // second 1/16: 4 pulse
        0, 1, 0, 1, 0, 1, 0, 1, // second 1/64: 4 pulse
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 64 - ((8+4+4+4)*2) = 24
        ];

    //display
    const NX: usize = SAMPLE_SIZE + 1;
    const NY: usize = VALUE_MAX + 1;//40;//(TERMINAL $LINES)

    let mut plot: [[bool; NY]; NX] =[[false; NY]; NX];

    for x in 0..SAMPLE_SIZE {
        let y: usize = (64 - BALL_PULSE[x]) as usize;//BALL_PULSE[x] as usize; //
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
}
