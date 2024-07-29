pub const HANGUL_SYLLABLE_START: u32 = 0xAC00; //가
pub const HANGUL_SYLLABLE_END: u32 = 0xD7A3; //힣

pub const CHOSEONGS: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

pub const JUNGSEONGS: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

pub const JONGSEONGS: [char; 28] = [
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

pub const HANGUL_COMPATIBILITY_JAMO_START: u32 = 0x3131;
pub const HANGUL_COMPATIBILITY_JAMO_END: u32 = 0x3163; //officially continues until 0x318E, but 0x3163 is the last character that is used in modern Korean,

pub const HANGUL_JUNGSEONG_START: u32 = 0x314F;

pub const HANGUL_JONGSEONG_START: u32 = 0x3131;

pub fn is_cho(ch: &char) -> bool {
    CHOSEONGS.contains(ch)
}

pub fn is_jung(ch: &char) -> bool {
    JUNGSEONGS.contains(ch)
}

pub fn is_jong(ch: &char) -> bool {
    JONGSEONGS.contains(ch)
}

pub fn cho_index(cho: &char) -> u32 {
    match cho {
        'ㄱ' => 0,
        'ㄲ' => 1,
        'ㄴ' => 2,
        'ㄷ' => 3,
        'ㄸ' => 4,
        'ㄹ' => 5,
        'ㅁ' => 6,
        'ㅂ' => 7,
        'ㅃ' => 8,
        'ㅅ' => 9,
        'ㅆ' => 10,
        'ㅇ' => 11,
        'ㅈ' => 12,
        'ㅉ' => 13,
        'ㅊ' => 14,
        'ㅋ' => 15,
        'ㅌ' => 16,
        'ㅍ' => 17,
        'ㅎ' => 18,
        _ => panic!("Invalid cho: {}", cho),
    }
}

pub fn jung_index(jung: &char) -> u32 {
    *jung as u32 - HANGUL_JUNGSEONG_START
}

pub fn jong_index(jong: &char) -> u32 {
    match jong {
        '\0' => 0,
        'ㄱ' => 1,
        'ㄲ' => 2,
        'ㄳ' => 3,
        'ㄴ' => 4,
        'ㄵ' => 5,
        'ㄶ' => 6,
        'ㄷ' => 7,
        'ㄹ' => 8,
        'ㄺ' => 9,
        'ㄻ' => 10,
        'ㄼ' => 11,
        'ㄽ' => 12,
        'ㄾ' => 13,
        'ㄿ' => 14,
        'ㅀ' => 15,
        'ㅁ' => 16,
        'ㅂ' => 17,
        'ㅄ' => 18,
        'ㅅ' => 19,
        'ㅆ' => 20,
        'ㅇ' => 21,
        'ㅈ' => 22,
        'ㅊ' => 23,
        'ㅋ' => 24,
        'ㅌ' => 25,
        'ㅍ' => 26,
        'ㅎ' => 27,

        _ => panic!("Invalid jong: {}", jong),
    }
}

pub fn assemble_hangul(cho: &char, jung: &char, jong: Option<&char>) -> char {
    let cho_index = cho_index(cho);
    let jung_index = jung_index(jung);
    let jong_index = jong.map(|jong| jong_index(jong));

    let syllable_index = 44032 + cho_index * 588 + jung_index * 28 + jong_index.unwrap_or(0);

    std::char::from_u32(syllable_index).unwrap()
}
