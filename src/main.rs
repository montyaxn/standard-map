use std::{f32::consts::PI, fs::File};

use image::{ImageBuffer, RgbaImage, codecs::gif::GifEncoder};
use rand::Rng;

const N: usize = 1000;
const SCALE: f32 = 100.;
const SAMPLES: usize = 1000;
const RATIO: f32 = 0.9;
const FRAME_NUM: usize = 500;

const IMG_WIDTH: u32 = (2. * PI * SCALE) as u32 + 1;
const IMG_HEIGHT: u32 = (2. * PI * SCALE) as u32 + 1;

fn main() {
    let mut frames = Vec::new();
    let mut k = 3.;
    for index in 0..FRAME_NUM {
        k += 0.01;
        let mut imgbuf: RgbaImage = ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);
        for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgba([255, 255, 255, 255]);
        }

        // for x in 0..imgx {
        //     for y in 0..imgy {
        //         let mut theta = x as f32 / SCALE;
        //         let mut p = y as f32 / SCALE;
        //         for _ in 0..N {
        //             p = p + K * theta.sin();
        //             theta = theta + p;
        //             while p >= 2. * PI {
        //                 p -= 2. * PI;
        //             }
        //             while theta >= 2. * PI {
        //                 theta -= 2. * PI;
        //             }
        //             while p < 0. {
        //                 p += 2. * PI;
        //             }
        //             while theta < 0. {
        //                 theta += 2. * PI;
        //             }
        //         }
        //         let result_x = (theta * SCALE) as u32 % imgx;
        //         let result_y = (p * SCALE) as u32 % imgy;
        //         let pixel = imgbuf.get_pixel_mut(result_x, result_y);
        //         *pixel = image::Rgb([0, 0, 0]);
        //         println!("{}/{}",imgx*x+y,imgx*imgy);
        //     }
        // }

        let mut rng = rand::thread_rng();
        for _ in 0..SAMPLES {
            let mut theta = rng.gen::<f32>() * 2. * PI;
            let mut p = rng.gen::<f32>();
            for _ in 0..N {
                p = p + k * (theta).sin();
                theta = theta*(3.*k).sin() +(1.5*k).cos()* p;
                // p = p + k * (theta).sin();
                // theta = theta*(k/theta).sin() +(1.5*k).cos()* p; strings3
                // p = p + k * (theta).sin();
                // theta = theta*(1./theta).tan() + p; load

                // p = p + k * (theta).sinh() + 0.3;
                // theta = (1./theta+p).sin() + p*p.cos(); maga tama 3d

                // theta = (1./theta+p).sin() + p;　strings2
                // theta = theta*(1./theta).sin()*theta + p; octpus
                // theta = theta*(1./theta).sin() + p; strings
                // theta = 0.8 * theta + k.sin()* p; netflix
                // theta = (0.9+0.8* theta.sin())* theta + k.cos()* p; 
                // theta = (1.+theta.sin())* theta + p; wave
                // while p > 2. * PI {
                //     p -= 2. * PI;
                // }
                // while p < 0. {
                //     p += 2. * PI;
                // }
                while p > PI {
                    p -= 2. * PI;
                }
                while p < -PI {
                    p += 2. * PI;
                }
                
                // while theta >= (2. * PI) {
                //     theta -= 2. * PI;
                // }
                // while theta < 0. {
                //     theta += 2. * PI;
                // }

                while theta > PI {
                    theta -= 2. * PI;
                }
                while theta < -PI {
                    theta += 2. * PI;
                }

                let pixel = imgbuf.get_pixel_mut(
                    ((theta + PI) * SCALE).floor() as u32,
                    ((p + PI) * SCALE).floor() as u32,
                );
                let image::Rgba(data) = *pixel;
                let gray = (data[0] as f32 * RATIO) as u8;
                let data = [gray, gray, gray, 255];

                *pixel = image::Rgba(data);
            }
        }
        let frame =
            image::Frame::from_parts(imgbuf, 0, 0, image::Delay::from_numer_denom_ms(10, 1));
        frames.push(frame);
        println!("{}/{}", index + 1, FRAME_NUM);
    }

    let file_out = File::create("target/out.gif").expect("couldn't make target file");
    let mut encoder = GifEncoder::new(file_out);

    println!("encoding, can open gif even while encoding");
    encoder.encode_frames(frames.into_iter()).unwrap();
}
