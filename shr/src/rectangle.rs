use cgm::{num_traits::NumCast, BaseNum, Point2, Vector2};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rectangle<T> {
    pub a: Vector2<T>,
    pub b: Vector2<T>,
}

impl<T> Rectangle<T> {
    pub fn new<A, B>(a: A, b: B) -> Self
    where
        A: Into<Vector2<T>>,
        B: Into<Vector2<T>>,
    {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn left(self) -> T {
        self.a.x
    }

    pub fn right(self) -> T {
        self.b.x
    }

    pub fn bot(self) -> T {
        self.a.y
    }

    pub fn top(self) -> T {
        self.b.y
    }
}

impl<T: Copy> Rectangle<T> {
    pub fn cast<U>(self) -> Option<Rectangle<U>>
    where
        T: NumCast,
        U: NumCast,
    {
        Some(Rectangle::new(self.a.cast()?, self.b.cast()?))
    }

    pub fn left_top(self) -> Vector2<T> {
        Vector2::new(self.left(), self.top())
    }

    pub fn right_bot(self) -> Vector2<T> {
        Vector2::new(self.right(), self.bot())
    }

    pub fn rect_points(self) -> [Vector2<T>; 4] {
        [self.a, self.right_bot(), self.b, self.left_top()]
    }
}

impl<T: BaseNum> Rectangle<T> {
    pub fn size(self) -> Vector2<T> {
        self.b - self.a
    }

    pub fn center(self) -> Vector2<T> {
        self.a + self.size() / T::from(2).unwrap()
    }

    pub fn intersects_point(self, pnt: Point2<T>) -> bool {
        self.left() <= pnt.x && pnt.x <= self.right() && self.top() <= pnt.y && pnt.y <= self.bot()
    }
}

impl<T, A: Into<Vector2<T>>, B: Into<Vector2<T>>> From<(A, B)> for Rectangle<T> {
    fn from((a, b): (A, B)) -> Self {
        Self::new(a, b)
    }
}
