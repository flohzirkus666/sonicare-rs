fn crc16(crc: u32, buffer: &[u8]) -> u32 {
    let mut crc = crc;
    for byte in buffer {
        crc ^= (*byte as u32) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
        crc &= 0xFFFF;
    }
    crc
}

fn compute_password(uid: &[u8], nfc_second: &str) -> String {
    let mut crc_calc: u32 = crc16(0x49A3, uid); // Calculate the NTAG UID CRC.
    let nfc_second_bytes = nfc_second.as_bytes(); // Convert nfc_second to bytes.
    crc_calc |= crc16(crc_calc, nfc_second_bytes) << 16; // Calculate the MFG CRC and combine with NTAG UID CRC.
    crc_calc = ((crc_calc >> 8) & 0x00FF00FF) | ((crc_calc << 8) & 0xFF00FF00); // Rotate the bytes.
    format!("0x{:08X}", crc_calc)
}

fn main() {
    let uid: [u8; 7] = [0x04, 0xD2, 0xE9, 0xB2, 0x11, 0x68, 0x80]; // NTAG UID.
    let nfc_second: &str = "210607 43M"; // Head MFG String, printed on Head and at memory location 0x23.

    let crc_expected: &str = "0x1D7E4BDF";
    let crc_actual: String = compute_password(&uid, nfc_second);

    println!(
        "by @ATC1441 NFC CRC : {} expected: {}. Check: {}",
        crc_actual,
        crc_expected,
        crc_actual == crc_expected
    );
}
