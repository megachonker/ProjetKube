struct Vertex<'b> { x: i32,y: i32,z: i32,
                    face_list:&'b [Face<'b>]
}

impl Vertex<'_> {
    fn print(&self){
        print!("x:{} y:{} z:{}",self.x,self.y,self.z);
    }
}
struct Face<'a> {vertex:[Vertex<'a>;3]}

impl Face<'_> {
    fn print(&self){
        for vertex in &self.vertex {
            vertex.print(); println!();
        }
    }
}

pub fn test() {
    let a = Vertex {x: 1,y: 2, z:3, face_list: &[]};
    let b = Vertex {x: 11,y: 11, z:11, face_list: &[]};
    let c = Vertex {x: 22,y: 22, z:22, face_list: &[]};

    let face = Face {vertex: [a,b,c]};

    face.print();
}

 //cube => 4 face => 1 face ces N point 