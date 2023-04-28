/*extern crate core;

use std::char::from_u32;
use std::fmt::{Debug, format, Formatter, write};
use std::time::Instant;
use image::GenericImageView;
use image::imageops::crop;
use image::io::Reader as ImageReader;

fn main() {
    match_image("1.png", "holder2.png");
}

fn match_image(match_image_path: &'static str, holder_image_path: &'static str) {
    let match_image = ImageReader::open(match_image_path).unwrap().decode().unwrap();
    let holder_image = ImageReader::open(holder_image_path).unwrap().decode().unwrap();

    let start = Instant::now();

    let mut match_matrix = Matrix::new(match_image.width() as usize, match_image.height() as usize, 0);
    for x in 0..match_image.width() {
        for y in 0..match_image.height() {
            if match_image.get_pixel(x, y).0 == [0, 0, 0, 255] {
                match_matrix.set(x as usize, y as usize, 1);
            }
        }
    }

    let mut holder_matrix = Matrix::new(holder_image.width() as usize, holder_image.height() as usize, 0);
    for x in 0..holder_image.width() {
        for y in 0..holder_image.height() {
            if holder_image.get_pixel(x ,y).0 == [255, 255, 255, 255] {
                holder_matrix.set(x as usize, y as usize, 1);
            }
        }
    }

    for x in 0..holder_matrix.width-match_matrix.width {
        for y in 0..holder_matrix.height-match_matrix.height {
            let crop_out = holder_matrix.crop(x, y, match_matrix.width, match_matrix.height);

            let mut matches = true;
            for x in 0..match_matrix.width {
                for y in 0..match_matrix.height {
                    if match_matrix.get(x ,y) == 1 {
                        if crop_out.get(x, y) != 1 {
                            matches = false;
                        }
                    }
                }
            }

            if matches {
                println!("{x}, {y}");
            }
        }
    }
    println!("took: {}", start.elapsed().as_millis())
}

#[derive(PartialEq)]
struct Matrix {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(width: usize, height: usize, default: u8) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    fn from_slice(width: usize, height: usize, data: &[u8]) -> Self {
        if data.len() != width * height {
            panic!("The provided data doesn't match the size of the matrix!");
        }

        Self {
            data: Vec::from(data),
            width,
            height
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let index = self.width * y + x;
        self.data[index]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        let index = self.width * y + x;
        self.data[index] = value;
    }

    fn crop(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Matrix {
        let mut crop_out = Matrix::new(width, height, 0);

        for x in 0..width {
            for y in 0.. height {
                let value = self.get(start_x + x, start_y + y);
                crop_out.set(x, y, value);
            }
        }

        crop_out
    }

    /*fn compare(&self, other: Self, value: u8, value_other: u8) -> bool {
        if self.width != other.width || self.height != other.height {
            panic!("Both matrices should have the same size");
        }

        for x in 0..self.width {
            for y in 0..self.height {

            }
        }

        true
    }*/
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut log: String = "".to_string();
        for y in 0..self.height {
            log += "\t";
            for x in 0..self.width {
                log += (self.get(x, y).to_string() + ", ").as_str();
            }
            log += "\n";
        }
        write!(f, "[\n{}]", log)
    }
}

#[test]
fn debug_test() {
    let test_matrix = Matrix::from_slice(2, 2, &[0, 1, 2 ,3]);
    let log = format!("{:?}", test_matrix);
    assert_eq!(log, "[\n\t0, 1, \n\t2, 3, \n]");
}

#[test]
fn get_test() {
    let test_matrix = Matrix::from_slice(3, 2,&[1, 2, 3, 7, 8, 9]);
    assert_eq!(test_matrix.get(0, 0), 1);
    assert_eq!(test_matrix.get(1, 0), 2);
    assert_eq!(test_matrix.get(2, 0), 3);
    assert_eq!(test_matrix.get(0, 1), 7);
    assert_eq!(test_matrix.get(1, 1), 8);
    assert_eq!(test_matrix.get(2, 1), 9);
}

