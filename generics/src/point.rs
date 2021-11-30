pub(crate) struct Point<T, U> {
    pub(crate) x: T,
    pub(crate) y: U,
}

impl<T, U> Point<T, U> {
    pub(crate) fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
