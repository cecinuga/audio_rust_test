use std::{
    io::BufReader,
    fs::File,
};
use rodio::{
    OutputStream, Sink, Decoder,
};

fn main(){ 
    let (_stream, handler) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handler).unwrap();

    let file = File::open("./assets_audacity16bit.wav").unwrap();
    sink.append(Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}    