#[test]
fn set_test() {
    let mut test_matrix = Matrix::new(3, 3, 0);
    test_matrix.set(1, 1, 1);
    assert_eq!(test_matrix.get(1, 1), 1);
}

#[test]
fn crop_test() {
    let mut test_matrix = Matrix::from_slice(5, 5, &[
        1,  2,  3,  4,  5 ,
        6,  7,  8,  9,  10,
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20,
        21, 22, 23, 24, 25,
    ]);

    let crop_out = test_matrix.crop(1, 1, 3, 3);

    assert_eq!(crop_out, Matrix::from_slice(3, 3, &[
        7, 8, 9,
        12, 13, 14,
        17, 18, 19
    ]));
}*/


// 740 on avarage
/*
extern crate core;

use std::char::from_u32;
use std::fmt::{Debug, format, Formatter, write};
use std::time::Instant;
use image::GenericImageView;
use image::imageops::crop;
use image::io::Reader as ImageReader;

fn main() {
    let start = Instant::now();
    let mut numbers: Vec<(usize, u8)> = vec![];

    let _ = match_image("0.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 0));
    });
    let _ = match_image("1.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 1));
    });
    let _ = match_image("2.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 2));
    });
    let _ = match_image("3.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 3));
    });
    let _ = match_image("4.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 4));
    });
    let _ = match_image("5.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 5));
    });
    let _ = match_image("6.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 6));
    });
    let _ = match_image("7.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 7));
    });
    let _ = match_image("8.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 8));
    });
    let _ = match_image("9.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 9));
    });

    sort_numbers(&mut numbers);
    println!("took: {}", start.elapsed().as_millis());
    let mut num_string = "".to_string();
    for number in numbers {
        num_string += number.1.to_string().as_str();
    }

    let num: u128 = num_string.parse().unwrap();
    println!("{num}");
}

fn sort_numbers(numbers: &mut Vec<(usize, u8)>) {
    let mut sorted = false;
    for i in 1..numbers.len() {
        if numbers[i] < numbers[i - 1] {
            numbers.swap(i, i-1);
            sorted = true;
        }
    }
    if sorted {
        sort_numbers(numbers);
    }
}

fn match_image(match_image_path: &'static str, holder_image_path: &'static str) -> Vec<(usize, usize)> {
    let match_image = ImageReader::open(match_image_path).unwrap().decode().unwrap();
    let holder_image = ImageReader::open(holder_image_path).unwrap().decode().unwrap();

    let mut match_matrix = Matrix::new(match_image.width() as usize, match_image.height() as usize, 0);
    for x in 0..match_image.width() {
        for y in 0..match_image.height() {
            if match_image.get_pixel(x, y).0 == [0, 0, 0, 255] {
                match_matrix.set(x as usize, y as usize, 1);
            }
        }
    }

    let mut holder_matrix = Matrix::new(holder_image.width() as usize, holder_image.height() as usize, 0);
    for x in 0..holder_image.width() {
        for y in 0..holder_image.height() {
            if holder_image.get_pixel(x ,y).0 == [255, 255, 255, 255] {
                holder_matrix.set(x as usize, y as usize, 1);
            }
        }
    }

    let mut matches = vec![];

    for x in 0..holder_matrix.width-match_matrix.width {
        for y in 0..holder_matrix.height-match_matrix.height {
            let crop_out = holder_matrix.crop(x, y, match_matrix.width, match_matrix.height);

            let mut matche = true;
            for x in 0..match_matrix.width {
                for y in 0..match_matrix.height {
                    if match_matrix.get(x ,y) == 1 {
                        if crop_out.get(x, y) != 1 {
                            matche = false;
                        }
                    }
                }
            }

            if matche {
                matches.push((x, y));
            }
        }
    }

    matches
}

#[derive(PartialEq)]
struct Matrix {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(width: usize, height: usize, default: u8) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    fn from_slice(width: usize, height: usize, data: &[u8]) -> Self {
        if data.len() != width * height {
            panic!("The provided data doesn't match the size of the matrix!");
        }

        Self {
            data: Vec::from(data),
            width,
            height
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let index = self.width * y + x;
        self.data[index]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        let index = self.width * y + x;
        self.data[index] = value;
    }

    fn crop(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Matrix {
        let mut crop_out = Matrix::new(width, height, 0);

        for x in 0..width {
            for y in 0.. height {
                let value = self.get(start_x + x, start_y + y);
                crop_out.set(x, y, value);
            }
        }

        crop_out
    }

    /*fn compare(&self, other: Self, value: u8, value_other: u8) -> bool {
        if self.width != other.width || self.height != other.height {
            panic!("Both matrices should have the same size");
        }

        for x in 0..self.width {
            for y in 0..self.height {

            }
        }

        true
    }*/
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut log: String = "".to_string();
        for y in 0..self.height {
            log += "\t";
            for x in 0..self.width {
                log += (self.get(x, y).to_string() + ", ").as_str();
            }
            log += "\n";
        }
        write!(f, "[\n{}]", log)
    }
}

