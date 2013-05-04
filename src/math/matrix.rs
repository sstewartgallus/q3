/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/matrix.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 16 component (4x4) matrix of floats
      for representing orientational data.
*/

use math::vec3::Vec3f;

type Component = f32; /* TODO: Template. */

struct Mat4x4
{
  data: [[f32, ..4], ..4]
}

impl Mat4x4
{

  #[inline(always)]
  pub fn new() -> Mat4x4
  {
    Mat4x4{data:[ [1.0, 0.0, 0.0, 0.0],
                  [0.0, 1.0, 0.0, 0.0],
                  [0.0, 0.0, 1.0, 0.0],
                  [0.0, 0.0, 0.0, 1.0]]}
  }

  #[inline(always)]
  pub fn new_perspective(fov: Component, aspect_ratio: Component, near: Component, far: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();

    let rad = (3.1415 * fov) / 180.0;
    let range = f32::tan(rad / 2.0) * near;
    let left = -range * aspect_ratio;
    let right = range * aspect_ratio;
    let bottom = -range;
    let top = range;

    mat.data[0][0] = (2.0 * near) / (right - left); mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = 0.0; 
    mat.data[0][1] = 0.0; mat.data[1][1] = (2.0 * near) / (top - bottom); mat.data[2][1] = 0.0; mat.data[3][1] = 0.0; 
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = -(far + near) / (far - near); mat.data[3][2] = -(2.0 * far * near) / (far - near); 
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = -1.0; mat.data[3][3] = 0.0;

    mat
  }

  #[inline(always)]
  pub fn new_orthographic(left: Component, right: Component, bottom: Component, top: Component,
                          near: Component, far: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = 2.0 / (right - left); mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] =  -(right + left) / (right - left); 
    mat.data[0][1] = 0.0; mat.data[1][1] = 2.0 / (top - bottom); mat.data[2][1] = 0.0; mat.data[3][1] = -(top + bottom) / (top - bottom); 
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = -2.0 / (far - near); mat.data[3][2] = -(far + near) / (far - near);
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;

