#[macro_use]
macro_rules! impl_immutable_op {
    ($target:ident, Neg) => {
        impl std::ops::Neg for $target {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self * -1.0
            }
        }
    };
    ($target:ident, Index+IndexMut) => {
        impl std::ops::Index<usize> for $target {
            type Output = f64;

            fn index(&self, idx: usize) -> &Self::Output {
                match idx {
                    0 => &self.0,
                    1 => &self.1,
                    2 => &self.2,
                    _ => panic!("Invalid dimension"),
                }
            }
        }

        impl std::ops::IndexMut<usize> for $target {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                match idx {
                    0 => &mut self.0,
                    1 => &mut self.1,
                    2 => &mut self.2,
                    _ => panic!("Invalid dimension"),
                }
            }
        }
    };
    ($target:ident, $trait_name:ident, $trait_fn:ident) => {
        impl $trait_name<Self> for $target {
            type Output = Self;

            fn $trait_fn(self, rhs: Self) -> Self::Output {
                Self::new(
                    $trait_name::$trait_fn(self.0, rhs.0),
                    $trait_name::$trait_fn(self.1, rhs.1),
                    $trait_name::$trait_fn(self.2, rhs.2),
                )
            }
        }

        impl $trait_name<f64> for $target {
            type Output = Self;

            fn $trait_fn(self, rhs: f64) -> Self::Output {
                Self::new(
                    $trait_name::$trait_fn(self.0, rhs),
                    $trait_name::$trait_fn(self.1, rhs),
                    $trait_name::$trait_fn(self.2, rhs),
                )
            }
        }

        impl $trait_name<$target> for f64 {
            type Output = $target;

            fn $trait_fn(self, rhs: $target) -> Self::Output {
                $target::new(
                    $trait_name::$trait_fn(self, rhs.0),
                    $trait_name::$trait_fn(self, rhs.1),
                    $trait_name::$trait_fn(self, rhs.2),
                )
            }
        }
    };
}

#[macro_use]
macro_rules! impl_assign_op {
    ($target:ident, $trait_name:ident, $trait_fn:ident) => {
        impl $trait_name<Self> for $target {
            fn $trait_fn(&mut self, rhs: Self) {
                $trait_name::$trait_fn(&mut self.0, rhs.0);
                $trait_name::$trait_fn(&mut self.1, rhs.1);
                $trait_name::$trait_fn(&mut self.2, rhs.2);
            }
        }
        impl $trait_name<f64> for $target {
            fn $trait_fn(&mut self, rhs: f64) {
                $trait_name::$trait_fn(&mut self.0, rhs);
                $trait_name::$trait_fn(&mut self.1, rhs);
                $trait_name::$trait_fn(&mut self.2, rhs);
            }
        }
    };
}
