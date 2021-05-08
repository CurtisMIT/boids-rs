use nannou::prelude::*;
use nannou::rand::random_range;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(PartialEq, Clone)]
struct Boid {
    position: Vector2,
    size: Vector2,
    velocity: Vector2,
}

impl Boid {
    fn random(min_x: f32, max_x: f32, min_y: f32, max_y: f32, size: Vector2) -> Self {
        let rand_x = random_range(min_x, max_x);
        let rand_y = random_range(min_y, max_y);
        Boid {
            position: vec2(rand_x, rand_y),
            size,
            velocity: vec2(random_f32(), random_f32()),
        }
    }
    fn init_boids(min_x: f32, max_x: f32, min_y: f32, max_y: f32, size: Vector2, n: u32) -> Vec<Boid> {
        let mut boids = Vec::new();
        for _i in 0..n {
            let boid = Boid::random(min_x, max_x, min_y, max_y, size);
            boids.push(boid);
        }
        return boids;
    }
    fn centering(boid: &Boid, neighbours: &Vec<Boid>) -> Vector2 {
        let mut c = vec2(0.0, 0.0);
        for n in neighbours {
            c += n.position;
        }
        c /= neighbours.len() as f32;
        c -= boid.position;
        c.with_magnitude(1.0)
    }
    fn separating(boid: &mut Boid, neighbours: &Vec<Boid>) -> Vector2 {
        let mut v = vec2(0.0, 0.0);
        for n in neighbours {
            if boid.position.distance(n.position) < 35.0 {
                v += boid.position - n.position;
            }
        }
        v.with_magnitude(4.0)
    }
    fn aligning(boid: &mut Boid, neighbours: &Vec<Boid>) -> Vector2 {
        let mut v = vec2(0.0, 0.0);
        for n in neighbours {
            v += n.velocity;
        }
        v /= neighbours.len() as f32;
        v -= boid.velocity;
        v.with_magnitude(1.0)
    }
    fn bounding(boid: &mut Boid, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        if boid.position.x < min_x {
            boid.position.x = max_x;
        } else if boid.position.x > max_x {
            boid.position.x = min_x;
        }
        if boid.position.y < min_y {
            boid.position.y = max_y;
        } else if boid.position.y > max_y {
            boid.position.y = min_y;
        }
    }
    fn limiting(boid: &mut Boid, max_velocity: f32) {
        boid.velocity = boid.velocity.limit_magnitude(max_velocity);
    }
    fn moving(boid: &mut Boid) {
        boid.position = boid.position + boid.velocity;
    }
    fn extract_neightbours(boid: &Boid, boids: &Vec<Boid>, max_dist: f32) -> Vec<Boid> {
        let mut neighbours = Vec::new();
        for potential_neighbour in boids.iter() {
            if boid != potential_neighbour {
                let dist = boid.position.distance(potential_neighbour.position);
                if dist < max_dist {
                    neighbours.push(potential_neighbour.clone());
                }
            }
        }
        neighbours
    }
}

struct Model {
    w_id: window::Id,
    boids: Vec<Boid>,
    max_dist: f32,
    max_velocity: f32,
}

fn model(app: &App) -> Model {
    let w_id = app.new_window().view(view).build().unwrap();
    let window = app.window(w_id).unwrap();
    let (win_w, win_h) = window.inner_size_points();
    let min_x = -win_w/2.0;
    let max_x = win_w/2.0;
    let min_y = -win_h/2.0;
    let max_y = win_h/2.0;
    let n: u32 = 40;
    let boids = Boid::init_boids(min_x, max_x, min_y, max_y, vec2(10.0, 10.0), n);
    let max_dist = 50.0;
    let max_velocity = 10.0;
    Model {
        w_id,
        boids,
        max_dist,
        max_velocity,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window = app.window(model.w_id).unwrap();
    let (win_w, win_h) = window.inner_size_points();
    let min_x = -win_w/2.0;
    let max_x = win_w/2.0;
    let min_y = -win_h/2.0;
    let max_y = win_h/2.0;
    let clones = model.boids.to_vec();
    for boid in model.boids.iter_mut() {
        let neighbours = Boid::extract_neightbours(boid, &clones, model.max_dist);
        if neighbours.len() > 0 {
            let v1 = Boid::centering(boid, &neighbours);
            let v2 = Boid::aligning(boid, &neighbours);
            let v3 = Boid::separating(boid, &neighbours);
            boid.velocity = boid.velocity + v1 + v2 + v3;
        }
        Boid::limiting(boid, model.max_velocity);
        Boid::bounding(boid, min_x, max_x, min_y, max_y);
        Boid::moving(boid);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for boid in model.boids.iter() {
        draw.ellipse().xy(boid.position).wh(boid.size).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}
