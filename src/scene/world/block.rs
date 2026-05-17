use raylib::{color::Color, drawing::RaylibDraw3D, math::Vector3};

pub fn draw_block(d: &mut impl RaylibDraw3D, x: f32, y: f32, z: f32, color: Color) {
    let (x0, x1) = (x - 0.5, x + 0.5);
    let (y0, y1) = (y, y + 1.0);
    let (z0, z1) = (z - 0.5, z + 0.5);

    let top = color;
    let side = color.brightness(-0.2);
    let bottom = color.brightness(-0.4);

    // top
    draw_quad(
        d,
        [x0, y1, z1],
        [x1, y1, z1],
        [x1, y1, z0],
        [x0, y1, z0],
        top,
    );
    // bottom
    draw_quad(
        d,
        [x0, y0, z1],
        [x1, y0, z1],
        [x1, y0, z0],
        [x0, y0, z0],
        bottom,
    );
    // front (z+)
    draw_quad(
        d,
        [x0, y0, z1],
        [x1, y0, z1],
        [x1, y1, z1],
        [x0, y1, z1],
        side,
    );
    // back (z-)
    draw_quad(
        d,
        [x1, y0, z0],
        [x0, y0, z0],
        [x0, y1, z0],
        [x1, y1, z0],
        side,
    );
    // right (x+)
    draw_quad(
        d,
        [x1, y0, z1],
        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y1, z1],
        side,
    );
    // left (x-)
    draw_quad(
        d,
        [x0, y0, z0],
        [x0, y0, z1],
        [x0, y1, z1],
        [x0, y1, z0],
        side,
    );
}

fn draw_quad(
    d: &mut impl RaylibDraw3D,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    e: [f32; 3],
    color: Color,
) {
    let [a, b, c, e] = [a, b, c, e].map(|v| Vector3::new(v[0], v[1], v[2]));
    d.draw_triangle3D(a, b, c, color);
    d.draw_triangle3D(a, c, e, color);
}
