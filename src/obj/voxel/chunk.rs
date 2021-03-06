/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/chunk.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A pageable 3D chunk of voxel data.
*/

use math::{ Vec3i8 };

#[path = "../../gl/mod.rs"]
mod gl;
#[path = "../../gl/util.rs"]
mod util;
#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

pub struct Chunk
{
  vbo: gl::GLuint,
  dimensions: Vec3i8,
  voxels: ~[Vec3i8],
}

impl Chunk
{
  pub fn new(dim: &Vec3i8) -> Chunk
  {
    let chunk = Chunk
    {
      vbo: check!(gl::gen_buffers(1))[0],
      dimensions: *dim,
      voxels: ~[],
    };

    chunk
  }

  //pub fn get(&self, x: i8, y: i8, z: i8) -> Behavior
  //{
  //  voxels[(z * self.dimensions.z * self.dimensions.z) + (y * self.dimensions.y) + x].get_behavior()
  //}

  pub fn draw(&self)
  {

  }
}

