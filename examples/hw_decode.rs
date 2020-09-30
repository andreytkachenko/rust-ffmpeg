extern crate ffmpeg_next as ffmpeg;
use ffmpeg::util::hw::{
    hw_device_type,
    HWDeviceContext,
    self,
};
use ffmpeg::{log, format, media, codec, Error, Packet, decoder};

use std::env;
use std::fs::File;

fn decode_write(_video_dec_ctx: &mut decoder::Video, _packet: &Packet) -> Result<(), Error> {
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    if env::args().len() < 4 {
        panic!("Usage: {} <device type> <input file> <output file>\n", env::args().nth(0).unwrap());
    }

    let device_type = env::args().nth(1).unwrap();
    let input_file = env::args().nth(2).unwrap();
    let output_file = env::args().nth(3).unwrap();

    let hw_type = match hw_device_type::find_type_by_name(&device_type) {
        None => {
            println!("Device type {} is not supported.\n", device_type);
            println!("Available device types:");
            for t in hw_device_type::hw_device_types() {
                println!("{}", hw_device_type::type_name(t));
            }
            std::process::exit(-1);
        },
        Some(t) => t,
    };

    ffmpeg::init().unwrap();
    ffmpeg::format::network::init();
    log::set_level(log::Level::Info);

    let mut ictx = format::input(&input_file)?;

    let best_video_stream = ictx.streams().best(media::Type::Video).ok_or(Error::StreamNotFound)?;
    let video_stream_index = best_video_stream.index();
    let codec = best_video_stream.parameters().codec().ok_or(Error::DecoderNotFound)?;

    let _hw_pix_fmt = codec.hw_configs()
        .filter(|cfg| cfg.device_type() == hw_type && cfg.methods().contains(hw::MethodFlags::HW_DEVICE_CTX))
        .nth(0).expect("Not found codec config").pix_fmt();

    let mut codec_ctx = codec::Context::create_for_codec(codec)?;
    codec_ctx.set_parameters(best_video_stream.parameters())?;
    let mut decoder_ctx = codec_ctx.decoder();

    //video_decoder_ctx.get_format

    let hw_device_ctx = HWDeviceContext::create(hw_type)?;
    decoder_ctx.set_hw_device_ctx(hw_device_ctx)?;

    let mut video_dec_ctx = decoder_ctx.video()?;

    let _out_file = File::open(&output_file)?;

    for (stream, packet) in ictx.packets() {
        if video_stream_index == stream.index() {
            decode_write(&mut video_dec_ctx, &packet)?;
        }
    }

    Ok(())
}