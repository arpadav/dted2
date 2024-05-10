use dted2::DTEDData;

#[test]
fn test_input_data() {
    let data = DTEDData::read("tests/test_data.dt2").unwrap();
    assert_eq!(data.metadata.origin_angle.lat.deg, 42);
    assert_eq!(data.metadata.origin_angle.lat.min, 0);
    assert_eq!(data.metadata.origin_angle.lat.sec, 0.0);
    assert_eq!(data.metadata.origin_angle.lon.deg, 15);
    assert_eq!(data.metadata.origin_angle.lon.min, 0);
    assert_eq!(data.metadata.origin_angle.lon.sec, 0.0);
    assert_eq!(data.metadata.interval.lat, 10.0 / 36000.0);
    assert_eq!(data.metadata.interval_secs.lat, 1.0);
    assert_eq!(data.metadata.interval.lon, 10.0 / 36000.0);
    assert_eq!(data.metadata.interval_secs.lon, 1.0);
    assert_eq!(data.metadata.count.lat, 3601);
    assert_eq!(data.metadata.count.lon, 3601);
}

#[test]
fn test_read_header_only() {
    let header = DTEDData::read_header("tests/test_data.dt2").unwrap();
    assert_eq!(header.origin_angle.lat.deg, 42);
    assert_eq!(header.origin_angle.lat.min, 0);
    assert_eq!(header.origin_angle.lat.sec, 0.0);
    assert_eq!(header.origin_angle.lon.deg, 15);
    assert_eq!(header.origin_angle.lon.min, 0);
    assert_eq!(header.origin_angle.lon.sec, 0.0);
    assert_eq!(header.interval.lat, 10.0 / 36000.0);
    assert_eq!(header.interval_secs.lat, 1.0);
    assert_eq!(header.interval.lon, 10.0 / 36000.0);
    assert_eq!(header.interval_secs.lon, 1.0);
    assert_eq!(header.count.lat, 3601);
    assert_eq!(header.count.lon, 3601);
}