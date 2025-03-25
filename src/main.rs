fn main() {
    let width = 512;
    let height =  512;

    println!("P3");
    println!("{width} {height}\n255");
    
    for i in 0..height {
        eprintln!("Scanlines remaining: {} ", height - i);
        for j in 0..width {
            let r = j as f32/height as f32;
            let g = i as f32/width as f32;
            let b = 0.0;

            let r = (r * 255.999) as u32;
            let g = (g * 255.999) as u32;
            let b = (b * 255.999) as u32;

            println!("{r} {g} {b}");
        }
    }
    eprintln!("Done.");
}
