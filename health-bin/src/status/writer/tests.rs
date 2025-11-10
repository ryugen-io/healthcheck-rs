use super::JsonWriter;

#[test]
fn json_writer_formats_booleans_and_numbers() {
    let mut writer = JsonWriter::new();
    writer.bool("ok", true);
    writer.u64("latency", 42);
    writer.f32("ratio", 12.5);
    let json = writer.finish();

    assert_eq!(
        "{\n  \"ok\": true,\n  \"latency\": 42,\n  \"ratio\": 12.5\n}",
        json
    );
}

#[test]
fn json_writer_escapes_strings() {
    let mut writer = JsonWriter::new();
    writer.str_opt("msg", Some("line\n\"quote\""));
    let json = writer.finish();

    assert_eq!("{\n  \"msg\": \"line\\n\\\"quote\\\"\"\n}", json);
}
