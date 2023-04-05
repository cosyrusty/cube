use std::f32::consts::PI;

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
    let cube = Cube::unit_cube();
    let mut angle = 0.0;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // angle += 90 degree; each second.
        angle = wrap_angle(angle + PI / 2. * get_frame_time());

        clear_background(WHITE);

        // NOTE: offsetting cube changes its origin hence, rotation must be done before offset
        cube.rotate_x(angle)
            .rotate_y(angle)
            .rotate_z(angle)
            .scale(75.)
            .offset(vec3(screen_width() / 2., screen_height() / 2., 0.))
            .project();

        next_frame().await
    }
}

struct Cube {
    vertices: [Vec3; 8],
}

impl Cube {
    pub fn unit_cube() -> Self {
        #[rustfmt::skip]
        let vertices: [Vec3; 8]= [
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

    pub fn scale(&self, factor: f32) -> Cube {
        Self {
            vertices: self.vertices.map(|v| v * factor),
        }
    }

    pub fn offset(&self, value: Vec3) -> Cube {
        Self {
            vertices: self.vertices.map(|v| v + value),
        }
    }

    pub fn rotate_x(&self, angle: f32) -> Cube {
        // NOTE: origin of 'cube' must be x: 0.0, y: 0.0, z: 0.0;

        #[rustfmt::skip]
        let rotation_matrix = Mat3 {
            x_axis: vec3(1.,           0.,          0.),
            y_axis: vec3(0.,  angle.cos(), angle.sin()),
            z_axis: vec3(0., -angle.sin(), angle.cos()),
        };

        Self {
            vertices: self.vertices.map(|v| rotation_matrix * v),
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Cube {
        // NOTE: origin of 'cube' must be x: 0.0, y: 0.0, z: 0.0;

        #[rustfmt::skip]
        let rotation_matrix = Mat3 {
            x_axis: vec3(angle.cos(), 0., -angle.sin()),
            y_axis: vec3(         0., 1.,           0.),
            z_axis: vec3(angle.sin(), 0.,  angle.cos()),
        };

        Self {
            vertices: self.vertices.map(|v| rotation_matrix * v),
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Cube {
        // NOTE: origin of 'cube' must be x: 0.0, y: 0.0, z: 0.0;

        #[rustfmt::skip]
        let rotation_matrix = Mat3 {
            x_axis: vec3( angle.cos(), angle.sin(), 0.),
            y_axis: vec3(-angle.sin(), angle.cos(), 0.),
            z_axis: vec3(          0.,          0., 1.),
        };

        Self {
            vertices: self.vertices.map(|v| rotation_matrix * v),
        }
    }

    pub fn project(&self) {
        let projection_matrix = Mat3 {
            x_axis: vec3(1., 0., 0.),
            y_axis: vec3(0., 1., 0.),
            z_axis: vec3(0., 0., 0.),
        };

        let projected_cube = self.vertices.map(|v| projection_matrix * v);

        for v in projected_cube {
            draw_circle(v.x, v.y, 3., RED);
        }

        let v = projected_cube;
        // front
        draw_line(v[0].x, v[0].y, v[1].x, v[1].y, 3., RED);
        draw_line(v[1].x, v[1].y, v[2].x, v[2].y, 3., RED);
        draw_line(v[2].x, v[2].y, v[3].x, v[3].y, 3., RED);
        draw_line(v[3].x, v[3].y, v[0].x, v[0].y, 3., RED);

        // back
        draw_line(v[4].x, v[4].y, v[5].x, v[5].y, 3., RED);
        draw_line(v[5].x, v[5].y, v[6].x, v[6].y, 3., RED);
        draw_line(v[6].x, v[6].y, v[7].x, v[7].y, 3., RED);
        draw_line(v[7].x, v[7].y, v[4].x, v[4].y, 3., RED);

        // side
        draw_line(v[0].x, v[0].y, v[4].x, v[4].y, 3., RED);
        draw_line(v[1].x, v[1].y, v[5].x, v[5].y, 3., RED);
        draw_line(v[2].x, v[2].y, v[6].x, v[6].y, 3., RED);
        draw_line(v[3].x, v[3].y, v[7].x, v[7].y, 3., RED);
    }
}

pub fn wrap_angle(angle: f32) -> f32 {
    if angle < 0.0 {
        let dif = 0.0 - angle;
        2. * PI - dif
    } else if angle > 2. * PI {
        angle - 2. * PI
    } else {
        angle
    }
}
