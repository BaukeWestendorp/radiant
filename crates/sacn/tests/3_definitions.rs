use sacn::*;

// SPEC: A set of up to 512 data slots identified by universe number.
#[test]
fn e131_3_2_universe_data_slots_size() {
    let mut universe = Universe::new(1);

    // Test initial state
    assert_eq!(universe.data_slots.len(), 0);
    assert_eq!(universe.data_slots.capacity(), MAX_UNIVERSE_SIZE);

    // Test pushing up to capacity
    for i in 0..MAX_UNIVERSE_SIZE {
        universe.data_slots.push(i as u8);
        assert_eq!(universe.data_slots.len(), i + 1);
    }

    // Verify max size
    assert_eq!(universe.data_slots.len(), MAX_UNIVERSE_SIZE);
    assert!(universe.data_slots.is_full());

    // Test that we can't push beyond capacity
    assert!(universe.data_slots.try_push(0).is_err());
}

//  Note: In E1.31 there may be multiple sources for a universe.
#[test]
#[ignore = "TODO"]
fn e131_3_2_multiple_sources_for_universe() {}

// SPEC: Each E1.31 Data Packet contains a universe number identifying the universe it carries.
#[test]
#[ignore = "TODO"]
fn e131_3_3_data_packet_has_universe_number() {}

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
        universe.data_slots.push(i as u8);
    }

    // Verify full universe has 513 slots
    let slots = universe.slots();
    assert_eq!(slots.len(), MAX_UNIVERSE_SIZE + 1);
    for i in 0..MAX_UNIVERSE_SIZE {
        assert_eq!(slots[i + 1], i as u8);
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
        universe.data_slots.push(i as u8);
    }

    let slots = universe.slots();

    // Verify slot 0 is still start code
    assert_eq!(slots[0], 0xAA);

    // Verify slots 1-512 are data slots
    for i in 0..MAX_UNIVERSE_SIZE {
        assert_eq!(slots[i + 1], i as u8);
    }
}
