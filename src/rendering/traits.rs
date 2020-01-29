macro_rules! create_impl {
    ($field:ident, $ty:ty) => {
        #[macro_export]
        macro_rules! $field {
            ($shape:ident) => {
                impl<'s> $shape<'s> {
                    pub fn $field(mut self, $field: $ty) -> Self {
                        self.$field = $field;
                        self
                    }
                }
            };
        }
    };
}

#[macro_export]
macro_rules! transform {
    ($shape:ident) => {
        impl<'s> $shape<'s> {
            pub fn transform(mut self, transform: Transform) -> Self {
                self.transform = transform;
                self
            }

            pub fn parent(mut self, parent: Transform) -> Self {
                self.parent = parent;
                self
            }

            pub fn position(mut self, position: Vec2<f32>) -> Self {
                self.transform.position = position;
                self
            }

            pub fn size(mut self, size: Vec2<f32>) -> Self {
                self.transform.size = size;
                self
            }
            
            pub fn scale(mut self, scale: f32) -> Self {
                self.transform.size = self.transform.size.normalize() * scale;
                self
            }

            pub fn rotation(mut self, rotation: f32) -> Self {
                self.transform.rotation = rotation;
                self
            }
        }
    }
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
