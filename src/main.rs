extern crate clap;
extern crate image;

use clap::{App, Arg, ArgMatches};
use thumbnailer::{BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize};

use crate::commands::{CmdBlur, CmdBrighten, CmdCombine, CmdContrast, CmdCrop, CmdExif, CmdFlip, CmdHuerotate, CmdInvert, CmdResize, CmdResizeFilter, CmdText, CmdUnsharpen, CommandTrait};

mod commands;

const NAME_FILE_IN: &str = "INPUT_PATH";
const NAME_FILE_OUT: &str = "OUTPUT_PATH";

const ARG_BLUR: &str = "blur";
const ARG_BRIGHTEN: &str = "brighten";
const ARG_CONTRAST: &str = "contrast";
const ARG_COMBINE_TL: &str = "combine_tl";
const ARG_COMBINE_TR: &str = "combine_tr";
const ARG_COMBINE_BL: &str = "combine_bl";
const ARG_COMBINE_BR: &str = "combine_br";
const ARG_CROP_BOX: &str = "crop_box";
const ARG_CROP_RATIO: &str = "crop_ratio";
const ARG_EXIF: &str = "exif";
const ARG_FLIP_HORIZONTAL: &str = "flip_horizontal";
const ARG_FLIP_VERTICAL: &str = "flip_vertical";
const ARG_HUEROTATE: &str = "huerotate";
const ARG_INVERT: &str = "invert";
const ARG_RESIZE: &str = "resize";
const ARG_RESIZE_N: &str = "resize_n";
const ARG_RESIZE_T: &str = "resize_t";
const ARG_RESIZE_C: &str = "resize_c";
const ARG_RESIZE_G: &str = "resize_g";
const ARG_RESIZE_L: &str = "resize_l";
const ARG_ROTATE90: &str = "rotate90";
const ARG_ROTATE180: &str = "rotate180";
const ARG_ROTATE270: &str = "rotate270";
const ARG_TEXT_TL: &str = "text_tl";
const ARG_TEXT_TR: &str = "text_tr";
const ARG_TEXT_BL: &str = "text_bl";
const ARG_TEXT_BR: &str = "text_br";
const ARG_UNSHARPEN: &str = "unsharpen";

const GROUP_COMBINE: &str = "Combine";
const GROUP_RESIZE: &str = "Resize";
const GROUP_TEXT: &str = "Text";

const VAL_COMBINE: [&str; 3] = ["IMAGE_PATH", "x_offset", "y_offset"];
const VAL_RESIZE: [&str; 3] = ["nwidth", "nheight", "exact"];
const VAL_TEXT: [&str; 3] = ["text", "x_offset", "y_offset"];

struct Commands {
    commands: Vec<Box<dyn CommandTrait>>,
}

//  .\target\debug\thumbnailer_cli.exe -h
//  .\target\debug\thumbnailer_cli.exe in.png out.png --flip_vertical  --brighten 15 --blur 34.151545 --unsharpen 56 14
//  .\target\debug\thumbnailer_cli.exe in.png out.png --flip_vertical  --brighten 15 --crop_ratio 4 3 --blur 34.151545 --unsharpen 56 14 --resize 89 4 true
//
//  .\target\debug\thumbnailer_cli.exe in.png out.png -c settings.conf -mmm watermark icon.png colored
//  .\target\debug\thumbnailer_cli.exe in.png out.png -c settings.conf -mmm --blur 6 --brighten 15
//  .\target\debug\thumbnailer_cli.exe in.png out.png --blur 6 --brighten 15

fn main() {
    let matches = get_matches();

    let file_in = String::from(matches.value_of(NAME_FILE_IN).unwrap());
    let file_out = String::from(matches.value_of(NAME_FILE_OUT).unwrap());
    let cmd_list = read_commands(matches);

    println!("Input file: {}", file_in);
    println!("Output file: {}", file_out);
    for i in 0..cmd_list.commands.len() {
        println!("{}", cmd_list.commands.get(i).unwrap().print());
    }

    // TODO more program logic goes here...
}

