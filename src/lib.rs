use std::fmt;
use crate::GraphError::MatrixDimensionMismatch;

pub type VertexIndex = usize;
pub type EdgeCost = usize;
#[derive(Debug)]
pub enum GraphError {
    MatrixDimensionMismatch {
        matrix_len: usize,
        vertex_count: usize,
        expected_len: usize
    }
}
impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::MatrixDimensionMismatch { matrix_len, vertex_count, expected_len } => {
                write!(f, "Adjacency matrix length {} does not match vertices count {} (expected adjacency matrix length to be {})",
                       matrix_len, vertex_count, expected_len)
            }
        }
    }
}
impl std::error::Error for GraphError {}
/*pub struct Graph<T, C: PartialOrd + Copy> {
    adjacency_matrix: Vec<C>,
    vertices: Vec<T>,
    vertex_count: usize
}*/
/*#[derive(Debug)]
pub struct Graph<T> {
    adjacency_matrix: Vec<usize>,
    vertices: Vec<T>,
    vertex_count: usize
}*/
#[derive(Debug)]
pub struct Graph {
    adjacency_matrix: Vec<usize>,
    vertices: Vec<u64>,
    vertex_count: usize
}

// impl<T, C> Graph<T, C> where C: PartialOrd + Copy {
//impl<T> Graph<T> {
impl Graph {
    //pub fn new(adjacency_matrix: Vec<usize>, vertices: Vec<T>) -> Result<Graph<T>, GraphError> {
    pub fn new(adjacency_matrix: Vec<usize>, vertices: Vec<u64>) -> Result<Graph, GraphError> {
        let vertex_count = vertices.len();
        let expected_len = vertex_count * vertex_count;

        if adjacency_matrix.len() == expected_len {
            Ok(Graph {
                adjacency_matrix,
                vertices,
                vertex_count
            })
        } else {
            Err(GraphError::MatrixDimensionMismatch {
                matrix_len: adjacency_matrix.len(),
                vertex_count: vertices.len(),
                expected_len
            })
        }
    }
    // Basic Operations
    pub fn adjacent(&self, x: VertexIndex, y: VertexIndex) -> bool {
        self.adjacency_matrix[x*self.vertex_count + y] > 0
    }
    pub fn neighbors(&self, x: VertexIndex) -> Vec<VertexIndex> {
        (x * self.vertex_count..x * self.vertex_count + self.vertex_count)
            .filter(|&i| self.adjacency_matrix[i] > 0)
            .collect()
    }
    //pub fn add_vertex(&mut self, v: T) -> VertexIndex {
    pub fn add_vertex(&mut self, v: u64) -> VertexIndex {
        let old_len = self.vertex_count;
        let new_len = old_len + 1;
        let new_adj_size = new_len * new_len;

        let mut new_adjacency_matrix = vec![0; new_adj_size];
        for i in 0..self.vertex_count {
            let old_lower = i * old_len;
            let new_lower = i * new_len;
            let old_upper = old_lower + old_len;
            let new_upper = new_lower + new_len;
            new_adjacency_matrix[new_lower..new_upper-1].copy_from_slice(&self.adjacency_matrix[old_lower..old_upper]);
        }

        self.adjacency_matrix = new_adjacency_matrix;
        self.vertices.push(v);
        self.vertex_count = self.vertices.len();

        self.vertex_count
    }
    pub fn remove_vertex(&mut self, x: VertexIndex) {
        let old_len = self.vertex_count;
        let new_len = old_len - 1;
        let new_adj_size = new_len * new_len;

        let mut new_adjacency_matrix = vec![0; new_adj_size];
        let mut new_i = 0; // Helper because we will skip a row, throwing off indexing
        for i in 0..old_len {
            if i == x { continue; } // Skip row, if not included in new adjacency_matrix,
            // Left to middle (non, inclusive) of row
            new_adjacency_matrix[new_i * new_len..new_i * new_len + x].copy_from_slice(&self.adjacency_matrix[i * old_len..i * old_len + x]);
            // Middle to end of row
            new_adjacency_matrix[new_i * new_len + x..new_i * new_len + new_len].copy_from_slice(&self.adjacency_matrix[i * old_len + x + 1.. i * old_len + old_len]);
            // Needs to be done here because of skipping row
            new_i = new_i + 1;
        }

        self.adjacency_matrix = new_adjacency_matrix;
        self.vertices.remove(x);
        self.vertex_count = self.vertices.len();
    }
    pub fn add_edge(&mut self, x: VertexIndex, y: VertexIndex, z: EdgeCost) {
        self.adjacency_matrix[x * self.vertex_count + y] = z;
        self.adjacency_matrix[y * self.vertex_count + x] = z;
    }
    pub fn remove_edge(&mut self, x: VertexIndex, y: VertexIndex) {
        self.adjacency_matrix[x * self.vertex_count + y] = 0;
        self.adjacency_matrix[y * self.vertex_count + x] = 0;
    }
    // Value-Associated Operations
    //pub fn get_vertex_value(&self, x: VertexIndex) -> Option<&T> {
    pub fn get_vertex_value(&self, x: VertexIndex) -> Option<&u64> {
        self.vertices.get(x)
    }
    //pub fn set_vertex_value(&mut self, x: VertexIndex, v: T) {
    pub fn set_vertex_value(&mut self, x: VertexIndex, v: u64) {
        self.vertices[x] = v
    }
    pub fn get_edge_value(&self, x: VertexIndex, y: VertexIndex) -> EdgeCost {
        self.adjacency_matrix[ x * self.vertex_count + y ]
    }
    pub fn set_edge_value(&mut self, x: VertexIndex, y: VertexIndex, v: EdgeCost) {
        self.adjacency_matrix[ x * self.vertex_count + y ] = v;
    }
    // Debug
    pub fn debug(&self) {
        println!("Vertices: {:?}", self.vertices);

        // Iterate through rows
        println!("Matrix:");
        for i in 0..self.vertex_count {
            let start = i * self.vertex_count;
            let end = start + self.vertex_count;

            if end <= self.adjacency_matrix.len() {
                println!("{:?}", &self.adjacency_matrix[start..end]);
            } else {
                println!("Matrix Row {}: [Error: Index out of bounds]", i);
                println!("Something with vertex_count is messed up");
            }
        }
        println!("Vertex count: {}", self.vertex_count);
    }
}

