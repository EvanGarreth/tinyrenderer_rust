#![allow(dead_code)]
extern crate cgmath;

use std::fs::File;
use std::io;
use std::io::prelude::*;

//#[derive(Copy, Clone)]
pub struct Model {
    // vector containing all vertices in an object
    pub vertices: Vec<cgmath::Vector3<f64>>,
    // vector containing vectors of vectors containing indices to vectors that correspond to
    // 0) vertex indices, 1) vertex texture coord indices, and 2) vertex normal indices
    // ie faces[0][0][0] = first vertex indice for the first face
    //    faces[0][1][0] = first vertex texture coord indice for the first face
    //    faces[0][2][0] = first vertex normal indice for the first face
    pub faces: Vec<Vec<i32>>,
}

impl Model {
    pub fn new(filename: &str) -> Model {
        let file = File::open(&filename).expect("error opening model");
        let reader = io::BufReader::new(file);

        let mut vertices: Vec<cgmath::Vector3<f64>> = Vec::new();
        let mut faces: Vec<Vec<i32>> = Vec::new();
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
            }
            // f denotes vertex indexes that make up a face
            else if &line[0..2] == "f " {
                let mut face: Vec<i32> = Vec::new();
                for vertex in split {
                    let group: Vec<&str> = vertex.split("/").collect();
                    // semi set up for grabbing texture/normals, ignoring for now and just using
                    // the vertices that make up a face
                    for vector in group {
                        if vector == "f" {
                            continue;
                        } // ignore the first element
                        let vect: i32 = vector.parse().unwrap();
                        face.push(vect - 1);
                        break;
                    }
                }
                // currently face is [] on the first passthrough, TODO find the "rust way" to
                // iterate over a collection, not important for now
                if face.len() > 0 {
                    faces.push(face)
                }
            }
            //else {
            //    println!("ELSE {:?}", line);
            //}
        }
        //println!("{:?}", vertices);
        //println!("{:?}", faces);
        Model { vertices, faces }
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
}