#[test]
fn debug_test() {
    let test_matrix = Matrix::from_slice(2, 2, &[0, 1, 2 ,3]);
    let log = format!("{:?}", test_matrix);
    assert_eq!(log, "[\n\t0, 1, \n\t2, 3, \n]");
}

#[test]
fn get_test() {
    let test_matrix = Matrix::from_slice(3, 2,&[1, 2, 3, 7, 8, 9]);
    assert_eq!(test_matrix.get(0, 0), 1);
    assert_eq!(test_matrix.get(1, 0), 2);
    assert_eq!(test_matrix.get(2, 0), 3);
    assert_eq!(test_matrix.get(0, 1), 7);
    assert_eq!(test_matrix.get(1, 1), 8);
    assert_eq!(test_matrix.get(2, 1), 9);
}

#[test]
fn set_test() {
    let mut test_matrix = Matrix::new(3, 3, 0);
    test_matrix.set(1, 1, 1);
    assert_eq!(test_matrix.get(1, 1), 1);
}

#[test]
fn crop_test() {
    let mut test_matrix = Matrix::from_slice(5, 5, &[
        1,  2,  3,  4,  5 ,
        6,  7,  8,  9,  10,
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20,
        21, 22, 23, 24, 25,
    ]);

    let crop_out = test_matrix.crop(1, 1, 3, 3);

    assert_eq!(crop_out, Matrix::from_slice(3, 3, &[
        7, 8, 9,
        12, 13, 14,
        17, 18, 19
    ]));
}*/



extern crate core;

use std::char::from_u32;
use std::fmt::{Debug, format, Formatter, write};
use std::time::Instant;
use image::GenericImageView;
use image::imageops::crop;
use image::io::Reader as ImageReader;

fn main() {
    let start = Instant::now();
    let mut numbers: Vec<(usize, u8)> = vec![];

    match_images("numbers/", "holder2.png");

    sort_numbers(&mut numbers);
    println!("took: {}", start.elapsed().as_millis());
    let mut num_string = "".to_string();
    for number in numbers {
        num_string += number.1.to_string().as_str();
    }

    let num: u128 = num_string.parse().unwrap();
    println!("{num}");
}

fn sort_numbers(numbers: &mut Vec<(usize, u8)>) {
    let mut sorted = false;
    for i in 1..numbers.len() {
        if numbers[i] < numbers[i - 1] {
            numbers.swap(i, i-1);
            sorted = true;
        }
    }
    if sorted {
        sort_numbers(numbers);
    }
}

