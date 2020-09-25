use std::marker::PhantomData;

use golem::{ElementBuffer, GeometryMode, VertexBuffer};

use crate::{
    draw::{ColorVertex, Quad, Vertex},
    Color, Context, Error, Point3,
};

pub struct Batch<V> {
    geometry_mode: GeometryMode,

    vertices: Vec<f32>,
    elements: Vec<u32>,

    buffers: Buffers<V>,
    is_dirty: bool,

    _phantom: PhantomData<V>,
}

impl<V: Vertex> Batch<V> {
    pub fn new_triangles(ctx: &Context) -> Result<Self, Error> {
        Self::new(ctx, GeometryMode::Triangles)
    }

    pub fn new_lines(ctx: &Context) -> Result<Self, Error> {
        Self::new(ctx, GeometryMode::Triangles)
    }

    pub fn new(ctx: &Context, geometry_mode: GeometryMode) -> Result<Self, Error> {
        let buffers = Buffers::new(ctx)?;

        Ok(Self {
            geometry_mode,
            vertices: Vec::new(),
            elements: Vec::new(),
            buffers,
            is_dirty: false,
            _phantom: PhantomData,
        })
    }

    pub fn geometry_mode(&self) -> GeometryMode {
        self.geometry_mode
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_elements(&self) -> usize {
        self.elements.len()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.elements.clear();
        self.is_dirty = true;
    }

    pub fn push_element(&mut self, element: u32) {
        self.elements.push(element);
        self.is_dirty = true;
    }

    pub fn push_vertex(&mut self, vertex: &V) {
        vertex.append(&mut self.vertices);
        self.is_dirty = true;
    }

    pub fn flush(&mut self) {
        if self.is_dirty {
            self.buffers.vertices.set_data(&self.vertices);
            self.buffers.elements.set_data(&self.elements);
            self.buffers.num_elements = self.elements.len();

            self.is_dirty = false;
        }
    }

    pub fn buffers(&self) -> &Buffers<V> {
        &self.buffers
    }
}

impl Batch<ColorVertex> {
    pub fn push_quad(&mut self, quad: &Quad, color: Color) {
        assert!(self.geometry_mode == GeometryMode::Triangles);

        let first_idx = self.num_elements() as u32;

        for corner in &quad.corners {
            self.push_vertex(&ColorVertex {
                world_pos: Point3::new(corner.x, corner.y, quad.z),
                color,
            });
        }

        self.elements.extend_from_slice(&[
            first_idx + 0,
            first_idx + 1,
            first_idx + 2,
            first_idx + 2,
            first_idx + 3,
            first_idx + 0,
        ]);
    }
}

pub struct Buffers<V> {
    pub(crate) vertices: VertexBuffer,
    pub(crate) elements: ElementBuffer,
    pub(crate) num_elements: usize,
    _phantom: PhantomData<V>,
}

impl<V> Buffers<V> {
    pub fn new(ctx: &Context) -> Result<Self, Error> {
        let vertices = VertexBuffer::new(ctx.golem_context())?;
        let elements = ElementBuffer::new(ctx.golem_context())?;

        Ok(Self {
            vertices,
            elements,
            num_elements: 0,
            _phantom: PhantomData,
        })
    }

    pub fn from_buffers_unchecked(
        vertices: VertexBuffer,
        elements: ElementBuffer,
        num_elements: usize,
    ) -> Self {
        Self {
            vertices,
            elements,
            num_elements,
            _phantom: PhantomData,
        }
    }
}