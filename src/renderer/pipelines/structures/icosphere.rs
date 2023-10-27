use wgpu::BufferUsages;

use crate::renderer::pipelines::buffers::{BufferContainer, Vertex, VertexBuffer};

use super::VertexStructure;

pub struct Icosphere<const STEPS: u16>;

impl<const STEPS: u16> VertexStructure<Vertex> for Icosphere<STEPS> {
    fn create_buffer(
        &self,
        device: &crate::renderer::matrix_renderer::renderer_system::DeviceQueue,
    ) -> crate::renderer::pipelines::buffers::VertexBuffer<Vertex> {
        let (vertecies, indexes) = icosahedron::make_icosphere(STEPS as _);

        let vertecies = vertecies
            .into_iter()
            .map(|[a, b, c]| Vertex {
                position: [a / 2., b / 2., c / 2.],
                texture_pos: [a / 2. + 0.5, c / 2. + 0.5],
            })
            .collect::<Vec<_>>();

        let indexes = indexes
            .into_iter()
            .map(|t| t.vertex.into_iter())
            .flatten()
            .collect::<Vec<_>>();

        VertexBuffer::new(
            BufferContainer::create_buffer(
                &vertecies,
                device,
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
                false,
            ),
            Some(BufferContainer::create_buffer(
                &indexes,
                device,
                BufferUsages::COPY_DST | BufferUsages::INDEX,
                false,
            )),
        )
    }
}

pub(super) mod icosahedron {
    use std::collections::HashMap;

    type Index = u16;
    type VertexList = Vec<[f32; 3]>;
    type TriangleList = Vec<Triangle>;
    type Lookup = HashMap<(Index, Index), Index>;
    #[derive(Debug, Clone, Copy)]
    pub struct Triangle {
        pub vertex: [Index; 3],
    }

    impl Triangle {
        const fn new(a: Index, b: Index, c: Index) -> Self {
            Triangle { vertex: [a, b, c] }
        }
    }

    pub const X: f32 = 0.525731112119133606;
    pub const Z: f32 = 0.850650808352039932;
    pub const N: f32 = 0.0;

    pub const VERTICES: &[[f32; 3]] = &[
        [-X, N, Z],
        [X, N, Z],
        [-X, N, -Z],
        [X, N, -Z],
        [N, Z, X],
        [N, Z, -X],
        [N, -Z, X],
        [N, -Z, -X],
        [Z, X, N],
        [-Z, X, N],
        [Z, -X, N],
        [-Z, -X, N],
    ];

    pub const TRIANGLES: &[Triangle] = &[
        Triangle::new(0, 1, 4),
        Triangle::new(0, 4, 9),
        Triangle::new(9, 4, 5),
        Triangle::new(4, 8, 5),
        Triangle::new(4, 1, 8),
        Triangle::new(8, 1, 10),
        Triangle::new(8, 10, 3),
        Triangle::new(5, 8, 3),
        Triangle::new(5, 3, 2),
        Triangle::new(2, 3, 7),
        Triangle::new(7, 3, 10),
        Triangle::new(7, 10, 6),
        Triangle::new(7, 6, 11),
        Triangle::new(11, 6, 0),
        Triangle::new(0, 6, 1),
        Triangle::new(6, 10, 1),
        Triangle::new(0, 9, 11),
        Triangle::new(11, 9, 2),
        Triangle::new(2, 9, 5),
        Triangle::new(7, 11, 2),
    ];

    fn vertex_for_edge(
        lookup: &mut Lookup,
        vertices: &mut VertexList,
        first: Index,
        second: Index,
    ) -> Index {
        let key = if first < second {
            (first, second)
        } else {
            (second, first)
        };

        if let Some(&index) = lookup.get(&key) {
            index
        } else {
            let edge0 = vertices[first as usize];
            let edge1 = vertices[second as usize];
            let point = normalize([
                edge0[0] + edge1[0],
                edge0[1] + edge1[1],
                edge0[2] + edge1[2],
            ]);
            let index = vertices.len() as Index;
            vertices.push(point);
            lookup.insert(key, index);
            index
        }
    }

    fn subdivide(vertices: &mut VertexList, triangles: &TriangleList, step: usize) -> TriangleList {
        let mut lookup: Lookup = HashMap::new();
        let mut result: TriangleList = Vec::new();

        for triangle in triangles {
            let mut mid: [Index; 3] = [0; 3];
            for edge in 0..3 {
                mid[edge] = vertex_for_edge(
                    &mut lookup,
                    vertices,
                    triangle.vertex[edge],
                    triangle.vertex[(edge + 1) % 3],
                );
            }

            // if step % 2 != 0 {
                result.push(Triangle::new(triangle.vertex[0], mid[0], mid[2]));
                result.push(Triangle::new(triangle.vertex[1], mid[1], mid[0]));
                result.push(Triangle::new(triangle.vertex[2], mid[2], mid[1]));
                result.push(Triangle::new(mid[0], mid[1], mid[2]));
            // } else {
            // result.push(Triangle::new(triangle.vertex[0], mid[2], mid[0]));
            // result.push(Triangle::new(triangle.vertex[1], mid[0], mid[1]));
            // result.push(Triangle::new(triangle.vertex[2], mid[1], mid[2]));
            // result.push(Triangle::new(mid[0], mid[2], mid[1]));
            // }
        }

        result
    }

    fn normalize(vector: [f32; 3]) -> [f32; 3] {
        let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
        [vector[0] / length, vector[1] / length, vector[2] / length]
    }

    type IndexedMesh = (VertexList, TriangleList);

    pub fn make_icosphere(subdivisions: usize) -> IndexedMesh {
        let mut vertices: VertexList = VERTICES.iter().cloned().collect();
        let mut triangles: TriangleList = TRIANGLES.iter().cloned().collect();

        for s in 0..subdivisions {
            triangles = subdivide(&mut vertices, &triangles, s);
        }

        (vertices, triangles)
    }
}