fn get_matches() -> clap::ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name(NAME_FILE_IN)
            .index(1)
            .help("Sets the input file to use")
            .required(true))

        .arg(Arg::with_name(NAME_FILE_OUT)
            .index(2)
            .default_value("thumbnail.png")
            .help("Sets the output file to save")
            .required(true))

        .arg(Arg::with_name(ARG_BLUR)
            .long(ARG_BLUR)
            .value_name("sigma")
            .help("Performs a Gaussian blur on the supplied image. sigma is a measure of how much to blur by.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_BRIGHTEN)
            .long(ARG_BRIGHTEN)
            .value_name("value")
            .help("Brighten the supplied image. value is the amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_CONTRAST)
            .long(ARG_CONTRAST)
            .value_name("contrast")
            .help("Adjust the contrast of the supplied image. contrast is the amount to adjust the contrast by. Negative values decrease the contrast and positive values increase the contrast.")
            .takes_value(true))

        //TODO add help!
        .arg(Arg::with_name(ARG_COMBINE_TL)
            .long(ARG_COMBINE_TL)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_TR)
            .long(ARG_COMBINE_TR)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_BL)
            .long(ARG_COMBINE_BL)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_BR)
            .long(ARG_COMBINE_BR)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("")
            .takes_value(true))

        .arg(Arg::with_name(ARG_CROP_BOX)
            .long(ARG_CROP_BOX)
            .value_name("x")
            .value_name("y")
            .value_name("width")
            .value_name("height")
            .help("Crop the supplied image(s) to the given width and height. x is the horizontal and y the vertical offset.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_CROP_RATIO)
            .long(ARG_CROP_RATIO)
            .value_name("x_ratio")
            .value_name("y_ratio")
            .help("Crop the supplied image(s) to the given ratio x_ratio:y_ratio. x_ratio is representing the horizontal and y_ratio the vertical.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_EXIF)
            .long(ARG_EXIF)
            .help("The metadata is taken from the source file and not removed, as is standard."))

        .arg(Arg::with_name(ARG_FLIP_HORIZONTAL)
            .long(ARG_FLIP_HORIZONTAL)
            .help("Flip the supplied image(s) horizontally."))
        .arg(Arg::with_name(ARG_FLIP_VERTICAL)
            .long(ARG_FLIP_VERTICAL)
            .help("Flip the supplied image(s) vertically."))

        .arg(Arg::with_name(ARG_HUEROTATE)
            .long(ARG_HUEROTATE)
            .value_name("value")
            .help("Hue rotate the supplied image(s). value is the degrees to rotate each pixel by. 0 and 360 do nothing, the rest rotates by the given degree value.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_INVERT)
            .long(ARG_INVERT)
            .help("Invert each pixel within the supplied image(s)."))

        .arg(Arg::with_name(ARG_RESIZE)
            .long(ARG_RESIZE)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_N)
            .long(ARG_RESIZE_N)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. Nearest is the used filter (Nearest Neighbor Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_T)
            .long(ARG_RESIZE_T)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. Triangle is the used filter (Linear Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_C)
            .long(ARG_RESIZE_C)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. CatmullRom is the used filter (Cubic Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_G)
            .long(ARG_RESIZE_G)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. Gaussian is the used filter (Gaussian Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_L)
            .long(ARG_RESIZE_L)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. Lanczos3 is the used filter (Lanczos with window 3). To resize only by one dimension, set the other to 0.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_ROTATE90)
            .short("r")
            .long(ARG_ROTATE90)
            .help("Rotate the supplied image(s) 90 degrees clockwise.")
            .multiple(true))
        .arg(Arg::with_name(ARG_ROTATE180)
            .long(ARG_ROTATE180)
            .help("Rotate the supplied image(s) 180 degrees clockwise."))
        .arg(Arg::with_name(ARG_ROTATE270)
            .long(ARG_ROTATE270)
            .help("Rotate the supplied image(s) 270 degrees clockwise."))

        // TODO add help!
        .arg(Arg::with_name(ARG_TEXT_TL)
            .long(ARG_TEXT_TL)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_TR)
            .long(ARG_TEXT_TR)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_BL)
            .long(ARG_TEXT_BL)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_BR)
            .long(ARG_TEXT_BR)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("")
            .takes_value(true))

        .arg(Arg::with_name(ARG_UNSHARPEN)
            .long(ARG_UNSHARPEN)
            .value_name("sigma")
            .value_name("threshold")
            .help("Performs an unsharpen mask on the supplied image. sigma is the amount to blur the image by. threshold is the threshold for the difference between.")
            .takes_value(true))
        .get_matches()
}

