/// Преобразует скан-код PS/2 Set 1 в ASCII байт.
/// Возвращает None если символ не печатный или это key-release (sc >= 0x80).
pub fn scancode_to_char(sc: u8, shift: bool) -> Option<u8> {
    if sc >= 0x80 { return None; } // key-release — игнорируем

    // US-раскладка
    let normal: &[u8] =
        b"\x00\x1b1234567890-=\x08\tqwertyuiop[]\n\x00asdfghjkl;'`\x00\\zxcvbnm,./\x00*\x00 ";
    let shifted: &[u8] =
        b"\x00\x1b!@#$%^&*()_+\x08\tQWERTYUIOP{}\n\x00ASDFGHJKL:\"~\x00|ZXCVBNM<>?\x00*\x00 ";

    let table = if shift { shifted } else { normal };
    let c = *table.get(sc as usize)?;
    if c != 0 { Some(c) } else { None }
}
