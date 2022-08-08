use ctor::ctor;
use log::info;
use pandoc::{self, OutputFormat, OutputKind};
use pandoc_filters::{dummy, echo};
use pretty_env_logger::env_logger::{Builder, Env};

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn dummy_filter() {
    let mut pandoc = pandoc::new();

    pandoc
        .add_input("./tests/simple_input.md")
        .add_filter(dummy)
        .set_output(OutputKind::Pipe)
        .set_output_format(OutputFormat::Markdown, vec![]);

    match pandoc.execute().unwrap() {
        pandoc::PandocOutput::ToBuffer(s) => info!("{}", s),
        _ => panic!("unhandled output"),
    }
}

#[test]
fn echo_filter() {
    let mut pandoc = pandoc::new();

    pandoc
        .add_input("./tests/simple_input.md")
        .add_filter(echo)
        .set_output(OutputKind::Pipe)
        .set_output_format(OutputFormat::Markdown, vec![]);

    match pandoc.execute().unwrap() {
        pandoc::PandocOutput::ToBuffer(s) => info!("{}", s),
        _ => panic!("unhandled output"),
    }
}
