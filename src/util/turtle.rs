use crate::util::Coordinate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurtleAction {
    DrawLine((Coordinate, Coordinate), (Coordinate, Coordinate)),
    SetPen(char),
}

pub struct Turtle {
    x: Coordinate,
    y: Coordinate,
    pen_down: bool,
    actions: Vec<TurtleAction>,
}

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            x: 0,
            y: 0,
            pen_down: true,
            actions: Vec::new(),
        }
    }

    pub fn with_position(x: Coordinate, y: Coordinate) -> Self {
        Turtle {
            x,
            y,
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

    pub fn set_pen(&mut self, c: char) {
        self.actions.push(TurtleAction::SetPen(c));
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn goto(&mut self, x: Coordinate, y: Coordinate) {
        if self.pen_down {
            self.actions.push(TurtleAction::DrawLine((self.x, self.y), (x, y)));
        }

        self.x = x;
        self.y = y;
    }

    pub(crate) fn actions(&self) -> &[TurtleAction] {
        &self.actions
    }
}