fn match_images(match_image_path: &'static str, holder_image_path: &'static str) -> Vec<(usize, usize)> {
    let holder_image = ImageReader::open(holder_image_path).unwrap().decode().unwrap();

    let mut match_matrices: Vec<Matrix> = vec![];

    for i in 0..10 {
        let image = ImageReader::open(match_image_path + i.to_string() + ".png").unwrap().decode().unwrap();

        let mut match_matrix = Matrix::new(image.width() as usize, image.height() as usize, 0);
        for x in 0..image.width() {
            for y in 0..image.height() {
                if image.get_pixel(x, y).0 == [0, 0, 0, 255] {
                    match_matrix.set(x as usize, y as usize, 1);
                }
            }
        }

        match_matrices.push(match_matrix);
    }

    let mut holder_matrix = Matrix::new(holder_image.width() as usize, holder_image.height() as usize, 0);
    for x in 0..holder_image.width() {
        for y in 0..holder_image.height() {
            if holder_image.get_pixel(x ,y).0 == [255, 255, 255, 255] {
                holder_matrix.set(x as usize, y as usize, 1);
            }
        }
    }

    let mut matches = vec![];

    for x in 0..holder_matrix.width-match_matrix.width {
        for y in 0..holder_matrix.height-match_matrix.height {
            let crop_out = holder_matrix.crop(x, y, match_matrix.width, match_matrix.height);

            let mut is_match = true;
            for x in 0..match_matrix.width {
                for y in 0..match_matrix.height {
                    if match_matrix.get(x ,y) == 1 {
                        if crop_out.get(x, y) != 1 {
                            is_match = false;
                        }
                    }
                }
            }

            if is_match {
                matches.push((x, y));
            }
        }
    }

    matches
}

#[derive(PartialEq)]
struct Matrix {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(width: usize, height: usize, default: u8) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    fn from_slice(width: usize, height: usize, data: &[u8]) -> Self {
        if data.len() != width * height {
            panic!("The provided data doesn't match the size of the matrix!");
        }

        Self {
            data: Vec::from(data),
            width,
            height
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let index = self.width * y + x;
        self.data[index]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        let index = self.width * y + x;
        self.data[index] = value;
    }

    fn crop(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Matrix {
        let mut crop_out = Matrix::new(width, height, 0);

        for x in 0..width {
            for y in 0.. height {
                let value = self.get(start_x + x, start_y + y);
                crop_out.set(x, y, value);
            }
        }

        crop_out
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut log: String = "".to_string();
        for y in 0..self.height {
            log += "\t";
            for x in 0..self.width {
                log += (self.get(x, y).to_string() + ", ").as_str();
            }
            log += "\n";
        }
        write!(f, "[\n{}]", log)
    }
}

#[test]
fn debug_test() {
    let test_matrix = Matrix::from_slice(2, 2, &[0, 1, 2 ,3]);
    let log = format!("{:?}", test_matrix);
    assert_eq!(log, "[\n\t0, 1, \n\t2, 3, \n]");
}

#[test]
fn get_test() {
    let test_matrix = Matrix::from_slice(3, 2,&[1, 2, 3, 7, 8, 9]);
    assert_eq!(test_matrix.get(0, 0), 1);
    assert_eq!(test_matrix.get(1, 0), 2);
    assert_eq!(test_matrix.get(2, 0), 3);
    assert_eq!(test_matrix.get(0, 1), 7);
    assert_eq!(test_matrix.get(1, 1), 8);
    assert_eq!(test_matrix.get(2, 1), 9);
}

#[test]
fn set_test() {
    let mut test_matrix = Matrix::new(3, 3, 0);
    test_matrix.set(1, 1, 1);
    assert_eq!(test_matrix.get(1, 1), 1);
}

#[test]
fn crop_test() {
    let mut test_matrix = Matrix::from_slice(5, 5, &[
        1,  2,  3,  4,  5 ,
        6,  7,  8,  9,  10,
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20,
        21, 22, 23, 24, 25,
    ]);

    let crop_out = test_matrix.crop(1, 1, 3, 3);

    assert_eq!(crop_out, Matrix::from_slice(3, 3, &[
        7, 8, 9,
        12, 13, 14,
        17, 18, 19
    ]));
}