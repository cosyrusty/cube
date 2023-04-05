use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_width: 256,
        window_height: 256,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cube = UnitCube::new();
    // scale cube by 50x
    cube.scale(50.);
    // move cube to center of screen
    cube.offset(vec3(screen_width() / 2., screen_height() / 2., 0.));

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(WHITE);
        cube.project();
        next_frame().await
    }
}

// length of each side is one
pub struct UnitCube {
    vertices: [Vec3; 8],
}

impl UnitCube {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let vertices: [Vec3; 8] = [
            // front
            vec3(-0.5, -0.5, -0.5),
            vec3( 0.5, -0.5, -0.5),
            vec3( 0.5,  0.5, -0.5),
            vec3(-0.5,  0.5, -0.5),
            // back
            vec3(-0.5, -0.5,  0.5),
            vec3( 0.5, -0.5,  0.5),
            vec3( 0.5,  0.5,  0.5),
            vec3(-0.5,  0.5,  0.5),
        ];
        Self { vertices }
    }

    pub fn scale(&mut self, factor: f32) {
        for v in &mut self.vertices {
            *v *= factor;
        }
    }

    pub fn offset(&mut self, value: Vec3) {
        for v in &mut self.vertices {
            *v += value;
        }
    }

    pub fn project(&self) {
        let projection_matrix = Mat3 {
            x_axis: vec3(1., 0., 0.),
            y_axis: vec3(0., 1., 0.),
            z_axis: vec3(0., 0., 0.),
        };

        let projected_vertices: [Vec3; 8] = self.vertices.map(|v| projection_matrix * v);

        for v in projected_vertices {
            draw_circle(v.x, v.y, 3., RED);
        }

        let v = projected_vertices;
        // front
        draw_line(v[0].x, v[0].y, v[1].x, v[1].y, 2., RED);
        draw_line(v[1].x, v[1].y, v[2].x, v[2].y, 2., RED);
        draw_line(v[2].x, v[2].y, v[3].x, v[3].y, 2., RED);
        draw_line(v[3].x, v[3].y, v[0].x, v[0].y, 2., RED);

        // back
        draw_line(v[4].x, v[4].y, v[5].x, v[5].y, 2., RED);
        draw_line(v[5].x, v[5].y, v[6].x, v[6].y, 2., RED);
        draw_line(v[6].x, v[6].y, v[7].x, v[7].y, 2., RED);
        draw_line(v[7].x, v[7].y, v[4].x, v[4].y, 2., RED);

        // side
        draw_line(v[0].x, v[0].y, v[4].x, v[4].y, 2., RED);
        draw_line(v[1].x, v[1].y, v[5].x, v[5].y, 2., RED);
        draw_line(v[2].x, v[2].y, v[6].x, v[6].y, 2., RED);
        draw_line(v[3].x, v[3].y, v[7].x, v[7].y, 2., RED);
    }
}
