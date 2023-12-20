use gif::*;
use image::RgbaImage;
use std::fs::DirEntry;
use std::{env, fs, fs::File};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args);
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            println!("Usage: gif-maker <dir_name> <delay>");
            panic!("{}", e.to_string());
        }
    };
    let images = match get_all_images(&config.dir_name) {
        Ok(images) => images,
        Err(e) => panic!("{}", e.to_string()),
    };
    images_to_gif(
        images,
        &config.dir_name,
        config.delay,
        config.end_frames_delay,
    );
}

fn get_all_images(dir_name: &str) -> Result<Vec<RgbaImage>, &str> {
    let files = fs::read_dir(dir_name);
    let files = match files {
        Ok(files) => files,
        Err(_) => return Err("invalid directory"),
    };
    let mut files: Vec<DirEntry> = files.map(|file| file.unwrap()).collect();

    let mut images: Vec<RgbaImage> = Vec::new();

    files.sort_by_cached_key(|file| {
        file.file_name()
            .to_string_lossy()
            .split('.')
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0)
    });

    for file in files {
        println!("{:?}", file.path());
        let path = file.path();
        let image = image::open(path).unwrap();
        images.push(image.to_rgba8());
    }
    Ok(images)
}

fn images_to_gif(images: Vec<RgbaImage>, dir_name: &str, delay: u16, end_frames_delay: u16) {
    let out_path = format!("{}/../out.gif", dir_name);
    let mut image = File::create(out_path).unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        images[0].width() as u16,
        images[0].height() as u16,
        &[],
    )
    .unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    println!("Creating gif...");

    let total_frames = images.len();
    let mut current_frame = 0;
    for image in images {
        current_frame += 1;
        let mut frame = Frame::from_rgba_speed(
            image.width() as u16,
            image.height() as u16,
            &mut image.into_raw(),
            20,
        );
        if current_frame == 1 || current_frame == total_frames {
            frame.delay = end_frames_delay;
        } else {
            frame.delay = delay;
        }
        encoder.write_frame(&frame).unwrap();

        println!("{} / {}", current_frame, total_frames);
    }
}

#[derive(Debug)]
struct Config {
    dir_name: String,
    delay: u16,
    end_frames_delay: u16,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 4 {
            return Err("not enough arguments");
        }
        let dir_name = args[1].clone();
        let delay;
        match args[2].clone().parse::<u16>() {
            Ok(time) => delay = time,
            Err(_) => return Err("invalid animation time"),
        };
        let end_frames_delay = match args[3].clone().parse::<u16>() {
            Ok(time) => time,
            Err(_) => return Err("invalid end frames delay"),
        };
        Ok(Config {
            dir_name,
            delay,
            end_frames_delay,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let args = vec![
            String::from("program_name"),
            String::from("dir_name"),
            String::from("10"),
            String::from("100"),
        ];
        let config = Config::new(&args);
        assert_eq!(config.is_err(), false);
    }

    #[test]
    fn config_new_not_enough_args() {
        let args = vec![String::from("program_name")];
        let config = Config::new(&args);
        assert_eq!(config.is_err(), true);
    }

    #[test]
    fn config_new_invalid_delay() {
        let args = vec![
            String::from("program_name"),
            String::from("dir_name"),
            String::from("invalid"),
        ];
        let config = Config::new(&args);
        assert_eq!(config.is_err(), true);
    }

    #[test]
    fn get_all_images_ok() {
        let dir_name = "gif_test/images";
        let images = get_all_images(dir_name).unwrap();
        assert_eq!(images.len(), 10);
    }

    #[test]
    fn get_all_images_err() {
        let dir_name = "gif_test/invalid_dir";
        let images = get_all_images(dir_name);
        assert_eq!(images.is_err(), true);
    }

    // use 10 images to create a gif
    #[test]
    fn images_to_gif_test() {
        let dir_name = "gif_test/images";
        let images = get_all_images(dir_name).unwrap();
        images_to_gif(images, dir_name, 10, 100);
        let outfile = fs::read(format!("{dir_name}/../out.gif"));
        assert_eq!(outfile.is_ok(), true);
        //fs::remove_file("gif_test/../out.gif").unwrap();
    }
}
