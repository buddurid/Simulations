extern crate nannou;
use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
const VELOCITY_X: i32 = 5;
const VELOCITY_Y: i32 = 5;
struct Circle {
    x: i32,   // x position
    y: i32,   // y position
    r: u32,   // r rayon
    v_x: i32, // velocity at x axe
    v_y: i32, // velocity at y axe
    color: Rgb8,
}
struct Model {
    win_x: i32,
    win_y: i32,
    objects: [Circle; 3],
}

fn model(_app: &App) -> Model {
    let c1 = Circle {
        x: 0,
        y: 0,
        r: 100,
        v_x: VELOCITY_X * 4,
        v_y: VELOCITY_Y * 1,
        color: BLUEVIOLET,
    };
    let c2 = Circle {
        x: -(_app.window_rect().wh()[0] as i32 / 4),
        y: (_app.window_rect().wh()[0] as i32 / 4),
        r: 100,
        v_x: VELOCITY_X,
        v_y: VELOCITY_Y * 2,
        color: ORANGERED,
    };
    let c3 = Circle {
        x: (_app.window_rect().wh()[0] as i32 / 4),
        y: -(_app.window_rect().wh()[0] as i32 / 4),
        r: 100,
        v_x: VELOCITY_X * 3,
        v_y: VELOCITY_Y,
        color: GREY,
    };

    Model {
        win_x: _app.window_rect().wh()[0] as i32,
        win_y: _app.window_rect().wh()[1] as i32,
        objects: [c1, c2, c3],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //////////////////////////////
    /// check collision with edges
    for object in &mut _model.objects {
        let x: i32 = object.x;
        let y: i32 = object.y;

        if (x.abs() + (object.r as i32 / 2)) >= (_model.win_x / 2) {
            // collides with vertical edge
            object.v_x *= -1;
        }
        if (y.abs() + (object.r as i32 / 2)) >= (_model.win_y / 2) {
            // collides with vertical edge
            object.v_y *= -1;
        }
    }
    //////////////////////////////
    /// check collision with other balls
    let objects = &mut _model.objects;
    for i in 0..3 {
        for j in (i + 1)..3 {
            let distance = ((objects[i].x - objects[j].x).pow(2)
                + (objects[i].y - objects[j].y).pow(2))
            .isqrt() as u32;
            if distance <= ((objects[i].r + objects[j].r) / 2) {
                // collision between i and j
                let mut ij = Vec2::new(
                    (objects[i].x - objects[j].x) as f32,
                    (objects[i].y - objects[j].y) as f32,
                );
                //ij.normalize();
                let i_v = Vec2::new(objects[i].v_x as f32, objects[i].v_y as f32);
                let j_v = Vec2::new(objects[j].v_x as f32, objects[j].v_y as f32);

                println!("{} , {} | {} ", i_v, j_v, ij);
                //i_v.normalize();
                //j_v.normalize();

                let new_i = (i_v + ij).normalize();
                let new_j = (j_v - ij).normalize();

                println!("new : {} , {} | {} ", new_i, new_j, ij);

                objects[i].v_x = (new_i[0] * VELOCITY_X as f32) as i32; // this needs to be checked later , some rounding when casting to i32 may lead to v=0
                objects[i].v_y = (new_i[1] * VELOCITY_Y as f32) as i32; // this needs to be checked later , some rounding when casting to i32 may lead to v=0
                objects[j].v_x = (new_j[0] * VELOCITY_X as f32) as i32; // this needs to be checked later , some rounding when casting to i32 may lead to v=0
                objects[j].v_y = (new_j[1] * VELOCITY_Y as f32) as i32; // this needs to be checked later , some rounding when casting to i32 may lead to v=0
            }
        }
        objects[i].x += objects[i].v_x;
        objects[i].y += objects[i].v_y;
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    draw.background().color(WHITE);
    for object in &_model.objects {
        draw.ellipse()
            .color(object.color)
            .xy(Vec2::new(object.x as f32, object.y as f32))
            .wh(Vec2::new(object.r as f32, object.r as f32));
    }

    draw.to_frame(_app, &frame).unwrap();
}
