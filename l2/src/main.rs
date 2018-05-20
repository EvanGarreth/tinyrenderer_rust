extern crate tgaimage_sys;
use std::ffi::CString;
use tgaimage_sys as tgaimage;

mod geometry;

fn line(
    p1: geometry::Point,
    p2: geometry::Point,
    image: &mut tgaimage::TGAImage,
    color: &mut tgaimage::TGAColor,
) {
    let mut x1 = p1.get_x();
    let mut x2 = p2.get_x();
    let mut y1 = p1.get_y();
    let mut y2 = p2.get_y();

    let x_diff = x1 - x2;
    let y_diff = y1 - y2;
    let steep = x_diff.abs() < y_diff.abs();
    // transpose the image
    if steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
    }

    let dx = x2 - x1;
    let dy = y2 - y1;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y1;
    for x in x1..x2 {
        unsafe {
            if steep {
                // remove the transpose
                tgaimage::TGAImage_set(image, y, x, color);
            } else {
                tgaimage::TGAImage_set(image, x, y, color);
            }
        }
        error2 += derror2;
        if error2 > dx {
            y += if y2 > y1 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

unsafe fn triangle(
    p1 : geometry::Point,
    p2 : geometry::Point,
    p3 : geometry::Point,
    image: &mut tgaimage::TGAImage,
    color: &mut tgaimage::TGAColor,
)
{
    line(p1, p2, image, color);
    line(p2, p3, image, color);
    line(p3, p1, image, color);
}


fn main() {
    let x1 = geometry::Point::new(10, 70, 1);
    let x2 = geometry::Point::new(50, 160, 1);
    let x3 = geometry::Point::new(70, 80, 1);

    let a1 = geometry::Point::new(180, 50, 1);
    let a2 = geometry::Point::new(150, 1, 1);
    let a3 = geometry::Point::new(70, 180, 1);

    let b1 = geometry::Point::new(180, 150, 1);
    let b2 = geometry::Point::new(120, 160, 1);
    let b3 = geometry::Point::new(130, 180, 1);



    unsafe {
        let mut image = tgaimage::TGAImage::new1(200, 200, tgaimage::TGAImage_Format::RGBA as i32);

        let mut white: tgaimage::TGAColor = tgaimage::TGAColor::new1(255, 255, 255, 255);
        let mut red: tgaimage::TGAColor = tgaimage::TGAColor::new1(255, 0, 0, 255);
        let mut green: tgaimage::TGAColor = tgaimage::TGAColor::new1(0, 255, 0, 255);


        triangle(x1, x2, x3, &mut image, &mut red);
        triangle(a1, a2, a3, &mut image, &mut white);
        triangle(b1, b2, b3, &mut image, &mut green);

        tgaimage::TGAImage_flip_vertically(&mut image);
        tgaimage::TGAImage_write_tga_file(
            &mut image,
            CString::new("output.tga").unwrap().as_ptr(),
            true,
        );
    }
}