fn read_commands(matches: ArgMatches<'static>) -> Commands {
    let mut cmd_list = Commands { commands: vec![] };

    if matches.is_present(ARG_BLUR) {
        let index = matches.index_of(ARG_BLUR).unwrap() as u32;
        let sigma = String::from(matches.value_of(ARG_BLUR).unwrap()).parse::<f32>().unwrap_or(0.0);

        cmd_list.commands.push(Box::new(CmdBlur { index, sigma }));
    }

    if matches.is_present(ARG_BRIGHTEN) {
        let index = matches.index_of(ARG_BRIGHTEN).unwrap() as u32;
        let value = String::from(matches.value_of(ARG_BRIGHTEN).unwrap()).parse::<i32>().unwrap_or(0);

        cmd_list.commands.push(Box::new(CmdBrighten { index, value }));
    }

    if matches.is_present(ARG_CONTRAST) {
        let index = matches.index_of(ARG_CONTRAST).unwrap() as u32;
        let value = String::from(matches.value_of(ARG_CONTRAST).unwrap()).parse::<f32>().unwrap_or(0.0);

        cmd_list.commands.push(Box::new(CmdContrast { index, value }));
    }

    if matches.is_present(ARG_COMBINE_TL) {
        //cmd_list.commands.push(Box::new(create_cmd_combine(matches.clone(), ARG_COMBINE_TL)));
    }
    if matches.is_present(ARG_COMBINE_TR) {
        //cmd_list.commands.push(Box::new(create_cmd_combine(matches.clone(), ARG_COMBINE_TR)));
    }
    if matches.is_present(ARG_COMBINE_BL) {
        //cmd_list.commands.push(Box::new(create_cmd_combine(matches.clone(), ARG_COMBINE_BL)));
    }
    if matches.is_present(ARG_COMBINE_BR) {
        //cmd_list.commands.push(Box::new(create_cmd_combine(matches.clone(), ARG_COMBINE_BR)));
    }

    if matches.is_present(ARG_CROP_BOX) {
        let index = matches.index_of(ARG_CROP_BOX).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_CROP_BOX).unwrap().collect();

        let x = values[0].parse::<u32>().unwrap_or(0);
        let y = values[1].parse::<u32>().unwrap_or(0);
        let width = values[2].parse::<u32>().unwrap_or(0);
        let height = values[3].parse::<u32>().unwrap_or(0);

        let crop = CmdCrop { index, config: Crop::Box(x, y, width, height) };
        cmd_list.commands.push(Box::new(crop));
    }

    if matches.is_present(ARG_CROP_RATIO) {
        let index = matches.index_of(ARG_CROP_RATIO).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_CROP_RATIO).unwrap().collect();

        let x_ratio = values[0].parse::<f32>().unwrap_or(0.0);
        let y_ratio = values[1].parse::<f32>().unwrap_or(0.0);

        let crop = CmdCrop { index, config: Crop::Ratio(x_ratio, y_ratio) };
        cmd_list.commands.push(Box::new(crop));
    }

    if matches.is_present(ARG_EXIF) {
        let index = matches.index_of(ARG_EXIF).unwrap() as u32;

        let exif = CmdExif { index, metadata: Exif::Keep };
        cmd_list.commands.push(Box::new(exif));
    }

    if matches.is_present(ARG_FLIP_HORIZONTAL) {
        let index = matches.index_of(ARG_FLIP_HORIZONTAL).unwrap() as u32;

        let flip = CmdFlip { index, orientation: Orientation::Horizontal };
        cmd_list.commands.push(Box::new(flip));
    }

    if matches.is_present(ARG_FLIP_VERTICAL) {
        let index = matches.index_of(ARG_FLIP_VERTICAL).unwrap() as u32;

        let flip = CmdFlip { index, orientation: Orientation::Vertical };
        cmd_list.commands.push(Box::new(flip));
    }

    if matches.is_present(ARG_HUEROTATE) {
        let index = matches.index_of(ARG_HUEROTATE).unwrap() as u32;
        let degree = String::from(matches.value_of(ARG_HUEROTATE).unwrap()).parse::<i32>().unwrap_or(0);

        let huerotate = CmdHuerotate { index, degree };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(ARG_INVERT) {
        let index = matches.index_of(ARG_INVERT).unwrap() as u32;

        let invert = CmdInvert { index };
        cmd_list.commands.push(Box::new(invert));
    }

    if matches.is_present(ARG_RESIZE) {
        let index = matches.index_of(ARG_RESIZE).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_RESIZE).unwrap().collect();

        let width = values[0].parse::<u32>().unwrap_or(0);
        let height = values[1].parse::<u32>().unwrap_or(0);
        let exact = values[2].parse::<bool>().unwrap_or(false);

        let resize;
        if height == 0 {
            resize = CmdResize { index, size: Resize::Width(width) };
        } else if width == 0 {
            resize = CmdResize { index, size: Resize::Height(height) };
        } else if !exact {
            resize = CmdResize { index, size: Resize::BoundingBox(width, height) };
        } else {
            resize = CmdResize { index, size: Resize::ExactBox(width, height) };
        }
        cmd_list.commands.push(Box::new(resize));
    }

    if matches.is_present(ARG_RESIZE_N) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(matches.clone(), ARG_RESIZE_N)));
    }
    if matches.is_present(ARG_RESIZE_T) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(matches.clone(), ARG_RESIZE_T)));
    }
    if matches.is_present(ARG_RESIZE_C) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(matches.clone(), ARG_RESIZE_C)));
    }
    if matches.is_present(ARG_RESIZE_G) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(matches.clone(), ARG_RESIZE_G)));
    }
    if matches.is_present(ARG_RESIZE_L) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(matches.clone(), ARG_RESIZE_L)));
    }

    if matches.is_present(ARG_ROTATE90) {
        let index = matches.index_of(ARG_ROTATE90).unwrap() as u32;
        let degree = 90 * matches.occurrences_of(ARG_ROTATE90) as i32;

        let huerotate = CmdHuerotate { index, degree };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(ARG_ROTATE180) {
        let index = matches.index_of(ARG_ROTATE180).unwrap() as u32;

        let huerotate = CmdHuerotate { index, degree: 180 };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(ARG_ROTATE270) {
        let index = matches.index_of(ARG_ROTATE270).unwrap() as u32;

        let huerotate = CmdHuerotate { index, degree: 270 };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(ARG_TEXT_TL) {
        cmd_list.commands.push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_TL)));
    }
    if matches.is_present(ARG_TEXT_TR) {
        cmd_list.commands.push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_TR)));
    }
    if matches.is_present(ARG_TEXT_BL) {
        cmd_list.commands.push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_BL)));
    }
    if matches.is_present(ARG_TEXT_BR) {
        cmd_list.commands.push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_BR)));
    }

    if matches.is_present(ARG_UNSHARPEN) {
        let index = matches.index_of(ARG_UNSHARPEN).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_UNSHARPEN).unwrap().collect();

        let sigma = values[0].parse::<f32>().unwrap_or(0.0);
        let threshold = values[1].parse::<i32>().unwrap_or(0);

        let unsharpen = CmdUnsharpen { index, sigma, threshold };
        cmd_list.commands.push(Box::new(unsharpen));
    }

    cmd_list.commands.sort();
    cmd_list
}

