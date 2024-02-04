#[test]
fn size_of_superblock() {
    assert_eq!(super::layout::SIZE, Some(1024));
}