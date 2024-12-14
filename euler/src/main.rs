fn d(x: f32, y: f32) -> f32 {
    let diffeq = (2.0 * x) + y;
    return diffeq;
}

fn main() {
    let x_coord_init: f32 = 1.0;
    let y_coord_init: f32 = 1.0;
    let delta_x: f32 = 0.2;
    let x_coord_final: f32 = 2.0;

    let mut x_coord = x_coord_init;
    let mut y_coord = y_coord_init;

    while x_coord < x_coord_final {
        println!("({:.001}, {})", x_coord as f32, y_coord as f32);
        let delta_y = d(x_coord, y_coord) * delta_x;
        y_coord += delta_y;
        x_coord += delta_x;
    }

    println!("({:.001}, {})", x_coord_final as f32, y_coord as f32);
}
