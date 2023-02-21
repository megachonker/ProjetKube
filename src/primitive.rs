struct Point { x: i32,y: i32,z: i32,}
struct Maille {point0: Point,point1: Point,point2: Point,point3: Point,point4: Point,point5: Point,point6: Point,point7: Point,}


fn create_point( x: i32, y: i32,z: i32) -> Point {
    Point {x, y,z }
}


fn create_maille(point0: Point,point1: Point,point2: Point,point3: Point,point4: Point,point5: Point,point6: Point,point7: Point) -> Maille {
     Maille { point0, point1, point2, point3,point4, point5, point6, point7 }
}

fn ppoint(point: Point){
    println!("X:{}\tY:{}\tZ:{}",point.x,point.y,point.z);
}

fn pmaille(maille : Maille){
    ppoint(maille.point0);
    ppoint(maille.point1);
    ppoint(maille.point2);
    ppoint(maille.point3);
}

fn cube() -> Maille{
    let point0 = create_point(-1, -1,  0);
    let point1 = create_point(-1, 1,  0);
    let point2 = create_point(1, -1,  1);
    let point3 = create_point(1, 1,  0);
    let point4 = create_point(-1, -1,  1);
    let point5 = create_point(-1, 1,  1);
    let point6 = create_point(1, -1,  1);
    let point7 = create_point(1, 1,  0);
    create_maille(point0,point1,point2,point3,point4,point5,point6,point7)
}

pub fn test() {

    let cubee = cube();
    pmaille(cubee);    
    
 }