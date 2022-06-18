

// // struct Display {
// //     data: Vec<u8>,
// //     width: u8,
// //     height: u8
// // }
// // impl Display {
// //     // escape sequence begin
// //     fn csi() {
// //         print!("{}", 0x9B);
// //     }
// //     fn esc() {
// //         print!("{}", 0x1B);
// //     }
// //     // moves cursor
// //     fn move(row: u8, col: u8) {

// //     }
// //     fn refresh() {
// //         for row in 0..self.height {
// //             for col in 0..self.width {
// //                 self.move(row, col);
// //             }
// //         }
// //     }

// //     // TODO: only update changes not replace all...
// // }

// fn csi() {
//     print!("{}[", 0x1B as char);
// }

// fn r#move(row: u8, col: u8) {
//     csi();
//     print!("{};{}H", row, col);
// }
// fn up(distance: u8) {
//     csi();
//     print!("{}A", distance);
// }
// fn down(distance: u8) {
//     csi();
//     print!("{}B", distance);
// }
// fn forward(distance: u8) {
//     csi();
//     print!("{}C", distance);
// }
// fn backward(distance: u8) {
//     csi();
//     print!("{}D", distance);
// }
// fn color(foreground: u8, background: u8) {
//     csi();
//     print!("38;5;{}m", foreground);  // set foreground red
//     csi();
//     print!("48;5;{}m", background);  // set background red
// }
// fn color_rgb(fr: u8, fg: u8, fb: u8, br: u8, bg: u8, bb: u8) {
//     csi();
//     print!("38;2;{};{};{}m", fr, fg, fb);  // set foreground red
//     csi();
//     print!("48;2;{};{};{}m", br, bg, bb);  // set background red
// }
// fn color_foreground_rgb(r: u8, g: u8, b: u8) {
//     csi();
//     print!("38;2;{};{};{}m", r, g, b);  // set foreground red
// }
// fn color_background_rgb(r: u8, g: u8, b: u8) {
//     csi();
//     print!("48;2;{};{};{}m", r, g, b);  // set background red
// }
// fn reset() {
//     // csi();
//     // print!("39m");  // reset foreground
//     // csi();
//     // print!("49m");  // reset background
//     csi();
//     print!("0m");
// }
// fn clear() {
//     csi();
//     print!("2J");
// }
// fn draw(symbol: char) {
//     print!("{}", symbol);
// }
// fn paint(width: u8, height: u8, frame: &Vec<Pixel>) {
//     hide();
//     r#move(0, 0);
//     for row in (0..height).step_by(2) {
//         for col in 0..(width) {
//             let findex = ((row as u16 * width as u16) + col as u16) as usize;
//             color_foreground_rgb(frame[findex].r, frame[findex].g, frame[findex].b);
//             let bindex = (((row as u16 + 1 as u16) * width as u16) + col as u16) as usize;
//             color_background_rgb(frame[bindex].r, frame[bindex].g, frame[bindex].b);
//             draw('▀');
//         }
//         println!();
//     }
// }
// fn altscreen(switch: bool) {
//     if switch {
//         csi();
//         print!("?1049h");
//     } else {
//         csi();
//         print!("?1049l");
//     }
// }
// fn hide() {
//     csi();
//     print!("8m");
// }

// struct Pixel {
//     r: u8, g: u8, b: u8
// }

// // TODO: try hiding the cursor through block printing
// // aka, instead of writing each char, we create a single string including all the render code and write it all at once...
// // 

// use std::io;
// fn main() {
//     reset();
//     altscreen(true);
//     let width = 128;
//     let height = 72;
//     let mut frame: Vec<Pixel> = Vec::new();
//     let mut delta = 1;
//     for row in 0..(height) {
//         for col in 0..(width) {
//             // let ratio = 255 / width;
//             // let alpha = ((row % width) * ratio) / delta;
//             let alpha = 255;
//             frame.push(Pixel {r: alpha, g: 0 , b: 0});
//         }
//     }
//     loop {
//         reset();
//         paint(width, height, &frame);
//         // if delta + 1 <= 255 {
//         //     delta += 1;
//         // } else {
//         //     delta = 1;
//         // }

//         // reset();
//         // let mut command = String::new();
//         // io::stdin()
//         //     .read_line(&mut command)
//         //     .expect("Failed to read line");
//         // command = String::from(command.trim());
//         // println!("{:?}", command);
//         // if command == "q" {
//         //     break;
//         // }
//     }
//     reset();
//     altscreen(false);
// }

mod printer;
use crate::printer::*;
mod picture;
use crate::picture::*;
use std::io;

fn main() {
    let mut printer = Printer::new();
    let pixels = vec![
        Pixel::new(0x00, 0x00, 0x00), Pixel::new(0xff, 0xff, 0xff), Pixel::new(0x00, 0x00, 0x00),
        Pixel::new(0xff, 0xff, 0xff), Pixel::new(0x00, 0x00, 0x00), Pixel::new(0x00, 0xff, 0xff),
        Pixel::new(0xff, 0xff, 0xff), Pixel::new(0xff, 0xff, 0xff), Pixel::new(0x00, 0xff, 0xff),
        Pixel::new(0xff, 0xff, 0xff), Pixel::new(0x00, 0x00, 0x00), Pixel::new(0x00, 0xff, 0xff),
        Pixel::new(0xff, 0xff, 0xff), Pixel::new(0x00, 0x00, 0x00), Pixel::new(0x00, 0xff, 0xff),
    ];
    let mut picture = BlockPicture::from(Picture::new(3, 5, pixels));
    
    printer.alt_on();
    printer.reset();
    loop {
        
        printer.reset();
        printer.repos(0, 0);
        // printer.clear();
        printer.reset();
        picture.print(&mut printer);
        printer.reset();
        printer.color_bg_rgb(200, 0, 0);
        printer.write("Hello World!");
        printer.reset();
        printer.newline();
        printer.output();
        printer.repos_prev_line();
        printer.clear_end();
        printer.repos_next_line();
        printer.output();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        command = String::from(command.trim());
        if command == "q" {
            break;
        }
        printer.write(&format!("{:?}", command));
        printer.output();
    }
    printer.reset();
    printer.alt_off();
    printer.output();
}