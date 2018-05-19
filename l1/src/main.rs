extern crate tgaimage_sys;
use tgaimage_sys as tgaimage;
use std::ffi::CString;

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut tgaimage::TGAImage, color: &mut tgaimage::TGAColor) {
    let x_diff = x0-x1;
    let y_diff = y0-y1;
    let steep = x_diff.abs() < y_diff.abs();
    // transpose the image
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0>x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1-x0;
    let dy = y1-y0;
    let derror2 = dy.abs() *2;
    let mut error2 = 0;
    let mut y = y0;
    for x in x0..x1 {
        unsafe {
            if steep {
                // remove the transpose
                tgaimage::TGAImage_set(image, x, y, color);
            } else {
                tgaimage::TGAImage_set(image, y, x, color);
            }
        }
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx *2;
         }
    }
}

fn main()
{
    unsafe {
        let mut image = tgaimage::TGAImage::new1(100,100, tgaimage::TGAImage_Format::RGBA as i32);
   
        let mut white: tgaimage::TGAColor = tgaimage::TGAColor::new1(255, 255, 255, 255);
        let mut red: tgaimage::TGAColor = tgaimage::TGAColor::new1(255, 0, 0, 255);

        line(13, 20, 80, 40, &mut image, &mut white);
        line(20, 13, 40, 80, &mut image, &mut red);
        line(80, 40, 13, 20, &mut image, &mut red);

        tgaimage::TGAImage_flip_vertically(&mut image);
        tgaimage::TGAImage_write_tga_file(&mut image, CString::new("output.tga").unwrap().as_ptr(), true);
    }
}
