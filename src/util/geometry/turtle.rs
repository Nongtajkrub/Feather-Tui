use crate::util::geometry::Coordinate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurtleAction {
    DrawLine((Coordinate, Coordinate), (Coordinate, Coordinate)),
    SetPen(char),
}

pub struct Turtle {
    x: Coordinate,
    y: Coordinate,
    radians: f32,
    pen_down: bool,
    actions: Vec<TurtleAction>,
}

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            x: 0,
            y: 0,
            radians: 0f32,
            pen_down: true,
            actions: Vec::new(),
        }
    }

    pub fn with_position(x: Coordinate, y: Coordinate) -> Self {
        Turtle {
            x,
            y,
            radians: 0f32,
            pen_down: true,
            actions: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.pen_down = true;
        self.actions.clear();
    }

    #[inline]
    pub fn set_pen(&mut self, c: char) {
        self.actions.push(TurtleAction::SetPen(c));
    }

    #[inline]
    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    #[inline]
    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    #[inline]
    pub fn right(&mut self, degree: u16) {
        self.radians = (self.radians + (degree as f32).to_radians())
            .rem_euclid(std::f32::consts::TAU);
    }

    #[inline]
    pub fn left(&mut self, degree: u16) {
        self.radians = (self.radians - (degree as f32).to_radians())
            .rem_euclid(std::f32::consts::TAU);
    }

    #[inline]
    fn calc_vector(&self, size: f32) -> (Coordinate, Coordinate) {
        let vec_x = (self.radians.cos() * size).round();
        let vec_y = (self.radians.sin() * size).round();
        (vec_x as Coordinate, vec_y as Coordinate)
    }

    #[inline]
    pub fn forward(&mut self, n: u16) {
        let (vec_x, vec_y) = self.calc_vector(n as f32);
        self.goto(self.x + vec_x, self.y + vec_y);
    }

    #[inline]
    pub fn backward(&mut self, n: u16) {
        let (vec_x, vec_y) = self.calc_vector(n as f32);
        self.goto(self.x - vec_x, self.y - vec_y);
    }

    pub fn goto(&mut self, x: Coordinate, y: Coordinate) {
        if self.pen_down {
            self.actions.push(TurtleAction::DrawLine((self.x, self.y), (x, y)));
        }

        self.x = x;
        self.y = y;
    }

    #[inline]
    pub(crate) fn actions(&self) -> &[TurtleAction] {
        &self.actions
    }
}
