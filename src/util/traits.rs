use crate::error::FtuiResult;

pub trait Renderable<T> {
    fn render(&self, surface: &mut T) -> FtuiResult<()>;
}

pub trait RenderableMut<T> {
    fn render(&mut self, surface: &mut T) -> FtuiResult<()>;
}
