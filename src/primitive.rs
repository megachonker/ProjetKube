use std::{vec};
use nalgebra::{Point3};
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
struct Vertex<'a> { coord: Point3<f64>,
                    face_list:Vec<Rc<Face<'a>>>
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
struct Face<'a> {vertices:[&'a Rc<Vertex<'a>>;3]}

impl <'a>Face<'a> {
    // constructeur qui va initialiser les valeur des face dans le vertex
    fn new(a: &'a Rc<RefCell<Vertex<'a>>>, b: &'a Rc<RefCell<Vertex<'a>>>, c: &'a Rc<RefCell<Vertex<'a>>>) -> Rc<Self> {
        let face = Rc::new(Self {
            vertices: [a.borrow().clone(), b.borrow().borrow().clone(), c.borrow().clone()],
        });
        a.borrow_mut().face_list.push(face.clone());
        b.borrow_mut().face_list.push(face.clone());
        c.borrow_mut().face_list.push(face.clone());
        face
    }
    
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
    // fn add_vertex<'a>(&'a self, point: & mut Vertex) -> Face{ //<= je veux un retour
    //     let ret;

    //     let A = point.dist(self.vertex[0]);
    //     let B = point.dist(self.vertex[1]);
    //     let C = point.dist(self.vertex[2]);
    //     if A > B && B > C{ ret = Face {vertex: [self.vertex[0],self.vertex[1],point]}; }
    //     else if B > A && A > C{ ret = Face {vertex: [self.vertex[1],self.vertex[2],point]}; }
    //     else if C > A && A > B{ ret = Face {vertex: [self.vertex[2],self.vertex[1],point]}; }
    //     else { ret = Face {vertex: [point,point,point]};}

    //     // let value = Face {vertex:  [self.vertex[2],self.vertex[1],&point]};
    //     // point.face_list.push(&value)   ;     
    //     // let lol = &ret::clone();
    //     // let mut truc = Face {vertex: [&point,&point,&point]};
    //     // &truc.push(&ret);
    //     // ret.vertex[0].face_list.push(&ret);

    //     // let truc = &ret;
    //     // ret.vertex[0].face_list.push(&ret);

    //     // for point in ret.vertex  {
    //     //     point.face_list.push(&ret);
    //     // }
    //     ret
    // }

    fn print(&self){
        for vertex in &self.vertices {
            vertex.print(); println!();
        }
    }
}


struct Polygons<'a>{faces: [&'a Face<'a>;2]}

impl <'a>Polygons<'a> {
    /// need malloc
    // fn new_from_vertex(&self, a: Vertex,b: Vertex,c: Vertex,d: Vertex){
    //     let aa = Face::new(a,b,c);
    //     let bb = Face::new(&d, &b, &b);
    //     new_from_faces(aa,bb);  
    // }
    fn new_from_faces(a:&'a Face<'a>,b:&'a Face<'a>) -> Self{
        // Self{
        //     faces = vec![a,b]
        // }
        Self { faces: [a,b] }
    }

    fn print(&self){
        for face in self.faces {
            face.print(); println!();
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



/// A---B
/// |   |
/// C---D
/// 
/// A---B
/// | /
/// C
/// 
/// +
///   B
///  /|
/// C-D

pub fn test() {
    
    let a = Rc::new(Vertex {coord: Point3::new(1.0,2.0,3.0), face_list: vec![]});
    let b = Rc::new(Vertex {coord: Point3::new(11.0,22.0,33.0), face_list: vec![]});
    let c = Rc::new(Vertex {coord: Point3::new(11.0,21.0,31.0), face_list: vec![]});
    let d = Rc::new(Vertex {coord: Point3::new(1.1,2.2,3.3), face_list: vec![]});

    // let face = Face::new(&a, &b,&c);
    // let face2 = Face::new(&d, &b,&c);
    
    // let polycon = Polygons::new_from_faces(&face, &face2);

    // polycon.print();
}

 //cube => 4 face => 1 face ces N point 