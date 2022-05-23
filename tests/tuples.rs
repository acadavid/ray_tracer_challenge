#[cfg(test)]
mod tuples {
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

    fn is_equal_to(&self, t: Tuple) -> bool {
      if (self.x - t.x).abs() < f32::EPSILON && (self.y - t.y).abs() < f32::EPSILON && (self.z - t.z).abs() < f32::EPSILON && (self.w - t.w).abs() < f32::EPSILON {
        true
      } else {
        false
      }
    }

    fn add(&self, t: Tuple) -> Tuple {
      Tuple {
        x: self.x + t.x,
        y: self.y + t.y,
        z: self.z + t.z,
        w: self.w + t.w
      }
    }

    fn substract(&self, t: Tuple) -> Tuple {
      Tuple {
        x: self.x - t.x,
        y: self.y - t.y,
        z: self.z - t.z,
        w: self.w - t.w
      }
    }
  }

  impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
      Self {
        x: self.x * -1.0,
        y: self.y * -1.0,
        z: self.z * -1.0,
        w: self.w * -1.0
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
  fn tuple_with_w_zero_us_a_vector() {
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

    assert_eq!(a.is_equal_to(b), true)
  }

  #[test]
  fn tuples_are_not_equal() {
    let a = build_tuple(4.3, -4.2, 3.1, 0.0);
    let b = build_tuple(4.3, -4.2001, 3.1, 0.0);

    assert_eq!(a.is_equal_to(b), false)
  }

  #[test]
  fn add_point_and_vector_makes_point() {
    let a = build_point(3.0, -2.0, 5.0);
    let b = build_vector(-2.0, 3.0, 1.0);

    let res = a.add(b);

    assert_eq!(res.x, 1.0);
    assert_eq!(res.y, 1.0);
    assert_eq!(res.z, 6.0);
    assert_eq!(res.is_point(), true);
  }

  #[test]
  fn add_vector_and_vector_makes_vector() {
    let a = build_vector(3.0, -2.0, 5.0);
    let b = build_vector(2.0, 3.0, 1.0);

    let res = a.add(b);

    assert_eq!(res.x, 5.0);
    assert_eq!(res.y, 1.0);
    assert_eq!(res.z, 6.0);
    assert_eq!(res.is_vector(), true);
  }

  #[test]
  fn substract_point_and_point_makes_vector() {
    let a = build_point(3.0, 2.0, 1.0);
    let b = build_point(5.0, 6.0, 7.0);

    let res = a.substract(b);

    assert_eq!(res.x, -2.0);
    assert_eq!(res.y, -4.0);
    assert_eq!(res.z, -6.0);
    assert_eq!(res.is_vector(), true);
  }

  #[test]
  fn substract_vector_from_point_makes_point() {
    let a = build_point(3.0, 2.0, 1.0);
    let b = build_vector(5.0, 6.0, 7.0);

    let res = a.substract(b);

    assert_eq!(res.x, -2.0);
    assert_eq!(res.y, -4.0);
    assert_eq!(res.z, -6.0);
    assert_eq!(res.is_point(), true);
  }

  #[test]
  fn substract_vector_from_vector() {
    let a = build_vector(3.0, 2.0, 1.0);
    let b = build_vector(5.0, 6.0, 7.0);

    let res = a.substract(b);

    assert_eq!(res.x, -2.0);
    assert_eq!(res.y, -4.0);
    assert_eq!(res.z, -6.0);
    assert_eq!(res.is_vector(), true);
  }

  #[test]
  fn negate_tuple() {
    let a = build_tuple(3.0, 2.0, 1.0, 1.0);

    assert_eq!(-a.x, -3.0);
    assert_eq!(-a.y, -2.0);
    assert_eq!(-a.z, -1.0);
    assert_eq!(-a.w, -1.0);
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
}
