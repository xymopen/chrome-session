/// Round down `size` to a multiple of alignment, which must be a power of two.
pub(crate) fn align_down(size: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two());
    return size & !(alignment - 1);
}

/// Round up `size` to a multiple of alignment, which must be a power of two.
pub(crate) fn align_up(size: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two());
    return (size + alignment - 1) & !(alignment - 1);
}
