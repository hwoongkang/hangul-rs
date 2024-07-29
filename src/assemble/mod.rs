#[derive(Debug)]
enum AssembleState {
    // initial state of the state machine
    // if choseong => Cho
    // if jungseong => Jung
    // if neither => append and back to initial
    Initial,
    // a choseong appeared.
    // if followed by a joinable choesongs => ChoCho
    // if followed by other choseongs => Cho, append the previous choseong
    // if followed by a jungseong => ChoJung
    // else => Initial,append the previous choseong and the current character
    Cho(char),
    // a jungseong appeared
    // if followed by a joinable jungseong => Jung, append the joined jungseong
    Jung(char),
    // joinable consonants appeared. third state is the joined consonant.
    // if followed by a jungseong => ChoJung, append the first consonant.
    // else => append joined choseong, back to initial with the current character.
    ChoCho(char, char, char),
    // a consonant followed by a vowel.
    ChoJung(char, char),
    // choseong-jungseong-jongseong.
    // if followed by a joinable jongseong => ChoJungJongJong
    // if followed by a jungseong => ChoJung, append the previous choseong and jungseong
    // else => append the assembled hangul, back to initial with the current character.
    ChoJungJong(char, char, char),
    // choseong-jungseong-jongseong-jongseong
    // cho, jung, jong1, jong2, joined
    // if followed by a jungseong => ChoJung(jong2, jung), append(cho, jung, jong1)
    // else => append(cho, jung, joined), back to initial with the current character.
    ChoJungJongJong(char, char, char, char, char),
}

mod utils;
use crate::hangul_unicodes::*;
use utils::*;
use AssembleState::*;

use crate::hangul_unicodes::assemble_hangul;
pub fn assemble(chars: &[char]) -> String {
    let mut result = String::new();

    let mut state = Initial;

    // made this to a function, because it is used more than twice.
    fn initialize(ch: char, state: &mut AssembleState, result: &mut String) {
        if is_cho(&ch) {
            *state = Cho(ch);
        } else if is_jung(&ch) {
            *state = Jung(ch);
        } else {
            result.push(ch);
            *state = Initial;
        }
    }

    for ch in chars.iter() {
        match state {
            Initial => {
                initialize(*ch, &mut state, &mut result);
            }
            Cho(prev) => {
                if is_cho(ch) {
                    if let Some(joined) = jong_maybe_joined(&prev, ch) {
                        state = ChoCho(prev, *ch, joined);
                    } else {
                        result.push(prev);
                        state = Cho(*ch);
                    }
                } else if is_jung(ch) {
                    state = ChoJung(prev, *ch);
                } else {
                    result.push(prev);
                    result.push(*ch);
                    state = Initial;
                }
            }
            ChoCho(char1, char2, joined) => {
                if is_jung(ch) {
                    result.push(char1);
                    state = ChoJung(char2, *ch);
                } else {
                    result.push(joined);
                    initialize(*ch, &mut state, &mut result);
                }
            }
            ChoJung(cho, jung) => {
                if is_jong(ch) {
                    state = ChoJungJong(cho, jung, *ch);
                } else if let Some(joined) = jung_maybe_joined(&jung, ch) {
                    state = ChoJung(cho, joined);
                } else {
                    result.push(assemble_hangul(&cho, &jung, None));
                    initialize(*ch, &mut state, &mut result);
                }
            }
            ChoJungJong(cho, jung, jong) => {
                if let Some(joined) = jong_maybe_joined(&jong, ch) {
                    state = ChoJungJongJong(cho, jung, jong, *ch, joined);
                } else if is_jung(ch) {
                    result.push(assemble_hangul(&cho, &jung, None));
                    state = ChoJung(jong, *ch);
                } else {
                    result.push(assemble_hangul(&cho, &jung, Some(&jong)));
                    initialize(*ch, &mut state, &mut result);
                }
            }
            ChoJungJongJong(cho, jung, jong1, jong2, joined) => {
                if is_jung(ch) {
                    result.push(assemble_hangul(&cho, &jung, Some(&jong1)));
                    state = ChoJung(jong2, *ch);
                } else {
                    result.push(assemble_hangul(&cho, &jung, Some(&joined)));
                    initialize(*ch, &mut state, &mut result);
                }
            }
            Jung(jung) => {
                if let Some(joined) = jung_maybe_joined(&jung, ch) {
                    result.push(joined);
                    state = Initial;
                } else {
                    result.push(jung);
                    initialize(*ch, &mut state, &mut result);
                }
            }
        }
    }
    println!("{:?}", state);

    // final flushing
    match state {
        Initial => {}
        Cho(char) => result.push(char),
        Jung(char) => result.push(char),
        ChoCho(_, _, joined) => {
            result.push(joined);
        }
        ChoJung(cho, jung) => {
            result.push(assemble_hangul(&cho, &jung, None));
        }
        ChoJungJong(cho, jung, jong) => {
            result.push(assemble_hangul(&cho, &jung, Some(&jong)));
        }
        ChoJungJongJong(cho, jung, _, _, joined) => {
            result.push(assemble_hangul(&cho, &jung, Some(&joined)));
        }
    }

    result
}

#[cfg(test)]
mod assemble_tests {

    use super::*;

