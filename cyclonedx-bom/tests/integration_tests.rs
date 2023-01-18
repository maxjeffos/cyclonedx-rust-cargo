use std::fs::File;
use std::io::BufReader;

use cyclonedx_bom::prelude::Bom;

#[test]
fn it_should_be_able_to_load_sbom_with_vulnerabilities() {
    let fixture_path = "fixtures/sboms/ex-sbom-with-vulnerabilities/sbom.json";
    let f = File::open(fixture_path).unwrap();
    let reader = BufReader::new(f);
    let bom = Bom::parse_from_json_v1_3(reader).unwrap();

    dbg!(bom.vulnerabilities);

    assert_eq!(0, 1);
}
