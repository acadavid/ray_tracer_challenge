use util::*;
mod util;

#[derive(Debug, Clone, Copy)]
struct Projectile {
    pos: Tuple,
    vel: Tuple,
}

#[derive(Debug, Clone, Copy)]
struct Environment {
    grav: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.pos + proj.vel;
    let velocity = proj.vel + env.grav + env.wind;

    return Projectile { pos: position, vel: velocity }
}

fn main() {
    let mut p = Projectile {
        pos: build_point(0.0, 1.1, 0.0),
        vel: build_vector(1.0, 1.0, 0.0).normalize(),
    };

    let e = Environment {
        grav: build_vector(0.0, -0.1, 0.0),
        wind: build_vector(-0.01, 0.0, 0.0),
    };

    let mut ticks = 0.0;
    loop {
        p = tick(e, p);
        ticks += 1.0;
        println!("Ticks: {}, Project.Y position: {}", ticks, p.pos.y);

        if p.pos.y <= 0.0 {
            break;
        }
    }

}
