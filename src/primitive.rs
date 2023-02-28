struct Point {
    id: i32,
    x: f64,
    y: f64,
    z: f64,
    is_valid: bool,
}

struct Face {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
}

struct Cube {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    p5: Point,
    p6: Point,
    p7: Point,
    p8: Point,
}

fn generate_cube(p1: Point, p2: Point, p3: Point, p4: Point, p5: Point, p6: Point, p7: Point, p8: Point) -> Cube {
    Cube {
        p1: p1,
        p2: p2,
        p3: p3,
        p4: p4,
        p5: p5,
        p6: p6,
        p7: p7,
        p8: p8,
    }
}

fn generate_cube_from_faces(face1: Face, face2: Face) -> Cube {
    let p1 = face1.p1;
    let p2 = face1.p2;
    let p3 = face1.p3;
    let p4 = face1.p4;
    let p5 = face2.p1;
    let p6 = face2.p2;
    let p7 = face2.p3;
    let p8 = face2.p4;

    Cube {
        p1: p1,
        p2: p2,
        p3: p3,
        p4: p4,
        p5: p5,
        p6: p6,
        p7: p7,
        p8: p8,
    }
}

fn split_face_to_triangles(face: Face) -> (FaceTriangle, FaceTriangle) {
    let p1 = face.p1;
    let p2 = face.p2;
    let p3 = face.p3;
    let p4 = face.p4;

    // On crée deux triangles à partir des points de la face carrée
    let triangle1 = FaceTriangle {
        p1: p1,
        p2: p2,
        p3: p3,
    };

    let triangle2 = FaceTriangle {
        p1: p1,
        p2: p3,
        p3: p4,
    };

    // On retourne la paire de triangles
    (triangle1, triangle2)
}
