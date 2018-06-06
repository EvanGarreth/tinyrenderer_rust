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
    texture_coords: &Vec<cgmath::Vector3<f64>>,
    zbuffer: &mut Vec<f64>,
    image: &mut tgaimage::TGAImage,
    diffuse: &mut tgaimage::TGAImage,
    intensity: f64,
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

            // use this to compare to the current value in the zbuffer
            point.z += points[0].z * barycentric_screen.x + points[1].z * barycentric_screen.y
                + points[2].z * barycentric_screen.z;

            let index = (point.x + point.y * width) as usize;
            // draw the point if it is closer to the screen than the current zbuffer value
            if zbuffer[index] < point.z {
                zbuffer[index] = point.z;

                // interpolate the vertices w/ barycentric coords to determine the points x,y
                let mut uv = barycentric_screen.x * texture_coords[0]
                    + barycentric_screen.y * texture_coords[1]
                    + barycentric_screen.z * texture_coords[2];

                // grab the pixel color from the diffuse map
                let mut color = diffuse.get(uv.x as i32, uv.y as i32);

                // multiply by the intensity for basic lighting
                for i in 0..3 {
                    color.bgra[i] = (color.bgra[i] as f64 * intensity) as u8;
                }
                // fix alpha
                color.bgra[3] = 255;

                image.set(x, y, &mut color);
            }
        }
    }
}

fn main() {
    let object = obj::Model::new("src/assets/head.obj");
    let light_dir = cgmath::vec3(0., 0., -1.);
    let camera = cgmath::vec3(0., 0., 3.0);
    let depth = 255.;
    let width = 800;
    let height = 800;

    unsafe {
        let mut image =
            tgaimage::TGAImage::new1(width, height, tgaimage::TGAImage_Format::RGBA as i32);
        // width/height are 0 because TGAImage_read_tga_file will overwrite the values anyway
        let mut diffuse =
            tgaimage::TGAImage::new1(0, 0, tgaimage::TGAImage_Format::RGBA as i32);
        tgaimage::TGAImage_read_tga_file(
            &mut diffuse,
            CString::new("src/assets/head_diffuse.tga")
                .unwrap()
                .as_ptr(),
        );
        diffuse.flip_vertically();

        let height = image.get_height() as f64;
        let width = image.get_width() as f64;

        let mut zbuffer = vec![std::f64::MIN; (width * height) as usize];

        for face in &object.faces {
            // holds the objects vertex coords manipulated to fit within the image bounds
            let mut screen_coords: Vec<cgmath::Vector3<f64>> = Vec::new();
            // the coords of the object as given
            let mut world_coords: Vec<cgmath::Vector3<f64>> = Vec::new();
            let mut texture_coords: Vec<cgmath::Vector3<f64>> = Vec::new();

            let (x,y) = (width/8., height/8.);
            let (w,h) = (width*(3./4.), height*(3./4.));

            // converts the world coords to x,y screen coordinates and a z depth buffer
            let viewport = cgmath::Matrix4::from_cols(
                cgmath::vec4(w/2., 0., 0., 0.),
                cgmath::vec4(0., h/2., 0., 0.),
                cgmath::vec4(0., 0., depth/2., 0.),
                cgmath::vec4(x+w/2., y+h/2., depth/2., 1.),
            );

            // line from the camera origin to the vector point, projecting a new point where the
            // line intersects with the image plane
            let projection_matrix = cgmath::Matrix4::from_cols(
                cgmath::vec4(1., 0., 0., 0.),
                cgmath::vec4(0., 1., 0., 0.),
                cgmath::vec4(0., 0., 1., -1. / camera.z),
                cgmath::vec4(0., 0., 0., 1.),
            );

            for vector in &face.vertices {
                let vector = *object.get_vertex(*vector);
                world_coords.push(vector);
                let projection = (viewport*projection_matrix)*vector.extend(1.);
                // project back to 3d by dividing by w and then dropping w
                let result = (projection / projection.w).truncate();
                screen_coords.push(result);
            }

            for indice in &face.texture_indices {
                let coord = *object.get_texture_coord(*indice);
                texture_coords.push(cgmath::vec3(
                    coord.x * (diffuse.get_width() as f64),
                    coord.y * (diffuse.get_height() as f64),
                    1.,
                ));
            }

            // normalize the cross product of the two sides of the current triangle and scale the
            // light_dir vector by it to determine the intensity of the color of the triangle
            let n = (world_coords[2] - world_coords[0])
                .cross(world_coords[1] - world_coords[0])
                .normalize();
            let scalar = n.dot(light_dir);
            if scalar >= 0. {
                // let intensity = (scalar * 255.) as u8;
                //let mut color = tgaimage::TGAColor::new1(intensity, intensity, intensity, 255);
                triangle(
                    &screen_coords,
                    &texture_coords,
                    &mut zbuffer,
                    &mut image,
                    &mut diffuse,
                    scalar,
                );
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
