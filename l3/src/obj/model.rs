#![allow(dead_code)]
extern crate cgmath;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Face {
    // vertices that make up a face
    pub vertices: Vec<usize>,
    // index to texture coords for the face
    pub texture_indices: Vec<usize>,
}

//#[derive(Copy, Clone)]
pub struct Model {
    // vector containing all vertices in an object
    pub vertices: Vec<cgmath::Vector3<f64>>,
    // vector containit all vertex coordinates
    pub texture_coords: Vec<cgmath::Vector3<f64>>,
    // vector containing each face struct
    pub faces: Vec<Face>,
}

impl Model {
    pub fn new(filename: &str) -> Model {
        let file = File::open(&filename).expect("error opening model");
        let reader = io::BufReader::new(file);

        let mut vertices: Vec<cgmath::Vector3<f64>> = Vec::new();
        let mut texture_coords: Vec<cgmath::Vector3<f64>> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let mut split: Vec<&str> = line.split(" ").collect();
            //ignore lines w/o data
            if line.len() < 2 {
                continue;
            }
            // v denotes a regular vertex
            if &line[0..2] == "v " {
                let x: f64 = split[1].parse().unwrap();
                let y: f64 = split[2].parse().unwrap();
                let z: f64 = split[3].parse().unwrap();

                let vector = cgmath::vec3(x, y, z);
                vertices.push(vector);
            } else if &line[0..3] == "vt " {
                let x: f64 = split[2].parse().unwrap();
                let y: f64 = split[3].parse().unwrap();
                let z: f64 = split[4].parse().unwrap();
                let vector = cgmath::vec3(x, y, z);
                texture_coords.push(vector);
            }
            // f denotes vertex indexes that make up a face
            else if &line[0..2] == "f " {
                let mut vertices: Vec<usize> = Vec::new();
                let mut texture_indices: Vec<usize> = Vec::new();
                //let mut normals: Vec<i32> = Vec::new();

                // ignore the first character "f"
                for vertex in &split[1..] {
                    let group: Vec<&str> = vertex.split("/").collect();
                    let vect: usize = group[0].parse().unwrap();
                    vertices.push(vect - 1);
                    if group.len() > 2 && group[1] != "" {
                        let vt: usize = group[1].parse().unwrap();
                        texture_indices.push(vt - 1);
                    }
                }
                let face = Face::new(vertices, texture_indices);
                faces.push(face);
            }
        }
        //println!("{:?}", faces);
        Model {
            vertices,
            texture_coords,
            faces,
        }
    }

    pub fn num_vertices(self) -> usize {
        self.vertices.len()
    }

    pub fn num_faces(self) -> usize {
        self.faces.len() - 1
    }
    pub fn get_vertex(&self, x: usize) -> &cgmath::Vector3<f64> {
        &self.vertices[x]
    }
    pub fn get_texture_coord(&self, x: usize) -> &cgmath::Vector3<f64> {
        &self.texture_coords[x]
    }
}

impl Face {
    pub fn new(vertices: Vec<usize>, texture_indices: Vec<usize>) -> Face {
        Face {
            vertices,
            texture_indices,
        }
    }
}
