use std::{vec};
use nalgebra::{Point3};

struct Vertex<'a> { coord: Point3<f64>,
                    face_list:Vec<&'a Face<'a>>
}

impl Vertex<'_> {
    //calculate distance
    fn dist(&self,vertex: &Vertex) -> f64{
        let distance = (self.coord - vertex.coord).norm();
        print!("Distance {}",&distance);
        distance
    }

    fn print(&self){
        print!("x:{} y:{} z:{}",self.coord.x,self.coord.y,self.coord.z);
    }
}
struct Face<'a> {vertex:[&'a Vertex<'a>;3]}

impl Face<'_> {
    /// ajoute une face a la distance la plus éloigner
    /// 
    /// A
    /// | \
    /// |   C
    /// | /
    /// B     D(new vertex)
    /// 
    /// A
    /// | \
    /// |   C
    /// | /  \
    /// B  -- D(new face C-B-D)
    /// 
    /// calcule distance 
    fn add_vertex<'a>(&'a self, azer: &'a Vertex) -> Face {
        let mut ret;

        let A = azer.dist(self.vertex[0]);
        let B = azer.dist(self.vertex[1]);
        let C = azer.dist(self.vertex[2]);
        if A > B && B > C{ ret = Face {vertex: [self.vertex[0],self.vertex[1],&azer]}; }
        else if B > A && A > C{ ret = Face {vertex: [self.vertex[1],self.vertex[2],&azer]}; }
        else if C > A && A > B{ ret = Face {vertex: [self.vertex[2],self.vertex[1],&azer]}; }
        else { ret = Face {vertex: [&azer,&azer,&azer]};}
        ret
    }

    fn print(&self){
        for vertex in &self.vertex {
            vertex.print(); println!();
        }
    }
}

struct Mesh<'a>{
    vertices: Vec<Vertex<'a>>,
    faces: Vec<Face<'a>>
}

// impl Mesh<'_> {
//     fn add_face(&mut self, face: Face) {
//         self.faces.push(face);
//     }
// }

pub fn test() {
    
    let a = Vertex {coord: Point3::new(1.0,2.0,3.0), face_list: vec![]};
    let b = Vertex {coord: Point3::new(11.0,22.0,33.0), face_list: vec![]};
    let c = Vertex {coord: Point3::new(11.0,21.0,31.0), face_list: vec![]};

    let face = Face {vertex: [&a,&b,&c]};
    let d = Vertex {coord: Point3::new(1.1,2.2,3.3), face_list: vec![]};

    let face2 = face.add_vertex(&d);
    //  Face {vertex: [&b,&c,&d]};

    println!("face1");
    face.print();
    println!("face2");
    face2.print();
}

 //cube => 4 face => 1 face ces N point 