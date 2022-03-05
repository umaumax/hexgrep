use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use itertools::Itertools;
use structopt::StructOpt;

#[derive(strum_macros::EnumString, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ColorWhen {
    Always,
    Never,
    Auto,
}

impl ColorWhen {
    pub fn mix_isatty_to_color_flag(&self, isatty: bool) -> bool {
        match self {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => isatty,
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(
        long = "hex",
        required_unless = "text-pattern",
        conflicts_with = "text-pattern",
        help = "e.g. --hex 6e616e6f6861 or use --text"
    )]
    hex_pattern: Option<String>,

    #[structopt(
        long = "text",
        required_unless = "hex-pattern",
        conflicts_with = "hex-pattern",
        help = "e.g. --text rust or use --hex"
    )]
    text_pattern: Option<String>,

    #[structopt(
        short = "C",
        default_value = "16",
        help = "print NUM bytes of output context"
    )]
    context: u64,

    #[structopt(
        long = "color",
        parse(try_from_str),
        default_value = "auto",
        help = "use markers to highlight the matching strings; WHEN is 'always', 'never', or 'auto'"
    )]
    color_when: ColorWhen,

    #[structopt(
        short,
        long = "in",
        parse(from_os_str),
        help = "target input binary file"
    )]
    input: PathBuf,
}

const EXIT_CODE_NO_HIT: i32 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let filepath = opt.input;
    let mut file = BufReader::new(File::open(&filepath)?);
    let metadata = fs::metadata(&filepath)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)?;

    let pattern = if let Some(ref hex_pattern) = opt.hex_pattern {
        hex::decode(hex_pattern)?
    } else if let Some(ref text_pattern) = opt.text_pattern {
        text_pattern.as_bytes().to_vec()
    } else {
        unreachable!();
    };

    let isatty = atty::is(atty::Stream::Stdout);
    let color_enable = opt.color_when.mix_isatty_to_color_flag(isatty);
    let before_range = opt.context;
    let after_range = opt.context;
    let mut offset = 0;
    while let Some(index) = twoway::find_bytes(&buffer, &pattern) {
        let pattern_length = pattern.len();
        let from_index = std::cmp::max(0, index as i64 - before_range as i64) as usize / 16 * 16;
        let to_index = std::cmp::min(
            buffer.len(),
            ((index + pattern_length) + after_range as usize + 15) / 16 * 16,
        );

        let roi_buffer = &buffer[from_index..to_index];
        let hex_iter = roi_buffer
            .iter()
            .enumerate()
            .map(|(i, x)| {
                if color_enable
                    && i >= index - from_index
                    && i < index - from_index + pattern_length
                {
                    ansi_term::Color::Yellow
                        .paint(format!("{:02x}", x))
                        .to_string()
                } else {
                    format!("{:02x}", x)
                }
            })
            .chunks(8);
        let hex_iter = hex_iter
            .into_iter()
            .map(|chunk| format!("{}", chunk.format(" ")))
            .chunks(2);
        let hex_iter = hex_iter
            .into_iter()
            .map(|chunk| format!("{}", chunk.format("  ")));

        let text_iter = roi_buffer
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let c = *n as char;
                let c = if c.is_ascii() && !c.is_ascii_control() {
                    c
                } else {
                    '.'
                };
                if color_enable
                    && i >= index - from_index
                    && i < index - from_index + pattern_length
                {
                    ansi_term::Color::Yellow.paint(c.to_string()).to_string()
                } else {
                    c.to_string()
                }
            })
            .chunks(16);
        let text_iter = text_iter
            .into_iter()
            .map(|chunk| format!("{}", chunk.format("")));

        let n = (to_index - from_index) / 16 * 16 + 1;
        let output = itertools::multizip((0..n, hex_iter, text_iter))
            .map(|(i, hex, text)| {
                format!(
                    "{:08x}  {}  |{}|\n",
                    offset + from_index + i * 16,
                    hex,
                    text,
                )
            })
            .collect::<String>();
        if offset != 0 {
            println!();
        }
        print!("{}", output);

        buffer.drain(0..index + 1);
        offset += index + 1;
    }
    if offset == 0 {
        // no hit
        process::exit(EXIT_CODE_NO_HIT)
    }
    Ok(())
}
