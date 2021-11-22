#![allow(dead_code)]

mod vector2i;
mod vector3i;
mod vector4i;

mod vector2f;
mod vector3f;
mod vector4f;

#[cfg(test)]
mod tests {
    use crate::vector2i::Vector2i;

    #[test]
    fn impl_ops() {
        let a = Vector2i::from(1, 2);
        let b = Vector2i::from(2, 3);
        let c = a + b;

        // Maintains old value
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);
        assert_eq!(b.x, 2);
        assert_eq!(b.y, 3);

        // Has new value for C
        assert_eq!(c.x, 3);
        assert_eq!(c.y, 5);
    }
}
