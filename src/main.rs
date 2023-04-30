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

/*extern crate core;

use std::char::from_u32;
use std::fmt::{Debug, format, Formatter, write};
use std::time::Instant;
use image::GenericImageView;
use image::imageops::crop;
use image::io::Reader as ImageReader;

fn main() {
    let start = Instant::now();
    let mut numbers: Vec<(usize, u8)> = vec![];

    let _ = match_image("numbers/0.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 0));
    });
    let _ = match_image("numbers/1.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 1));
    });
    let _ = match_image("numbers/2.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 2));
    });
    let _ = match_image("numbers/3.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 3));
    });
    let _ = match_image("numbers/4.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 4));
    });
    let _ = match_image("numbers/5.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 5));
    });
    let _ = match_image("numbers/6.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 6));
    });
    let _ = match_image("numbers/7.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 7));
    });
    let _ = match_image("numbers/8.png", "holder2.png").into_iter().for_each(|item| {
        numbers.push((item.0, 8));
    });
    let _ = match_image("numbers/9.png", "holder2.png").into_iter().for_each(|item| {
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
        let crop_out = holder_matrix.crop(x, 24 - match_matrix.height + 1, match_matrix.width, match_matrix.height);

        let mut matching = true;
        for x in 0..match_matrix.width {
            for y in 0..match_matrix.height {
                if match_matrix.get(x ,y) == 1 {
                    if crop_out.get(x, y) != 1 {
                        matching = false;
                    }
                }
            }
        }

        if matching {
            matches.push((x, 24));
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
            for y in 0..height {
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
use std::cmp::{max, min};
use std::fmt::{Debug, format, Formatter, write};
use std::fs;
use std::time::Instant;
use image::{GenericImageView, ImageBuffer, Rgba};
use image::imageops::crop;
use image::io::Reader as ImageReader;

fn generate_grayscale_template(sample_images_folder: &'static str, color: [u8; 4]) -> Matrix {
    let mut total_matrix: Option<Matrix> = None;
    for path in fs::read_dir(sample_images_folder).unwrap() {
        let sample = ImageReader::open(path.unwrap().path()).unwrap().decode().unwrap();
        let mut sample_matrix = Matrix::new(sample.width() as usize, sample.height() as usize, 0);

        for x in 0..sample.width() {
            for y in 0..sample.height() {
                if sample.get_pixel(x, y).0 == color {
                    sample_matrix.set(x as usize, y as usize, 1);
                }
            }
        }
        if total_matrix.clone() == None {
            total_matrix = Some(sample_matrix.clone());
        } else {
            total_matrix = Some(find_best_match(&total_matrix.unwrap(), &sample_matrix));
        }
    }
    total_matrix.unwrap()
}

fn find_best_match(a_matrix: &Matrix, b_matrix: &Matrix) -> Matrix {
    let min_height = min(a_matrix.height, b_matrix.height);
    let max_height = max(a_matrix.height, b_matrix.height);
    let min_width = min(a_matrix.width, b_matrix.width);
    let max_width = max(a_matrix.width, b_matrix.width);

    let smaller_matrix = if a_matrix.data.len() > b_matrix.data.len() {
        b_matrix
    } else {
        a_matrix
    };

    let bigger_matrix = if smaller_matrix == a_matrix {
        b_matrix
    } else {
        a_matrix
    };

    let mut prev_test: Option<Matrix> = None;

    for offset_x in 0..max_width - min_width {
        for offset_y in 0..max_height - min_height {
            let mut test = Matrix::new(smaller_matrix.width, smaller_matrix.height, 0);
            for x in 0..min_width {
                for y in 0..min_height {
                    let small_sample = smaller_matrix.get(x, y) == 1;
                    let big_sample = if bigger_matrix.height < offset_x + x || bigger_matrix.width < offset_y + y {0} else {bigger_matrix.get(offset_x + x, offset_y + y)} == 1;

                    if small_sample && big_sample {
                        test.set(x, y, 1);
                    }
                }
            }
            println!("1");
            if prev_test != None {
                println!("2");
                if test.count(1) > prev_test.clone().unwrap().count(1) {
                    println!("3");
                    prev_test = Some(test);
                }
            } else {
                println!("4");
                prev_test = Some(test);
            }
        }
    }

    if max_width - min_width == 0 || max_height - min_height == 0 {
        let mut test = Matrix::new(smaller_matrix.width, smaller_matrix.height, 0);
        for x in 0..min_width {
            for y in 0..min_height {
                let small_sample = smaller_matrix.get(x, y) == 1;
                let big_sample = bigger_matrix.get(x, y) == 1;

                if small_sample && big_sample {
                    test.set(x, y, 1);
                }
            }
        }
        if prev_test != None {
            if test.count(1) > prev_test.clone().unwrap().count(1) {
                prev_test = Some(test);
            }
        } else {
            prev_test = Some(test);
        }
    }

    prev_test.unwrap()
}

fn main() {
    generate_grayscale_template("8samples", [255, 255, 255, 255]).to_image().save("grayscale_8.png").unwrap();
    return;
    let start = Instant::now();
    let mut numbers = match_images("numbers/", "holder3.png", 29);
    println!("took: {}", start.elapsed().as_millis());
    sort_numbers(&mut numbers);
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

fn match_images(match_image_folder_path: &'static str, holder_image_path: &'static str, text_bottom_line: usize) -> Vec<(usize, u8)> {

    let mut match_matrices: Vec<Matrix> = vec![];

    for i in 0..10 {
        let match_matrix = Matrix::from_image(match_image_folder_path.to_owned()+ i.to_string().as_str() + ".png", [0, 0, 0, 255]);
        match_matrices.push(match_matrix);
    }

    let mut holder_matrix = Matrix::from_image(holder_image_path.to_string(), [255, 255, 255, 255]);

    let mut matches: Vec<(usize, u8)> = vec![];

    let mut skip = 0;

    'holder_x: for mut x in 0..holder_matrix.width {
        x += skip;
        skip = 0;
        if x > holder_matrix.width {
            break 'holder_x;
        }

        'number: for num_index in 0..match_matrices.len() {
            if x + match_matrices[num_index].width >= holder_matrix.width {
                continue 'holder_x;
            }
            let crop_out = holder_matrix.crop(x, text_bottom_line - match_matrices[num_index].height + 1, match_matrices[num_index].width, match_matrices[num_index].height);

            let mut matching = true;
            for x in 0..match_matrices[num_index].width {
                for y in 0..match_matrices[num_index].height {
                    if match_matrices[num_index].get(x ,y) == 1 {
                        if crop_out.get(x, y) != 1 {
                            matching = false;
                            continue 'number;
                        }
                    }
                }
            }

            if matching {
                skip = match_matrices[num_index].width;
                matches.push((x, num_index as u8));
            }
        }
    }

    matches
}

#[derive(Clone, PartialEq)]
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

    fn from_image(path: String, trigger: [u8; 4]) -> Self {
        let image = ImageReader::open(path).unwrap().decode().unwrap();

        let mut matrix = Matrix::new(image.width() as usize, image.height() as usize, 0);
        for x in 0..image.width() {
            for y in 0..image.height() {
                if image.get_pixel(x, y).0 == trigger {
                    matrix.set(x as usize, y as usize, 1);
                }
            }
        }

        matrix
    }

    fn to_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if self.get(x as usize, y as usize) == 1 {
                *pixel = Rgba([0, 0, 0, 255]);
            } else {
                *pixel = Rgba([255, 255, 255, 255]);
            }
        }

        img
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

    fn count(&self, value: u8) -> usize {
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) == value {
                    count += 1;
                }
            }
        }
        count
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