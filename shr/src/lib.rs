mod rectangle;

pub mod shapes {
    pub use crate::rectangle::Rectangle;

    pub type Rect = Rectangle<f32>;
    pub type IRect = Rectangle<i32>;
    pub type URect = Rectangle<u32>;
}

pub mod cgm {
    pub use cgm as cgmath;
    pub use cgm::prelude::*;

    pub type Vec2 = cgm::Vector2<f32>;
    pub type Vec3 = cgm::Vector3<f32>;
    pub type Vec4 = cgm::Vector4<f32>;

    pub type IVec2 = cgm::Vector2<i32>;
    pub type IVec3 = cgm::Vector3<i32>;
    pub type IVec4 = cgm::Vector4<i32>;

    pub type UVec2 = cgm::Vector2<u32>;
    pub type UVec3 = cgm::Vector3<u32>;
    pub type UVec4 = cgm::Vector4<u32>;

    pub type Mat2 = cgm::Matrix2<f32>;
    pub type Mat3 = cgm::Matrix3<f32>;
    pub type Mat4 = cgm::Matrix4<f32>;

    pub type Pnt2 = cgm::Point2<f32>;
    pub type Pnt3 = cgm::Point3<f32>;

    pub type Quat = cgm::Quaternion<f32>;

    pub type Rad = cgm::Rad<f32>;

    pub trait IntoRad {
        fn rad(self) -> Rad;
    }

    impl IntoRad for f32 {
        fn rad(self) -> Rad {
            cgmath::Rad(self)
        }
    }
}
