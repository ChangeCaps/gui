macro_rules! create_impl {
    ($field:ident, $ty:ty) => {
        #[macro_export]
        macro_rules! $field {
            ($shape:ident) => {
                impl<'s, 'f> $shape<'s, 'f> {
                    pub fn $field(mut self, $field: $ty) -> Self {
                        self.$field = $field;
                        self
                    }
                }
            };
        }
    };
}

create_impl!(position, Vec2<f32>);
create_impl!(size, Vec2<f32>);
create_impl!(rotation, f32);
create_impl!(scaling, bool);
create_impl!(color, [f32; 4]);
create_impl!(anchor, Anchor);
create_impl!(pivot, Anchor);
create_impl!(smooth, bool);
create_impl!(width, f32);
create_impl!(scale, f32);
create_impl!(depth, f32);