use ntest::timeout;
use sacn::{
    packet::{DataFraming, Dmp, RootLayer, SyncFraming},
    source::SourceConfig,
    *,
};

mod common;

// SPEC: A set of up to 512 data slots identified by universe number.
#[test]
fn e131_3_2_universe_data_slots_size() {
    let mut universe = Universe::new(1);

    // Test initial state
    assert_eq!(universe.data_slots.len(), 0);
    assert_eq!(universe.data_slots.capacity(), MAX_UNIVERSE_SIZE);

    // Test pushing up to capacity
    for i in 0..MAX_UNIVERSE_SIZE {
        universe.data_slots.push(i as Slot);
        assert_eq!(universe.data_slots.len(), i + 1);
    }

    // Verify max size
    assert_eq!(universe.data_slots.len(), MAX_UNIVERSE_SIZE);
    assert!(universe.data_slots.is_full());

    // Test that we can't push beyond capacity
    assert!(universe.data_slots.try_push(0).is_err());
}

// SPEC: Each E1.31 Data Packet contains a universe number identifying the universe it carries.
#[test]
fn e131_3_3_data_packet_has_universe_number() {
    let universe = common::create_test_universe!();

    let data_framing = DataFraming::from_source_config(
        &SourceConfig::default(),
        0,
        true,
        universe.number,
        Dmp::new(universe.slots()),
    )
    .unwrap();

    assert_eq!(data_framing.universe(), universe.number);
}

// SPEC: A slot is a sequentially numbered octet in a DMX512-A [DMX] packet.
#[test]
fn e131_3_4_slot_is_sequentially_numbered_octet() {
    let a: Slot = 0;
    let b: Slot = 1;
    assert!(a < b);
}

// SPEC: A single Universe contains a maximum of 513 Slots, starting at slot 0.
#[test]
fn e131_3_4_universe_contains_max_slots() {
    let mut universe = Universe::with_start_code(1, 0xAA);
    let slots = universe.slots();

    // Test capacity is 513 slots (512 data slots + start code)
    assert_eq!(slots.capacity(), MAX_UNIVERSE_SIZE + 1);

    // Test initial state has only start code
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0], 0xAA);

    // Fill universe to capacity
    for i in 0..MAX_UNIVERSE_SIZE {
        universe.data_slots.push(i as Slot);
    }

    // Verify full universe has 513 slots
    let slots = universe.slots();
    assert_eq!(slots.len(), MAX_UNIVERSE_SIZE + 1);
    for i in 0..MAX_UNIVERSE_SIZE {
        assert_eq!(slots[i + 1], i as Slot);
    }
}

// SPEC: Slot 0 is the DMX512-A [DMX] START Code. Slots 1 through 512 are data slots.
#[test]
fn e131_3_4_universe_slot_0_start_code_1_512_data_slots() {
    let mut universe = Universe::with_start_code(1, 0xAA);
    let slots = universe.slots();

    // Verify slot 0 is start code
    assert_eq!(slots[0], 0xAA);
    assert_eq!(slots.len(), 1);

    // Add 512 data slots
    for i in 0..MAX_UNIVERSE_SIZE {
        universe.data_slots.push(i as Slot);
    }

    let slots = universe.slots();

    // Verify slot 0 is still start code
    assert_eq!(slots[0], 0xAA);

    // Verify slots 1-512 are data slots
    for i in 0..MAX_UNIVERSE_SIZE {
        assert_eq!(slots[i + 1], i as Slot);
    }
}

// SPEC: A stream of E1.31 Packets for a universe is said to be sent from a source.
#[test]
#[serial_test::serial]
#[timeout(3000)]
fn e131_3_5_stream_of_packets_for_universe_sent_from_source() {
    let universe = common::create_test_universe!();

    let source = common::start_test_source_thread();
    source.set_universe(universe.clone());

    let receiver = common::start_test_receiver();
    let recv_universe = common::recv(&receiver);
    assert_eq!(recv_universe, universe);

    source.shutdown().ok();
}

// SPEC: A source is uniquely identified by a number in the header of the packet (see field CID in Table 4-1).
#[test]
fn e131_3_5_source_identified_by_packet_header_cid() {
    let cid = ComponentIdentifier::new_v4();
    let root_layer = RootLayer::new(cid, false, packet::Pdu::SyncFraming(SyncFraming::new(0, 0)));
    assert_eq!(root_layer.cid(), &cid);
}

// SPEC: A source may output multiple streams of data, each for a different universe.
#[test]
#[serial_test::serial]
#[timeout(3000)]
fn e131_3_5_source_may_output_multiple_streams() {
    let universe_a = common::create_test_universe!(1);
    let universe_b = common::create_test_universe!(2);
    let universe_c = common::create_test_universe!(3);

    let source = common::start_test_source_thread();
    source.set_universe(universe_a.clone());
    source.set_universe(universe_b.clone());
    source.set_universe(universe_c.clone());

    let receiver = common::start_test_receiver();
    common::recv_until_all(&receiver, vec![universe_a, universe_b, universe_c]);

    source.shutdown().ok();
}

// SPEC: A receiver may listen on multiple universes.
// SPEC: Also, multiple sources may output data for a given universe.
#[test]
#[serial_test::serial]
#[timeout(3000)]
fn e131_3_5_multiple_sources_output_data_for_universe() {
    let universe_a = common::create_test_universe!(1);
    let universe_b = common::create_test_universe!(2);

    let source_a = common::start_test_source_thread();
    source_a.set_universe(universe_a.clone());

    let source_b = common::start_test_source_thread();
    source_b.set_universe(universe_b.clone());

    let receiver = common::start_test_receiver();
    common::recv_until_all(&receiver, vec![universe_a, universe_b]);

    source_a.shutdown().ok();
    source_b.shutdown().ok();
}
