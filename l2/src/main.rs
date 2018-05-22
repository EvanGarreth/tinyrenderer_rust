extern crate cgmath;
extern crate tgaimage_sys;

use std::ffi::CString;
use tgaimage_sys as tgaimage;

fn barycentric(points: &Vec<cgmath::Vector2<i32>>, x: i32, y: i32) -> cgmath::Vector3<f64> {
    
    let v1 = cgmath::vec3(points[2].x-points[0].x,
                          points[1].x-points[0].x,
                          points[0].x-x,
                          );
    let v2 = cgmath::vec3(points[2].y-points[0].y,
                          points[1].y-points[0].y,
                          points[0].y-y,
                          );
    let u = v1.cross(v2);
    if u.z.abs() < 1 { return cgmath::vec3(-1., 1., 1.); } //degenerate triangle
    // convert to floats for a precise result
    return cgmath::vec3(1. - (u.x as f64 + u.y as f64) / u.z as f64,
                        u.y as f64 / u.z as f64,
                        u.x as f64 / u.z as f64,
                        );
}

unsafe fn triangle(
    points: &Vec<cgmath::Vector2<i32>>,
    image: &mut tgaimage::TGAImage,
    color: &mut tgaimage::TGAColor,
) {
    //points.sort_by(|a, b| a.y.cmp(&b.y));
    let mut bounding_box_min = cgmath::vec2(image.get_width() - 1, image.get_height() - 1);
    let mut bounding_box_max = cgmath::vec2(0, 0);

    // use a clamp to keep triangles within max image bounds ( dont draw triangles with coords
    // outside the image range)
    let clamp = cgmath::vec2(image.get_width() - 1, image.get_height() - 1);

    //determine the min/max x and y values to determine the bounds to draw in
    for i in 0..3 {
        for j in 0..2 {
            bounding_box_min[j] =
                std::cmp::max(0, std::cmp::min(bounding_box_min[j], points[i][j]));
            bounding_box_max[j] =
                std::cmp::min(clamp[j], std::cmp::max(bounding_box_max[j], points[i][j]));
        }
    }

    // check all pixels in the resulting bounding box and color them if they lay within a triangle
    for x in bounding_box_min.x..bounding_box_max.x+1 {
        for y in bounding_box_min.y..bounding_box_max.y+1 {
            let barycentric_screen = barycentric(&points, x, y);
            // if any of x,y,z are negative then point is not inside the triangle
            if barycentric_screen.x < 0. || barycentric_screen.y < 0. || barycentric_screen.z < 0. {
                continue;
            }
            image.set(x, y, color);
        }
    }
}

fn main() {
    let t1 = vec![
        cgmath::vec2(10, 70),
        cgmath::vec2(50, 160),
        cgmath::vec2(70, 80),
    ];

    let t2 = vec![
        cgmath::vec2(180, 50),
        cgmath::vec2(150, 1),
        cgmath::vec2(70, 180),
    ];

    let t3 = vec![
        cgmath::vec2(180, 150),
        cgmath::vec2(120, 160),
        cgmath::vec2(130, 180),
    ];

    unsafe {
        let mut image = tgaimage::TGAImage::new1(200, 200, tgaimage::TGAImage_Format::RGBA as i32);

        let mut white: tgaimage::TGAColor = tgaimage::TGAColor::new1(255, 255, 255, 255);
        let mut red: tgaimage::TGAColor = tgaimage::TGAColor::new1(255, 0, 0, 255);
        let mut green: tgaimage::TGAColor = tgaimage::TGAColor::new1(0, 255, 0, 255);

        triangle(&t1, &mut image, &mut red);
        triangle(&t2, &mut image, &mut white);
        triangle(&t3, &mut image, &mut green);

        tgaimage::TGAImage_flip_vertically(&mut image);
        tgaimage::TGAImage_write_tga_file(
            &mut image,
            CString::new("output.tga").unwrap().as_ptr(),
            true,
        );
    }
}
