use std::env::args;

use image::{open, GrayImage};

fn main() {
    let args: Vec<String> = args().collect();
    let mut img: GrayImage = open(&args[1]).unwrap().into_luma8();
    into_binary(&mut img, &args);
    img.save(&args[2]).unwrap();
}

fn into_binary(img: &mut GrayImage, args: &Vec<String>) {
    let threshold = find_threshold_value(img, args);
    for p in img.pixels_mut() {
        if p.0[0] > threshold {
            p.0[0] = 255;
        } else {
            p.0[0] = 0;
        }
    }
}

fn find_threshold_value(img: &mut GrayImage, args: &Vec<String>) -> u8 {
    let mut initial_t: i32 = 128;
    match args.len() {
        4 => {
            initial_t = args[3].parse().unwrap();
        }
        _ => {}
    }

    let mut t = 0 + initial_t;
    loop {
        let mut g1: i32 = 0;
        let mut g1_i: i32 = 1;
        let mut g2: i32 = 0;
        let mut g2_i: i32 = 1;

        for p in img.pixels_mut() {
            if p.0[0] > t as u8 {
                g1 += p.0[0] as i32;
                g1_i += 1;
            } else {
                g2 += p.0[0] as i32;
                g2_i += 1;
            }
        }

        let m1 = g1 / g1_i;
        let m2 = g2 / g2_i;

        let t_new = (m1 + m2) / 2;
        let selisih_abs: i32 = (t - t_new).abs();
        let selisih_t: i32 = (t - initial_t).abs();

        println!("kyaa");
        println!(
            "selisih_abs: {} selisih_t: {} T: {}",
            selisih_abs, selisih_t, t_new
        );
        if !(selisih_abs > selisih_t) {
            break;
        } else {
            t = t_new;
        }
    }

    t as u8
}
