use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

fn main() {
    let mut a = 0.0_f32;
    let mut b = 0.0_f32;
    
    let shade = ".,-~:;=!*#$@";
    let colors = [
        "\x1b[34m", // Biru
        "\x1b[36m", // Cyan
        "\x1b[32m", // Hijau
        "\x1b[92m", // Hijau terang
        "\x1b[93m", // Kuning
        "\x1b[91m", // Merah
        "\x1b[95m", // Magenta
        "\x1b[97m", // Putih
    ];
    
    // Clear screen
    print!("\x1b[2J");
    
    loop {
        let mut buffer = [' '; 1760];
        let mut z_buffer = [0.0_f32; 1760];
        let mut color_buffer = [0_usize; 1760];
        
        let mut j = 0.0_f32;
        while j < 2.0 * PI {
            let mut i = 0.0_f32;
            while i < 2.0 * PI {
                let c = i.sin();
                let d = j.cos();
                let e = a.sin();
                let f = j.sin();
                let g = a.cos();
                let h = d + 2.0;
                let big_d = 1.0 / (c * h * e + f * g + 5.0);
                let l = i.cos();
                let m = b.cos();
                let n = b.sin();
                let t = c * h * g - f * e;
                
                let x = (40.0 + 30.0 * big_d * (l * h * m - t * n)) as i32;
                let y = (12.0 + 15.0 * big_d * (l * h * n + t * m)) as i32;
                let o = (x + 80 * y) as usize;
                
                let mut big_n = (8.0 * ((f * e - c * d * g) * m 
                                - c * d * e - f * g - l * d * n)) as i32;
                
                if y > 0 && y < 22 && x > 0 && x < 80 && o < 1760 {
                    if big_d > z_buffer[o] {
                        z_buffer[o] = big_d;
                        if big_n < 0 { big_n = 0; }
                        if big_n > 11 { big_n = 11; }
                        buffer[o] = shade.chars().nth(big_n as usize).unwrap();
                        color_buffer[o] = (big_n % 8) as usize;
                    }
                }
                
                i += 0.02;
            }
            j += 0.07;
        }
        
        
        print!("\x1b[H");
        
        for k in 0..1760 {
            if k % 80 != 0 {
                print!("{}{}", colors[color_buffer[k]], buffer[k]);
            } else {
                println!();
            }
        }
        
        
        print!("\x1b[0m");
        
        a += 0.04;
        b += 0.02;
        
        thread::sleep(Duration::from_millis(30));
    }
}

// Pusing jir wkwkkw
