use tklog::handle::FileTimeMode;
use tklog::{trace, Format, LEVEL, LOG};

fn main() {
    LOG.set_console(true)
        .set_level(LEVEL::Trace)
        .set_format(Format::LevelFlag);

    trace!("Hello world!");
}