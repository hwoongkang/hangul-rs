use crate::hangul_unicodes::*;

pub fn disassemble(hangul_string: &str) -> Vec<char> {
    let mut result = vec![];
    for char in hangul_string.chars() {
        let unicode = char as u32;
        match unicode {
            HANGUL_SYLLABLE_START..=HANGUL_SYLLABLE_END => {
                result.append(&mut disassemble_syallable(char))
            }
            HANGUL_COMPATIBILITY_JAMO_START..=HANGUL_COMPATIBILITY_JAMO_END => {
                result.append(&mut disassemble_jamo(char));
            }
            _ => {
                result.push(char);
            }
        }
    }
    result
}

fn disassemble_syallable(syllable: char) -> Vec<char> {
    let mut result = vec![];
    let syllable_index = syllable as u32 - HANGUL_SYLLABLE_START;
    let choseong_index = syllable_index / 588;
    let jungseong_index = (syllable_index % 588) / 28;
    let jongseong_index = syllable_index % 28;

    let choseong = CHOSEONGS[choseong_index as usize];
    let jungseong = JUNGSEONGS[jungseong_index as usize];

    result.append(&mut disassemble_jamo(choseong));
    result.append(&mut disassemble_jamo(jungseong));

    if jongseong_index != 0 {
        let jongseong = JONGSEONGS[jongseong_index as usize];
        result.append(&mut disassemble_jamo(jongseong));
    }

    result
}

fn disassemble_jamo(jamo: char) -> Vec<char> {
    match jamo {
        'ㄳ' => vec!['ㄱ', 'ㅅ'],
        'ㄵ' => vec!['ㄴ', 'ㅈ'],
        'ㄶ' => vec!['ㄴ', 'ㅎ'],
        'ㄺ' => vec!['ㄹ', 'ㄱ'],
        'ㄻ' => vec!['ㄹ', 'ㅁ'],
        'ㄼ' => vec!['ㄹ', 'ㅂ'],
        'ㄽ' => vec!['ㄹ', 'ㅅ'],
        'ㄾ' => vec!['ㄹ', 'ㅌ'],
        'ㄿ' => vec!['ㄹ', 'ㅍ'],
        'ㅀ' => vec!['ㄹ', 'ㅎ'],
        'ㅄ' => vec!['ㅂ', 'ㅅ'],
        'ㅘ' => vec!['ㅗ', 'ㅏ'],
        'ㅙ' => vec!['ㅗ', 'ㅐ'],
        'ㅚ' => vec!['ㅗ', 'ㅣ'],
        'ㅝ' => vec!['ㅜ', 'ㅓ'],
        'ㅞ' => vec!['ㅜ', 'ㅔ'],
        'ㅟ' => vec!['ㅜ', 'ㅣ'],
        'ㅢ' => vec!['ㅡ', 'ㅣ'],
        _ => vec![jamo],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_functionality() {
        // 기본동작
        assert_eq!(disassemble("가나다"), ['ㄱ', 'ㅏ', 'ㄴ', 'ㅏ', 'ㄷ', 'ㅏ']);
    }

    #[test]
    fn jongseong() {
        // 받침
        assert_eq!(disassemble("비행"), ['ㅂ', 'ㅣ', 'ㅎ', 'ㅐ', 'ㅇ']);
    }

    #[test]
    fn choseong_double_consonant() {
        // 초성에 쌍자음
        assert_eq!(disassemble("쓸다"), ['ㅆ', 'ㅡ', 'ㄹ', 'ㄷ', 'ㅏ']);
    }

    #[test]
    fn jungseong_complex_vowels() {
        // 중성에 복합모음
        assert_eq!(disassemble("의사"), ['ㅇ', 'ㅡ', 'ㅣ', 'ㅅ', 'ㅏ']);
    }

    #[test]
    fn jongseong_complex_consonants() {
        // 종성에 복합자음
        assert_eq!(
            disassemble("닭고기"),
            ['ㄷ', 'ㅏ', 'ㄹ', 'ㄱ', 'ㄱ', 'ㅗ', 'ㄱ', 'ㅣ']
        );
    }

    #[test]
    fn mixed() {
        // 혼합
        assert_eq!(
            disassemble("짧은"),
            ['ㅉ', 'ㅏ', 'ㄹ', 'ㅂ', 'ㅇ', 'ㅡ', 'ㄴ']
        );
    }

    #[test]
    fn typo() {
        // 오타
        assert_eq!(disassemble("옽ㅏ"), ['ㅇ', 'ㅗ', 'ㅌ', 'ㅏ']);
    }

    #[test]
    fn multi_language() {
        // 다국어
        assert_eq!(
            disassemble("AB삵e$@%2324sdf낄캌ㅋㅋㅋㅋ"),
            [
                'A', 'B', 'ㅅ', 'ㅏ', 'ㄹ', 'ㄱ', 'e', '$', '@', '%', '2', '3', '2', '4', 's', 'd',
                'f', 'ㄲ', 'ㅣ', 'ㄹ', 'ㅋ', 'ㅏ', 'ㅋ', 'ㅋ', 'ㅋ', 'ㅋ', 'ㅋ'
            ]
        );
    }

    #[test]
    fn uncommon_syllables() {
        // 흔하지 않은 음절
        assert_eq!(
            disassemble("뷁궬릪쯻튋"),
            [
                'ㅂ', 'ㅜ', 'ㅔ', 'ㄹ', 'ㄱ', 'ㄱ', 'ㅜ', 'ㅔ', 'ㄹ', 'ㄹ', 'ㅡ', 'ㅣ', 'ㅍ', 'ㅉ',
                'ㅡ', 'ㅣ', 'ㄹ', 'ㅂ', 'ㅌ', 'ㅜ', 'ㅣ', 'ㄹ', 'ㅂ'
            ]
        );
    }

    #[test]
    fn single_complex_consonant() {
        // 단일 복합자음
        assert_eq!(disassemble("ㄳ"), ['ㄱ', 'ㅅ']);
    }

    #[test]
    fn single_complex_vowel() {
        // 단일 복합모음
        assert_eq!(disassemble("ㅙ"), ['ㅗ', 'ㅐ']);
    }
}