    mat
  }
  
  pub fn new_scale(x: Component, y: Component, z: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = x;   mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = 0.0;
    mat.data[0][1] = 0.0; mat.data[1][1] = y;   mat.data[2][1] = 0.0; mat.data[3][1] = 0.0;
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = z;   mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;
  
    mat
  }
    
  pub fn new_translation(x: Component, y: Component, z: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = 1.0; mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = x; 
    mat.data[0][1] = 0.0; mat.data[1][1] = 1.0; mat.data[2][1] = 0.0; mat.data[3][1] = y; 
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = 1.0; mat.data[3][2] = z; 
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;
  
    mat
  }
  
  #[inline(always)]
  pub fn new_rotation_x(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = 1.0; mat.data[1][0] = 0.0;            mat.data[2][0] = 0.0;           mat.data[3][0] = 0.0;
    mat.data[0][1] = 0.0; mat.data[1][1] = f32::cos(rad);  mat.data[2][1] = f32::sin(rad); mat.data[3][1] = 0.0;
    mat.data[0][2] = 0.0; mat.data[1][2] = -f32::sin(rad); mat.data[2][2] = f32::cos(rad); mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0;            mat.data[2][3] = 0.0;           mat.data[3][3] = 1.0;
  
    mat
  }

  #[inline(always)]
  pub fn new_rotation_y(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = f32::cos(rad);  mat.data[1][0] = 0.0; mat.data[2][0] = f32::sin(rad); mat.data[3][0] = 0.0;  
    mat.data[0][1] = 0.0;            mat.data[1][1] = 1.0; mat.data[2][1] = 0.0;           mat.data[3][1] = 0.0;
    mat.data[0][2] = -f32::sin(rad); mat.data[1][2] = 0.0; mat.data[2][2] = f32::cos(rad); mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0;            mat.data[1][3] = 0.0; mat.data[2][3] = 0.0;           mat.data[3][3] = 1.0;
    
    mat
  }

  #[inline(always)]
  pub fn new_rotation_z(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = f32::cos(rad);  mat.data[1][0] = f32::sin(rad); mat.data[2][0] = 0.0; mat.data[3][0] = 0.0;
    mat.data[0][1] = -f32::sin(rad); mat.data[1][1] = f32::cos(rad); mat.data[2][1] = 0.0; mat.data[3][1] = 0.0;
    mat.data[0][2] = 0.0;            mat.data[1][2] = 0.0;           mat.data[2][2] = 1.0; mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0;            mat.data[1][3] = 0.0;           mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;
  
    mat
  }

  #[inline(always)]
  pub fn new_lookat(position: Vec3f, target: Vec3f, up: Vec3f) -> Mat4x4
  {
    let mut forward = target - position;
    forward.normalize();

    let mut side = forward.cross(&up);
    side.normalize();

    let mut proper_up = side.cross(&forward);
    proper_up.normalize();

    let mut mat = Mat4x4::new();
    mat.data[0][0] = side.x;      mat.data[1][0] = side.y;      mat.data[2][0] = side.z;      mat.data[3][0] = -side.dot(&position); 
    mat.data[0][1] = proper_up.x; mat.data[1][1] = proper_up.y; mat.data[2][1] = proper_up.z; mat.data[3][1] = -proper_up.dot(&position);
    mat.data[0][2] = -forward.x;  mat.data[1][2] = -forward.y;  mat.data[2][2] = -forward.z;  mat.data[3][2] = forward.dot(&position);
    mat.data[0][3] = 0.0;         mat.data[1][3] = 0.0;         mat.data[2][3] = 0.0;         mat.data[3][3] = 1.0;

    mat
  }

  #[inline(always)]
  pub fn get_width(&self) -> uint
  { return 4; }
  #[inline(always)]
  pub fn get_height(&self) -> uint
  { return 4; }

  #[inline(always)]
  pub fn up(&self) -> Vec3f
  { Vec3f::new(self.data[0][1], self.data[1][1], self.data[2][1]) }
  #[inline(always)]
  pub fn down(&self) -> Vec3f
  { Vec3f::new(-self.data[0][1], -self.data[1][1], -self.data[2][1]) }
  #[inline(always)]
  pub fn left(&self) -> Vec3f
  { Vec3f::new(-self.data[0][0], -self.data[1][0], -self.data[2][0]) }
  #[inline(always)]
  pub fn right(&self) -> Vec3f
  { Vec3f::new(self.data[0][0], self.data[1][0], self.data[2][0]) }
  #[inline(always)]
  pub fn forward(&self) -> Vec3f
  { Vec3f::new(-self.data[0][2], -self.data[1][2], -self.data[2][2]) }
  #[inline(always)]
  pub fn backward(&self) -> Vec3f
  { Vec3f::new(self.data[0][2], self.data[1][2], self.data[2][2]) }

  #[inline(always)]
  pub fn identity(&mut self)
  {
    self.data = [ [1.0, 0.0, 0.0, 0.0],
                  [0.0, 1.0, 0.0, 0.0],
                  [0.0, 0.0, 1.0, 0.0],
                  [0.0, 0.0, 0.0, 1.0]];
  }

  #[inline(always)]
  pub unsafe fn to_ptr(&self) -> *Mat4x4
  { ptr::to_unsafe_ptr(self) }

  pub fn show(&self)
  {
    io::println("----------");
    for i32::range(0, 4) |y|
    {
      io::print("|");
      for i32::range(0, 4) |x|
      {
        io::print(f32::to_str(self.data[x][y]) + " ");
      }
      io::println("|");
    }
    io::println("----------");
  }
}

/***** Operator Overloads *****/
impl Mul<Mat4x4, Mat4x4> for Mat4x4
{
  pub fn mul(&self, rhs: &Mat4x4) -> Mat4x4
  {
    let mut mat = Mat4x4::new();

    for i32::range(0, 4) |i|
    {
      for i32::range(0, 4) |j|
      {
        mat.data[i][j] =  self.data[i][0] * rhs.data[0][j] +
                          self.data[i][1] * rhs.data[1][j] +
                          self.data[i][2] * rhs.data[2][j] +
                          self.data[i][3] * rhs.data[3][j];
      }
    }

    mat
  }
}

