use std::fs::File;
use std::io::prelude::*;
use black::{Vec2, Vec3, Vec4};

// -----------------------------------------------------
// Geometry Error
// -----------------------------------------------------

#[derive(Debug)]
pub struct GeometryParseError {
    message: String,
}
impl GeometryParseError {
    pub fn new(message: String) -> GeometryParseError {
        GeometryParseError { message }
    }
}
#[derive(Debug)]
pub enum GeometryError {
    IoError(std::io::Error),
    ParseError(GeometryParseError),
}
impl From<std::num::ParseFloatError> for GeometryError {
    fn from(error: std::num::ParseFloatError) -> GeometryError {
        let inner = GeometryParseError::new(format!("unable to parse float: {:?}", error));
        GeometryError::ParseError(inner)
    }
}
impl From<std::num::ParseIntError> for GeometryError {
    fn from(error: std::num::ParseIntError) -> GeometryError {
        let inner = GeometryParseError::new(format!("unable to parse integer: {:?}", error));
        GeometryError::ParseError(inner)
    }
}
impl From<std::io::Error> for GeometryError {
    fn from(error: std::io::Error) -> GeometryError {
        GeometryError::IoError(error)
    }
}
// -----------------------------------------------------
// Default Vertex
// -----------------------------------------------------
#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub color:    Vec4,
    pub normal:   Vec3,
    pub uv:       Vec2,
}


// -----------------------------------------------------
// Geometry
// -----------------------------------------------------
#[derive(Debug, Clone)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices:  Vec<usize>,
}
impl Geometry {

    #[allow(dead_code)]
    pub fn cube(s: f32) -> Geometry {
        let positions = vec![
            /* front  */
            Vec4::new(-s, -s, s,  1.0),
            Vec4::new(s, -s, s,   1.0),
            Vec4::new(s, s, s,    1.0),
            Vec4::new(-s, s, s,   1.0),
            /* back   */
            Vec4::new(-s, -s, -s, 1.0),
            Vec4::new(-s, s, -s,  1.0),
            Vec4::new(s, s, -s,   1.0),
            Vec4::new(s, -s, -s,  1.0),
            /* top    */
            Vec4::new(-s, s, -s, 1.0),
            Vec4::new(-s, s, s,  1.0),
            Vec4::new(s, s, s,   1.0),
            Vec4::new(s, s, -s,  1.0),
            /* bottom */
            Vec4::new(-s, -s, -s, 1.0),
            Vec4::new(s, -s, -s,  1.0),
            Vec4::new(s, -s, s,   1.0),
            Vec4::new(-s, -s, s,  1.0),
            /* right  */
            Vec4::new(s, -s, -s, 1.0),
            Vec4::new(s, s, -s,  1.0),
            Vec4::new(s, s, s,   1.0),
            Vec4::new(s, -s, s,  1.0),
            /* left   */
            Vec4::new(-s, -s, -s, 1.0),
            Vec4::new(-s, -s, s,  1.0),
            Vec4::new(-s, s, s,   1.0),
            Vec4::new(-s, s, -s,  1.0),
        ];
        let colors = vec![
            /* front  */
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            /* back   */
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            /* top    */
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            /* bottom */
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            /* right  */
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            /* left   */
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let normals = vec![
            /* front  */
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            /* back   */
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            /* top    */
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            /* bottom */
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            /* right  */
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            /* left   */
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
        ];

        let uvs = vec![
            /* front  */
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            /* back   */
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            /* top    */
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            /* bottom */
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            /* right  */
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            /* left   */
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];
        let mut vertices = vec![];
        for i in 0..positions.len() {
            vertices.push(Vertex {
                position: positions[i],
                color:    colors[i],
                normal:   normals[i],
                uv:       uvs[i],
            })
        }
        let indices = vec![
            0, 1, 2, 0, 2, 3, // front
            4, 5, 6, 4, 6, 7, // back
            8, 9, 10, 8, 10, 11, // top
            12, 13, 14, 12, 14, 15, // bottom
            16, 17, 18, 16, 18, 19, // right
            20, 21, 22, 20, 22, 23, // left
        ];
        Geometry { vertices, indices }
    }
    pub fn obj(path: &str) -> Result<Geometry, GeometryError> {
        // geometry accumulators
        let mut acc_v: Vec<Vec4> = vec![];
        let mut acc_vn: Vec<Vec3> = vec![];
        let mut acc_vt: Vec<Vec2> = vec![];

        // geometry data - built from accumulators.
        let mut positions: Vec<Vec4> = vec![];
        let mut normals: Vec<Vec3> = vec![];
        let mut uvs: Vec<Vec2> = vec![];
        let mut indices: Vec<usize> = vec![];

        let mut file = File::open(path)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;
        let lines = content.split("\n");
        for line in lines {
            let parts = line.split(" ").map(|x| x.trim()).collect::<Vec<_>>();
            if parts.len() > 0 {
                match parts[0] {
                    "v" => {
                        let x = parts[1].parse::<f32>()?;
                        let y = parts[2].parse::<f32>()?;
                        let z = parts[3].parse::<f32>()?;
                        acc_v.push(Vec4::new(x, y, z, 1.0));
                    }
                    "vn" => {
                        let x = parts[1].parse::<f32>()?;
                        let y = parts[2].parse::<f32>()?;
                        let z = parts[3].parse::<f32>()?;
                        acc_vn.push(Vec3::new(x, y, z));
                    }
                    "vt" => {
                        let x = parts[1].parse::<f32>()?;
                        let y = parts[2].parse::<f32>()?;
                        acc_vt.push(Vec2::new(x, y));
                    }
                    "f" => {
                        for i in 1..=3 {
                            let face = parts[i].split("/").collect::<Vec<_>>();
                            let i_v = face[0].parse::<usize>()? - 1;
                            let i_vt = face[1].parse::<usize>()? - 1;
                            let i_vn = face[2].parse::<usize>()? - 1;
                            if i_v > acc_v.len() - 1 {
                                let error = GeometryParseError::new(format!(
                                    "Invalid position index for face: {:?}",
                                    parts[0]
                                ));
                                return Err(GeometryError::ParseError(error));
                            }
                            if i_vt > acc_vt.len() - 1 {
                                let error = GeometryParseError::new(format!(
                                    "Invalid texcoord index for face: {:?}",
                                    parts[0]
                                ));
                                return Err(GeometryError::ParseError(error));
                            }
                            if i_vn > acc_vn.len() - 1 {
                                let error = GeometryParseError::new(format!(
                                    "Invalid normal index for face: {:?}",
                                    parts[0]
                                ));
                                return Err(GeometryError::ParseError(error));
                            }
                            positions.push(acc_v[i_v]);
                            normals.push(acc_vn[i_vn]);
                            uvs.push(acc_vt[i_vt]);
                            indices.push(indices.len());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // push vertices array.
        let mut vertices = vec![];
        for i in 0..positions.len() {
            vertices.push(Vertex {
                position: positions[i],
                normal:   normals[i],
                uv:      uvs[i],
                color:   Vec4::one()
            })
        }
        Ok(Geometry {
            vertices, 
            indices
        })
    }
}