    #[test]
    fn test_assemble_basic() {
        assert_eq!(assemble(&['ㄱ', 'ㅏ', 'ㄴ', 'ㅏ', 'ㄷ', 'ㅏ']), "가나다",);
    }

    #[test]
    fn test_assemble_vowel() {
        assert_eq!(assemble(&['ㅂ', 'ㅣ', 'ㅎ', 'ㅐ', 'ㅇ']), "비행",);
    }

    #[test]
    fn test_assemble_consonant() {
        assert_eq!(assemble(&['ㅆ', 'ㅡ', 'ㄹ', 'ㄷ', 'ㅏ']), "쓸다",);
    }

    #[test]
    fn test_assemble_mixed() {
        assert_eq!(assemble(&['ㅇ', 'ㅡ', 'ㅣ', 'ㅅ', 'ㅏ']), "의사",);
    }

    #[test]
    fn test_assemble_long() {
        assert_eq!(
            assemble(&['ㅉ', 'ㅏ', 'ㄹ', 'ㅂ', 'ㅇ', 'ㅡ', 'ㄴ']),
            "짧은",
        );
    }

    #[test]
    fn test_assemble_special_characters() {
        assert_eq!(
            assemble(&['ㄷ', 'ㅏ', 'ㄹ', 'ㄱ', 'ㄱ', 'ㅗ', 'ㄱ', 'ㅣ']),
            "닭고기",
        );
    }

    #[test]
    fn test_assemble_invalid_characters() {
        assert_eq!(
            assemble(&[
                'A', 'B', 'ㅅ', 'ㅏ', 'ㄹ', 'ㄱ', 'e', '$', '@', '%', '2', '3', '2', '4', 's', 'd',
                'f', 'ㄲ', 'ㅣ', 'ㄹ', 'ㅋ', 'ㅏ', 'ㅋ', 'ㅋ', 'ㅋ', 'ㅋ', 'ㅋ'
            ]),
            "AB삵e$@%2324sdf낄캌ㅋㅋㅋㅋ",
        );
    }

    #[test]
    fn test_assemble_repeated_characters() {
        assert_eq!(
            assemble(&[
                'ㅂ', 'ㅜ', 'ㅔ', 'ㄹ', 'ㄱ', 'ㄱ', 'ㅜ', 'ㅔ', 'ㄹ', 'ㄹ', 'ㅡ', 'ㅣ', 'ㅍ', 'ㅉ',
                'ㅡ', 'ㅣ', 'ㄹ', 'ㅂ', 'ㅌ', 'ㅜ', 'ㅣ', 'ㄹ', 'ㅂ'
            ]),
            "뷁궬릪쯻튋",
        );
    }

    #[test]
    fn test_assemble_single_character() {
        assert_eq!(assemble(&['ㄱ', 'ㅅ']), "ㄳ",);
    }

    #[test]
    fn test_assemble_vowel_combination() {
        assert_eq!(assemble(&['ㅗ', 'ㅐ']), "ㅙ",);
    }

    #[test]
    fn test_assemble_consonant_vowel_combination() {
        assert_eq!(assemble(&['ㅈ', 'ㅅ', 'ㅏ']), "ㅈ사",);
    }

    #[test]
    fn test_assemble_repeated_consonants() {
        assert_eq!(assemble(&['ㄱ', 'ㅅ', 'ㄱ', 'ㅅ']), "ㄳㄳ",);
    }

    #[test]
    fn test_assemble_repeated_vowels() {
        assert_eq!(assemble(&['ㅗ', 'ㅐ', 'ㅗ', 'ㅐ']), "ㅙㅙ",);
    }

    #[test]
    fn test_assemble_mixed_repeated() {
        assert_eq!(assemble(&['ㅈ', 'ㅗ', 'ㅗ', 'ㅐ']), "조ㅙ",);
    }

    #[test]
    fn test_assemble_mixed_invalid() {
        assert_eq!(assemble(&['ㅣ', 'ㅗ', 'ㅐ']), "ㅣㅙ",);
    }

    #[test]
    fn test_assemble_double_consonants() {
        assert_eq!(assemble(&['ㅃ', 'ㅉ', 'ㅏ', 'ㄸ']), "ㅃ짜ㄸ",);
    }

    #[test]
    fn test_assemble_double_vowels() {
        assert_eq!(assemble(&['ㅒ', 'ㅗ', 'ㅒ']), "ㅒㅗㅒ",);
    }

    #[test]
    fn test_assemble_mixed_double() {
        assert_eq!(assemble(&['ㅃ', 'ㅞ', 'ㄹ', 'ㄱ', 'ㅅ']), "쀍ㅅ",);
    }

    #[test]
    fn test_assemble_mixed_double_invalid() {
        assert_eq!(assemble(&['ㅃ', 'ㅞ', 'ㄹ', 'ㄱ', 'ㅏ']), "쀌가",);
    }

    #[test]
    fn test_assemble_mixed_double_repeated() {
        assert_eq!(
            assemble(&[
                'ㅃ', 'ㅞ', 'ㄹ', 'ㄱ', 'ㅞ', 'ㄹ', 'ㄱ', 'ㅞ', 'ㄹ', 'ㄱ', 'ㅂ'
            ]),
            "쀌궬궭ㅂ",
        );
    }
}
