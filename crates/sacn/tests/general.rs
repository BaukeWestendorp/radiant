use sacn::{
    ComponentIdentifier,
    packet::{DataFraming, Dmp, Packet},
    source::SourceConfig,
};

mod common;

#[test]
fn packet_pdu_block_size() {
    let universe = common::create_test_universe!();
    let dmp = Dmp::new(universe.slots());
    let data_framing =
        DataFraming::from_source_config(&SourceConfig::default(), 0, false, 1, dmp).unwrap();
    let pdu = sacn::packet::Pdu::DataFraming(data_framing);
    let packet = Packet::new(ComponentIdentifier::new_v4(), pdu);

    assert_eq!(packet.block.pdus().len(), 1);
}
