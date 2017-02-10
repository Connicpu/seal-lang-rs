use std::cmp::Ordering;

pub trait EmojiChar {
    fn is_emoji(self) -> bool;
    fn is_emoji_modifier_base(self) -> bool;
    fn is_emoji_modifier(self) -> bool;
}

fn range_ord(&(low, high): &(u32, u32), c: char) -> Ordering {
    if (c as u32) < low {
        Ordering::Greater
    } else if (c as u32) > high {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

impl EmojiChar for char {
    fn is_emoji(self) -> bool {
        EMOJI.binary_search_by(|r| range_ord(r, self)).is_ok()
    }

    fn is_emoji_modifier_base(self) -> bool {
        EMOJI_MODIFIER_BASE.binary_search_by(|r| range_ord(r, self)).is_ok()
    }

    fn is_emoji_modifier(self) -> bool {
        let c = self as u32;
        c >= 0x1F3FB && c <= 0x1F3FF
    }
}

static EMOJI: &'static [(u32, u32)] = &[(0x231A, 0x231B),
                                        (0x23E9, 0x23EC),
                                        (0x23F0, 0x23F0),
                                        (0x23F3, 0x23F3),
                                        (0x25FD, 0x25FE),
                                        (0x2614, 0x2615),
                                        (0x2648, 0x2653),
                                        (0x267F, 0x267F),
                                        (0x2693, 0x2693),
                                        (0x26A1, 0x26A1),
                                        (0x26AA, 0x26AB),
                                        (0x26BD, 0x26BE),
                                        (0x26C4, 0x26C5),
                                        (0x26CE, 0x26CE),
                                        (0x26D4, 0x26D4),
                                        (0x26EA, 0x26EA),
                                        (0x26F2, 0x26F3),
                                        (0x26F5, 0x26F5),
                                        (0x26FA, 0x26FA),
                                        (0x26FD, 0x26FD),
                                        (0x2705, 0x2705),
                                        (0x270A, 0x270B),
                                        (0x2728, 0x2728),
                                        (0x274C, 0x274C),
                                        (0x274E, 0x274E),
                                        (0x2753, 0x2755),
                                        (0x2757, 0x2757),
                                        (0x2795, 0x2797),
                                        (0x27B0, 0x27B0),
                                        (0x27BF, 0x27BF),
                                        (0x2B1B, 0x2B1C),
                                        (0x2B50, 0x2B50),
                                        (0x2B55, 0x2B55),
                                        (0x1F004, 0x1F004),
                                        (0x1F0CF, 0x1F0CF),
                                        (0x1F18E, 0x1F18E),
                                        (0x1F191, 0x1F19A),
                                        (0x1F1E6, 0x1F1FF),
                                        (0x1F201, 0x1F201),
                                        (0x1F21A, 0x1F21A),
                                        (0x1F22F, 0x1F22F),
                                        (0x1F232, 0x1F236),
                                        (0x1F238, 0x1F23A),
                                        (0x1F250, 0x1F251),
                                        (0x1F300, 0x1F320),
                                        (0x1F32D, 0x1F32F),
                                        (0x1F330, 0x1F335),
                                        (0x1F337, 0x1F37C),
                                        (0x1F37E, 0x1F37F),
                                        (0x1F380, 0x1F393),
                                        (0x1F3A0, 0x1F3C4),
                                        (0x1F3C5, 0x1F3C5),
                                        (0x1F3C6, 0x1F3CA),
                                        (0x1F3CF, 0x1F3D3),
                                        (0x1F3E0, 0x1F3F0),
                                        (0x1F3F4, 0x1F3F4),
                                        (0x1F3F8, 0x1F3FF),
                                        (0x1F400, 0x1F43E),
                                        (0x1F440, 0x1F440),
                                        (0x1F442, 0x1F4F7),
                                        (0x1F4F8, 0x1F4F8),
                                        (0x1F4F9, 0x1F4FC),
                                        (0x1F4FF, 0x1F4FF),
                                        (0x1F500, 0x1F53D),
                                        (0x1F54B, 0x1F54E),
                                        (0x1F550, 0x1F567),
                                        (0x1F57A, 0x1F57A),
                                        (0x1F595, 0x1F596),
                                        (0x1F5A4, 0x1F5A4),
                                        (0x1F5FB, 0x1F5FF),
                                        (0x1F600, 0x1F600),
                                        (0x1F601, 0x1F610),
                                        (0x1F611, 0x1F611),
                                        (0x1F612, 0x1F614),
                                        (0x1F615, 0x1F615),
                                        (0x1F616, 0x1F616),
                                        (0x1F617, 0x1F617),
                                        (0x1F618, 0x1F618),
                                        (0x1F619, 0x1F619),
                                        (0x1F61A, 0x1F61A),
                                        (0x1F61B, 0x1F61B),
                                        (0x1F61C, 0x1F61E),
                                        (0x1F61F, 0x1F61F),
                                        (0x1F620, 0x1F625),
                                        (0x1F626, 0x1F627),
                                        (0x1F628, 0x1F62B),
                                        (0x1F62C, 0x1F62C),
                                        (0x1F62D, 0x1F62D),
                                        (0x1F62E, 0x1F62F),
                                        (0x1F630, 0x1F633),
                                        (0x1F634, 0x1F634),
                                        (0x1F635, 0x1F640),
                                        (0x1F641, 0x1F642),
                                        (0x1F643, 0x1F644),
                                        (0x1F645, 0x1F64F),
                                        (0x1F680, 0x1F6C5),
                                        (0x1F6CC, 0x1F6CC),
                                        (0x1F6D0, 0x1F6D0),
                                        (0x1F6D1, 0x1F6D2),
                                        (0x1F6EB, 0x1F6EC),
                                        (0x1F6F4, 0x1F6F6),
                                        (0x1F910, 0x1F918),
                                        (0x1F919, 0x1F91E),
                                        (0x1F920, 0x1F927),
                                        (0x1F930, 0x1F930),
                                        (0x1F933, 0x1F93A),
                                        (0x1F93C, 0x1F93E),
                                        (0x1F940, 0x1F945),
                                        (0x1F947, 0x1F94B),
                                        (0x1F950, 0x1F95E),
                                        (0x1F980, 0x1F984),
                                        (0x1F985, 0x1F991),
                                        (0x1F9C0, 0x1F9C0)];

static EMOJI_MODIFIER_BASE: &'static [(u32, u32)] = &[(0x261D, 0x261D),
                                                      (0x26F9, 0x26F9),
                                                      (0x270A, 0x270B),
                                                      (0x270C, 0x270D),
                                                      (0x1F385, 0x1F385),
                                                      (0x1F3C2, 0x1F3C4),
                                                      (0x1F3C7, 0x1F3C7),
                                                      (0x1F3CA, 0x1F3CA),
                                                      (0x1F3CB, 0x1F3CC),
                                                      (0x1F442, 0x1F443),
                                                      (0x1F446, 0x1F450),
                                                      (0x1F466, 0x1F469),
                                                      (0x1F46E, 0x1F46E),
                                                      (0x1F470, 0x1F478),
                                                      (0x1F47C, 0x1F47C),
                                                      (0x1F481, 0x1F483),
                                                      (0x1F485, 0x1F487),
                                                      (0x1F4AA, 0x1F4AA),
                                                      (0x1F574, 0x1F575),
                                                      (0x1F57A, 0x1F57A),
                                                      (0x1F590, 0x1F590),
                                                      (0x1F595, 0x1F596),
                                                      (0x1F645, 0x1F647),
                                                      (0x1F64B, 0x1F64F),
                                                      (0x1F6A3, 0x1F6A3),
                                                      (0x1F6B4, 0x1F6B6),
                                                      (0x1F6C0, 0x1F6C0),
                                                      (0x1F6CC, 0x1F6CC),
                                                      (0x1F918, 0x1F918),
                                                      (0x1F919, 0x1F91C),
                                                      (0x1F91E, 0x1F91E),
                                                      (0x1F926, 0x1F926),
                                                      (0x1F930, 0x1F930),
                                                      (0x1F933, 0x1F939),
                                                      (0x1F93D, 0x1F93E)];
