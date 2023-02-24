// use nalgebra::;

struct Point{
    x: i64,
    y: i64,
    z: i64,
}

struct vecteur{
    x: i64,
    y: i64,
    z: i64,
}

fn gen_normal(){
    let pt2 = Point {x:1,y:-1,z:0};
    let pt3 = Point {x:1,y:1,z:0};
    let pt6 = Point {x:1,y:-1,z:1};
    
    
    let vec1 : vecteur = vecteur{x: pt2.x-pt3.x,y: pt2.y-pt3.y,z: pt2.z-pt3.z};
    println!("p2-p3 = ({},{},{}) - ({},{},{})= (x:{} y:{} z:{})",pt2.x,pt2.y,pt2.z,pt6.x,pt6.y,pt6.z,vec1.x,vec1.y,vec1.z);

    let vec2 : vecteur = vecteur { x: pt2.x-pt6.x, y: pt2.y-pt6.y,z: pt2.z-pt6.z};
    println!("p2-p6 = ({},{},{}) - ({},{},{})= (x:{} y:{} z:{})",pt2.x,pt2.y,pt2.z,pt6.x,pt6.y,pt6.z,vec2.x,vec2.y,vec2.z);


    let n : vecteur = vecteur { x: vec1.y * vec2.z, y: vec1.z *  vec2.x, z: vec1.x *  vec2.y };

    println!("n is = (x:{} y:{} z:{})",n.x,n.y,n.z);
    
    let d = n.x*(pt3.x) + n.y*(pt3.y) + n.z*(pt3.z) ;
    println!("d={}",d);
}

pub fn test() {
    gen_normal(); 
}