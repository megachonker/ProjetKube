struct Point { x: i32,y: i32,z: i32,}
struct Maille {point0: Point,point1: Point,point2: Point,point3: Point,point4: Point,point5: Point,point6: Point,point7: Point,}


fn create_point( x: i32, y: i32,z: i32) -> Point {
    Point {x, y,z }
}


fn create_maille(point0: Point,point1: Point,point2: Point,point3: Point,point4: Point,point5: Point,point6: Point,point7: Point) -> Maille {
     Maille { point0, point1, point2, point3,point4, point5, point6, point7 }
}



fn main() {
    let point0:Point = create_point(-1, -1,  0);
    let point1:Point = create_point(-1, 1,  0);
    let point2:Point = create_point(1, -1,  1);
    let point3:Point = create_point(1, 1,  0);
    let point4:Point = create_point(-1, -1,  1);
    let point5:Point = create_point(-1, 1,  1);
    let point6:Point = create_point(1, -1,  1);
    let point7:Point = create_point(1, 1,  0);
 
    let maille:Maille=create_maille(point0 ,point1 ,point2 ,point3 ,point4 ,point5 ,point6 ,point7 );
    

 }