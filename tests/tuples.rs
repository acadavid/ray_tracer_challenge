#[cfg(test)]
mod tuples {
  use std::ops::{Add, Div, Mul, Neg, Sub};

  #[derive(Debug, PartialEq)]
  struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
  }

  impl Tuple {
    fn is_point(&self) -> bool {
      self.w == 1.0
    }

    fn is_vector(&self) -> bool {
      self.w == 0.0
    }

    fn to_array(&self) -> [f32; 4] {
      [self.x, self.y, self.z, self.w]
    }

    fn magnitude(&self) -> f32 {
      return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();
    }

    fn normalize(&self) -> Tuple {
      let m = self.magnitude();

      return build_vector(self.x / m, self.y / m, self.z / m);
    }
  }

  impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
      Self {
        x: self.x * -1.0,
        y: self.y * -1.0,
        z: self.z * -1.0,
        w: self.w * -1.0,
      }
    }
  }

  impl Add for Tuple {
    type Output = Self;

    fn add(self, t: Self) -> Self {
      Self {
        x: self.x + t.x,
        y: self.y + t.y,
        z: self.z + t.z,
        w: self.w + t.w,
      }
    }
  }

  impl Sub for Tuple {
    type Output = Self;

    fn sub(self, t: Self) -> Self {
      Self {
        x: self.x - t.x,
        y: self.y - t.y,
        z: self.z - t.z,
        w: self.w - t.w,
      }
    }
  }

  impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
      Self {
        x: self.x * s,
        y: self.y * s,
        z: self.z * s,
        w: self.w * s,
      }
    }
  }

  impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, s: f32) -> Self {
      Self {
        x: self.x / s,
        y: self.y / s,
        z: self.z / s,
        w: self.w / s,
      }
    }
  }

  fn build_tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple {
      x: x,
      y: y,
      z: z,
      w: w,
    }
  }

  fn build_point(x: f32, y: f32, z: f32) -> Tuple {
    build_tuple(x, y, z, 1.0)
  }

  fn build_vector(x: f32, y: f32, z: f32) -> Tuple {
    build_tuple(x, y, z, 0.0)
  }

  // Tests for build functions
  #[test]
  fn build_point_creates_point() {
    assert_eq!(build_point(4.3, -4.2, 3.1).w, 1.0)
  }

  #[test]
  fn build_point_creates_vector() {
    assert_eq!(
      build_vector(4.3, -4.2, 3.1).to_array(),
      [4.3, -4.2, 3.1, 0.0]
    );
  }

  #[test]
  fn tuple_with_w_one_is_a_point() {
    let a = build_tuple(4.3, -4.2, 3.1, 1.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert_eq!(a.is_point(), true);
    assert_eq!(a.is_vector(), false);
  }

  #[test]
  fn tuple_with_w_zero_is_a_vector() {
    let a = build_tuple(4.3, -4.2, 3.1, 0.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.w, 0.0);
    assert_eq!(a.is_point(), false);
    assert_eq!(a.is_vector(), true);
  }

  #[test]
  fn tuples_are_equal() {
    let a = build_tuple(4.3, -4.2, 3.1, 0.0);
    let b = build_tuple(4.3, -4.2, 3.1, 0.0);

    assert_eq!(a == b, true)
  }

  #[test]
  fn tuples_are_not_equal() {
    let a = build_tuple(4.3, -4.2, 3.1, 0.0);
    let b = build_tuple(4.3, -4.2001, 3.1, 0.0);

    assert_eq!(a == b, false)
  }

  #[test]
  fn add_point_and_vector_makes_point() {
    let a = build_point(3.0, -2.0, 5.0);
    let b = build_vector(-2.0, 3.0, 1.0);

    let res = a + b;

    assert_eq!(
      Tuple {
        x: 1.0,
        y: 1.0,
        z: 6.0,
        w: 1.0
      },
      res
    );
    assert_eq!(res.is_point(), true);
  }

  #[test]
  fn add_vector_and_vector_makes_vector() {
    let a = build_vector(3.0, -2.0, 5.0);
    let b = build_vector(2.0, 3.0, 1.0);

    let res = a + b;

    assert_eq!(
      Tuple {
        x: 5.0,
        y: 1.0,
        z: 6.0,
        w: 0.0
      },
      res
    );
    assert_eq!(res.is_vector(), true);
  }

  #[test]
  fn substract_point_and_point_makes_vector() {
    let a = build_point(3.0, 2.0, 1.0);
    let b = build_point(5.0, 6.0, 7.0);

    let res = a - b;

    assert_eq!(
      Tuple {
        x: -2.0,
        y: -4.0,
        z: -6.0,
        w: 0.0
      },
      res
    );
    assert_eq!(res.is_vector(), true);
  }

  #[test]
  fn substract_vector_from_point_makes_point() {
    let a = build_point(3.0, 2.0, 1.0);
    let b = build_vector(5.0, 6.0, 7.0);

    let res = a - b;

    assert_eq!(
      Tuple {
        x: -2.0,
        y: -4.0,
        z: -6.0,
        w: 1.0
      },
      res
    );
    assert_eq!(res.is_point(), true);
  }

  #[test]
  fn substract_vector_from_vector() {
    let a = build_vector(3.0, 2.0, 1.0);
    let b = build_vector(5.0, 6.0, 7.0);

    let res = a - b;

    assert_eq!(
      Tuple {
        x: -2.0,
        y: -4.0,
        z: -6.0,
        w: 0.0
      },
      res
    );
    assert_eq!(res.is_vector(), true);
  }

  #[test]
  fn multiply_tuple_by_scalar() {
    let a = build_tuple(1.0, -2.0, 3.0, -4.0);
    let b = 2.0;

    let res = a * b;

    assert_eq!(
      Tuple {
        x: 2.0,
        y: -4.0,
        z: 6.0,
        w: -8.0
      },
      res
    );
  }

  #[test]
  fn divide_tuple_by_scalar() {
    let a = build_tuple(6.0, -7.0, 8.0, -9.0);
    let b = 2.0;

    let res = a / b;

    assert_eq!(
      Tuple {
        x: 3.0,
        y: -3.5,
        z: 4.0,
        w: -4.5
      },
      res
    );
  }

  #[test]
  fn negate_tuple() {
    let a = build_tuple(3.0, 2.0, 1.0, 1.0);

    assert_eq!(
      Tuple {
        x: -3.0,
        y: -2.0,
        z: -1.0,
        w: -1.0
      },
      -a
    );
  }

  #[test]
  fn compute_vector_magnitud() {
    let v = build_vector(-1.0, -2.0, -3.0);

    let radicand = 14.0_f32;
    assert_eq!(v.magnitude(), radicand.sqrt())
  }

  #[test]
  fn compute_unit_vector_magnitud() {
    let v1 = build_vector(1.0, 0.0, 0.0);
    let v2 = build_vector(0.0, 1.0, 0.0);
    let v3 = build_vector(0.0, 0.0, 1.0);

    assert_eq!(v1.magnitude(), 1.0);
    assert_eq!(v2.magnitude(), 1.0);
    assert_eq!(v3.magnitude(), 1.0);
  }

  #[test]
  fn normalize_vectors() {
    let v = build_vector(4.0, 0.0, 0.0);
    let v2 = build_vector(1.0, 2.0, 3.0);

    let radicand = 14.0_f32;
    assert_eq!(
      v.normalize(),
      Tuple {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0
      }
    );
    assert_eq!(
      v2.normalize(),
      Tuple {
        x: 1.0 / radicand.sqrt(),
        y: 2.0 / radicand.sqrt(),
        z: 3.0 / radicand.sqrt(),
        w: 0.0
      }
    )
  }
}
