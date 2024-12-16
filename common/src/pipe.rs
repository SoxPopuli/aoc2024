pub trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }

    fn pipe_ref<T>(self, f: impl FnOnce(&Self) -> T) -> T {
        f(&self)
    }

    fn pipe_mut<T>(mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        f(&mut self)
    }
}

impl<T> Pipe for T {}

pub trait Tap: Sized {
    fn tap(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }

    fn tap_ref(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }

    fn tap_mut(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}
impl<T> Tap for T {}
