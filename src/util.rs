pub fn concat(x: u8, y: u8) -> u16 {
    let x16 = x as u16;
    let y16 = y as u16;
    (x16 << 8) | y16
}

mod test {
    #[test]
    fn concat_test() {
        assert_eq!(concat(0xAF, 0x30), 0xAF30);
        assert_eq!(concat(0x00, 0x00), 0x0000);
        assert_eq!(concat(0x00, 0xFF), 0x00FF);
        assert_eq!(concat(0xFF, 0x00), 0xFF00);
        assert_eq!(concat(0xFF, 0xFF), 0xFFFF);
    }
}
