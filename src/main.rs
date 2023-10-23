use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window: Window = Window::new_centered("Pendulum", (800, 400)).unwrap();

    let win: MyWindowHandler = MyWindowHandler {
        pendulum: Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    pendulum: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.pendulum.update();
        self.pendulum.draw(graphics);

        helper.request_redraw();
    }
}

struct Pendulum {
    //This vector is the position of the pendulum
    origin: vector::Vector,

    //This vector is the position of the ball
    position: vector::Vector,

    //This is the angle of the pendulum
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, //The length of the pendulum
    g: f32, //The gravity
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: vector::Vector::new(x, y),
            position: vector::Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r: r,
            g: 1.5,
        }
    }

    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular acceleration
        self.angular_velocity += self.angular_acceleration;

        //The angle is the angle plus the angular velocity
        self.angle += self.angular_velocity;

        //The position is the polar coordinates translated to cartesian coordinates
        self.position.set(vector::Vector::new(
            self.r * self.angle.sin(),
            self.r * self.angle.cos(),
        ));

        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other_vector: &Vector) -> &Vector {
            self.x += other_vector.x;
            self.y += other_vector.y;
            self
        }

        pub fn set(&mut self, other_vector: Vector) -> &Vector {
            self.x = other_vector.x;
            self.y = other_vector.y;
            self
        }
    }
}
