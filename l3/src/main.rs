extern crate cgmath;
extern crate rand;
extern crate tgaimage_sys;

mod obj;
use cgmath::InnerSpace;
use std::ffi::CString;
use tgaimage_sys as tgaimage;

fn barycentric(
    points: &Vec<cgmath::Vector3<f64>>,
    point: cgmath::Vector3<f64>,
) -> cgmath::Vector3<f64> {
    let v1 = cgmath::vec3(
        points[2].x - points[0].x,
        points[1].x - points[0].x,
        points[0].x - point.x,
    );
    let v2 = cgmath::vec3(
        points[2].y - points[0].y,
        points[1].y - points[0].y,
        points[0].y - point.y,
    );
    let u = v1.cross(v2);
    // degenerate triangle
    if u.z.abs() < 0. {
        return cgmath::vec3(-1., 1., 1.);
    }
    // convert to floats for a precise result
    return cgmath::vec3(1. - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
}

unsafe fn triangle(
    points: &Vec<cgmath::Vector3<f64>>,
    zbuffer: &mut Vec<f64>,
    image: &mut tgaimage::TGAImage,
    color: &mut tgaimage::TGAColor,
) {
    let width = image.get_width() as f64;
    let height = image.get_height() as f64;
    let mut bounding_box_min = cgmath::vec2(std::f64::MAX, std::f64::MAX);
    let mut bounding_box_max = cgmath::vec2(std::f64::MIN, std::f64::MIN);

    // use a clamp to keep triangles within max image bounds ( dont draw triangles with coords
    // outside the image range)
    let clamp = cgmath::vec2(width - 1., height - 1.);

    //determine the min/max x and y values to determine the bounds to draw in
    for i in 0..3 {
        for j in 0..2 {
            let _min = points[i][j].min(bounding_box_min[j]);
            let _max = points[i][j].max(bounding_box_max[j]);
            bounding_box_min[j] = _min.max(0.0);
            bounding_box_max[j] = _max.min(clamp[j]);
        }
    }

    let (x_min, y_min) = (bounding_box_min.x as i32, bounding_box_min.y as i32);
    let (x_max, y_max) = (bounding_box_max.x as i32 + 1, bounding_box_max.y as i32 + 1);

    // check all pixels in the resulting bounding box and color them if they lay within a triangle
    for x in x_min..x_max {
        for y in y_min..y_max {
            let mut point = cgmath::vec3(x as f64, y as f64, 0.);
            let barycentric_screen = barycentric(points, point);
            // if any of x,y,z are negative then point is not inside the triangle
            if barycentric_screen.x < 0. || barycentric_screen.y < 0. || barycentric_screen.z < 0. {
                continue;
            }
            // this bit of math I don't fuly understand
            point.z += points[0].z * barycentric_screen.x + points[1].z * barycentric_screen.y
                + points[2].z * barycentric_screen.z;
            let index = (point.x + point.y * width) as usize;
            // draw the point if it is closer to the screen than the current zbuffer value
            if zbuffer[index] < point.z {
                zbuffer[index] = point.z;
                image.set(x, y, color);
            }
        }
    }
}

fn main() {
    let object = obj::Model::new("src/assets/head.obj");
    let light_dir = cgmath::vec3(0., 0., -1.);
    unsafe {
        let mut image =
            tgaimage::TGAImage::new1(2000, 2000, tgaimage::TGAImage_Format::RGBA as i32);

        let height = image.get_height() as f64;
        let width = image.get_width() as f64;

        let mut zbuffer = vec![std::f64::MIN; (width * height) as usize];

        for face in &object.faces {
            // holds the objects vertex coords manipulated to fit within the image bounds
            let mut screen_coords: Vec<cgmath::Vector3<f64>> = Vec::new();
            // the coords of the object as given
            let mut world_coords: Vec<cgmath::Vector3<f64>> = Vec::new();

            for vector in face {
                let vector = *object.get_vertex(*vector as usize);
                world_coords.push(vector);
                // fit the x,y world coords to the bounds of the 2d image
                screen_coords.push(cgmath::vec3(
                    (vector.x + 1.) * width / 2.,
                    (vector.y + 1.) * height / 2.,
                    vector.z,
                ));
            }
            // normalize the cross product of the two sides of the current triangle and scale the
            // light_dir vector by it to determine the intensity of the color of the triangle
            let n = (world_coords[2] - world_coords[0])
                .cross(world_coords[1] - world_coords[0])
                .normalize();
            let scalar = n.dot(light_dir);
            if scalar >= 0. {
                let intensity = (scalar * 255.) as u8;
                let mut color = tgaimage::TGAColor::new1(intensity, intensity, intensity, 255);
                triangle(&screen_coords, &mut zbuffer, &mut image, &mut color);
            }
        }

        tgaimage::TGAImage_flip_vertically(&mut image);
        tgaimage::TGAImage_write_tga_file(
            &mut image,
            CString::new("output.tga").unwrap().as_ptr(),
            true,
        );
    }
}
