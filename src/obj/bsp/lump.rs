/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/bsp/lump.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Lump definitions for Q3 BSP maps.
*/

use math::{ Vec2i, Vec2f, Vec3f, Vec4u8 };

pub enum Lump_Type
{
  Entity_Type = 0x00,
  Texture_Type = 0x01,
  Plane_Type = 0x02,
  Node_Type = 0x03,
  Leaf_Type = 0x04,
  Leaf_Face_Type = 0x05,
  Leaf_Brush_Type = 0x06,
  Model_Type = 0x07,
  Brush_Type = 0x08,
  Brush_Side_Type = 0x09,
  Vertex_Type = 0x0A,
  Mesh_Vert_Type = 0x0B,
  Effect_Type = 0x0C,
  Face_Type = 0x0D,
  Light_Map_Type = 0x0E,
  Light_Vol_Type = 0x0F,
  Vis_Data_Type = 0x10
}

static version: i32 = 0x2E;

#[packed]
pub struct Lump
{
  /* Offset from the BOF to the lump. */
  offset: i32,
  /* Always a multiple of 4. */
  length: i32
} 
impl Lump
{
  #[inline(always)]
  pub fn new() -> Lump
  { Lump{ offset: 0, length: 0 } }
}

#[packed]
pub struct Header
{
  /* Always "IBSP" */
  magic: [i8, ..4],
  /* Should be 0x2e for Q3 files. */
  version: i32,
  /* Fixed number of lumps. */
  lumps: [Lump, ..17]
}
impl Header
{
  #[inline(always)]
  pub fn new() -> Header
  { Header{ magic: [0, ..4], version: 0, lumps: [Lump::new(), ..17] } }
}

#[packed]
pub struct Entity
{
  /* Size of the buffer. */
  size: i32,
  buffer: ~[i8] /* TODO: Read binary into this? */
}
impl Entity
{
  #[inline(always)]
  pub fn new() -> Entity
  { Entity{ size: 0, buffer: ~[] } }
}
#[packed]
pub struct Texture
{
  name: [i8, ..64],
  surface_flags: i32,
  content_flags: i32
}

#[packed]
pub struct Plane
{
  normal: Vec3f,
  /* Distance the plane is from the origin, along the normal. */
  distance: f32
}

#[packed]
pub struct Node
{
  /* Index of the corresponding plane. */
  plane: i32,
  /* Child indices; negative means lead: -(leaf + 1) */
  children: [i32, ..2],
  /* Bounding box. */
  mins: [i32, ..3], /* TODO: Templated Vec3? */
  maxs: [i32, ..3]
}

#[packed]
pub struct Leaf
{
  /* Visdata cluster index. */
  cluster: i32,
  /* Areaportal area. */
  area: i32,
  /* Bounding box. */ /* TODO: Struct? */
  mins: [i32, ..2],
  maxs: [i32, ..2],
  /* First leaf face. */
  face: i32,
  num_faces: i32,
  /* First leaf brush. */
  brush: i32,
  num_brushes: i32
}

#[packed]
pub struct Leaf_Face
{
  /* Face index. */
  face: i32
}

#[packed]
pub struct Leaf_Brush
{
  /* Brush index. */
  brush: i32
}

#[packed]
pub struct Model
{
  /* Bounding box. */
  mins: [f32, ..2],
  maxs: [f32, ..2],
  /* First face. */
  face: i32,
  num_faces: i32,
  /* First brush. */
  brush: i32,
  num_brushes: i32
}

#[packed]
pub struct Brush
{
  /* First brush side. */
  side: i32,
  num_sides: i32,
  /* Texture index. */
  texture: i32
}

#[packed]
pub struct Brush_Side
{
  /* Plane index. */
  plane: i32,
  /* Texture index. */
  texture: i32
}

#[packed]
pub struct Vertex
{
  position: Vec3f,
  tex_coords: [Vec2f, ..2], /* 0 = Surface; 1 = Lightmap */
  normal: Vec3f,
  color: Vec4u8 
}
impl Vertex
{
  #[inline(always)]
  pub fn new() -> Vertex
  { Vertex {  position: Vec3f::zero(),
              tex_coords: [Vec2f::zero(), ..2],
              normal: Vec3f::zero(),
              color: Vec4u8::new(1, 1, 1, 1) } }
}

#[packed]
pub struct Mesh_Vert
{
  /* Vertex index offset, relative to the first vertex of face. */
  offset: i32
}
impl Mesh_Vert
{
  #[inline(always)]
  pub fn new() -> Mesh_Vert
  { Mesh_Vert { offset: 0 } }
}

#[packed]
pub struct Effect
{
  name: [i8, ..64],
  /* Brush that generated this effect. */
  brush: i32,
  /* Always seems to be 5. */
  unknown: i32
}

#[packed]
pub struct Face
{
  /* Texture index. */
  texture: i32,
  /* Effect index. */
  effect: i32,
  kind: i32, /* 1 = Polygon; 2 = Patch; 3 = Mesh; 4 = Billboard */
  /* Index of first vertex. */
  start_vertex: i32,
  num_vertices: i32,
  /* Index of first mesh vert. */
  start_mesh_vertex: i32,
  num_mesh_vertices: i32,
  /* Light map index. */
  lightmap: i32,
  lightmap_corner: Vec2i,
  lightmap_size: Vec2i,
  lightmap_origin: Vec3f,
  /* World-space s and t unit vectors. */
  lightmap_vecs: [Vec3f, ..2],
  normal: Vec3f,
  /* Patch dimensions. */
  patch_size: Vec2i,
}
impl Face
{
  #[inline(always)]
  pub fn new() -> Face
  { 
    Face {  texture: 0,
            effect: 0,
            kind: 0,
            start_vertex: 0,
            num_vertices: 0,
            start_mesh_vertex: 0, 
            num_mesh_vertices: 0,
            lightmap: 0,
            lightmap_corner: Vec2i::zero(),
            lightmap_size: Vec2i::zero(),
            lightmap_origin: Vec3f::zero(),
            lightmap_vecs: [Vec3f::zero(), ..2],
            normal: Vec3f::zero(),
            patch_size: Vec2i::zero() } 
  }
}

#[packed]
pub struct Light_Map
{
  data: [[[u8, ..128], ..128], ..3]
}

#[packed]
pub struct Light_Vol
{
  /* Ambient color compontn RGB. */
  ambient: [u8, ..3],
  /* Directional color component RGB. */
  directional: [u8, ..3],
  /* Direction to the light. */
  direction: [u8, ..2] /* 0 = phi; 1 = theta */
}

#[packed]
pub struct Vis_Data
{
  num_clusters: i32,
  bytes_per_cluster: i32,
  buffer: ~[u8]
}

