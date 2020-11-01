use super::Config;
use clap::{App, Arg};

fn positive_int(s: String) -> Result<(), String> {
    s.parse::<usize>()
        .map_err(|err| err.to_string())
        .and_then(|v| {
            if v != 0 {
                Ok(())
            } else {
                Err(String::from("Expected non zero value"))
            }
        })
}

fn positive_int_or_alias(s: String) -> Result<(), String> {
    match s.to_uppercase().as_ref() {
        "HD" => Ok(()),
        "FULLHD" | "FULL_HD" => Ok(()),
        "4K" => Ok(()),
        _ => positive_int(s),
    }
}

impl Config {
    /// Parse config from CLI arguments
    ///
    /// use --help to get more info
    pub fn from_args() -> Self {
        let matches = App::new("Ray tracing")
            .author("tomitheninja (Südi Tamás)")
            .about("Ray tracing written in rust")
            .arg(
                Arg::with_name("image width")
                    .short("w")
                    .long("width")
                    .validator(positive_int)
                    .takes_value(true)
                    .help(" [default: 16/9 aspect ratio]"),
            )
            .arg(
                Arg::with_name("image height")
                    .index(1)
                    .default_value("HD")
                    .help("Height in pixel or common alias")
                    .validator(positive_int_or_alias)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("samples per pixel")
                    .short("s")
                    .long("samples")
                    .default_value("100")
                    .validator(positive_int)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("max ray depth")
                    .short("d")
                    .long("ray-depth")
                    .default_value("50")
                    .validator(positive_int)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output path")
                    .short("o")
                    .long("output")
                    .default_value("img.png")
                    .help("Only png is supported")
                    .takes_value(true),
            )
            .get_matches();

        let height = matches
            .value_of("image height")
            .map(|v| match v.to_uppercase().as_ref() {
                "HD" => "720",
                "FULLHD" | "FULL_HD" => "1080",
                "4K" => "2160",
                _ => v,
            })
            .and_then(|w| w.parse().ok())
            .unwrap();

        let height_from_width = (height as f64 * 16.0 / 9.0) as usize;

        let width = match matches.value_of("image width") {
            Some(val) => val.parse().unwrap(),
            None => height_from_width,
        };

        let samples_per_pixel = matches
            .value_of("samples per pixel")
            .and_then(|s| s.parse().ok())
            .unwrap();

        let max_ray_depth = matches
            .value_of("max ray depth")
            .and_then(|d| d.parse().ok())
            .unwrap();

        let output_file = matches.value_of("output path").unwrap().to_owned();

        Self {
            img_width: width,
            img_height: height,
            samples_per_pixel,
            max_ray_depth,
            output_file,
        }
    }
}