/*
fn create_cmd_combine(matches: ArgMatches<'static>, arg: &str) -> CmdCombine {
    let index = matches.index_of(arg).unwrap() as u32;
    let values: Vec<_> = matches.values_of(arg).unwrap().collect();

    let image = values[0];
    let x_offset = String::from(values[1]).parse::<u32>().unwrap_or(0);
    let y_offset = String::from(values[2]).parse::<u32>().unwrap_or(0);

    let position = match values[1] {
        _ if arg == ARG_COMBINE_TL => BoxPosition::TopLeft(x_offset, y_offset),
        _ if arg == ARG_COMBINE_TR => BoxPosition::TopRight(x_offset, y_offset),
        _ if arg == ARG_COMBINE_BL => BoxPosition::BottomLeft(x_offset, y_offset),
        _ if arg == ARG_COMBINE_BR => BoxPosition::BottomRight(x_offset, y_offset),
        _ => BoxPosition::TopLeft(x_offset, y_offset),
    };

    CmdCombine {index, image: StaticThumbnail{ image: open(image)}, position}
}
 */

fn create_cmd_resize_filter(matches: ArgMatches<'static>, arg: &str) -> CmdResizeFilter {
    let index = matches.index_of(arg).unwrap() as u32;
    let values: Vec<_> = matches.values_of(arg).unwrap().collect();

    let width = values[0].parse::<u32>().unwrap_or(0);
    let height = values[1].parse::<u32>().unwrap_or(0);
    let exact = values[2].parse::<bool>().unwrap_or(false);

    let filter = match values[3] {
        _ if arg == ARG_RESIZE_N => ResampleFilter::Nearest,
        _ if arg == ARG_RESIZE_T => ResampleFilter::Triangle,
        _ if arg == ARG_RESIZE_C => ResampleFilter::CatmullRom,
        _ if arg == ARG_RESIZE_G => ResampleFilter::Gaussian,
        _ if arg == ARG_RESIZE_L => ResampleFilter::Lanczos3,
        _ => ResampleFilter::Nearest,
    };

    let resize_filter;
    if height == 0 {
        resize_filter = CmdResizeFilter { index, size: Resize::Width(width), filter };
    } else if width == 0 {
        resize_filter = CmdResizeFilter { index, size: Resize::Height(height), filter };
    } else if !exact {
        resize_filter = CmdResizeFilter { index, size: Resize::BoundingBox(width, height), filter };
    } else {
        resize_filter = CmdResizeFilter { index, size: Resize::ExactBox(width, height), filter };
    }
    resize_filter
}

fn create_cmd_text(matches: ArgMatches<'static>, arg: &str) -> CmdText {
    let index = matches.index_of(arg).unwrap() as u32;
    let values: Vec<_> = matches.values_of(arg).unwrap().collect();

    let text = String::from(values[0]);
    let x_offset = String::from(values[1]).parse::<u32>().unwrap_or(0);
    let y_offset = String::from(values[2]).parse::<u32>().unwrap_or(0);

    let position = match values[1] {
        _ if arg == ARG_COMBINE_TL => BoxPosition::TopLeft(x_offset, y_offset),
        _ if arg == ARG_COMBINE_TR => BoxPosition::TopRight(x_offset, y_offset),
        _ if arg == ARG_COMBINE_BL => BoxPosition::BottomLeft(x_offset, y_offset),
        _ if arg == ARG_COMBINE_BR => BoxPosition::BottomRight(x_offset, y_offset),
        _ => BoxPosition::TopLeft(x_offset, y_offset),
    };

    CmdText { index, text, position }
}