macro_rules! impl_serde_vec2 {
    ($t:ty, $vec2:ident) => {
        impl nanoserde::DeJson for $vec2 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let x: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let y: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.block_close(i)?;
                    $vec2 { x, y }
                })
            }
        }

        impl nanoserde::SerJson for $vec2 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                s.out.push('[');
                self.x.ser_json(d, s);
                s.out.push(',');
                self.y.ser_json(d, s);
                s.out.push(']');
            }
        }

        #[test]
        fn test_vec2_serde() {
            let a = $vec2::new(V1, V2);
            let serialized = a.serialize_json();
            assert_eq!(SX2, serialized);
            let deserialized = $vec2::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $vec2::deserialize_json(SX0);
            assert!(deserialized.is_err());
            let deserialized = $vec2::deserialize_json(SX1);
            assert!(deserialized.is_err());
            let deserialized = $vec2::deserialize_json(SX3);
            assert!(deserialized.is_err());

            let deserialized = $vec2::deserialize_json(ST0);
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_vec3 {
    ($t:ty, $vec3:ident) => {
        impl_serde_vec3!($t, $vec3, test_vec3_serde);
    };
    ($t:ty, $vec3:ident, $test_name:ident) => {
        impl nanoserde::DeJson for $vec3 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let x: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let y: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let z: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.block_close(i)?;
                    $vec3::new(x, y, z)
                })
            }
        }

        impl nanoserde::SerJson for $vec3 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                s.out.push('[');
                self.x.ser_json(d, s);
                s.out.push(',');
                self.y.ser_json(d, s);
                s.out.push(',');
                self.z.ser_json(d, s);
                s.out.push(']');
            }
        }

        #[test]
        fn $test_name() {
            let a = $vec3::new(V1, V2, V3);
            let serialized = a.serialize_json();
            assert_eq!(SX3, serialized);
            let deserialized = $vec3::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $vec3::deserialize_json(SX0);
            assert!(deserialized.is_err());
            let deserialized = $vec3::deserialize_json(SX1);
            assert!(deserialized.is_err());
            let deserialized = $vec3::deserialize_json(SX2);
            assert!(deserialized.is_err());
            let deserialized = $vec3::deserialize_json(SX4);
            assert!(deserialized.is_err());
            let deserialized = $vec3::deserialize_json(ST0);
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_vec4 {
    ($t:ty, $vec4:ident) => {
        impl nanoserde::DeJson for $vec4 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let x: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let y: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let z: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let w: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.block_close(i)?;
                    $vec4::new(x, y, z, w)
                })
            }
        }

        impl nanoserde::SerJson for $vec4 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                s.out.push('[');
                self.x.ser_json(d, s);
                s.out.push(',');
                self.y.ser_json(d, s);
                s.out.push(',');
                self.z.ser_json(d, s);
                s.out.push(',');
                self.w.ser_json(d, s);
                s.out.push(']');
            }
        }

        #[test]
        fn test_vec4_serde() {
            let a = $vec4::new(V1, V2, V3, V4);
            let serialized = a.serialize_json();
            assert_eq!(SX4, serialized);
            let deserialized = $vec4::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $vec4::deserialize_json(SX0);
            assert!(deserialized.is_err());
            let deserialized = $vec4::deserialize_json(SX1);
            assert!(deserialized.is_err());
            let deserialized = $vec4::deserialize_json(SX2);
            assert!(deserialized.is_err());
            let deserialized = $vec4::deserialize_json(SX3);
            assert!(deserialized.is_err());
            let deserialized = $vec4::deserialize_json(SX5);
            assert!(deserialized.is_err());
            let deserialized = $vec4::deserialize_json(ST0);
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_quat {
    ($t:ty, $quat:ident) => {
        impl nanoserde::DeJson for $quat {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let x: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let y: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let z: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let w: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.block_close(i)?;
                    $quat::from_xyzw(x, y, z, w)
                })
            }
        }

        impl nanoserde::SerJson for $quat {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                s.out.push('[');
                self.x.ser_json(d, s);
                s.out.push(',');
                self.y.ser_json(d, s);
                s.out.push(',');
                self.z.ser_json(d, s);
                s.out.push(',');
                self.w.ser_json(d, s);
                s.out.push(']');
            }
        }

        #[test]
        fn test_quat_serde() {
            let a = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            let serialized = a.serialize_json();
            assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
            let deserialized = $quat::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $quat::deserialize_json("[]");
            assert!(deserialized.is_err());
            let deserialized = $quat::deserialize_json("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = $quat::deserialize_json("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = $quat::deserialize_json("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = $quat::deserialize_json("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized = $quat::deserialize_json("{}");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_mat2 {
    ($t:ty, $mat2:ident) => {
        impl nanoserde::DeJson for $mat2 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let m00: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let m01: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let m10: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let m11: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.block_close(i)?;
                    $mat2::from_cols_array(&[m00, m01, m10, m11])
                })
            }
        }

        impl nanoserde::SerJson for $mat2 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                let cols: [$t; 4] = self.to_cols_array();
                s.out.push('[');
                cols[0].ser_json(d, s);
                s.out.push(',');
                cols[1].ser_json(d, s);
                s.out.push(',');
                cols[2].ser_json(d, s);
                s.out.push(',');
                cols[3].ser_json(d, s);
                s.out.push(']');
            }
        }

        #[test]
        fn test_mat2_serde() {
            let a = $mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]);
            let serialized = a.serialize_json();
            assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
            let deserialized = $mat2::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $mat2::deserialize_json("[]");
            assert!(deserialized.is_err());
            let deserialized = $mat2::deserialize_json("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat2::deserialize_json("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat2::deserialize_json("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat2::deserialize_json("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat2::deserialize_json("[[1.0,2.0],[3.0,4.0]]");
            assert!(deserialized.is_err());
            let deserialized = $mat2::deserialize_json("{}");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_mat3 {
    ($t:ty, $mat3:ident) => {
        impl_serde_mat3!($t, $mat3, test_mat3_serde);
    };
    ($t:ty, $mat3:ident, $test_name:ident) => {
        impl nanoserde::DeJson for $mat3 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let v1: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v2: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v3: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v4: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v5: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v6: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v7: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v8: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.eat_comma_block(i)?;
                    let v9: $t = nanoserde::DeJson::de_json(s, i)?;
                    s.block_close(i)?;
                    $mat3::from_cols_array(&[v1, v2, v3, v4, v5, v6, v7, v8, v9])
                })
            }
        }

        impl nanoserde::SerJson for $mat3 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                let cols: [$t; 9] = self.to_cols_array();
                s.out.push('[');
                for i in 0..9 {
                    cols[i].ser_json(d, s);
                    if i != 8 {
                        s.out.push(',');
                    }
                }
                s.out.push(']');
            }
        }

        #[test]
        fn $test_name() {
            let a = $mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            let serialized = a.serialize_json();
            assert_eq!(serialized, "[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0]");
            let deserialized = $mat3::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $mat3::deserialize_json("[]");
            assert!(deserialized.is_err());
            let deserialized = $mat3::deserialize_json("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat3::deserialize_json("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat3::deserialize_json("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat3::deserialize_json("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized =
                $mat3::deserialize_json("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
            assert!(deserialized.is_err());
            let deserialized = $mat3::deserialize_json("{}");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_mat4 {
    ($t:ty, $mat4:ident) => {
        impl nanoserde::DeJson for $mat4 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let mut vals: [$t; 16] = [0.0; 16];
                    for idx in 0..16 {
                        vals[idx] = nanoserde::DeJson::de_json(s, i)?;
                        if idx != 15 {
                            s.eat_comma_block(i)?;
                        }
                    }
                    $mat4::from_cols_array(&vals)
                })
            }
        }

        impl nanoserde::SerJson for $mat4 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                let cols: [$t; 16] = self.to_cols_array();
                s.out.push('[');
                for i in 0..16 {
                    cols[i].ser_json(d, s);
                    if i != 15 {
                        s.out.push(',');
                    }
                }
                s.out.push(']');
            }
        }

        #[test]
        fn test_mat4_serde() {
            let a = $mat4::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ]);
            let serialized = a.serialize_json();
            assert_eq!(
                serialized,
                "[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0]"
            );
            let deserialized = $mat4::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = $mat4::deserialize_json("[]");
            assert!(deserialized.is_err());
            let deserialized = $mat4::deserialize_json("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat4::deserialize_json("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat4::deserialize_json("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = $mat4::deserialize_json("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized =
                $mat4::deserialize_json("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
            assert!(deserialized.is_err());
            let deserialized = $mat4::deserialize_json(
                "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
            );
            assert!(deserialized.is_err());
            let deserialized = $mat4::deserialize_json("{}");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_affine2 {
    ($t:ty, $affine2:ident) => {
        impl nanoserde::DeJson for $affine2 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let mut vals: [$t; 6] = [0.0; 6];
                    for idx in 0..6 {
                        vals[idx] = nanoserde::DeJson::de_json(s, i)?;
                        if idx != 5 {
                            s.eat_comma_block(i)?;
                        }
                    }
                    $affine2::from_cols_array(&vals)
                })
            }
        }

        impl nanoserde::SerJson for $affine2 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                let cols: [$t; 6] = self.to_cols_array();
                s.out.push('[');
                for i in 0..6 {
                    cols[i].ser_json(d, s);
                    if i != 5 {
                        s.out.push(',');
                    }
                }
                s.out.push(']');
            }
        }

        #[test]
        fn test_affine2_serde() {
            let a = $affine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]);
            let serialized = a.serialize_json();
            assert_eq!(serialized, "[1.0,0.0,2.0,0.0,3.0,4.0]");
            let deserialized = $affine2::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);

            let deserialized = $affine2::deserialize_json("[]");
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json("[[1.0,2.0],[3.0,4.0],[5.0,6.0]]");
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json(
                "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
            );
            assert!(deserialized.is_err());
            let deserialized = $affine2::deserialize_json("{}");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_affine3 {
    ($testname:ident, $t:ty, $affine3:ident) => {
        /// Serialize as a sequence of 12 values.
        impl nanoserde::DeJson for $affine3 {
            fn de_json(
                s: &mut nanoserde::DeJsonState,
                i: &mut core::str::Chars,
            ) -> Result<Self, nanoserde::DeJsonErr> {
                Ok({
                    s.block_open(i)?;
                    let mut vals: [$t; 12] = [0.0; 12];
                    for idx in 0..12 {
                        vals[idx] = nanoserde::DeJson::de_json(s, i)?;
                        if idx != 11 {
                            s.eat_comma_block(i)?;
                        }
                    }
                    $affine3::from_cols_array(&vals)
                })
            }
        }

        impl nanoserde::SerJson for $affine3 {
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
                let cols: [$t; 12] = self.to_cols_array();
                s.out.push('[');
                for i in 0..12 {
                    cols[i].ser_json(d, s);
                    if i != 11 {
                        s.out.push(',');
                    }
                }
                s.out.push(']');
            }
        }

        #[test]
        fn $testname() {
            let a = $affine3::from_cols_array(&[
                1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
            ]);
            let serialized = a.serialize_json();
            assert_eq!(
                serialized,
                "[1.0,0.0,0.0,0.0,2.0,0.0,0.0,0.0,3.0,4.0,5.0,6.0]"
            );
            let deserialized = $affine3::deserialize_json(&serialized).unwrap();
            assert_eq!(a, deserialized);

            let deserialized = $affine3::deserialize_json("[]");
            assert!(deserialized.is_err());
            let deserialized = $affine3::deserialize_json("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine3::deserialize_json("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine3::deserialize_json("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = $affine3::deserialize_json("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized =
                $affine3::deserialize_json("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
            assert!(deserialized.is_err());
            let deserialized = $affine3::deserialize_json(
                "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
            );
            assert!(deserialized.is_err());
            let deserialized = $affine3::deserialize_json("{}");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_vec_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_vec2!($t, $vec2);
        impl_serde_vec3!($t, $vec3);
        impl_serde_vec4!($t, $vec4);
    };
}

macro_rules! impl_serde_float_types {
    ($t:ty, $affine2:ident, $affine3:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_affine2!($t, $affine2);
        impl_serde_affine3!(test_affine3_serde, $t, $affine3);
        impl_serde_mat2!($t, $mat2);
        impl_serde_mat3!($t, $mat3);
        impl_serde_mat4!($t, $mat4);
        impl_serde_quat!($t, $quat);
        impl_serde_vec_types!($t, $vec2, $vec3, $vec4);
    };
}

#[cfg(test)]
mod test_f32 {
    pub const V1: f32 = 1.0;
    pub const V2: f32 = 2.0;
    pub const V3: f32 = 3.0;
    pub const V4: f32 = 4.0;
}

#[cfg(test)]
mod test_f64 {
    pub const V1: f64 = 1.0;
    pub const V2: f64 = 2.0;
    pub const V3: f64 = 3.0;
    pub const V4: f64 = 4.0;
}

#[cfg(test)]
mod test_i8 {
    pub const V1: i8 = 1;
    pub const V2: i8 = 2;
    pub const V3: i8 = 3;
    pub const V4: i8 = 4;
}

#[cfg(test)]
mod test_i16 {
    pub const V1: i16 = 1;
    pub const V2: i16 = 2;
    pub const V3: i16 = 3;
    pub const V4: i16 = 4;
}

#[cfg(test)]
mod test_i32 {
    pub const V1: i32 = 1;
    pub const V2: i32 = 2;
    pub const V3: i32 = 3;
    pub const V4: i32 = 4;
}

#[cfg(test)]
mod test_i64 {
    pub const V1: i64 = 1;
    pub const V2: i64 = 2;
    pub const V3: i64 = 3;
    pub const V4: i64 = 4;
}

#[cfg(test)]
mod test_u8 {
    pub const V1: u8 = 1;
    pub const V2: u8 = 2;
    pub const V3: u8 = 3;
    pub const V4: u8 = 4;
}

#[cfg(test)]
mod test_u16 {
    pub const V1: u16 = 1;
    pub const V2: u16 = 2;
    pub const V3: u16 = 3;
    pub const V4: u16 = 4;
}

#[cfg(test)]
mod test_u32 {
    pub const V1: u32 = 1;
    pub const V2: u32 = 2;
    pub const V3: u32 = 3;
    pub const V4: u32 = 4;
}

#[cfg(test)]
mod test_u64 {
    pub const V1: u64 = 1;
    pub const V2: u64 = 2;
    pub const V3: u64 = 3;
    pub const V4: u64 = 4;
}

#[cfg(test)]
mod test_usize {
    pub const V1: usize = 1;
    pub const V2: usize = 2;
    pub const V3: usize = 3;
    pub const V4: usize = 4;
}

#[cfg(test)]
mod test_float {
    pub const SX0: &str = "[]";
    pub const SX1: &str = "[1.0]";
    pub const SX2: &str = "[1.0,2.0]";
    pub const SX3: &str = "[1.0,2.0,3.0]";
    pub const SX4: &str = "[1.0,2.0,3.0,4.0]";
    pub const SX5: &str = "[1.0,2.0,3.0,4.0,5.0]";
    pub const ST0: &str = "{}";
}

#[cfg(test)]
mod test_int {
    pub const SX0: &str = "[]";
    pub const SX1: &str = "[1]";
    pub const SX2: &str = "[1,2]";
    pub const SX3: &str = "[1,2,3]";
    pub const SX4: &str = "[1,2,3,4]";
    pub const SX5: &str = "[1,2,3,4,5]";
    pub const ST0: &str = "{}";
}

#[cfg(test)]
mod test_bool_mask {
    pub const SX0: &str = "[]";
    pub const SX1: &str = "[true]";
    pub const SX2: &str = "[true,true]";
    pub const SX3: &str = "[true,true,true]";
    pub const SX4: &str = "[true,true,true,true]";
    pub const SX5: &str = "[true,true,true,true,true]";
    pub const ST0: &str = "{}";
    pub const V1: bool = true;
    pub const V2: bool = true;
    pub const V3: bool = true;
    pub const V4: bool = true;
}

mod f32 {
    #[cfg(test)]
    use super::test_f32::*;
    #[cfg(test)]
    use super::test_float::*;
    use crate::{
        Affine2, Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4,
    };
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_float_types!(f32, Affine2, Affine3, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);
    impl_serde_affine3!(test_affine3a_serde, f32, Affine3A);
    impl_serde_mat3!(f32, Mat3A, test_mat3a_serde);
    impl_serde_vec3!(f32, Vec3A, test_vec3a_serde);
}

mod f64 {
    #[cfg(test)]
    use super::test_f64::*;
    #[cfg(test)]
    use super::test_float::*;
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_float_types!(
        f64, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4
    );
}

mod i8 {
    #[cfg(test)]
    use super::test_i8::*;
    #[cfg(test)]
    use super::test_int::*;
    use crate::{I8Vec2, I8Vec3, I8Vec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(i8, I8Vec2, I8Vec3, I8Vec4);
}

mod i16 {
    #[cfg(test)]
    use super::test_i16::*;
    #[cfg(test)]
    use super::test_int::*;
    use crate::{I16Vec2, I16Vec3, I16Vec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(i16, I16Vec2, I16Vec3, I16Vec4);
}

mod i32 {
    #[cfg(test)]
    use super::test_i32::*;
    #[cfg(test)]
    use super::test_int::*;
    use crate::{IVec2, IVec3, IVec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(i32, IVec2, IVec3, IVec4);
}

mod i64 {
    #[cfg(test)]
    use super::test_i64::*;
    #[cfg(test)]
    use super::test_int::*;
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(i64, I64Vec2, I64Vec3, I64Vec4);
}

mod u8 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u8::*;
    use crate::{U8Vec2, U8Vec3, U8Vec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(u8, U8Vec2, U8Vec3, U8Vec4);
}

mod u16 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u16::*;
    use crate::{U16Vec2, U16Vec3, U16Vec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(u16, U16Vec2, U16Vec3, U16Vec4);
}

mod u32 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u32::*;
    use crate::{UVec2, UVec3, UVec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(u32, UVec2, UVec3, UVec4);
}

mod u64 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u64::*;
    use crate::{U64Vec2, U64Vec3, U64Vec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(u64, U64Vec2, U64Vec3, U64Vec4);
}

mod usize {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_usize::*;
    use crate::{USizeVec2, USizeVec3, USizeVec4};
    use core::fmt;
    use nanoserde::{DeJson, SerJson};

    impl_serde_vec_types!(usize, USizeVec2, USizeVec3, USizeVec4);
}
