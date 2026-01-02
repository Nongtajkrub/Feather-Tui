use crate::error::FtuiResult;

pub trait Renderable<S> {
    fn render(&self, surface: &mut S) -> FtuiResult<()>;
}

pub trait RenderableMut<S> {
    fn render(&mut self, surface: &mut S) -> FtuiResult<()>;
}
