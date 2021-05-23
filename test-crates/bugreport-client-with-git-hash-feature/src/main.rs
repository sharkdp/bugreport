use bugreport::{bugreport, collector::*, format::Markdown};

fn main() {
    bugreport!()
        .info(SoftwareVersion::default())
        .print::<Markdown>();
}
