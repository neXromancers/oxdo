use x11rb::protocol::xproto::Keysym;

pub struct Codepair {
    pub keysym: Keysym,
    pub ucs: char,
}

pub static KEYSYM_TO_CHAR: [Codepair; 775] = [
    Codepair { keysym: 0x01a1, ucs: '\u{0104}' }, /*                     Aogonek Ą LATIN CAPITAL LETTER A WITH OGONEK */
    Codepair { keysym: 0x01a2, ucs: '\u{02d8}' }, /*                       breve ˘ BREVE */
    Codepair { keysym: 0x01a3, ucs: '\u{0141}' }, /*                     Lstroke Ł LATIN CAPITAL LETTER L WITH STROKE */
    Codepair { keysym: 0x01a5, ucs: '\u{013d}' }, /*                      Lcaron Ľ LATIN CAPITAL LETTER L WITH CARON */
    Codepair { keysym: 0x01a6, ucs: '\u{015a}' }, /*                      Sacute Ś LATIN CAPITAL LETTER S WITH ACUTE */
    Codepair { keysym: 0x01a9, ucs: '\u{0160}' }, /*                      Scaron Š LATIN CAPITAL LETTER S WITH CARON */
    Codepair { keysym: 0x01aa, ucs: '\u{015e}' }, /*                    Scedilla Ş LATIN CAPITAL LETTER S WITH CEDILLA */
    Codepair { keysym: 0x01ab, ucs: '\u{0164}' }, /*                      Tcaron Ť LATIN CAPITAL LETTER T WITH CARON */
    Codepair { keysym: 0x01ac, ucs: '\u{0179}' }, /*                      Zacute Ź LATIN CAPITAL LETTER Z WITH ACUTE */
    Codepair { keysym: 0x01ae, ucs: '\u{017d}' }, /*                      Zcaron Ž LATIN CAPITAL LETTER Z WITH CARON */
    Codepair { keysym: 0x01af, ucs: '\u{017b}' }, /*                   Zabovedot Ż LATIN CAPITAL LETTER Z WITH DOT ABOVE */
    Codepair { keysym: 0x01b1, ucs: '\u{0105}' }, /*                     aogonek ą LATIN SMALL LETTER A WITH OGONEK */
    Codepair { keysym: 0x01b2, ucs: '\u{02db}' }, /*                      ogonek ˛ OGONEK */
    Codepair { keysym: 0x01b3, ucs: '\u{0142}' }, /*                     lstroke ł LATIN SMALL LETTER L WITH STROKE */
    Codepair { keysym: 0x01b5, ucs: '\u{013e}' }, /*                      lcaron ľ LATIN SMALL LETTER L WITH CARON */
    Codepair { keysym: 0x01b6, ucs: '\u{015b}' }, /*                      sacute ś LATIN SMALL LETTER S WITH ACUTE */
    Codepair { keysym: 0x01b7, ucs: '\u{02c7}' }, /*                       caron ˇ CARON */
    Codepair { keysym: 0x01b9, ucs: '\u{0161}' }, /*                      scaron š LATIN SMALL LETTER S WITH CARON */
    Codepair { keysym: 0x01ba, ucs: '\u{015f}' }, /*                    scedilla ş LATIN SMALL LETTER S WITH CEDILLA */
    Codepair { keysym: 0x01bb, ucs: '\u{0165}' }, /*                      tcaron ť LATIN SMALL LETTER T WITH CARON */
    Codepair { keysym: 0x01bc, ucs: '\u{017a}' }, /*                      zacute ź LATIN SMALL LETTER Z WITH ACUTE */
    Codepair { keysym: 0x01bd, ucs: '\u{02dd}' }, /*                 doubleacute ˝ DOUBLE ACUTE ACCENT */
    Codepair { keysym: 0x01be, ucs: '\u{017e}' }, /*                      zcaron ž LATIN SMALL LETTER Z WITH CARON */
    Codepair { keysym: 0x01bf, ucs: '\u{017c}' }, /*                   zabovedot ż LATIN SMALL LETTER Z WITH DOT ABOVE */
    Codepair { keysym: 0x01c0, ucs: '\u{0154}' }, /*                      Racute Ŕ LATIN CAPITAL LETTER R WITH ACUTE */
    Codepair { keysym: 0x01c3, ucs: '\u{0102}' }, /*                      Abreve Ă LATIN CAPITAL LETTER A WITH BREVE */
    Codepair { keysym: 0x01c5, ucs: '\u{0139}' }, /*                      Lacute Ĺ LATIN CAPITAL LETTER L WITH ACUTE */
    Codepair { keysym: 0x01c6, ucs: '\u{0106}' }, /*                      Cacute Ć LATIN CAPITAL LETTER C WITH ACUTE */
    Codepair { keysym: 0x01c8, ucs: '\u{010c}' }, /*                      Ccaron Č LATIN CAPITAL LETTER C WITH CARON */
    Codepair { keysym: 0x01ca, ucs: '\u{0118}' }, /*                     Eogonek Ę LATIN CAPITAL LETTER E WITH OGONEK */
    Codepair { keysym: 0x01cc, ucs: '\u{011a}' }, /*                      Ecaron Ě LATIN CAPITAL LETTER E WITH CARON */
    Codepair { keysym: 0x01cf, ucs: '\u{010e}' }, /*                      Dcaron Ď LATIN CAPITAL LETTER D WITH CARON */
    Codepair { keysym: 0x01d0, ucs: '\u{0110}' }, /*                     Dstroke Đ LATIN CAPITAL LETTER D WITH STROKE */
    Codepair { keysym: 0x01d1, ucs: '\u{0143}' }, /*                      Nacute Ń LATIN CAPITAL LETTER N WITH ACUTE */
    Codepair { keysym: 0x01d2, ucs: '\u{0147}' }, /*                      Ncaron Ň LATIN CAPITAL LETTER N WITH CARON */
    Codepair { keysym: 0x01d5, ucs: '\u{0150}' }, /*                Odoubleacute Ő LATIN CAPITAL LETTER O WITH DOUBLE ACUTE */
    Codepair { keysym: 0x01d8, ucs: '\u{0158}' }, /*                      Rcaron Ř LATIN CAPITAL LETTER R WITH CARON */
    Codepair { keysym: 0x01d9, ucs: '\u{016e}' }, /*                       Uring Ů LATIN CAPITAL LETTER U WITH RING ABOVE */
    Codepair { keysym: 0x01db, ucs: '\u{0170}' }, /*                Udoubleacute Ű LATIN CAPITAL LETTER U WITH DOUBLE ACUTE */
    Codepair { keysym: 0x01de, ucs: '\u{0162}' }, /*                    Tcedilla Ţ LATIN CAPITAL LETTER T WITH CEDILLA */
    Codepair { keysym: 0x01e0, ucs: '\u{0155}' }, /*                      racute ŕ LATIN SMALL LETTER R WITH ACUTE */
    Codepair { keysym: 0x01e3, ucs: '\u{0103}' }, /*                      abreve ă LATIN SMALL LETTER A WITH BREVE */
    Codepair { keysym: 0x01e5, ucs: '\u{013a}' }, /*                      lacute ĺ LATIN SMALL LETTER L WITH ACUTE */
    Codepair { keysym: 0x01e6, ucs: '\u{0107}' }, /*                      cacute ć LATIN SMALL LETTER C WITH ACUTE */
    Codepair { keysym: 0x01e8, ucs: '\u{010d}' }, /*                      ccaron č LATIN SMALL LETTER C WITH CARON */
    Codepair { keysym: 0x01ea, ucs: '\u{0119}' }, /*                     eogonek ę LATIN SMALL LETTER E WITH OGONEK */
    Codepair { keysym: 0x01ec, ucs: '\u{011b}' }, /*                      ecaron ě LATIN SMALL LETTER E WITH CARON */
    Codepair { keysym: 0x01ef, ucs: '\u{010f}' }, /*                      dcaron ď LATIN SMALL LETTER D WITH CARON */
    Codepair { keysym: 0x01f0, ucs: '\u{0111}' }, /*                     dstroke đ LATIN SMALL LETTER D WITH STROKE */
    Codepair { keysym: 0x01f1, ucs: '\u{0144}' }, /*                      nacute ń LATIN SMALL LETTER N WITH ACUTE */
    Codepair { keysym: 0x01f2, ucs: '\u{0148}' }, /*                      ncaron ň LATIN SMALL LETTER N WITH CARON */
    Codepair { keysym: 0x01f5, ucs: '\u{0151}' }, /*                odoubleacute ő LATIN SMALL LETTER O WITH DOUBLE ACUTE */
    Codepair { keysym: 0x01f8, ucs: '\u{0159}' }, /*                      rcaron ř LATIN SMALL LETTER R WITH CARON */
    Codepair { keysym: 0x01f9, ucs: '\u{016f}' }, /*                       uring ů LATIN SMALL LETTER U WITH RING ABOVE */
    Codepair { keysym: 0x01fb, ucs: '\u{0171}' }, /*                udoubleacute ű LATIN SMALL LETTER U WITH DOUBLE ACUTE */
    Codepair { keysym: 0x01fe, ucs: '\u{0163}' }, /*                    tcedilla ţ LATIN SMALL LETTER T WITH CEDILLA */
    Codepair { keysym: 0x01ff, ucs: '\u{02d9}' }, /*                    abovedot ˙ DOT ABOVE */
    Codepair { keysym: 0x02a1, ucs: '\u{0126}' }, /*                     Hstroke Ħ LATIN CAPITAL LETTER H WITH STROKE */
    Codepair { keysym: 0x02a6, ucs: '\u{0124}' }, /*                 Hcircumflex Ĥ LATIN CAPITAL LETTER H WITH CIRCUMFLEX */
    Codepair { keysym: 0x02a9, ucs: '\u{0130}' }, /*                   Iabovedot İ LATIN CAPITAL LETTER I WITH DOT ABOVE */
    Codepair { keysym: 0x02ab, ucs: '\u{011e}' }, /*                      Gbreve Ğ LATIN CAPITAL LETTER G WITH BREVE */
    Codepair { keysym: 0x02ac, ucs: '\u{0134}' }, /*                 Jcircumflex Ĵ LATIN CAPITAL LETTER J WITH CIRCUMFLEX */
    Codepair { keysym: 0x02b1, ucs: '\u{0127}' }, /*                     hstroke ħ LATIN SMALL LETTER H WITH STROKE */
    Codepair { keysym: 0x02b6, ucs: '\u{0125}' }, /*                 hcircumflex ĥ LATIN SMALL LETTER H WITH CIRCUMFLEX */
    Codepair { keysym: 0x02b9, ucs: '\u{0131}' }, /*                    idotless ı LATIN SMALL LETTER DOTLESS I */
    Codepair { keysym: 0x02bb, ucs: '\u{011f}' }, /*                      gbreve ğ LATIN SMALL LETTER G WITH BREVE */
    Codepair { keysym: 0x02bc, ucs: '\u{0135}' }, /*                 jcircumflex ĵ LATIN SMALL LETTER J WITH CIRCUMFLEX */
    Codepair { keysym: 0x02c5, ucs: '\u{010a}' }, /*                   Cabovedot Ċ LATIN CAPITAL LETTER C WITH DOT ABOVE */
    Codepair { keysym: 0x02c6, ucs: '\u{0108}' }, /*                 Ccircumflex Ĉ LATIN CAPITAL LETTER C WITH CIRCUMFLEX */
    Codepair { keysym: 0x02d5, ucs: '\u{0120}' }, /*                   Gabovedot Ġ LATIN CAPITAL LETTER G WITH DOT ABOVE */
    Codepair { keysym: 0x02d8, ucs: '\u{011c}' }, /*                 Gcircumflex Ĝ LATIN CAPITAL LETTER G WITH CIRCUMFLEX */
    Codepair { keysym: 0x02dd, ucs: '\u{016c}' }, /*                      Ubreve Ŭ LATIN CAPITAL LETTER U WITH BREVE */
    Codepair { keysym: 0x02de, ucs: '\u{015c}' }, /*                 Scircumflex Ŝ LATIN CAPITAL LETTER S WITH CIRCUMFLEX */
    Codepair { keysym: 0x02e5, ucs: '\u{010b}' }, /*                   cabovedot ċ LATIN SMALL LETTER C WITH DOT ABOVE */
    Codepair { keysym: 0x02e6, ucs: '\u{0109}' }, /*                 ccircumflex ĉ LATIN SMALL LETTER C WITH CIRCUMFLEX */
    Codepair { keysym: 0x02f5, ucs: '\u{0121}' }, /*                   gabovedot ġ LATIN SMALL LETTER G WITH DOT ABOVE */
    Codepair { keysym: 0x02f8, ucs: '\u{011d}' }, /*                 gcircumflex ĝ LATIN SMALL LETTER G WITH CIRCUMFLEX */
    Codepair { keysym: 0x02fd, ucs: '\u{016d}' }, /*                      ubreve ŭ LATIN SMALL LETTER U WITH BREVE */
    Codepair { keysym: 0x02fe, ucs: '\u{015d}' }, /*                 scircumflex ŝ LATIN SMALL LETTER S WITH CIRCUMFLEX */
    Codepair { keysym: 0x03a2, ucs: '\u{0138}' }, /*                         kra ĸ LATIN SMALL LETTER KRA */
    Codepair { keysym: 0x03a3, ucs: '\u{0156}' }, /*                    Rcedilla Ŗ LATIN CAPITAL LETTER R WITH CEDILLA */
    Codepair { keysym: 0x03a5, ucs: '\u{0128}' }, /*                      Itilde Ĩ LATIN CAPITAL LETTER I WITH TILDE */
    Codepair { keysym: 0x03a6, ucs: '\u{013b}' }, /*                    Lcedilla Ļ LATIN CAPITAL LETTER L WITH CEDILLA */
    Codepair { keysym: 0x03aa, ucs: '\u{0112}' }, /*                     Emacron Ē LATIN CAPITAL LETTER E WITH MACRON */
    Codepair { keysym: 0x03ab, ucs: '\u{0122}' }, /*                    Gcedilla Ģ LATIN CAPITAL LETTER G WITH CEDILLA */
    Codepair { keysym: 0x03ac, ucs: '\u{0166}' }, /*                      Tslash Ŧ LATIN CAPITAL LETTER T WITH STROKE */
    Codepair { keysym: 0x03b3, ucs: '\u{0157}' }, /*                    rcedilla ŗ LATIN SMALL LETTER R WITH CEDILLA */
    Codepair { keysym: 0x03b5, ucs: '\u{0129}' }, /*                      itilde ĩ LATIN SMALL LETTER I WITH TILDE */
    Codepair { keysym: 0x03b6, ucs: '\u{013c}' }, /*                    lcedilla ļ LATIN SMALL LETTER L WITH CEDILLA */
    Codepair { keysym: 0x03ba, ucs: '\u{0113}' }, /*                     emacron ē LATIN SMALL LETTER E WITH MACRON */
    Codepair { keysym: 0x03bb, ucs: '\u{0123}' }, /*                    gcedilla ģ LATIN SMALL LETTER G WITH CEDILLA */
    Codepair { keysym: 0x03bc, ucs: '\u{0167}' }, /*                      tslash ŧ LATIN SMALL LETTER T WITH STROKE */
    Codepair { keysym: 0x03bd, ucs: '\u{014a}' }, /*                         ENG Ŋ LATIN CAPITAL LETTER ENG */
    Codepair { keysym: 0x03bf, ucs: '\u{014b}' }, /*                         eng ŋ LATIN SMALL LETTER ENG */
    Codepair { keysym: 0x03c0, ucs: '\u{0100}' }, /*                     Amacron Ā LATIN CAPITAL LETTER A WITH MACRON */
    Codepair { keysym: 0x03c7, ucs: '\u{012e}' }, /*                     Iogonek Į LATIN CAPITAL LETTER I WITH OGONEK */
    Codepair { keysym: 0x03cc, ucs: '\u{0116}' }, /*                   Eabovedot Ė LATIN CAPITAL LETTER E WITH DOT ABOVE */
    Codepair { keysym: 0x03cf, ucs: '\u{012a}' }, /*                     Imacron Ī LATIN CAPITAL LETTER I WITH MACRON */
    Codepair { keysym: 0x03d1, ucs: '\u{0145}' }, /*                    Ncedilla Ņ LATIN CAPITAL LETTER N WITH CEDILLA */
    Codepair { keysym: 0x03d2, ucs: '\u{014c}' }, /*                     Omacron Ō LATIN CAPITAL LETTER O WITH MACRON */
    Codepair { keysym: 0x03d3, ucs: '\u{0136}' }, /*                    Kcedilla Ķ LATIN CAPITAL LETTER K WITH CEDILLA */
    Codepair { keysym: 0x03d9, ucs: '\u{0172}' }, /*                     Uogonek Ų LATIN CAPITAL LETTER U WITH OGONEK */
    Codepair { keysym: 0x03dd, ucs: '\u{0168}' }, /*                      Utilde Ũ LATIN CAPITAL LETTER U WITH TILDE */
    Codepair { keysym: 0x03de, ucs: '\u{016a}' }, /*                     Umacron Ū LATIN CAPITAL LETTER U WITH MACRON */
    Codepair { keysym: 0x03e0, ucs: '\u{0101}' }, /*                     amacron ā LATIN SMALL LETTER A WITH MACRON */
    Codepair { keysym: 0x03e7, ucs: '\u{012f}' }, /*                     iogonek į LATIN SMALL LETTER I WITH OGONEK */
    Codepair { keysym: 0x03ec, ucs: '\u{0117}' }, /*                   eabovedot ė LATIN SMALL LETTER E WITH DOT ABOVE */
    Codepair { keysym: 0x03ef, ucs: '\u{012b}' }, /*                     imacron ī LATIN SMALL LETTER I WITH MACRON */
    Codepair { keysym: 0x03f1, ucs: '\u{0146}' }, /*                    ncedilla ņ LATIN SMALL LETTER N WITH CEDILLA */
    Codepair { keysym: 0x03f2, ucs: '\u{014d}' }, /*                     omacron ō LATIN SMALL LETTER O WITH MACRON */
    Codepair { keysym: 0x03f3, ucs: '\u{0137}' }, /*                    kcedilla ķ LATIN SMALL LETTER K WITH CEDILLA */
    Codepair { keysym: 0x03f9, ucs: '\u{0173}' }, /*                     uogonek ų LATIN SMALL LETTER U WITH OGONEK */
    Codepair { keysym: 0x03fd, ucs: '\u{0169}' }, /*                      utilde ũ LATIN SMALL LETTER U WITH TILDE */
    Codepair { keysym: 0x03fe, ucs: '\u{016b}' }, /*                     umacron ū LATIN SMALL LETTER U WITH MACRON */
    Codepair { keysym: 0x047e, ucs: '\u{203e}' }, /*                    overline ‾ OVERLINE */
    Codepair { keysym: 0x04a1, ucs: '\u{3002}' }, /*               kana_fullstop 。 IDEOGRAPHIC FULL STOP */
    Codepair { keysym: 0x04a2, ucs: '\u{300c}' }, /*         kana_openingbracket 「 LEFT CORNER BRACKET */
    Codepair { keysym: 0x04a3, ucs: '\u{300d}' }, /*         kana_closingbracket 」 RIGHT CORNER BRACKET */
    Codepair { keysym: 0x04a4, ucs: '\u{3001}' }, /*                  kana_comma 、 IDEOGRAPHIC COMMA */
    Codepair { keysym: 0x04a5, ucs: '\u{30fb}' }, /*            kana_conjunctive ・ KATAKANA MIDDLE DOT */
    Codepair { keysym: 0x04a6, ucs: '\u{30f2}' }, /*                     kana_WO ヲ KATAKANA LETTER WO */
    Codepair { keysym: 0x04a7, ucs: '\u{30a1}' }, /*                      kana_a ァ KATAKANA LETTER SMALL A */
    Codepair { keysym: 0x04a8, ucs: '\u{30a3}' }, /*                      kana_i ィ KATAKANA LETTER SMALL I */
    Codepair { keysym: 0x04a9, ucs: '\u{30a5}' }, /*                      kana_u ゥ KATAKANA LETTER SMALL U */
    Codepair { keysym: 0x04aa, ucs: '\u{30a7}' }, /*                      kana_e ェ KATAKANA LETTER SMALL E */
    Codepair { keysym: 0x04ab, ucs: '\u{30a9}' }, /*                      kana_o ォ KATAKANA LETTER SMALL O */
    Codepair { keysym: 0x04ac, ucs: '\u{30e3}' }, /*                     kana_ya ャ KATAKANA LETTER SMALL YA */
    Codepair { keysym: 0x04ad, ucs: '\u{30e5}' }, /*                     kana_yu ュ KATAKANA LETTER SMALL YU */
    Codepair { keysym: 0x04ae, ucs: '\u{30e7}' }, /*                     kana_yo ョ KATAKANA LETTER SMALL YO */
    Codepair { keysym: 0x04af, ucs: '\u{30c3}' }, /*                    kana_tsu ッ KATAKANA LETTER SMALL TU */
    Codepair { keysym: 0x04b0, ucs: '\u{30fc}' }, /*              prolongedsound ー KATAKANA-HIRAGANA PROLONGED SOUND MARK */
    Codepair { keysym: 0x04b1, ucs: '\u{30a2}' }, /*                      kana_A ア KATAKANA LETTER A */
    Codepair { keysym: 0x04b2, ucs: '\u{30a4}' }, /*                      kana_I イ KATAKANA LETTER I */
    Codepair { keysym: 0x04b3, ucs: '\u{30a6}' }, /*                      kana_U ウ KATAKANA LETTER U */
    Codepair { keysym: 0x04b4, ucs: '\u{30a8}' }, /*                      kana_E エ KATAKANA LETTER E */
    Codepair { keysym: 0x04b5, ucs: '\u{30aa}' }, /*                      kana_O オ KATAKANA LETTER O */
    Codepair { keysym: 0x04b6, ucs: '\u{30ab}' }, /*                     kana_KA カ KATAKANA LETTER KA */
    Codepair { keysym: 0x04b7, ucs: '\u{30ad}' }, /*                     kana_KI キ KATAKANA LETTER KI */
    Codepair { keysym: 0x04b8, ucs: '\u{30af}' }, /*                     kana_KU ク KATAKANA LETTER KU */
    Codepair { keysym: 0x04b9, ucs: '\u{30b1}' }, /*                     kana_KE ケ KATAKANA LETTER KE */
    Codepair { keysym: 0x04ba, ucs: '\u{30b3}' }, /*                     kana_KO コ KATAKANA LETTER KO */
    Codepair { keysym: 0x04bb, ucs: '\u{30b5}' }, /*                     kana_SA サ KATAKANA LETTER SA */
    Codepair { keysym: 0x04bc, ucs: '\u{30b7}' }, /*                    kana_SHI シ KATAKANA LETTER SI */
    Codepair { keysym: 0x04bd, ucs: '\u{30b9}' }, /*                     kana_SU ス KATAKANA LETTER SU */
    Codepair { keysym: 0x04be, ucs: '\u{30bb}' }, /*                     kana_SE セ KATAKANA LETTER SE */
    Codepair { keysym: 0x04bf, ucs: '\u{30bd}' }, /*                     kana_SO ソ KATAKANA LETTER SO */
    Codepair { keysym: 0x04c0, ucs: '\u{30bf}' }, /*                     kana_TA タ KATAKANA LETTER TA */
    Codepair { keysym: 0x04c1, ucs: '\u{30c1}' }, /*                    kana_CHI チ KATAKANA LETTER TI */
    Codepair { keysym: 0x04c2, ucs: '\u{30c4}' }, /*                    kana_TSU ツ KATAKANA LETTER TU */
    Codepair { keysym: 0x04c3, ucs: '\u{30c6}' }, /*                     kana_TE テ KATAKANA LETTER TE */
    Codepair { keysym: 0x04c4, ucs: '\u{30c8}' }, /*                     kana_TO ト KATAKANA LETTER TO */
    Codepair { keysym: 0x04c5, ucs: '\u{30ca}' }, /*                     kana_NA ナ KATAKANA LETTER NA */
    Codepair { keysym: 0x04c6, ucs: '\u{30cb}' }, /*                     kana_NI ニ KATAKANA LETTER NI */
    Codepair { keysym: 0x04c7, ucs: '\u{30cc}' }, /*                     kana_NU ヌ KATAKANA LETTER NU */
    Codepair { keysym: 0x04c8, ucs: '\u{30cd}' }, /*                     kana_NE ネ KATAKANA LETTER NE */
    Codepair { keysym: 0x04c9, ucs: '\u{30ce}' }, /*                     kana_NO ノ KATAKANA LETTER NO */
    Codepair { keysym: 0x04ca, ucs: '\u{30cf}' }, /*                     kana_HA ハ KATAKANA LETTER HA */
    Codepair { keysym: 0x04cb, ucs: '\u{30d2}' }, /*                     kana_HI ヒ KATAKANA LETTER HI */
    Codepair { keysym: 0x04cc, ucs: '\u{30d5}' }, /*                     kana_FU フ KATAKANA LETTER HU */
    Codepair { keysym: 0x04cd, ucs: '\u{30d8}' }, /*                     kana_HE ヘ KATAKANA LETTER HE */
    Codepair { keysym: 0x04ce, ucs: '\u{30db}' }, /*                     kana_HO ホ KATAKANA LETTER HO */
    Codepair { keysym: 0x04cf, ucs: '\u{30de}' }, /*                     kana_MA マ KATAKANA LETTER MA */
    Codepair { keysym: 0x04d0, ucs: '\u{30df}' }, /*                     kana_MI ミ KATAKANA LETTER MI */
    Codepair { keysym: 0x04d1, ucs: '\u{30e0}' }, /*                     kana_MU ム KATAKANA LETTER MU */
    Codepair { keysym: 0x04d2, ucs: '\u{30e1}' }, /*                     kana_ME メ KATAKANA LETTER ME */
    Codepair { keysym: 0x04d3, ucs: '\u{30e2}' }, /*                     kana_MO モ KATAKANA LETTER MO */
    Codepair { keysym: 0x04d4, ucs: '\u{30e4}' }, /*                     kana_YA ヤ KATAKANA LETTER YA */
    Codepair { keysym: 0x04d5, ucs: '\u{30e6}' }, /*                     kana_YU ユ KATAKANA LETTER YU */
    Codepair { keysym: 0x04d6, ucs: '\u{30e8}' }, /*                     kana_YO ヨ KATAKANA LETTER YO */
    Codepair { keysym: 0x04d7, ucs: '\u{30e9}' }, /*                     kana_RA ラ KATAKANA LETTER RA */
    Codepair { keysym: 0x04d8, ucs: '\u{30ea}' }, /*                     kana_RI リ KATAKANA LETTER RI */
    Codepair { keysym: 0x04d9, ucs: '\u{30eb}' }, /*                     kana_RU ル KATAKANA LETTER RU */
    Codepair { keysym: 0x04da, ucs: '\u{30ec}' }, /*                     kana_RE レ KATAKANA LETTER RE */
    Codepair { keysym: 0x04db, ucs: '\u{30ed}' }, /*                     kana_RO ロ KATAKANA LETTER RO */
    Codepair { keysym: 0x04dc, ucs: '\u{30ef}' }, /*                     kana_WA ワ KATAKANA LETTER WA */
    Codepair { keysym: 0x04dd, ucs: '\u{30f3}' }, /*                      kana_N ン KATAKANA LETTER N */
    Codepair { keysym: 0x04de, ucs: '\u{309b}' }, /*                 voicedsound ゛ KATAKANA-HIRAGANA VOICED SOUND MARK */
    Codepair { keysym: 0x04df, ucs: '\u{309c}' }, /*             semivoicedsound ゜ KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK */
    Codepair { keysym: 0x05ac, ucs: '\u{060c}' }, /*                Arabic_comma ، ARABIC COMMA */
    Codepair { keysym: 0x05bb, ucs: '\u{061b}' }, /*            Arabic_semicolon ؛ ARABIC SEMICOLON */
    Codepair { keysym: 0x05bf, ucs: '\u{061f}' }, /*        Arabic_question_mark ؟ ARABIC QUESTION MARK */
    Codepair { keysym: 0x05c1, ucs: '\u{0621}' }, /*                Arabic_hamza ء ARABIC LETTER HAMZA */
    Codepair { keysym: 0x05c2, ucs: '\u{0622}' }, /*          Arabic_maddaonalef آ ARABIC LETTER ALEF WITH MADDA ABOVE */
    Codepair { keysym: 0x05c3, ucs: '\u{0623}' }, /*          Arabic_hamzaonalef أ ARABIC LETTER ALEF WITH HAMZA ABOVE */
    Codepair { keysym: 0x05c4, ucs: '\u{0624}' }, /*           Arabic_hamzaonwaw ؤ ARABIC LETTER WAW WITH HAMZA ABOVE */
    Codepair { keysym: 0x05c5, ucs: '\u{0625}' }, /*       Arabic_hamzaunderalef إ ARABIC LETTER ALEF WITH HAMZA BELOW */
    Codepair { keysym: 0x05c6, ucs: '\u{0626}' }, /*           Arabic_hamzaonyeh ئ ARABIC LETTER YEH WITH HAMZA ABOVE */
    Codepair { keysym: 0x05c7, ucs: '\u{0627}' }, /*                 Arabic_alef ا ARABIC LETTER ALEF */
    Codepair { keysym: 0x05c8, ucs: '\u{0628}' }, /*                  Arabic_beh ب ARABIC LETTER BEH */
    Codepair { keysym: 0x05c9, ucs: '\u{0629}' }, /*           Arabic_tehmarbuta ة ARABIC LETTER TEH MARBUTA */
    Codepair { keysym: 0x05ca, ucs: '\u{062a}' }, /*                  Arabic_teh ت ARABIC LETTER TEH */
    Codepair { keysym: 0x05cb, ucs: '\u{062b}' }, /*                 Arabic_theh ث ARABIC LETTER THEH */
    Codepair { keysym: 0x05cc, ucs: '\u{062c}' }, /*                 Arabic_jeem ج ARABIC LETTER JEEM */
    Codepair { keysym: 0x05cd, ucs: '\u{062d}' }, /*                  Arabic_hah ح ARABIC LETTER HAH */
    Codepair { keysym: 0x05ce, ucs: '\u{062e}' }, /*                 Arabic_khah خ ARABIC LETTER KHAH */
    Codepair { keysym: 0x05cf, ucs: '\u{062f}' }, /*                  Arabic_dal د ARABIC LETTER DAL */
    Codepair { keysym: 0x05d0, ucs: '\u{0630}' }, /*                 Arabic_thal ذ ARABIC LETTER THAL */
    Codepair { keysym: 0x05d1, ucs: '\u{0631}' }, /*                   Arabic_ra ر ARABIC LETTER REH */
    Codepair { keysym: 0x05d2, ucs: '\u{0632}' }, /*                 Arabic_zain ز ARABIC LETTER ZAIN */
    Codepair { keysym: 0x05d3, ucs: '\u{0633}' }, /*                 Arabic_seen س ARABIC LETTER SEEN */
    Codepair { keysym: 0x05d4, ucs: '\u{0634}' }, /*                Arabic_sheen ش ARABIC LETTER SHEEN */
    Codepair { keysym: 0x05d5, ucs: '\u{0635}' }, /*                  Arabic_sad ص ARABIC LETTER SAD */
    Codepair { keysym: 0x05d6, ucs: '\u{0636}' }, /*                  Arabic_dad ض ARABIC LETTER DAD */
    Codepair { keysym: 0x05d7, ucs: '\u{0637}' }, /*                  Arabic_tah ط ARABIC LETTER TAH */
    Codepair { keysym: 0x05d8, ucs: '\u{0638}' }, /*                  Arabic_zah ظ ARABIC LETTER ZAH */
    Codepair { keysym: 0x05d9, ucs: '\u{0639}' }, /*                  Arabic_ain ع ARABIC LETTER AIN */
    Codepair { keysym: 0x05da, ucs: '\u{063a}' }, /*                Arabic_ghain غ ARABIC LETTER GHAIN */
    Codepair { keysym: 0x05e0, ucs: '\u{0640}' }, /*              Arabic_tatweel ـ ARABIC TATWEEL */
    Codepair { keysym: 0x05e1, ucs: '\u{0641}' }, /*                  Arabic_feh ف ARABIC LETTER FEH */
    Codepair { keysym: 0x05e2, ucs: '\u{0642}' }, /*                  Arabic_qaf ق ARABIC LETTER QAF */
    Codepair { keysym: 0x05e3, ucs: '\u{0643}' }, /*                  Arabic_kaf ك ARABIC LETTER KAF */
    Codepair { keysym: 0x05e4, ucs: '\u{0644}' }, /*                  Arabic_lam ل ARABIC LETTER LAM */
    Codepair { keysym: 0x05e5, ucs: '\u{0645}' }, /*                 Arabic_meem م ARABIC LETTER MEEM */
    Codepair { keysym: 0x05e6, ucs: '\u{0646}' }, /*                 Arabic_noon ن ARABIC LETTER NOON */
    Codepair { keysym: 0x05e7, ucs: '\u{0647}' }, /*                   Arabic_ha ه ARABIC LETTER HEH */
    Codepair { keysym: 0x05e8, ucs: '\u{0648}' }, /*                  Arabic_waw و ARABIC LETTER WAW */
    Codepair { keysym: 0x05e9, ucs: '\u{0649}' }, /*          Arabic_alefmaksura ى ARABIC LETTER ALEF MAKSURA */
    Codepair { keysym: 0x05ea, ucs: '\u{064a}' }, /*                  Arabic_yeh ي ARABIC LETTER YEH */
    Codepair { keysym: 0x05eb, ucs: '\u{064b}' }, /*             Arabic_fathatan ً ARABIC FATHATAN */
    Codepair { keysym: 0x05ec, ucs: '\u{064c}' }, /*             Arabic_dammatan ٌ ARABIC DAMMATAN */
    Codepair { keysym: 0x05ed, ucs: '\u{064d}' }, /*             Arabic_kasratan ٍ ARABIC KASRATAN */
    Codepair { keysym: 0x05ee, ucs: '\u{064e}' }, /*                Arabic_fatha َ ARABIC FATHA */
    Codepair { keysym: 0x05ef, ucs: '\u{064f}' }, /*                Arabic_damma ُ ARABIC DAMMA */
    Codepair { keysym: 0x05f0, ucs: '\u{0650}' }, /*                Arabic_kasra ِ ARABIC KASRA */
    Codepair { keysym: 0x05f1, ucs: '\u{0651}' }, /*               Arabic_shadda ّ ARABIC SHADDA */
    Codepair { keysym: 0x05f2, ucs: '\u{0652}' }, /*                Arabic_sukun ْ ARABIC SUKUN */
    Codepair { keysym: 0x06a1, ucs: '\u{0452}' }, /*                 Serbian_dje ђ CYRILLIC SMALL LETTER DJE */
    Codepair { keysym: 0x06a2, ucs: '\u{0453}' }, /*               Macedonia_gje ѓ CYRILLIC SMALL LETTER GJE */
    Codepair { keysym: 0x06a3, ucs: '\u{0451}' }, /*                 Cyrillic_io ё CYRILLIC SMALL LETTER IO */
    Codepair { keysym: 0x06a4, ucs: '\u{0454}' }, /*                Ukrainian_ie є CYRILLIC SMALL LETTER UKRAINIAN IE */
    Codepair { keysym: 0x06a5, ucs: '\u{0455}' }, /*               Macedonia_dse ѕ CYRILLIC SMALL LETTER DZE */
    Codepair { keysym: 0x06a6, ucs: '\u{0456}' }, /*                 Ukrainian_i і CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I */
    Codepair { keysym: 0x06a7, ucs: '\u{0457}' }, /*                Ukrainian_yi ї CYRILLIC SMALL LETTER YI */
    Codepair { keysym: 0x06a8, ucs: '\u{0458}' }, /*                 Cyrillic_je ј CYRILLIC SMALL LETTER JE */
    Codepair { keysym: 0x06a9, ucs: '\u{0459}' }, /*                Cyrillic_lje љ CYRILLIC SMALL LETTER LJE */
    Codepair { keysym: 0x06aa, ucs: '\u{045a}' }, /*                Cyrillic_nje њ CYRILLIC SMALL LETTER NJE */
    Codepair { keysym: 0x06ab, ucs: '\u{045b}' }, /*                Serbian_tshe ћ CYRILLIC SMALL LETTER TSHE */
    Codepair { keysym: 0x06ac, ucs: '\u{045c}' }, /*               Macedonia_kje ќ CYRILLIC SMALL LETTER KJE */
    Codepair { keysym: 0x06ad, ucs: '\u{0491}' }, /*   Ukrainian_ghe_with_upturn ґ CYRILLIC SMALL LETTER GHE WITH UPTURN */
    Codepair { keysym: 0x06ae, ucs: '\u{045e}' }, /*         Byelorussian_shortu ў CYRILLIC SMALL LETTER SHORT U */
    Codepair { keysym: 0x06af, ucs: '\u{045f}' }, /*               Cyrillic_dzhe џ CYRILLIC SMALL LETTER DZHE */
    Codepair { keysym: 0x06b0, ucs: '\u{2116}' }, /*                  numerosign № NUMERO SIGN */
    Codepair { keysym: 0x06b1, ucs: '\u{0402}' }, /*                 Serbian_DJE Ђ CYRILLIC CAPITAL LETTER DJE */
    Codepair { keysym: 0x06b2, ucs: '\u{0403}' }, /*               Macedonia_GJE Ѓ CYRILLIC CAPITAL LETTER GJE */
    Codepair { keysym: 0x06b3, ucs: '\u{0401}' }, /*                 Cyrillic_IO Ё CYRILLIC CAPITAL LETTER IO */
    Codepair { keysym: 0x06b4, ucs: '\u{0404}' }, /*                Ukrainian_IE Є CYRILLIC CAPITAL LETTER UKRAINIAN IE */
    Codepair { keysym: 0x06b5, ucs: '\u{0405}' }, /*               Macedonia_DSE Ѕ CYRILLIC CAPITAL LETTER DZE */
    Codepair { keysym: 0x06b6, ucs: '\u{0406}' }, /*                 Ukrainian_I І CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I */
    Codepair { keysym: 0x06b7, ucs: '\u{0407}' }, /*                Ukrainian_YI Ї CYRILLIC CAPITAL LETTER YI */
    Codepair { keysym: 0x06b8, ucs: '\u{0408}' }, /*                 Cyrillic_JE Ј CYRILLIC CAPITAL LETTER JE */
    Codepair { keysym: 0x06b9, ucs: '\u{0409}' }, /*                Cyrillic_LJE Љ CYRILLIC CAPITAL LETTER LJE */
    Codepair { keysym: 0x06ba, ucs: '\u{040a}' }, /*                Cyrillic_NJE Њ CYRILLIC CAPITAL LETTER NJE */
    Codepair { keysym: 0x06bb, ucs: '\u{040b}' }, /*                Serbian_TSHE Ћ CYRILLIC CAPITAL LETTER TSHE */
    Codepair { keysym: 0x06bc, ucs: '\u{040c}' }, /*               Macedonia_KJE Ќ CYRILLIC CAPITAL LETTER KJE */
    Codepair { keysym: 0x06bd, ucs: '\u{0490}' }, /*   Ukrainian_GHE_WITH_UPTURN Ґ CYRILLIC CAPITAL LETTER GHE WITH UPTURN */
    Codepair { keysym: 0x06be, ucs: '\u{040e}' }, /*         Byelorussian_SHORTU Ў CYRILLIC CAPITAL LETTER SHORT U */
    Codepair { keysym: 0x06bf, ucs: '\u{040f}' }, /*               Cyrillic_DZHE Џ CYRILLIC CAPITAL LETTER DZHE */
    Codepair { keysym: 0x06c0, ucs: '\u{044e}' }, /*                 Cyrillic_yu ю CYRILLIC SMALL LETTER YU */
    Codepair { keysym: 0x06c1, ucs: '\u{0430}' }, /*                  Cyrillic_a а CYRILLIC SMALL LETTER A */
    Codepair { keysym: 0x06c2, ucs: '\u{0431}' }, /*                 Cyrillic_be б CYRILLIC SMALL LETTER BE */
    Codepair { keysym: 0x06c3, ucs: '\u{0446}' }, /*                Cyrillic_tse ц CYRILLIC SMALL LETTER TSE */
    Codepair { keysym: 0x06c4, ucs: '\u{0434}' }, /*                 Cyrillic_de д CYRILLIC SMALL LETTER DE */
    Codepair { keysym: 0x06c5, ucs: '\u{0435}' }, /*                 Cyrillic_ie е CYRILLIC SMALL LETTER IE */
    Codepair { keysym: 0x06c6, ucs: '\u{0444}' }, /*                 Cyrillic_ef ф CYRILLIC SMALL LETTER EF */
    Codepair { keysym: 0x06c7, ucs: '\u{0433}' }, /*                Cyrillic_ghe г CYRILLIC SMALL LETTER GHE */
    Codepair { keysym: 0x06c8, ucs: '\u{0445}' }, /*                 Cyrillic_ha х CYRILLIC SMALL LETTER HA */
    Codepair { keysym: 0x06c9, ucs: '\u{0438}' }, /*                  Cyrillic_i и CYRILLIC SMALL LETTER I */
    Codepair { keysym: 0x06ca, ucs: '\u{0439}' }, /*             Cyrillic_shorti й CYRILLIC SMALL LETTER SHORT I */
    Codepair { keysym: 0x06cb, ucs: '\u{043a}' }, /*                 Cyrillic_ka к CYRILLIC SMALL LETTER KA */
    Codepair { keysym: 0x06cc, ucs: '\u{043b}' }, /*                 Cyrillic_el л CYRILLIC SMALL LETTER EL */
    Codepair { keysym: 0x06cd, ucs: '\u{043c}' }, /*                 Cyrillic_em м CYRILLIC SMALL LETTER EM */
    Codepair { keysym: 0x06ce, ucs: '\u{043d}' }, /*                 Cyrillic_en н CYRILLIC SMALL LETTER EN */
    Codepair { keysym: 0x06cf, ucs: '\u{043e}' }, /*                  Cyrillic_o о CYRILLIC SMALL LETTER O */
    Codepair { keysym: 0x06d0, ucs: '\u{043f}' }, /*                 Cyrillic_pe п CYRILLIC SMALL LETTER PE */
    Codepair { keysym: 0x06d1, ucs: '\u{044f}' }, /*                 Cyrillic_ya я CYRILLIC SMALL LETTER YA */
    Codepair { keysym: 0x06d2, ucs: '\u{0440}' }, /*                 Cyrillic_er р CYRILLIC SMALL LETTER ER */
    Codepair { keysym: 0x06d3, ucs: '\u{0441}' }, /*                 Cyrillic_es с CYRILLIC SMALL LETTER ES */
    Codepair { keysym: 0x06d4, ucs: '\u{0442}' }, /*                 Cyrillic_te т CYRILLIC SMALL LETTER TE */
    Codepair { keysym: 0x06d5, ucs: '\u{0443}' }, /*                  Cyrillic_u у CYRILLIC SMALL LETTER U */
    Codepair { keysym: 0x06d6, ucs: '\u{0436}' }, /*                Cyrillic_zhe ж CYRILLIC SMALL LETTER ZHE */
    Codepair { keysym: 0x06d7, ucs: '\u{0432}' }, /*                 Cyrillic_ve в CYRILLIC SMALL LETTER VE */
    Codepair { keysym: 0x06d8, ucs: '\u{044c}' }, /*           Cyrillic_softsign ь CYRILLIC SMALL LETTER SOFT SIGN */
    Codepair { keysym: 0x06d9, ucs: '\u{044b}' }, /*               Cyrillic_yeru ы CYRILLIC SMALL LETTER YERU */
    Codepair { keysym: 0x06da, ucs: '\u{0437}' }, /*                 Cyrillic_ze з CYRILLIC SMALL LETTER ZE */
    Codepair { keysym: 0x06db, ucs: '\u{0448}' }, /*                Cyrillic_sha ш CYRILLIC SMALL LETTER SHA */
    Codepair { keysym: 0x06dc, ucs: '\u{044d}' }, /*                  Cyrillic_e э CYRILLIC SMALL LETTER E */
    Codepair { keysym: 0x06dd, ucs: '\u{0449}' }, /*              Cyrillic_shcha щ CYRILLIC SMALL LETTER SHCHA */
    Codepair { keysym: 0x06de, ucs: '\u{0447}' }, /*                Cyrillic_che ч CYRILLIC SMALL LETTER CHE */
    Codepair { keysym: 0x06df, ucs: '\u{044a}' }, /*           Cyrillic_hardsign ъ CYRILLIC SMALL LETTER HARD SIGN */
    Codepair { keysym: 0x06e0, ucs: '\u{042e}' }, /*                 Cyrillic_YU Ю CYRILLIC CAPITAL LETTER YU */
    Codepair { keysym: 0x06e1, ucs: '\u{0410}' }, /*                  Cyrillic_A А CYRILLIC CAPITAL LETTER A */
    Codepair { keysym: 0x06e2, ucs: '\u{0411}' }, /*                 Cyrillic_BE Б CYRILLIC CAPITAL LETTER BE */
    Codepair { keysym: 0x06e3, ucs: '\u{0426}' }, /*                Cyrillic_TSE Ц CYRILLIC CAPITAL LETTER TSE */
    Codepair { keysym: 0x06e4, ucs: '\u{0414}' }, /*                 Cyrillic_DE Д CYRILLIC CAPITAL LETTER DE */
    Codepair { keysym: 0x06e5, ucs: '\u{0415}' }, /*                 Cyrillic_IE Е CYRILLIC CAPITAL LETTER IE */
    Codepair { keysym: 0x06e6, ucs: '\u{0424}' }, /*                 Cyrillic_EF Ф CYRILLIC CAPITAL LETTER EF */
    Codepair { keysym: 0x06e7, ucs: '\u{0413}' }, /*                Cyrillic_GHE Г CYRILLIC CAPITAL LETTER GHE */
    Codepair { keysym: 0x06e8, ucs: '\u{0425}' }, /*                 Cyrillic_HA Х CYRILLIC CAPITAL LETTER HA */
    Codepair { keysym: 0x06e9, ucs: '\u{0418}' }, /*                  Cyrillic_I И CYRILLIC CAPITAL LETTER I */
    Codepair { keysym: 0x06ea, ucs: '\u{0419}' }, /*             Cyrillic_SHORTI Й CYRILLIC CAPITAL LETTER SHORT I */
    Codepair { keysym: 0x06eb, ucs: '\u{041a}' }, /*                 Cyrillic_KA К CYRILLIC CAPITAL LETTER KA */
    Codepair { keysym: 0x06ec, ucs: '\u{041b}' }, /*                 Cyrillic_EL Л CYRILLIC CAPITAL LETTER EL */
    Codepair { keysym: 0x06ed, ucs: '\u{041c}' }, /*                 Cyrillic_EM М CYRILLIC CAPITAL LETTER EM */
    Codepair { keysym: 0x06ee, ucs: '\u{041d}' }, /*                 Cyrillic_EN Н CYRILLIC CAPITAL LETTER EN */
    Codepair { keysym: 0x06ef, ucs: '\u{041e}' }, /*                  Cyrillic_O О CYRILLIC CAPITAL LETTER O */
    Codepair { keysym: 0x06f0, ucs: '\u{041f}' }, /*                 Cyrillic_PE П CYRILLIC CAPITAL LETTER PE */
    Codepair { keysym: 0x06f1, ucs: '\u{042f}' }, /*                 Cyrillic_YA Я CYRILLIC CAPITAL LETTER YA */
    Codepair { keysym: 0x06f2, ucs: '\u{0420}' }, /*                 Cyrillic_ER Р CYRILLIC CAPITAL LETTER ER */
    Codepair { keysym: 0x06f3, ucs: '\u{0421}' }, /*                 Cyrillic_ES С CYRILLIC CAPITAL LETTER ES */
    Codepair { keysym: 0x06f4, ucs: '\u{0422}' }, /*                 Cyrillic_TE Т CYRILLIC CAPITAL LETTER TE */
    Codepair { keysym: 0x06f5, ucs: '\u{0423}' }, /*                  Cyrillic_U У CYRILLIC CAPITAL LETTER U */
    Codepair { keysym: 0x06f6, ucs: '\u{0416}' }, /*                Cyrillic_ZHE Ж CYRILLIC CAPITAL LETTER ZHE */
    Codepair { keysym: 0x06f7, ucs: '\u{0412}' }, /*                 Cyrillic_VE В CYRILLIC CAPITAL LETTER VE */
    Codepair { keysym: 0x06f8, ucs: '\u{042c}' }, /*           Cyrillic_SOFTSIGN Ь CYRILLIC CAPITAL LETTER SOFT SIGN */
    Codepair { keysym: 0x06f9, ucs: '\u{042b}' }, /*               Cyrillic_YERU Ы CYRILLIC CAPITAL LETTER YERU */
    Codepair { keysym: 0x06fa, ucs: '\u{0417}' }, /*                 Cyrillic_ZE З CYRILLIC CAPITAL LETTER ZE */
    Codepair { keysym: 0x06fb, ucs: '\u{0428}' }, /*                Cyrillic_SHA Ш CYRILLIC CAPITAL LETTER SHA */
    Codepair { keysym: 0x06fc, ucs: '\u{042d}' }, /*                  Cyrillic_E Э CYRILLIC CAPITAL LETTER E */
    Codepair { keysym: 0x06fd, ucs: '\u{0429}' }, /*              Cyrillic_SHCHA Щ CYRILLIC CAPITAL LETTER SHCHA */
    Codepair { keysym: 0x06fe, ucs: '\u{0427}' }, /*                Cyrillic_CHE Ч CYRILLIC CAPITAL LETTER CHE */
    Codepair { keysym: 0x06ff, ucs: '\u{042a}' }, /*           Cyrillic_HARDSIGN Ъ CYRILLIC CAPITAL LETTER HARD SIGN */
    Codepair { keysym: 0x07a1, ucs: '\u{0386}' }, /*           Greek_ALPHAaccent Ά GREEK CAPITAL LETTER ALPHA WITH TONOS */
    Codepair { keysym: 0x07a2, ucs: '\u{0388}' }, /*         Greek_EPSILONaccent Έ GREEK CAPITAL LETTER EPSILON WITH TONOS */
    Codepair { keysym: 0x07a3, ucs: '\u{0389}' }, /*             Greek_ETAaccent Ή GREEK CAPITAL LETTER ETA WITH TONOS */
    Codepair { keysym: 0x07a4, ucs: '\u{038a}' }, /*            Greek_IOTAaccent Ί GREEK CAPITAL LETTER IOTA WITH TONOS */
    Codepair { keysym: 0x07a5, ucs: '\u{03aa}' }, /*         Greek_IOTAdiaeresis Ϊ GREEK CAPITAL LETTER IOTA WITH DIALYTIKA */
    Codepair { keysym: 0x07a7, ucs: '\u{038c}' }, /*         Greek_OMICRONaccent Ό GREEK CAPITAL LETTER OMICRON WITH TONOS */
    Codepair { keysym: 0x07a8, ucs: '\u{038e}' }, /*         Greek_UPSILONaccent Ύ GREEK CAPITAL LETTER UPSILON WITH TONOS */
    Codepair { keysym: 0x07a9, ucs: '\u{03ab}' }, /*       Greek_UPSILONdieresis Ϋ GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA */
    Codepair { keysym: 0x07ab, ucs: '\u{038f}' }, /*           Greek_OMEGAaccent Ώ GREEK CAPITAL LETTER OMEGA WITH TONOS */
    Codepair { keysym: 0x07ae, ucs: '\u{0385}' }, /*        Greek_accentdieresis ΅ GREEK DIALYTIKA TONOS */
    Codepair { keysym: 0x07af, ucs: '\u{2015}' }, /*              Greek_horizbar ― HORIZONTAL BAR */
    Codepair { keysym: 0x07b1, ucs: '\u{03ac}' }, /*           Greek_alphaaccent ά GREEK SMALL LETTER ALPHA WITH TONOS */
    Codepair { keysym: 0x07b2, ucs: '\u{03ad}' }, /*         Greek_epsilonaccent έ GREEK SMALL LETTER EPSILON WITH TONOS */
    Codepair { keysym: 0x07b3, ucs: '\u{03ae}' }, /*             Greek_etaaccent ή GREEK SMALL LETTER ETA WITH TONOS */
    Codepair { keysym: 0x07b4, ucs: '\u{03af}' }, /*            Greek_iotaaccent ί GREEK SMALL LETTER IOTA WITH TONOS */
    Codepair { keysym: 0x07b5, ucs: '\u{03ca}' }, /*          Greek_iotadieresis ϊ GREEK SMALL LETTER IOTA WITH DIALYTIKA */
    Codepair { keysym: 0x07b6, ucs: '\u{0390}' }, /*    Greek_iotaaccentdieresis ΐ GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS */
    Codepair { keysym: 0x07b7, ucs: '\u{03cc}' }, /*         Greek_omicronaccent ό GREEK SMALL LETTER OMICRON WITH TONOS */
    Codepair { keysym: 0x07b8, ucs: '\u{03cd}' }, /*         Greek_upsilonaccent ύ GREEK SMALL LETTER UPSILON WITH TONOS */
    Codepair { keysym: 0x07b9, ucs: '\u{03cb}' }, /*       Greek_upsilondieresis ϋ GREEK SMALL LETTER UPSILON WITH DIALYTIKA */
    Codepair { keysym: 0x07ba, ucs: '\u{03b0}' }, /* Greek_upsilonaccentdieresis ΰ GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS */
    Codepair { keysym: 0x07bb, ucs: '\u{03ce}' }, /*           Greek_omegaaccent ώ GREEK SMALL LETTER OMEGA WITH TONOS */
    Codepair { keysym: 0x07c1, ucs: '\u{0391}' }, /*                 Greek_ALPHA Α GREEK CAPITAL LETTER ALPHA */
    Codepair { keysym: 0x07c2, ucs: '\u{0392}' }, /*                  Greek_BETA Β GREEK CAPITAL LETTER BETA */
    Codepair { keysym: 0x07c3, ucs: '\u{0393}' }, /*                 Greek_GAMMA Γ GREEK CAPITAL LETTER GAMMA */
    Codepair { keysym: 0x07c4, ucs: '\u{0394}' }, /*                 Greek_DELTA Δ GREEK CAPITAL LETTER DELTA */
    Codepair { keysym: 0x07c5, ucs: '\u{0395}' }, /*               Greek_EPSILON Ε GREEK CAPITAL LETTER EPSILON */
    Codepair { keysym: 0x07c6, ucs: '\u{0396}' }, /*                  Greek_ZETA Ζ GREEK CAPITAL LETTER ZETA */
    Codepair { keysym: 0x07c7, ucs: '\u{0397}' }, /*                   Greek_ETA Η GREEK CAPITAL LETTER ETA */
    Codepair { keysym: 0x07c8, ucs: '\u{0398}' }, /*                 Greek_THETA Θ GREEK CAPITAL LETTER THETA */
    Codepair { keysym: 0x07c9, ucs: '\u{0399}' }, /*                  Greek_IOTA Ι GREEK CAPITAL LETTER IOTA */
    Codepair { keysym: 0x07ca, ucs: '\u{039a}' }, /*                 Greek_KAPPA Κ GREEK CAPITAL LETTER KAPPA */
    Codepair { keysym: 0x07cb, ucs: '\u{039b}' }, /*                Greek_LAMBDA Λ GREEK CAPITAL LETTER LAMDA */
    Codepair { keysym: 0x07cc, ucs: '\u{039c}' }, /*                    Greek_MU Μ GREEK CAPITAL LETTER MU */
    Codepair { keysym: 0x07cd, ucs: '\u{039d}' }, /*                    Greek_NU Ν GREEK CAPITAL LETTER NU */
    Codepair { keysym: 0x07ce, ucs: '\u{039e}' }, /*                    Greek_XI Ξ GREEK CAPITAL LETTER XI */
    Codepair { keysym: 0x07cf, ucs: '\u{039f}' }, /*               Greek_OMICRON Ο GREEK CAPITAL LETTER OMICRON */
    Codepair { keysym: 0x07d0, ucs: '\u{03a0}' }, /*                    Greek_PI Π GREEK CAPITAL LETTER PI */
    Codepair { keysym: 0x07d1, ucs: '\u{03a1}' }, /*                   Greek_RHO Ρ GREEK CAPITAL LETTER RHO */
    Codepair { keysym: 0x07d2, ucs: '\u{03a3}' }, /*                 Greek_SIGMA Σ GREEK CAPITAL LETTER SIGMA */
    Codepair { keysym: 0x07d4, ucs: '\u{03a4}' }, /*                   Greek_TAU Τ GREEK CAPITAL LETTER TAU */
    Codepair { keysym: 0x07d5, ucs: '\u{03a5}' }, /*               Greek_UPSILON Υ GREEK CAPITAL LETTER UPSILON */
    Codepair { keysym: 0x07d6, ucs: '\u{03a6}' }, /*                   Greek_PHI Φ GREEK CAPITAL LETTER PHI */
    Codepair { keysym: 0x07d7, ucs: '\u{03a7}' }, /*                   Greek_CHI Χ GREEK CAPITAL LETTER CHI */
    Codepair { keysym: 0x07d8, ucs: '\u{03a8}' }, /*                   Greek_PSI Ψ GREEK CAPITAL LETTER PSI */
    Codepair { keysym: 0x07d9, ucs: '\u{03a9}' }, /*                 Greek_OMEGA Ω GREEK CAPITAL LETTER OMEGA */
    Codepair { keysym: 0x07e1, ucs: '\u{03b1}' }, /*                 Greek_alpha α GREEK SMALL LETTER ALPHA */
    Codepair { keysym: 0x07e2, ucs: '\u{03b2}' }, /*                  Greek_beta β GREEK SMALL LETTER BETA */
    Codepair { keysym: 0x07e3, ucs: '\u{03b3}' }, /*                 Greek_gamma γ GREEK SMALL LETTER GAMMA */
    Codepair { keysym: 0x07e4, ucs: '\u{03b4}' }, /*                 Greek_delta δ GREEK SMALL LETTER DELTA */
    Codepair { keysym: 0x07e5, ucs: '\u{03b5}' }, /*               Greek_epsilon ε GREEK SMALL LETTER EPSILON */
    Codepair { keysym: 0x07e6, ucs: '\u{03b6}' }, /*                  Greek_zeta ζ GREEK SMALL LETTER ZETA */
    Codepair { keysym: 0x07e7, ucs: '\u{03b7}' }, /*                   Greek_eta η GREEK SMALL LETTER ETA */
    Codepair { keysym: 0x07e8, ucs: '\u{03b8}' }, /*                 Greek_theta θ GREEK SMALL LETTER THETA */
    Codepair { keysym: 0x07e9, ucs: '\u{03b9}' }, /*                  Greek_iota ι GREEK SMALL LETTER IOTA */
    Codepair { keysym: 0x07ea, ucs: '\u{03ba}' }, /*                 Greek_kappa κ GREEK SMALL LETTER KAPPA */
    Codepair { keysym: 0x07eb, ucs: '\u{03bb}' }, /*                Greek_lambda λ GREEK SMALL LETTER LAMDA */
    Codepair { keysym: 0x07ec, ucs: '\u{03bc}' }, /*                    Greek_mu μ GREEK SMALL LETTER MU */
    Codepair { keysym: 0x07ed, ucs: '\u{03bd}' }, /*                    Greek_nu ν GREEK SMALL LETTER NU */
    Codepair { keysym: 0x07ee, ucs: '\u{03be}' }, /*                    Greek_xi ξ GREEK SMALL LETTER XI */
    Codepair { keysym: 0x07ef, ucs: '\u{03bf}' }, /*               Greek_omicron ο GREEK SMALL LETTER OMICRON */
    Codepair { keysym: 0x07f0, ucs: '\u{03c0}' }, /*                    Greek_pi π GREEK SMALL LETTER PI */
    Codepair { keysym: 0x07f1, ucs: '\u{03c1}' }, /*                   Greek_rho ρ GREEK SMALL LETTER RHO */
    Codepair { keysym: 0x07f2, ucs: '\u{03c3}' }, /*                 Greek_sigma σ GREEK SMALL LETTER SIGMA */
    Codepair { keysym: 0x07f3, ucs: '\u{03c2}' }, /*       Greek_finalsmallsigma ς GREEK SMALL LETTER FINAL SIGMA */
    Codepair { keysym: 0x07f4, ucs: '\u{03c4}' }, /*                   Greek_tau τ GREEK SMALL LETTER TAU */
    Codepair { keysym: 0x07f5, ucs: '\u{03c5}' }, /*               Greek_upsilon υ GREEK SMALL LETTER UPSILON */
    Codepair { keysym: 0x07f6, ucs: '\u{03c6}' }, /*                   Greek_phi φ GREEK SMALL LETTER PHI */
    Codepair { keysym: 0x07f7, ucs: '\u{03c7}' }, /*                   Greek_chi χ GREEK SMALL LETTER CHI */
    Codepair { keysym: 0x07f8, ucs: '\u{03c8}' }, /*                   Greek_psi ψ GREEK SMALL LETTER PSI */
    Codepair { keysym: 0x07f9, ucs: '\u{03c9}' }, /*                 Greek_omega ω GREEK SMALL LETTER OMEGA */
    Codepair { keysym: 0x08a1, ucs: '\u{23b7}' }, /*                 leftradical ⎷ ??? */
    Codepair { keysym: 0x08a2, ucs: '\u{250c}' }, /*              topleftradical ┌ BOX DRAWINGS LIGHT DOWN AND RIGHT */
    Codepair { keysym: 0x08a3, ucs: '\u{2500}' }, /*              horizconnector ─ BOX DRAWINGS LIGHT HORIZONTAL */
    Codepair { keysym: 0x08a4, ucs: '\u{2320}' }, /*                 topintegral ⌠ TOP HALF INTEGRAL */
    Codepair { keysym: 0x08a5, ucs: '\u{2321}' }, /*                 botintegral ⌡ BOTTOM HALF INTEGRAL */
    Codepair { keysym: 0x08a6, ucs: '\u{2502}' }, /*               vertconnector │ BOX DRAWINGS LIGHT VERTICAL */
    Codepair { keysym: 0x08a7, ucs: '\u{23a1}' }, /*            topleftsqbracket ⎡ ??? */
    Codepair { keysym: 0x08a8, ucs: '\u{23a3}' }, /*            botleftsqbracket ⎣ ??? */
    Codepair { keysym: 0x08a9, ucs: '\u{23a4}' }, /*           toprightsqbracket ⎤ ??? */
    Codepair { keysym: 0x08aa, ucs: '\u{23a6}' }, /*           botrightsqbracket ⎦ ??? */
    Codepair { keysym: 0x08ab, ucs: '\u{239b}' }, /*               topleftparens ⎛ ??? */
    Codepair { keysym: 0x08ac, ucs: '\u{239d}' }, /*               botleftparens ⎝ ??? */
    Codepair { keysym: 0x08ad, ucs: '\u{239e}' }, /*              toprightparens ⎞ ??? */
    Codepair { keysym: 0x08ae, ucs: '\u{23a0}' }, /*              botrightparens ⎠ ??? */
    Codepair { keysym: 0x08af, ucs: '\u{23a8}' }, /*        leftmiddlecurlybrace ⎨ ??? */
    Codepair { keysym: 0x08b0, ucs: '\u{23ac}' }, /*       rightmiddlecurlybrace ⎬ ??? */
    Codepair { keysym: 0x08bc, ucs: '\u{2264}' }, /*               lessthanequal ≤ LESS-THAN OR EQUAL TO */
    Codepair { keysym: 0x08bd, ucs: '\u{2260}' }, /*                    notequal ≠ NOT EQUAL TO */
    Codepair { keysym: 0x08be, ucs: '\u{2265}' }, /*            greaterthanequal ≥ GREATER-THAN OR EQUAL TO */
    Codepair { keysym: 0x08bf, ucs: '\u{222b}' }, /*                    integral ∫ INTEGRAL */
    Codepair { keysym: 0x08c0, ucs: '\u{2234}' }, /*                   therefore ∴ THEREFORE */
    Codepair { keysym: 0x08c1, ucs: '\u{221d}' }, /*                   variation ∝ PROPORTIONAL TO */
    Codepair { keysym: 0x08c2, ucs: '\u{221e}' }, /*                    infinity ∞ INFINITY */
    Codepair { keysym: 0x08c5, ucs: '\u{2207}' }, /*                       nabla ∇ NABLA */
    Codepair { keysym: 0x08c8, ucs: '\u{223c}' }, /*                 approximate ∼ TILDE OPERATOR */
    Codepair { keysym: 0x08c9, ucs: '\u{2243}' }, /*                similarequal ≃ ASYMPTOTICALLY EQUAL TO */
    Codepair { keysym: 0x08cd, ucs: '\u{21d4}' }, /*                    ifonlyif ⇔ LEFT RIGHT DOUBLE ARROW */
    Codepair { keysym: 0x08ce, ucs: '\u{21d2}' }, /*                     implies ⇒ RIGHTWARDS DOUBLE ARROW */
    Codepair { keysym: 0x08cf, ucs: '\u{2261}' }, /*                   identical ≡ IDENTICAL TO */
    Codepair { keysym: 0x08d6, ucs: '\u{221a}' }, /*                     radical √ SQUARE ROOT */
    Codepair { keysym: 0x08da, ucs: '\u{2282}' }, /*                  includedin ⊂ SUBSET OF */
    Codepair { keysym: 0x08db, ucs: '\u{2283}' }, /*                    includes ⊃ SUPERSET OF */
    Codepair { keysym: 0x08dc, ucs: '\u{2229}' }, /*                intersection ∩ INTERSECTION */
    Codepair { keysym: 0x08dd, ucs: '\u{222a}' }, /*                       union ∪ UNION */
    Codepair { keysym: 0x08de, ucs: '\u{2227}' }, /*                  logicaland ∧ LOGICAL AND */
    Codepair { keysym: 0x08df, ucs: '\u{2228}' }, /*                   logicalor ∨ LOGICAL OR */
    Codepair { keysym: 0x08ef, ucs: '\u{2202}' }, /*           partialderivative ∂ PARTIAL DIFFERENTIAL */
    Codepair { keysym: 0x08f6, ucs: '\u{0192}' }, /*                    function ƒ LATIN SMALL LETTER F WITH HOOK */
    Codepair { keysym: 0x08fb, ucs: '\u{2190}' }, /*                   leftarrow ← LEFTWARDS ARROW */
    Codepair { keysym: 0x08fc, ucs: '\u{2191}' }, /*                     uparrow ↑ UPWARDS ARROW */
    Codepair { keysym: 0x08fd, ucs: '\u{2192}' }, /*                  rightarrow → RIGHTWARDS ARROW */
    Codepair { keysym: 0x08fe, ucs: '\u{2193}' }, /*                   downarrow ↓ DOWNWARDS ARROW */
    Codepair { keysym: 0x09e0, ucs: '\u{25c6}' }, /*                soliddiamond ◆ BLACK DIAMOND */
    Codepair { keysym: 0x09e1, ucs: '\u{2592}' }, /*                checkerboard ▒ MEDIUM SHADE */
    Codepair { keysym: 0x09e2, ucs: '\u{2409}' }, /*                          ht ␉ SYMBOL FOR HORIZONTAL TABULATION */
    Codepair { keysym: 0x09e3, ucs: '\u{240c}' }, /*                          ff ␌ SYMBOL FOR FORM FEED */
    Codepair { keysym: 0x09e4, ucs: '\u{240d}' }, /*                          cr ␍ SYMBOL FOR CARRIAGE RETURN */
    Codepair { keysym: 0x09e5, ucs: '\u{240a}' }, /*                          lf ␊ SYMBOL FOR LINE FEED */
    Codepair { keysym: 0x09e8, ucs: '\u{2424}' }, /*                          nl ␤ SYMBOL FOR NEWLINE */
    Codepair { keysym: 0x09e9, ucs: '\u{240b}' }, /*                          vt ␋ SYMBOL FOR VERTICAL TABULATION */
    Codepair { keysym: 0x09ea, ucs: '\u{2518}' }, /*              lowrightcorner ┘ BOX DRAWINGS LIGHT UP AND LEFT */
    Codepair { keysym: 0x09eb, ucs: '\u{2510}' }, /*               uprightcorner ┐ BOX DRAWINGS LIGHT DOWN AND LEFT */
    Codepair { keysym: 0x09ec, ucs: '\u{250c}' }, /*                upleftcorner ┌ BOX DRAWINGS LIGHT DOWN AND RIGHT */
    Codepair { keysym: 0x09ed, ucs: '\u{2514}' }, /*               lowleftcorner └ BOX DRAWINGS LIGHT UP AND RIGHT */
    Codepair { keysym: 0x09ee, ucs: '\u{253c}' }, /*               crossinglines ┼ BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL */
    Codepair { keysym: 0x09ef, ucs: '\u{23ba}' }, /*              horizlinescan1 ⎺ HORIZONTAL SCAN LINE-1 (Unicode 3.2 draft) */
    Codepair { keysym: 0x09f0, ucs: '\u{23bb}' }, /*              horizlinescan3 ⎻ HORIZONTAL SCAN LINE-3 (Unicode 3.2 draft) */
    Codepair { keysym: 0x09f1, ucs: '\u{2500}' }, /*              horizlinescan5 ─ BOX DRAWINGS LIGHT HORIZONTAL */
    Codepair { keysym: 0x09f2, ucs: '\u{23bc}' }, /*              horizlinescan7 ⎼ HORIZONTAL SCAN LINE-7 (Unicode 3.2 draft) */
    Codepair { keysym: 0x09f3, ucs: '\u{23bd}' }, /*              horizlinescan9 ⎽ HORIZONTAL SCAN LINE-9 (Unicode 3.2 draft) */
    Codepair { keysym: 0x09f4, ucs: '\u{251c}' }, /*                       leftt ├ BOX DRAWINGS LIGHT VERTICAL AND RIGHT */
    Codepair { keysym: 0x09f5, ucs: '\u{2524}' }, /*                      rightt ┤ BOX DRAWINGS LIGHT VERTICAL AND LEFT */
    Codepair { keysym: 0x09f6, ucs: '\u{2534}' }, /*                        bott ┴ BOX DRAWINGS LIGHT UP AND HORIZONTAL */
    Codepair { keysym: 0x09f7, ucs: '\u{252c}' }, /*                        topt ┬ BOX DRAWINGS LIGHT DOWN AND HORIZONTAL */
    Codepair { keysym: 0x09f8, ucs: '\u{2502}' }, /*                     vertbar │ BOX DRAWINGS LIGHT VERTICAL */
    Codepair { keysym: 0x0aa1, ucs: '\u{2003}' }, /*                     emspace   EM SPACE */
    Codepair { keysym: 0x0aa2, ucs: '\u{2002}' }, /*                     enspace   EN SPACE */
    Codepair { keysym: 0x0aa3, ucs: '\u{2004}' }, /*                    em3space   THREE-PER-EM SPACE */
    Codepair { keysym: 0x0aa4, ucs: '\u{2005}' }, /*                    em4space   FOUR-PER-EM SPACE */
    Codepair { keysym: 0x0aa5, ucs: '\u{2007}' }, /*                  digitspace   FIGURE SPACE */
    Codepair { keysym: 0x0aa6, ucs: '\u{2008}' }, /*                  punctspace   PUNCTUATION SPACE */
    Codepair { keysym: 0x0aa7, ucs: '\u{2009}' }, /*                   thinspace   THIN SPACE */
    Codepair { keysym: 0x0aa8, ucs: '\u{200a}' }, /*                   hairspace   HAIR SPACE */
    Codepair { keysym: 0x0aa9, ucs: '\u{2014}' }, /*                      emdash — EM DASH */
    Codepair { keysym: 0x0aaa, ucs: '\u{2013}' }, /*                      endash – EN DASH */
    Codepair { keysym: 0x0aac, ucs: '\u{2423}' }, /*                 signifblank ␣ OPEN BOX */
    Codepair { keysym: 0x0aae, ucs: '\u{2026}' }, /*                    ellipsis … HORIZONTAL ELLIPSIS */
    Codepair { keysym: 0x0aaf, ucs: '\u{2025}' }, /*             doubbaselinedot ‥ TWO DOT LEADER */
    Codepair { keysym: 0x0ab0, ucs: '\u{2153}' }, /*                    onethird ⅓ VULGAR FRACTION ONE THIRD */
    Codepair { keysym: 0x0ab1, ucs: '\u{2154}' }, /*                   twothirds ⅔ VULGAR FRACTION TWO THIRDS */
    Codepair { keysym: 0x0ab2, ucs: '\u{2155}' }, /*                    onefifth ⅕ VULGAR FRACTION ONE FIFTH */
    Codepair { keysym: 0x0ab3, ucs: '\u{2156}' }, /*                   twofifths ⅖ VULGAR FRACTION TWO FIFTHS */
    Codepair { keysym: 0x0ab4, ucs: '\u{2157}' }, /*                 threefifths ⅗ VULGAR FRACTION THREE FIFTHS */
    Codepair { keysym: 0x0ab5, ucs: '\u{2158}' }, /*                  fourfifths ⅘ VULGAR FRACTION FOUR FIFTHS */
    Codepair { keysym: 0x0ab6, ucs: '\u{2159}' }, /*                    onesixth ⅙ VULGAR FRACTION ONE SIXTH */
    Codepair { keysym: 0x0ab7, ucs: '\u{215a}' }, /*                  fivesixths ⅚ VULGAR FRACTION FIVE SIXTHS */
    Codepair { keysym: 0x0ab8, ucs: '\u{2105}' }, /*                      careof ℅ CARE OF */
    Codepair { keysym: 0x0abb, ucs: '\u{2012}' }, /*                     figdash ‒ FIGURE DASH */
    Codepair { keysym: 0x0abc, ucs: '\u{27e8}' }, /*            leftanglebracket ⟨ MATHEMATICAL LEFT ANGLE BRACKET */
    Codepair { keysym: 0x0abd, ucs: '\u{002e}' }, /*                decimalpoint . FULL STOP */
    Codepair { keysym: 0x0abe, ucs: '\u{27e9}' }, /*           rightanglebracket ⟩ MATHEMATICAL RIGHT ANGLE BRACKET */
    Codepair { keysym: 0x0ac3, ucs: '\u{215b}' }, /*                   oneeighth ⅛ VULGAR FRACTION ONE EIGHTH */
    Codepair { keysym: 0x0ac4, ucs: '\u{215c}' }, /*                threeeighths ⅜ VULGAR FRACTION THREE EIGHTHS */
    Codepair { keysym: 0x0ac5, ucs: '\u{215d}' }, /*                 fiveeighths ⅝ VULGAR FRACTION FIVE EIGHTHS */
    Codepair { keysym: 0x0ac6, ucs: '\u{215e}' }, /*                seveneighths ⅞ VULGAR FRACTION SEVEN EIGHTHS */
    Codepair { keysym: 0x0ac9, ucs: '\u{2122}' }, /*                   trademark ™ TRADE MARK SIGN */
    Codepair { keysym: 0x0aca, ucs: '\u{2613}' }, /*               signaturemark ☓ SALTIRE */
    Codepair { keysym: 0x0acc, ucs: '\u{25c1}' }, /*            leftopentriangle ◁ WHITE LEFT-POINTING TRIANGLE */
    Codepair { keysym: 0x0acd, ucs: '\u{25b7}' }, /*           rightopentriangle ▷ WHITE RIGHT-POINTING TRIANGLE */
    Codepair { keysym: 0x0ace, ucs: '\u{25cb}' }, /*                emopencircle ○ WHITE CIRCLE */
    Codepair { keysym: 0x0acf, ucs: '\u{25af}' }, /*             emopenrectangle ▯ WHITE VERTICAL RECTANGLE */
    Codepair { keysym: 0x0ad0, ucs: '\u{2018}' }, /*         leftsinglequotemark ‘ LEFT SINGLE QUOTATION MARK */
    Codepair { keysym: 0x0ad1, ucs: '\u{2019}' }, /*        rightsinglequotemark ’ RIGHT SINGLE QUOTATION MARK */
    Codepair { keysym: 0x0ad2, ucs: '\u{201c}' }, /*         leftdoublequotemark “ LEFT DOUBLE QUOTATION MARK */
    Codepair { keysym: 0x0ad3, ucs: '\u{201d}' }, /*        rightdoublequotemark ” RIGHT DOUBLE QUOTATION MARK */
    Codepair { keysym: 0x0ad4, ucs: '\u{211e}' }, /*                prescription ℞ PRESCRIPTION TAKE */
    Codepair { keysym: 0x0ad5, ucs: '\u{2030}' }, /*                    permille ‰ PER MILLE SIGN */
    Codepair { keysym: 0x0ad6, ucs: '\u{2032}' }, /*                     minutes ′ PRIME */
    Codepair { keysym: 0x0ad7, ucs: '\u{2033}' }, /*                     seconds ″ DOUBLE PRIME */
    Codepair { keysym: 0x0ad9, ucs: '\u{271d}' }, /*                  latincross ✝ LATIN CROSS */
    Codepair { keysym: 0x0adb, ucs: '\u{25ac}' }, /*            filledrectbullet ▬ BLACK RECTANGLE */
    Codepair { keysym: 0x0adc, ucs: '\u{25c0}' }, /*         filledlefttribullet ◀ BLACK LEFT-POINTING TRIANGLE */
    Codepair { keysym: 0x0add, ucs: '\u{25b6}' }, /*        filledrighttribullet ▶ BLACK RIGHT-POINTING TRIANGLE */
    Codepair { keysym: 0x0ade, ucs: '\u{25cf}' }, /*              emfilledcircle ● BLACK CIRCLE */
    Codepair { keysym: 0x0adf, ucs: '\u{25ae}' }, /*                emfilledrect ▮ BLACK VERTICAL RECTANGLE */
    Codepair { keysym: 0x0ae0, ucs: '\u{25e6}' }, /*            enopencircbullet ◦ WHITE BULLET */
    Codepair { keysym: 0x0ae1, ucs: '\u{25ab}' }, /*          enopensquarebullet ▫ WHITE SMALL SQUARE */
    Codepair { keysym: 0x0ae2, ucs: '\u{25ad}' }, /*              openrectbullet ▭ WHITE RECTANGLE */
    Codepair { keysym: 0x0ae3, ucs: '\u{25b3}' }, /*             opentribulletup △ WHITE UP-POINTING TRIANGLE */
    Codepair { keysym: 0x0ae4, ucs: '\u{25bd}' }, /*           opentribulletdown ▽ WHITE DOWN-POINTING TRIANGLE */
    Codepair { keysym: 0x0ae5, ucs: '\u{2606}' }, /*                    openstar ☆ WHITE STAR */
    Codepair { keysym: 0x0ae6, ucs: '\u{2022}' }, /*          enfilledcircbullet • BULLET */
    Codepair { keysym: 0x0ae7, ucs: '\u{25aa}' }, /*            enfilledsqbullet ▪ BLACK SMALL SQUARE */
    Codepair { keysym: 0x0ae8, ucs: '\u{25b2}' }, /*           filledtribulletup ▲ BLACK UP-POINTING TRIANGLE */
    Codepair { keysym: 0x0ae9, ucs: '\u{25bc}' }, /*         filledtribulletdown ▼ BLACK DOWN-POINTING TRIANGLE */
    Codepair { keysym: 0x0aea, ucs: '\u{261c}' }, /*                 leftpointer ☜ WHITE LEFT POINTING INDEX */
    Codepair { keysym: 0x0aeb, ucs: '\u{261e}' }, /*                rightpointer ☞ WHITE RIGHT POINTING INDEX */
    Codepair { keysym: 0x0aec, ucs: '\u{2663}' }, /*                        club ♣ BLACK CLUB SUIT */
    Codepair { keysym: 0x0aed, ucs: '\u{2666}' }, /*                     diamond ♦ BLACK DIAMOND SUIT */
    Codepair { keysym: 0x0aee, ucs: '\u{2665}' }, /*                       heart ♥ BLACK HEART SUIT */
    Codepair { keysym: 0x0af0, ucs: '\u{2720}' }, /*                maltesecross ✠ MALTESE CROSS */
    Codepair { keysym: 0x0af1, ucs: '\u{2020}' }, /*                      dagger † DAGGER */
    Codepair { keysym: 0x0af2, ucs: '\u{2021}' }, /*                doubledagger ‡ DOUBLE DAGGER */
    Codepair { keysym: 0x0af3, ucs: '\u{2713}' }, /*                   checkmark ✓ CHECK MARK */
    Codepair { keysym: 0x0af4, ucs: '\u{2717}' }, /*                 ballotcross ✗ BALLOT X */
    Codepair { keysym: 0x0af5, ucs: '\u{266f}' }, /*                musicalsharp ♯ MUSIC SHARP SIGN */
    Codepair { keysym: 0x0af6, ucs: '\u{266d}' }, /*                 musicalflat ♭ MUSIC FLAT SIGN */
    Codepair { keysym: 0x0af7, ucs: '\u{2642}' }, /*                  malesymbol ♂ MALE SIGN */
    Codepair { keysym: 0x0af8, ucs: '\u{2640}' }, /*                femalesymbol ♀ FEMALE SIGN */
    Codepair { keysym: 0x0af9, ucs: '\u{260e}' }, /*                   telephone ☎ BLACK TELEPHONE */
    Codepair { keysym: 0x0afa, ucs: '\u{2315}' }, /*           telephonerecorder ⌕ TELEPHONE RECORDER */
    Codepair { keysym: 0x0afb, ucs: '\u{2117}' }, /*         phonographcopyright ℗ SOUND RECORDING COPYRIGHT */
    Codepair { keysym: 0x0afc, ucs: '\u{2038}' }, /*                       caret ‸ CARET */
    Codepair { keysym: 0x0afd, ucs: '\u{201a}' }, /*          singlelowquotemark ‚ SINGLE LOW-9 QUOTATION MARK */
    Codepair { keysym: 0x0afe, ucs: '\u{201e}' }, /*          doublelowquotemark „ DOUBLE LOW-9 QUOTATION MARK */
    Codepair { keysym: 0x0ba3, ucs: '\u{003c}' }, /*                   leftcaret < LESS-THAN SIGN */
    Codepair { keysym: 0x0ba6, ucs: '\u{003e}' }, /*                  rightcaret > GREATER-THAN SIGN */
    Codepair { keysym: 0x0ba8, ucs: '\u{2228}' }, /*                   downcaret ∨ LOGICAL OR */
    Codepair { keysym: 0x0ba9, ucs: '\u{2227}' }, /*                     upcaret ∧ LOGICAL AND */
    Codepair { keysym: 0x0bc0, ucs: '\u{00af}' }, /*                     overbar ¯ MACRON */
    Codepair { keysym: 0x0bc2, ucs: '\u{22a4}' }, /*                    downtack ⊤ DOWN TACK */
    Codepair { keysym: 0x0bc3, ucs: '\u{2229}' }, /*                      upshoe ∩ INTERSECTION */
    Codepair { keysym: 0x0bc4, ucs: '\u{230a}' }, /*                   downstile ⌊ LEFT FLOOR */
    Codepair { keysym: 0x0bc6, ucs: '\u{005f}' }, /*                    underbar _ LOW LINE */
    Codepair { keysym: 0x0bca, ucs: '\u{2218}' }, /*                         jot ∘ RING OPERATOR */
    Codepair { keysym: 0x0bcc, ucs: '\u{2395}' }, /*                        quad ⎕ APL FUNCTIONAL SYMBOL QUAD (Unicode 3.0) */
    Codepair { keysym: 0x0bce, ucs: '\u{22a5}' }, /*                      uptack ⊥ UP TACK */
    Codepair { keysym: 0x0bcf, ucs: '\u{25cb}' }, /*                      circle ○ WHITE CIRCLE */
    Codepair { keysym: 0x0bd3, ucs: '\u{2308}' }, /*                     upstile ⌈ LEFT CEILING */
    Codepair { keysym: 0x0bd6, ucs: '\u{222a}' }, /*                    downshoe ∪ UNION */
    Codepair { keysym: 0x0bd8, ucs: '\u{2283}' }, /*                   rightshoe ⊃ SUPERSET OF */
    Codepair { keysym: 0x0bda, ucs: '\u{2282}' }, /*                    leftshoe ⊂ SUBSET OF */
    Codepair { keysym: 0x0bdc, ucs: '\u{22a2}' }, /*                    lefttack ⊢ RIGHT TACK */
    Codepair { keysym: 0x0bfc, ucs: '\u{22a3}' }, /*                   righttack ⊣ LEFT TACK */
    Codepair { keysym: 0x0cdf, ucs: '\u{2017}' }, /*        hebrew_doublelowline ‗ DOUBLE LOW LINE */
    Codepair { keysym: 0x0ce0, ucs: '\u{05d0}' }, /*                hebrew_aleph א HEBREW LETTER ALEF */
    Codepair { keysym: 0x0ce1, ucs: '\u{05d1}' }, /*                  hebrew_bet ב HEBREW LETTER BET */
    Codepair { keysym: 0x0ce2, ucs: '\u{05d2}' }, /*                hebrew_gimel ג HEBREW LETTER GIMEL */
    Codepair { keysym: 0x0ce3, ucs: '\u{05d3}' }, /*                hebrew_dalet ד HEBREW LETTER DALET */
    Codepair { keysym: 0x0ce4, ucs: '\u{05d4}' }, /*                   hebrew_he ה HEBREW LETTER HE */
    Codepair { keysym: 0x0ce5, ucs: '\u{05d5}' }, /*                  hebrew_waw ו HEBREW LETTER VAV */
    Codepair { keysym: 0x0ce6, ucs: '\u{05d6}' }, /*                 hebrew_zain ז HEBREW LETTER ZAYIN */
    Codepair { keysym: 0x0ce7, ucs: '\u{05d7}' }, /*                 hebrew_chet ח HEBREW LETTER HET */
    Codepair { keysym: 0x0ce8, ucs: '\u{05d8}' }, /*                  hebrew_tet ט HEBREW LETTER TET */
    Codepair { keysym: 0x0ce9, ucs: '\u{05d9}' }, /*                  hebrew_yod י HEBREW LETTER YOD */
    Codepair { keysym: 0x0cea, ucs: '\u{05da}' }, /*            hebrew_finalkaph ך HEBREW LETTER FINAL KAF */
    Codepair { keysym: 0x0ceb, ucs: '\u{05db}' }, /*                 hebrew_kaph כ HEBREW LETTER KAF */
    Codepair { keysym: 0x0cec, ucs: '\u{05dc}' }, /*                hebrew_lamed ל HEBREW LETTER LAMED */
    Codepair { keysym: 0x0ced, ucs: '\u{05dd}' }, /*             hebrew_finalmem ם HEBREW LETTER FINAL MEM */
    Codepair { keysym: 0x0cee, ucs: '\u{05de}' }, /*                  hebrew_mem מ HEBREW LETTER MEM */
    Codepair { keysym: 0x0cef, ucs: '\u{05df}' }, /*             hebrew_finalnun ן HEBREW LETTER FINAL NUN */
    Codepair { keysym: 0x0cf0, ucs: '\u{05e0}' }, /*                  hebrew_nun נ HEBREW LETTER NUN */
    Codepair { keysym: 0x0cf1, ucs: '\u{05e1}' }, /*               hebrew_samech ס HEBREW LETTER SAMEKH */
    Codepair { keysym: 0x0cf2, ucs: '\u{05e2}' }, /*                 hebrew_ayin ע HEBREW LETTER AYIN */
    Codepair { keysym: 0x0cf3, ucs: '\u{05e3}' }, /*              hebrew_finalpe ף HEBREW LETTER FINAL PE */
    Codepair { keysym: 0x0cf4, ucs: '\u{05e4}' }, /*                   hebrew_pe פ HEBREW LETTER PE */
    Codepair { keysym: 0x0cf5, ucs: '\u{05e5}' }, /*            hebrew_finalzade ץ HEBREW LETTER FINAL TSADI */
    Codepair { keysym: 0x0cf6, ucs: '\u{05e6}' }, /*                 hebrew_zade צ HEBREW LETTER TSADI */
    Codepair { keysym: 0x0cf7, ucs: '\u{05e7}' }, /*                 hebrew_qoph ק HEBREW LETTER QOF */
    Codepair { keysym: 0x0cf8, ucs: '\u{05e8}' }, /*                 hebrew_resh ר HEBREW LETTER RESH */
    Codepair { keysym: 0x0cf9, ucs: '\u{05e9}' }, /*                 hebrew_shin ש HEBREW LETTER SHIN */
    Codepair { keysym: 0x0cfa, ucs: '\u{05ea}' }, /*                  hebrew_taw ת HEBREW LETTER TAV */
    Codepair { keysym: 0x0da1, ucs: '\u{0e01}' }, /*                  Thai_kokai ก THAI CHARACTER KO KAI */
    Codepair { keysym: 0x0da2, ucs: '\u{0e02}' }, /*                Thai_khokhai ข THAI CHARACTER KHO KHAI */
    Codepair { keysym: 0x0da3, ucs: '\u{0e03}' }, /*               Thai_khokhuat ฃ THAI CHARACTER KHO KHUAT */
    Codepair { keysym: 0x0da4, ucs: '\u{0e04}' }, /*               Thai_khokhwai ค THAI CHARACTER KHO KHWAI */
    Codepair { keysym: 0x0da5, ucs: '\u{0e05}' }, /*                Thai_khokhon ฅ THAI CHARACTER KHO KHON */
    Codepair { keysym: 0x0da6, ucs: '\u{0e06}' }, /*             Thai_khorakhang ฆ THAI CHARACTER KHO RAKHANG */
    Codepair { keysym: 0x0da7, ucs: '\u{0e07}' }, /*                 Thai_ngongu ง THAI CHARACTER NGO NGU */
    Codepair { keysym: 0x0da8, ucs: '\u{0e08}' }, /*                Thai_chochan จ THAI CHARACTER CHO CHAN */
    Codepair { keysym: 0x0da9, ucs: '\u{0e09}' }, /*               Thai_choching ฉ THAI CHARACTER CHO CHING */
    Codepair { keysym: 0x0daa, ucs: '\u{0e0a}' }, /*               Thai_chochang ช THAI CHARACTER CHO CHANG */
    Codepair { keysym: 0x0dab, ucs: '\u{0e0b}' }, /*                   Thai_soso ซ THAI CHARACTER SO SO */
    Codepair { keysym: 0x0dac, ucs: '\u{0e0c}' }, /*                Thai_chochoe ฌ THAI CHARACTER CHO CHOE */
    Codepair { keysym: 0x0dad, ucs: '\u{0e0d}' }, /*                 Thai_yoying ญ THAI CHARACTER YO YING */
    Codepair { keysym: 0x0dae, ucs: '\u{0e0e}' }, /*                Thai_dochada ฎ THAI CHARACTER DO CHADA */
    Codepair { keysym: 0x0daf, ucs: '\u{0e0f}' }, /*                Thai_topatak ฏ THAI CHARACTER TO PATAK */
    Codepair { keysym: 0x0db0, ucs: '\u{0e10}' }, /*                Thai_thothan ฐ THAI CHARACTER THO THAN */
    Codepair { keysym: 0x0db1, ucs: '\u{0e11}' }, /*          Thai_thonangmontho ฑ THAI CHARACTER THO NANGMONTHO */
    Codepair { keysym: 0x0db2, ucs: '\u{0e12}' }, /*             Thai_thophuthao ฒ THAI CHARACTER THO PHUTHAO */
    Codepair { keysym: 0x0db3, ucs: '\u{0e13}' }, /*                  Thai_nonen ณ THAI CHARACTER NO NEN */
    Codepair { keysym: 0x0db4, ucs: '\u{0e14}' }, /*                  Thai_dodek ด THAI CHARACTER DO DEK */
    Codepair { keysym: 0x0db5, ucs: '\u{0e15}' }, /*                  Thai_totao ต THAI CHARACTER TO TAO */
    Codepair { keysym: 0x0db6, ucs: '\u{0e16}' }, /*               Thai_thothung ถ THAI CHARACTER THO THUNG */
    Codepair { keysym: 0x0db7, ucs: '\u{0e17}' }, /*              Thai_thothahan ท THAI CHARACTER THO THAHAN */
    Codepair { keysym: 0x0db8, ucs: '\u{0e18}' }, /*               Thai_thothong ธ THAI CHARACTER THO THONG */
    Codepair { keysym: 0x0db9, ucs: '\u{0e19}' }, /*                   Thai_nonu น THAI CHARACTER NO NU */
    Codepair { keysym: 0x0dba, ucs: '\u{0e1a}' }, /*               Thai_bobaimai บ THAI CHARACTER BO BAIMAI */
    Codepair { keysym: 0x0dbb, ucs: '\u{0e1b}' }, /*                  Thai_popla ป THAI CHARACTER PO PLA */
    Codepair { keysym: 0x0dbc, ucs: '\u{0e1c}' }, /*               Thai_phophung ผ THAI CHARACTER PHO PHUNG */
    Codepair { keysym: 0x0dbd, ucs: '\u{0e1d}' }, /*                   Thai_fofa ฝ THAI CHARACTER FO FA */
    Codepair { keysym: 0x0dbe, ucs: '\u{0e1e}' }, /*                Thai_phophan พ THAI CHARACTER PHO PHAN */
    Codepair { keysym: 0x0dbf, ucs: '\u{0e1f}' }, /*                  Thai_fofan ฟ THAI CHARACTER FO FAN */
    Codepair { keysym: 0x0dc0, ucs: '\u{0e20}' }, /*             Thai_phosamphao ภ THAI CHARACTER PHO SAMPHAO */
    Codepair { keysym: 0x0dc1, ucs: '\u{0e21}' }, /*                   Thai_moma ม THAI CHARACTER MO MA */
    Codepair { keysym: 0x0dc2, ucs: '\u{0e22}' }, /*                  Thai_yoyak ย THAI CHARACTER YO YAK */
    Codepair { keysym: 0x0dc3, ucs: '\u{0e23}' }, /*                  Thai_rorua ร THAI CHARACTER RO RUA */
    Codepair { keysym: 0x0dc4, ucs: '\u{0e24}' }, /*                     Thai_ru ฤ THAI CHARACTER RU */
    Codepair { keysym: 0x0dc5, ucs: '\u{0e25}' }, /*                 Thai_loling ล THAI CHARACTER LO LING */
    Codepair { keysym: 0x0dc6, ucs: '\u{0e26}' }, /*                     Thai_lu ฦ THAI CHARACTER LU */
    Codepair { keysym: 0x0dc7, ucs: '\u{0e27}' }, /*                 Thai_wowaen ว THAI CHARACTER WO WAEN */
    Codepair { keysym: 0x0dc8, ucs: '\u{0e28}' }, /*                 Thai_sosala ศ THAI CHARACTER SO SALA */
    Codepair { keysym: 0x0dc9, ucs: '\u{0e29}' }, /*                 Thai_sorusi ษ THAI CHARACTER SO RUSI */
    Codepair { keysym: 0x0dca, ucs: '\u{0e2a}' }, /*                  Thai_sosua ส THAI CHARACTER SO SUA */
    Codepair { keysym: 0x0dcb, ucs: '\u{0e2b}' }, /*                  Thai_hohip ห THAI CHARACTER HO HIP */
    Codepair { keysym: 0x0dcc, ucs: '\u{0e2c}' }, /*                Thai_lochula ฬ THAI CHARACTER LO CHULA */
    Codepair { keysym: 0x0dcd, ucs: '\u{0e2d}' }, /*                   Thai_oang อ THAI CHARACTER O ANG */
    Codepair { keysym: 0x0dce, ucs: '\u{0e2e}' }, /*               Thai_honokhuk ฮ THAI CHARACTER HO NOKHUK */
    Codepair { keysym: 0x0dcf, ucs: '\u{0e2f}' }, /*              Thai_paiyannoi ฯ THAI CHARACTER PAIYANNOI */
    Codepair { keysym: 0x0dd0, ucs: '\u{0e30}' }, /*                  Thai_saraa ะ THAI CHARACTER SARA A */
    Codepair { keysym: 0x0dd1, ucs: '\u{0e31}' }, /*             Thai_maihanakat ั THAI CHARACTER MAI HAN-AKAT */
    Codepair { keysym: 0x0dd2, ucs: '\u{0e32}' }, /*                 Thai_saraaa า THAI CHARACTER SARA AA */
    Codepair { keysym: 0x0dd3, ucs: '\u{0e33}' }, /*                 Thai_saraam ำ THAI CHARACTER SARA AM */
    Codepair { keysym: 0x0dd4, ucs: '\u{0e34}' }, /*                  Thai_sarai ิ THAI CHARACTER SARA I */
    Codepair { keysym: 0x0dd5, ucs: '\u{0e35}' }, /*                 Thai_saraii ี THAI CHARACTER SARA II */
    Codepair { keysym: 0x0dd6, ucs: '\u{0e36}' }, /*                 Thai_saraue ึ THAI CHARACTER SARA UE */
    Codepair { keysym: 0x0dd7, ucs: '\u{0e37}' }, /*                Thai_sarauee ื THAI CHARACTER SARA UEE */
    Codepair { keysym: 0x0dd8, ucs: '\u{0e38}' }, /*                  Thai_sarau ุ THAI CHARACTER SARA U */
    Codepair { keysym: 0x0dd9, ucs: '\u{0e39}' }, /*                 Thai_sarauu ู THAI CHARACTER SARA UU */
    Codepair { keysym: 0x0dda, ucs: '\u{0e3a}' }, /*                Thai_phinthu ฺ THAI CHARACTER PHINTHU */
    Codepair { keysym: 0x0dde, ucs: '\u{0e3e}' }, /*      Thai_maihanakat_maitho ฾ ??? */
    Codepair { keysym: 0x0ddf, ucs: '\u{0e3f}' }, /*                   Thai_baht ฿ THAI CURRENCY SYMBOL BAHT */
    Codepair { keysym: 0x0de0, ucs: '\u{0e40}' }, /*                  Thai_sarae เ THAI CHARACTER SARA E */
    Codepair { keysym: 0x0de1, ucs: '\u{0e41}' }, /*                 Thai_saraae แ THAI CHARACTER SARA AE */
    Codepair { keysym: 0x0de2, ucs: '\u{0e42}' }, /*                  Thai_sarao โ THAI CHARACTER SARA O */
    Codepair { keysym: 0x0de3, ucs: '\u{0e43}' }, /*          Thai_saraaimaimuan ใ THAI CHARACTER SARA AI MAIMUAN */
    Codepair { keysym: 0x0de4, ucs: '\u{0e44}' }, /*         Thai_saraaimaimalai ไ THAI CHARACTER SARA AI MAIMALAI */
    Codepair { keysym: 0x0de5, ucs: '\u{0e45}' }, /*            Thai_lakkhangyao ๅ THAI CHARACTER LAKKHANGYAO */
    Codepair { keysym: 0x0de6, ucs: '\u{0e46}' }, /*               Thai_maiyamok ๆ THAI CHARACTER MAIYAMOK */
    Codepair { keysym: 0x0de7, ucs: '\u{0e47}' }, /*              Thai_maitaikhu ็ THAI CHARACTER MAITAIKHU */
    Codepair { keysym: 0x0de8, ucs: '\u{0e48}' }, /*                  Thai_maiek ่ THAI CHARACTER MAI EK */
    Codepair { keysym: 0x0de9, ucs: '\u{0e49}' }, /*                 Thai_maitho ้ THAI CHARACTER MAI THO */
    Codepair { keysym: 0x0dea, ucs: '\u{0e4a}' }, /*                 Thai_maitri ๊ THAI CHARACTER MAI TRI */
    Codepair { keysym: 0x0deb, ucs: '\u{0e4b}' }, /*            Thai_maichattawa ๋ THAI CHARACTER MAI CHATTAWA */
    Codepair { keysym: 0x0dec, ucs: '\u{0e4c}' }, /*            Thai_thanthakhat ์ THAI CHARACTER THANTHAKHAT */
    Codepair { keysym: 0x0ded, ucs: '\u{0e4d}' }, /*               Thai_nikhahit ํ THAI CHARACTER NIKHAHIT */
    Codepair { keysym: 0x0df0, ucs: '\u{0e50}' }, /*                 Thai_leksun ๐ THAI DIGIT ZERO */
    Codepair { keysym: 0x0df1, ucs: '\u{0e51}' }, /*                Thai_leknung ๑ THAI DIGIT ONE */
    Codepair { keysym: 0x0df2, ucs: '\u{0e52}' }, /*                Thai_leksong ๒ THAI DIGIT TWO */
    Codepair { keysym: 0x0df3, ucs: '\u{0e53}' }, /*                 Thai_leksam ๓ THAI DIGIT THREE */
    Codepair { keysym: 0x0df4, ucs: '\u{0e54}' }, /*                  Thai_leksi ๔ THAI DIGIT FOUR */
    Codepair { keysym: 0x0df5, ucs: '\u{0e55}' }, /*                  Thai_lekha ๕ THAI DIGIT FIVE */
    Codepair { keysym: 0x0df6, ucs: '\u{0e56}' }, /*                 Thai_lekhok ๖ THAI DIGIT SIX */
    Codepair { keysym: 0x0df7, ucs: '\u{0e57}' }, /*                Thai_lekchet ๗ THAI DIGIT SEVEN */
    Codepair { keysym: 0x0df8, ucs: '\u{0e58}' }, /*                Thai_lekpaet ๘ THAI DIGIT EIGHT */
    Codepair { keysym: 0x0df9, ucs: '\u{0e59}' }, /*                 Thai_lekkao ๙ THAI DIGIT NINE */
    Codepair { keysym: 0x0ea1, ucs: '\u{3131}' }, /*               Hangul_Kiyeog ㄱ HANGUL LETTER KIYEOK */
    Codepair { keysym: 0x0ea2, ucs: '\u{3132}' }, /*          Hangul_SsangKiyeog ㄲ HANGUL LETTER SSANGKIYEOK */
    Codepair { keysym: 0x0ea3, ucs: '\u{3133}' }, /*           Hangul_KiyeogSios ㄳ HANGUL LETTER KIYEOK-SIOS */
    Codepair { keysym: 0x0ea4, ucs: '\u{3134}' }, /*                Hangul_Nieun ㄴ HANGUL LETTER NIEUN */
    Codepair { keysym: 0x0ea5, ucs: '\u{3135}' }, /*           Hangul_NieunJieuj ㄵ HANGUL LETTER NIEUN-CIEUC */
    Codepair { keysym: 0x0ea6, ucs: '\u{3136}' }, /*           Hangul_NieunHieuh ㄶ HANGUL LETTER NIEUN-HIEUH */
    Codepair { keysym: 0x0ea7, ucs: '\u{3137}' }, /*               Hangul_Dikeud ㄷ HANGUL LETTER TIKEUT */
    Codepair { keysym: 0x0ea8, ucs: '\u{3138}' }, /*          Hangul_SsangDikeud ㄸ HANGUL LETTER SSANGTIKEUT */
    Codepair { keysym: 0x0ea9, ucs: '\u{3139}' }, /*                Hangul_Rieul ㄹ HANGUL LETTER RIEUL */
    Codepair { keysym: 0x0eaa, ucs: '\u{313a}' }, /*          Hangul_RieulKiyeog ㄺ HANGUL LETTER RIEUL-KIYEOK */
    Codepair { keysym: 0x0eab, ucs: '\u{313b}' }, /*           Hangul_RieulMieum ㄻ HANGUL LETTER RIEUL-MIEUM */
    Codepair { keysym: 0x0eac, ucs: '\u{313c}' }, /*           Hangul_RieulPieub ㄼ HANGUL LETTER RIEUL-PIEUP */
    Codepair { keysym: 0x0ead, ucs: '\u{313d}' }, /*            Hangul_RieulSios ㄽ HANGUL LETTER RIEUL-SIOS */
    Codepair { keysym: 0x0eae, ucs: '\u{313e}' }, /*           Hangul_RieulTieut ㄾ HANGUL LETTER RIEUL-THIEUTH */
    Codepair { keysym: 0x0eaf, ucs: '\u{313f}' }, /*          Hangul_RieulPhieuf ㄿ HANGUL LETTER RIEUL-PHIEUPH */
    Codepair { keysym: 0x0eb0, ucs: '\u{3140}' }, /*           Hangul_RieulHieuh ㅀ HANGUL LETTER RIEUL-HIEUH */
    Codepair { keysym: 0x0eb1, ucs: '\u{3141}' }, /*                Hangul_Mieum ㅁ HANGUL LETTER MIEUM */
    Codepair { keysym: 0x0eb2, ucs: '\u{3142}' }, /*                Hangul_Pieub ㅂ HANGUL LETTER PIEUP */
    Codepair { keysym: 0x0eb3, ucs: '\u{3143}' }, /*           Hangul_SsangPieub ㅃ HANGUL LETTER SSANGPIEUP */
    Codepair { keysym: 0x0eb4, ucs: '\u{3144}' }, /*            Hangul_PieubSios ㅄ HANGUL LETTER PIEUP-SIOS */
    Codepair { keysym: 0x0eb5, ucs: '\u{3145}' }, /*                 Hangul_Sios ㅅ HANGUL LETTER SIOS */
    Codepair { keysym: 0x0eb6, ucs: '\u{3146}' }, /*            Hangul_SsangSios ㅆ HANGUL LETTER SSANGSIOS */
    Codepair { keysym: 0x0eb7, ucs: '\u{3147}' }, /*                Hangul_Ieung ㅇ HANGUL LETTER IEUNG */
    Codepair { keysym: 0x0eb8, ucs: '\u{3148}' }, /*                Hangul_Jieuj ㅈ HANGUL LETTER CIEUC */
    Codepair { keysym: 0x0eb9, ucs: '\u{3149}' }, /*           Hangul_SsangJieuj ㅉ HANGUL LETTER SSANGCIEUC */
    Codepair { keysym: 0x0eba, ucs: '\u{314a}' }, /*                Hangul_Cieuc ㅊ HANGUL LETTER CHIEUCH */
    Codepair { keysym: 0x0ebb, ucs: '\u{314b}' }, /*               Hangul_Khieuq ㅋ HANGUL LETTER KHIEUKH */
    Codepair { keysym: 0x0ebc, ucs: '\u{314c}' }, /*                Hangul_Tieut ㅌ HANGUL LETTER THIEUTH */
    Codepair { keysym: 0x0ebd, ucs: '\u{314d}' }, /*               Hangul_Phieuf ㅍ HANGUL LETTER PHIEUPH */
    Codepair { keysym: 0x0ebe, ucs: '\u{314e}' }, /*                Hangul_Hieuh ㅎ HANGUL LETTER HIEUH */
    Codepair { keysym: 0x0ebf, ucs: '\u{314f}' }, /*                    Hangul_A ㅏ HANGUL LETTER A */
    Codepair { keysym: 0x0ec0, ucs: '\u{3150}' }, /*                   Hangul_AE ㅐ HANGUL LETTER AE */
    Codepair { keysym: 0x0ec1, ucs: '\u{3151}' }, /*                   Hangul_YA ㅑ HANGUL LETTER YA */
    Codepair { keysym: 0x0ec2, ucs: '\u{3152}' }, /*                  Hangul_YAE ㅒ HANGUL LETTER YAE */
    Codepair { keysym: 0x0ec3, ucs: '\u{3153}' }, /*                   Hangul_EO ㅓ HANGUL LETTER EO */
    Codepair { keysym: 0x0ec4, ucs: '\u{3154}' }, /*                    Hangul_E ㅔ HANGUL LETTER E */
    Codepair { keysym: 0x0ec5, ucs: '\u{3155}' }, /*                  Hangul_YEO ㅕ HANGUL LETTER YEO */
    Codepair { keysym: 0x0ec6, ucs: '\u{3156}' }, /*                   Hangul_YE ㅖ HANGUL LETTER YE */
    Codepair { keysym: 0x0ec7, ucs: '\u{3157}' }, /*                    Hangul_O ㅗ HANGUL LETTER O */
    Codepair { keysym: 0x0ec8, ucs: '\u{3158}' }, /*                   Hangul_WA ㅘ HANGUL LETTER WA */
    Codepair { keysym: 0x0ec9, ucs: '\u{3159}' }, /*                  Hangul_WAE ㅙ HANGUL LETTER WAE */
    Codepair { keysym: 0x0eca, ucs: '\u{315a}' }, /*                   Hangul_OE ㅚ HANGUL LETTER OE */
    Codepair { keysym: 0x0ecb, ucs: '\u{315b}' }, /*                   Hangul_YO ㅛ HANGUL LETTER YO */
    Codepair { keysym: 0x0ecc, ucs: '\u{315c}' }, /*                    Hangul_U ㅜ HANGUL LETTER U */
    Codepair { keysym: 0x0ecd, ucs: '\u{315d}' }, /*                  Hangul_WEO ㅝ HANGUL LETTER WEO */
    Codepair { keysym: 0x0ece, ucs: '\u{315e}' }, /*                   Hangul_WE ㅞ HANGUL LETTER WE */
    Codepair { keysym: 0x0ecf, ucs: '\u{315f}' }, /*                   Hangul_WI ㅟ HANGUL LETTER WI */
    Codepair { keysym: 0x0ed0, ucs: '\u{3160}' }, /*                   Hangul_YU ㅠ HANGUL LETTER YU */
    Codepair { keysym: 0x0ed1, ucs: '\u{3161}' }, /*                   Hangul_EU ㅡ HANGUL LETTER EU */
    Codepair { keysym: 0x0ed2, ucs: '\u{3162}' }, /*                   Hangul_YI ㅢ HANGUL LETTER YI */
    Codepair { keysym: 0x0ed3, ucs: '\u{3163}' }, /*                    Hangul_I ㅣ HANGUL LETTER I */
    Codepair { keysym: 0x0ed4, ucs: '\u{11a8}' }, /*             Hangul_J_Kiyeog ᆨ HANGUL JONGSEONG KIYEOK */
    Codepair { keysym: 0x0ed5, ucs: '\u{11a9}' }, /*        Hangul_J_SsangKiyeog ᆩ HANGUL JONGSEONG SSANGKIYEOK */
    Codepair { keysym: 0x0ed6, ucs: '\u{11aa}' }, /*         Hangul_J_KiyeogSios ᆪ HANGUL JONGSEONG KIYEOK-SIOS */
    Codepair { keysym: 0x0ed7, ucs: '\u{11ab}' }, /*              Hangul_J_Nieun ᆫ HANGUL JONGSEONG NIEUN */
    Codepair { keysym: 0x0ed8, ucs: '\u{11ac}' }, /*         Hangul_J_NieunJieuj ᆬ HANGUL JONGSEONG NIEUN-CIEUC */
    Codepair { keysym: 0x0ed9, ucs: '\u{11ad}' }, /*         Hangul_J_NieunHieuh ᆭ HANGUL JONGSEONG NIEUN-HIEUH */
    Codepair { keysym: 0x0eda, ucs: '\u{11ae}' }, /*             Hangul_J_Dikeud ᆮ HANGUL JONGSEONG TIKEUT */
    Codepair { keysym: 0x0edb, ucs: '\u{11af}' }, /*              Hangul_J_Rieul ᆯ HANGUL JONGSEONG RIEUL */
    Codepair { keysym: 0x0edc, ucs: '\u{11b0}' }, /*        Hangul_J_RieulKiyeog ᆰ HANGUL JONGSEONG RIEUL-KIYEOK */
    Codepair { keysym: 0x0edd, ucs: '\u{11b1}' }, /*         Hangul_J_RieulMieum ᆱ HANGUL JONGSEONG RIEUL-MIEUM */
    Codepair { keysym: 0x0ede, ucs: '\u{11b2}' }, /*         Hangul_J_RieulPieub ᆲ HANGUL JONGSEONG RIEUL-PIEUP */
    Codepair { keysym: 0x0edf, ucs: '\u{11b3}' }, /*          Hangul_J_RieulSios ᆳ HANGUL JONGSEONG RIEUL-SIOS */
    Codepair { keysym: 0x0ee0, ucs: '\u{11b4}' }, /*         Hangul_J_RieulTieut ᆴ HANGUL JONGSEONG RIEUL-THIEUTH */
    Codepair { keysym: 0x0ee1, ucs: '\u{11b5}' }, /*        Hangul_J_RieulPhieuf ᆵ HANGUL JONGSEONG RIEUL-PHIEUPH */
    Codepair { keysym: 0x0ee2, ucs: '\u{11b6}' }, /*         Hangul_J_RieulHieuh ᆶ HANGUL JONGSEONG RIEUL-HIEUH */
    Codepair { keysym: 0x0ee3, ucs: '\u{11b7}' }, /*              Hangul_J_Mieum ᆷ HANGUL JONGSEONG MIEUM */
    Codepair { keysym: 0x0ee4, ucs: '\u{11b8}' }, /*              Hangul_J_Pieub ᆸ HANGUL JONGSEONG PIEUP */
    Codepair { keysym: 0x0ee5, ucs: '\u{11b9}' }, /*          Hangul_J_PieubSios ᆹ HANGUL JONGSEONG PIEUP-SIOS */
    Codepair { keysym: 0x0ee6, ucs: '\u{11ba}' }, /*               Hangul_J_Sios ᆺ HANGUL JONGSEONG SIOS */
    Codepair { keysym: 0x0ee7, ucs: '\u{11bb}' }, /*          Hangul_J_SsangSios ᆻ HANGUL JONGSEONG SSANGSIOS */
    Codepair { keysym: 0x0ee8, ucs: '\u{11bc}' }, /*              Hangul_J_Ieung ᆼ HANGUL JONGSEONG IEUNG */
    Codepair { keysym: 0x0ee9, ucs: '\u{11bd}' }, /*              Hangul_J_Jieuj ᆽ HANGUL JONGSEONG CIEUC */
    Codepair { keysym: 0x0eea, ucs: '\u{11be}' }, /*              Hangul_J_Cieuc ᆾ HANGUL JONGSEONG CHIEUCH */
    Codepair { keysym: 0x0eeb, ucs: '\u{11bf}' }, /*             Hangul_J_Khieuq ᆿ HANGUL JONGSEONG KHIEUKH */
    Codepair { keysym: 0x0eec, ucs: '\u{11c0}' }, /*              Hangul_J_Tieut ᇀ HANGUL JONGSEONG THIEUTH */
    Codepair { keysym: 0x0eed, ucs: '\u{11c1}' }, /*             Hangul_J_Phieuf ᇁ HANGUL JONGSEONG PHIEUPH */
    Codepair { keysym: 0x0eee, ucs: '\u{11c2}' }, /*              Hangul_J_Hieuh ᇂ HANGUL JONGSEONG HIEUH */
    Codepair { keysym: 0x0eef, ucs: '\u{316d}' }, /*     Hangul_RieulYeorinHieuh ㅭ HANGUL LETTER RIEUL-YEORINHIEUH */
    Codepair { keysym: 0x0ef0, ucs: '\u{3171}' }, /*    Hangul_SunkyeongeumMieum ㅱ HANGUL LETTER KAPYEOUNMIEUM */
    Codepair { keysym: 0x0ef1, ucs: '\u{3178}' }, /*    Hangul_SunkyeongeumPieub ㅸ HANGUL LETTER KAPYEOUNPIEUP */
    Codepair { keysym: 0x0ef2, ucs: '\u{317f}' }, /*              Hangul_PanSios ㅿ HANGUL LETTER PANSIOS */
    Codepair { keysym: 0x0ef4, ucs: '\u{3184}' }, /*   Hangul_SunkyeongeumPhieuf ㆄ HANGUL LETTER KAPYEOUNPHIEUPH */
    Codepair { keysym: 0x0ef5, ucs: '\u{3186}' }, /*          Hangul_YeorinHieuh ㆆ HANGUL LETTER YEORINHIEUH */
    Codepair { keysym: 0x0ef6, ucs: '\u{318d}' }, /*                Hangul_AraeA ㆍ HANGUL LETTER ARAEA */
    Codepair { keysym: 0x0ef7, ucs: '\u{318e}' }, /*               Hangul_AraeAE ㆎ HANGUL LETTER ARAEAE */
    Codepair { keysym: 0x0ef8, ucs: '\u{11eb}' }, /*            Hangul_J_PanSios ᇫ HANGUL JONGSEONG PANSIOS */
    Codepair { keysym: 0x0ef9, ucs: '\u{11f0}' }, /*  Hangul_J_KkogjiDalrinIeung ᇰ HANGUL JONGSEONG YESIEUNG */
    Codepair { keysym: 0x0efa, ucs: '\u{11f9}' }, /*        Hangul_J_YeorinHieuh ᇹ HANGUL JONGSEONG YEORINHIEUH */
    Codepair { keysym: 0x0eff, ucs: '\u{20a9}' }, /*                  Korean_Won ₩ WON SIGN */
    Codepair { keysym: 0x13a4, ucs: '\u{20ac}' }, /*                        Euro € EURO SIGN */
    Codepair { keysym: 0x13bc, ucs: '\u{0152}' }, /*                          OE Œ LATIN CAPITAL LIGATURE OE */
    Codepair { keysym: 0x13bd, ucs: '\u{0153}' }, /*                          oe œ LATIN SMALL LIGATURE OE */
    Codepair { keysym: 0x13be, ucs: '\u{0178}' }, /*                  Ydiaeresis Ÿ LATIN CAPITAL LETTER Y WITH DIAERESIS */
    Codepair { keysym: 0x20a0, ucs: '\u{20a0}' }, /*                     EcuSign ₠ EURO-CURRENCY SIGN */
    Codepair { keysym: 0x20a1, ucs: '\u{20a1}' }, /*                   ColonSign ₡ COLON SIGN */
    Codepair { keysym: 0x20a2, ucs: '\u{20a2}' }, /*                CruzeiroSign ₢ CRUZEIRO SIGN */
    Codepair { keysym: 0x20a3, ucs: '\u{20a3}' }, /*                  FFrancSign ₣ FRENCH FRANC SIGN */
    Codepair { keysym: 0x20a4, ucs: '\u{20a4}' }, /*                    LiraSign ₤ LIRA SIGN */
    Codepair { keysym: 0x20a5, ucs: '\u{20a5}' }, /*                    MillSign ₥ MILL SIGN */
    Codepair { keysym: 0x20a6, ucs: '\u{20a6}' }, /*                   NairaSign ₦ NAIRA SIGN */
    Codepair { keysym: 0x20a7, ucs: '\u{20a7}' }, /*                  PesetaSign ₧ PESETA SIGN */
    Codepair { keysym: 0x20a8, ucs: '\u{20a8}' }, /*                   RupeeSign ₨ RUPEE SIGN */
    Codepair { keysym: 0x20a9, ucs: '\u{20a9}' }, /*                     WonSign ₩ WON SIGN */
    Codepair { keysym: 0x20aa, ucs: '\u{20aa}' }, /*               NewSheqelSign ₪ NEW SHEQEL SIGN */
    Codepair { keysym: 0x20ab, ucs: '\u{20ab}' }, /*                    DongSign ₫ DONG SIGN */
    Codepair { keysym: 0x20ac, ucs: '\u{20ac}' }, /*                    EuroSign € EURO SIGN */
];

pub static CHAR_TO_KEYSYM: [Codepair; 775] = [
    Codepair { ucs: '\u{002e}', keysym: 0x0abd }, /*                decimalpoint . FULL STOP */
    Codepair { ucs: '\u{003c}', keysym: 0x0ba3 }, /*                   leftcaret < LESS-THAN SIGN */
    Codepair { ucs: '\u{003e}', keysym: 0x0ba6 }, /*                  rightcaret > GREATER-THAN SIGN */
    Codepair { ucs: '\u{005f}', keysym: 0x0bc6 }, /*                    underbar _ LOW LINE */
    Codepair { ucs: '\u{00af}', keysym: 0x0bc0 }, /*                     overbar ¯ MACRON */
    Codepair { ucs: '\u{0100}', keysym: 0x03c0 }, /*                     Amacron Ā LATIN CAPITAL LETTER A WITH MACRON */
    Codepair { ucs: '\u{0101}', keysym: 0x03e0 }, /*                     amacron ā LATIN SMALL LETTER A WITH MACRON */
    Codepair { ucs: '\u{0102}', keysym: 0x01c3 }, /*                      Abreve Ă LATIN CAPITAL LETTER A WITH BREVE */
    Codepair { ucs: '\u{0103}', keysym: 0x01e3 }, /*                      abreve ă LATIN SMALL LETTER A WITH BREVE */
    Codepair { ucs: '\u{0104}', keysym: 0x01a1 }, /*                     Aogonek Ą LATIN CAPITAL LETTER A WITH OGONEK */
    Codepair { ucs: '\u{0105}', keysym: 0x01b1 }, /*                     aogonek ą LATIN SMALL LETTER A WITH OGONEK */
    Codepair { ucs: '\u{0106}', keysym: 0x01c6 }, /*                      Cacute Ć LATIN CAPITAL LETTER C WITH ACUTE */
    Codepair { ucs: '\u{0107}', keysym: 0x01e6 }, /*                      cacute ć LATIN SMALL LETTER C WITH ACUTE */
    Codepair { ucs: '\u{0108}', keysym: 0x02c6 }, /*                 Ccircumflex Ĉ LATIN CAPITAL LETTER C WITH CIRCUMFLEX */
    Codepair { ucs: '\u{0109}', keysym: 0x02e6 }, /*                 ccircumflex ĉ LATIN SMALL LETTER C WITH CIRCUMFLEX */
    Codepair { ucs: '\u{010a}', keysym: 0x02c5 }, /*                   Cabovedot Ċ LATIN CAPITAL LETTER C WITH DOT ABOVE */
    Codepair { ucs: '\u{010b}', keysym: 0x02e5 }, /*                   cabovedot ċ LATIN SMALL LETTER C WITH DOT ABOVE */
    Codepair { ucs: '\u{010c}', keysym: 0x01c8 }, /*                      Ccaron Č LATIN CAPITAL LETTER C WITH CARON */
    Codepair { ucs: '\u{010d}', keysym: 0x01e8 }, /*                      ccaron č LATIN SMALL LETTER C WITH CARON */
    Codepair { ucs: '\u{010e}', keysym: 0x01cf }, /*                      Dcaron Ď LATIN CAPITAL LETTER D WITH CARON */
    Codepair { ucs: '\u{010f}', keysym: 0x01ef }, /*                      dcaron ď LATIN SMALL LETTER D WITH CARON */
    Codepair { ucs: '\u{0110}', keysym: 0x01d0 }, /*                     Dstroke Đ LATIN CAPITAL LETTER D WITH STROKE */
    Codepair { ucs: '\u{0111}', keysym: 0x01f0 }, /*                     dstroke đ LATIN SMALL LETTER D WITH STROKE */
    Codepair { ucs: '\u{0112}', keysym: 0x03aa }, /*                     Emacron Ē LATIN CAPITAL LETTER E WITH MACRON */
    Codepair { ucs: '\u{0113}', keysym: 0x03ba }, /*                     emacron ē LATIN SMALL LETTER E WITH MACRON */
    Codepair { ucs: '\u{0116}', keysym: 0x03cc }, /*                   Eabovedot Ė LATIN CAPITAL LETTER E WITH DOT ABOVE */
    Codepair { ucs: '\u{0117}', keysym: 0x03ec }, /*                   eabovedot ė LATIN SMALL LETTER E WITH DOT ABOVE */
    Codepair { ucs: '\u{0118}', keysym: 0x01ca }, /*                     Eogonek Ę LATIN CAPITAL LETTER E WITH OGONEK */
    Codepair { ucs: '\u{0119}', keysym: 0x01ea }, /*                     eogonek ę LATIN SMALL LETTER E WITH OGONEK */
    Codepair { ucs: '\u{011a}', keysym: 0x01cc }, /*                      Ecaron Ě LATIN CAPITAL LETTER E WITH CARON */
    Codepair { ucs: '\u{011b}', keysym: 0x01ec }, /*                      ecaron ě LATIN SMALL LETTER E WITH CARON */
    Codepair { ucs: '\u{011c}', keysym: 0x02d8 }, /*                 Gcircumflex Ĝ LATIN CAPITAL LETTER G WITH CIRCUMFLEX */
    Codepair { ucs: '\u{011d}', keysym: 0x02f8 }, /*                 gcircumflex ĝ LATIN SMALL LETTER G WITH CIRCUMFLEX */
    Codepair { ucs: '\u{011e}', keysym: 0x02ab }, /*                      Gbreve Ğ LATIN CAPITAL LETTER G WITH BREVE */
    Codepair { ucs: '\u{011f}', keysym: 0x02bb }, /*                      gbreve ğ LATIN SMALL LETTER G WITH BREVE */
    Codepair { ucs: '\u{0120}', keysym: 0x02d5 }, /*                   Gabovedot Ġ LATIN CAPITAL LETTER G WITH DOT ABOVE */
    Codepair { ucs: '\u{0121}', keysym: 0x02f5 }, /*                   gabovedot ġ LATIN SMALL LETTER G WITH DOT ABOVE */
    Codepair { ucs: '\u{0122}', keysym: 0x03ab }, /*                    Gcedilla Ģ LATIN CAPITAL LETTER G WITH CEDILLA */
    Codepair { ucs: '\u{0123}', keysym: 0x03bb }, /*                    gcedilla ģ LATIN SMALL LETTER G WITH CEDILLA */
    Codepair { ucs: '\u{0124}', keysym: 0x02a6 }, /*                 Hcircumflex Ĥ LATIN CAPITAL LETTER H WITH CIRCUMFLEX */
    Codepair { ucs: '\u{0125}', keysym: 0x02b6 }, /*                 hcircumflex ĥ LATIN SMALL LETTER H WITH CIRCUMFLEX */
    Codepair { ucs: '\u{0126}', keysym: 0x02a1 }, /*                     Hstroke Ħ LATIN CAPITAL LETTER H WITH STROKE */
    Codepair { ucs: '\u{0127}', keysym: 0x02b1 }, /*                     hstroke ħ LATIN SMALL LETTER H WITH STROKE */
    Codepair { ucs: '\u{0128}', keysym: 0x03a5 }, /*                      Itilde Ĩ LATIN CAPITAL LETTER I WITH TILDE */
    Codepair { ucs: '\u{0129}', keysym: 0x03b5 }, /*                      itilde ĩ LATIN SMALL LETTER I WITH TILDE */
    Codepair { ucs: '\u{012a}', keysym: 0x03cf }, /*                     Imacron Ī LATIN CAPITAL LETTER I WITH MACRON */
    Codepair { ucs: '\u{012b}', keysym: 0x03ef }, /*                     imacron ī LATIN SMALL LETTER I WITH MACRON */
    Codepair { ucs: '\u{012e}', keysym: 0x03c7 }, /*                     Iogonek Į LATIN CAPITAL LETTER I WITH OGONEK */
    Codepair { ucs: '\u{012f}', keysym: 0x03e7 }, /*                     iogonek į LATIN SMALL LETTER I WITH OGONEK */
    Codepair { ucs: '\u{0130}', keysym: 0x02a9 }, /*                   Iabovedot İ LATIN CAPITAL LETTER I WITH DOT ABOVE */
    Codepair { ucs: '\u{0131}', keysym: 0x02b9 }, /*                    idotless ı LATIN SMALL LETTER DOTLESS I */
    Codepair { ucs: '\u{0134}', keysym: 0x02ac }, /*                 Jcircumflex Ĵ LATIN CAPITAL LETTER J WITH CIRCUMFLEX */
    Codepair { ucs: '\u{0135}', keysym: 0x02bc }, /*                 jcircumflex ĵ LATIN SMALL LETTER J WITH CIRCUMFLEX */
    Codepair { ucs: '\u{0136}', keysym: 0x03d3 }, /*                    Kcedilla Ķ LATIN CAPITAL LETTER K WITH CEDILLA */
    Codepair { ucs: '\u{0137}', keysym: 0x03f3 }, /*                    kcedilla ķ LATIN SMALL LETTER K WITH CEDILLA */
    Codepair { ucs: '\u{0138}', keysym: 0x03a2 }, /*                         kra ĸ LATIN SMALL LETTER KRA */
    Codepair { ucs: '\u{0139}', keysym: 0x01c5 }, /*                      Lacute Ĺ LATIN CAPITAL LETTER L WITH ACUTE */
    Codepair { ucs: '\u{013a}', keysym: 0x01e5 }, /*                      lacute ĺ LATIN SMALL LETTER L WITH ACUTE */
    Codepair { ucs: '\u{013b}', keysym: 0x03a6 }, /*                    Lcedilla Ļ LATIN CAPITAL LETTER L WITH CEDILLA */
    Codepair { ucs: '\u{013c}', keysym: 0x03b6 }, /*                    lcedilla ļ LATIN SMALL LETTER L WITH CEDILLA */
    Codepair { ucs: '\u{013d}', keysym: 0x01a5 }, /*                      Lcaron Ľ LATIN CAPITAL LETTER L WITH CARON */
    Codepair { ucs: '\u{013e}', keysym: 0x01b5 }, /*                      lcaron ľ LATIN SMALL LETTER L WITH CARON */
    Codepair { ucs: '\u{0141}', keysym: 0x01a3 }, /*                     Lstroke Ł LATIN CAPITAL LETTER L WITH STROKE */
    Codepair { ucs: '\u{0142}', keysym: 0x01b3 }, /*                     lstroke ł LATIN SMALL LETTER L WITH STROKE */
    Codepair { ucs: '\u{0143}', keysym: 0x01d1 }, /*                      Nacute Ń LATIN CAPITAL LETTER N WITH ACUTE */
    Codepair { ucs: '\u{0144}', keysym: 0x01f1 }, /*                      nacute ń LATIN SMALL LETTER N WITH ACUTE */
    Codepair { ucs: '\u{0145}', keysym: 0x03d1 }, /*                    Ncedilla Ņ LATIN CAPITAL LETTER N WITH CEDILLA */
    Codepair { ucs: '\u{0146}', keysym: 0x03f1 }, /*                    ncedilla ņ LATIN SMALL LETTER N WITH CEDILLA */
    Codepair { ucs: '\u{0147}', keysym: 0x01d2 }, /*                      Ncaron Ň LATIN CAPITAL LETTER N WITH CARON */
    Codepair { ucs: '\u{0148}', keysym: 0x01f2 }, /*                      ncaron ň LATIN SMALL LETTER N WITH CARON */
    Codepair { ucs: '\u{014a}', keysym: 0x03bd }, /*                         ENG Ŋ LATIN CAPITAL LETTER ENG */
    Codepair { ucs: '\u{014b}', keysym: 0x03bf }, /*                         eng ŋ LATIN SMALL LETTER ENG */
    Codepair { ucs: '\u{014c}', keysym: 0x03d2 }, /*                     Omacron Ō LATIN CAPITAL LETTER O WITH MACRON */
    Codepair { ucs: '\u{014d}', keysym: 0x03f2 }, /*                     omacron ō LATIN SMALL LETTER O WITH MACRON */
    Codepair { ucs: '\u{0150}', keysym: 0x01d5 }, /*                Odoubleacute Ő LATIN CAPITAL LETTER O WITH DOUBLE ACUTE */
    Codepair { ucs: '\u{0151}', keysym: 0x01f5 }, /*                odoubleacute ő LATIN SMALL LETTER O WITH DOUBLE ACUTE */
    Codepair { ucs: '\u{0152}', keysym: 0x13bc }, /*                          OE Œ LATIN CAPITAL LIGATURE OE */
    Codepair { ucs: '\u{0153}', keysym: 0x13bd }, /*                          oe œ LATIN SMALL LIGATURE OE */
    Codepair { ucs: '\u{0154}', keysym: 0x01c0 }, /*                      Racute Ŕ LATIN CAPITAL LETTER R WITH ACUTE */
    Codepair { ucs: '\u{0155}', keysym: 0x01e0 }, /*                      racute ŕ LATIN SMALL LETTER R WITH ACUTE */
    Codepair { ucs: '\u{0156}', keysym: 0x03a3 }, /*                    Rcedilla Ŗ LATIN CAPITAL LETTER R WITH CEDILLA */
    Codepair { ucs: '\u{0157}', keysym: 0x03b3 }, /*                    rcedilla ŗ LATIN SMALL LETTER R WITH CEDILLA */
    Codepair { ucs: '\u{0158}', keysym: 0x01d8 }, /*                      Rcaron Ř LATIN CAPITAL LETTER R WITH CARON */
    Codepair { ucs: '\u{0159}', keysym: 0x01f8 }, /*                      rcaron ř LATIN SMALL LETTER R WITH CARON */
    Codepair { ucs: '\u{015a}', keysym: 0x01a6 }, /*                      Sacute Ś LATIN CAPITAL LETTER S WITH ACUTE */
    Codepair { ucs: '\u{015b}', keysym: 0x01b6 }, /*                      sacute ś LATIN SMALL LETTER S WITH ACUTE */
    Codepair { ucs: '\u{015c}', keysym: 0x02de }, /*                 Scircumflex Ŝ LATIN CAPITAL LETTER S WITH CIRCUMFLEX */
    Codepair { ucs: '\u{015d}', keysym: 0x02fe }, /*                 scircumflex ŝ LATIN SMALL LETTER S WITH CIRCUMFLEX */
    Codepair { ucs: '\u{015e}', keysym: 0x01aa }, /*                    Scedilla Ş LATIN CAPITAL LETTER S WITH CEDILLA */
    Codepair { ucs: '\u{015f}', keysym: 0x01ba }, /*                    scedilla ş LATIN SMALL LETTER S WITH CEDILLA */
    Codepair { ucs: '\u{0160}', keysym: 0x01a9 }, /*                      Scaron Š LATIN CAPITAL LETTER S WITH CARON */
    Codepair { ucs: '\u{0161}', keysym: 0x01b9 }, /*                      scaron š LATIN SMALL LETTER S WITH CARON */
    Codepair { ucs: '\u{0162}', keysym: 0x01de }, /*                    Tcedilla Ţ LATIN CAPITAL LETTER T WITH CEDILLA */
    Codepair { ucs: '\u{0163}', keysym: 0x01fe }, /*                    tcedilla ţ LATIN SMALL LETTER T WITH CEDILLA */
    Codepair { ucs: '\u{0164}', keysym: 0x01ab }, /*                      Tcaron Ť LATIN CAPITAL LETTER T WITH CARON */
    Codepair { ucs: '\u{0165}', keysym: 0x01bb }, /*                      tcaron ť LATIN SMALL LETTER T WITH CARON */
    Codepair { ucs: '\u{0166}', keysym: 0x03ac }, /*                      Tslash Ŧ LATIN CAPITAL LETTER T WITH STROKE */
    Codepair { ucs: '\u{0167}', keysym: 0x03bc }, /*                      tslash ŧ LATIN SMALL LETTER T WITH STROKE */
    Codepair { ucs: '\u{0168}', keysym: 0x03dd }, /*                      Utilde Ũ LATIN CAPITAL LETTER U WITH TILDE */
    Codepair { ucs: '\u{0169}', keysym: 0x03fd }, /*                      utilde ũ LATIN SMALL LETTER U WITH TILDE */
    Codepair { ucs: '\u{016a}', keysym: 0x03de }, /*                     Umacron Ū LATIN CAPITAL LETTER U WITH MACRON */
    Codepair { ucs: '\u{016b}', keysym: 0x03fe }, /*                     umacron ū LATIN SMALL LETTER U WITH MACRON */
    Codepair { ucs: '\u{016c}', keysym: 0x02dd }, /*                      Ubreve Ŭ LATIN CAPITAL LETTER U WITH BREVE */
    Codepair { ucs: '\u{016d}', keysym: 0x02fd }, /*                      ubreve ŭ LATIN SMALL LETTER U WITH BREVE */
    Codepair { ucs: '\u{016e}', keysym: 0x01d9 }, /*                       Uring Ů LATIN CAPITAL LETTER U WITH RING ABOVE */
    Codepair { ucs: '\u{016f}', keysym: 0x01f9 }, /*                       uring ů LATIN SMALL LETTER U WITH RING ABOVE */
    Codepair { ucs: '\u{0170}', keysym: 0x01db }, /*                Udoubleacute Ű LATIN CAPITAL LETTER U WITH DOUBLE ACUTE */
    Codepair { ucs: '\u{0171}', keysym: 0x01fb }, /*                udoubleacute ű LATIN SMALL LETTER U WITH DOUBLE ACUTE */
    Codepair { ucs: '\u{0172}', keysym: 0x03d9 }, /*                     Uogonek Ų LATIN CAPITAL LETTER U WITH OGONEK */
    Codepair { ucs: '\u{0173}', keysym: 0x03f9 }, /*                     uogonek ų LATIN SMALL LETTER U WITH OGONEK */
    Codepair { ucs: '\u{0178}', keysym: 0x13be }, /*                  Ydiaeresis Ÿ LATIN CAPITAL LETTER Y WITH DIAERESIS */
    Codepair { ucs: '\u{0179}', keysym: 0x01ac }, /*                      Zacute Ź LATIN CAPITAL LETTER Z WITH ACUTE */
    Codepair { ucs: '\u{017a}', keysym: 0x01bc }, /*                      zacute ź LATIN SMALL LETTER Z WITH ACUTE */
    Codepair { ucs: '\u{017b}', keysym: 0x01af }, /*                   Zabovedot Ż LATIN CAPITAL LETTER Z WITH DOT ABOVE */
    Codepair { ucs: '\u{017c}', keysym: 0x01bf }, /*                   zabovedot ż LATIN SMALL LETTER Z WITH DOT ABOVE */
    Codepair { ucs: '\u{017d}', keysym: 0x01ae }, /*                      Zcaron Ž LATIN CAPITAL LETTER Z WITH CARON */
    Codepair { ucs: '\u{017e}', keysym: 0x01be }, /*                      zcaron ž LATIN SMALL LETTER Z WITH CARON */
    Codepair { ucs: '\u{0192}', keysym: 0x08f6 }, /*                    function ƒ LATIN SMALL LETTER F WITH HOOK */
    Codepair { ucs: '\u{02c7}', keysym: 0x01b7 }, /*                       caron ˇ CARON */
    Codepair { ucs: '\u{02d8}', keysym: 0x01a2 }, /*                       breve ˘ BREVE */
    Codepair { ucs: '\u{02d9}', keysym: 0x01ff }, /*                    abovedot ˙ DOT ABOVE */
    Codepair { ucs: '\u{02db}', keysym: 0x01b2 }, /*                      ogonek ˛ OGONEK */
    Codepair { ucs: '\u{02dd}', keysym: 0x01bd }, /*                 doubleacute ˝ DOUBLE ACUTE ACCENT */
    Codepair { ucs: '\u{0385}', keysym: 0x07ae }, /*        Greek_accentdieresis ΅ GREEK DIALYTIKA TONOS */
    Codepair { ucs: '\u{0386}', keysym: 0x07a1 }, /*           Greek_ALPHAaccent Ά GREEK CAPITAL LETTER ALPHA WITH TONOS */
    Codepair { ucs: '\u{0388}', keysym: 0x07a2 }, /*         Greek_EPSILONaccent Έ GREEK CAPITAL LETTER EPSILON WITH TONOS */
    Codepair { ucs: '\u{0389}', keysym: 0x07a3 }, /*             Greek_ETAaccent Ή GREEK CAPITAL LETTER ETA WITH TONOS */
    Codepair { ucs: '\u{038a}', keysym: 0x07a4 }, /*            Greek_IOTAaccent Ί GREEK CAPITAL LETTER IOTA WITH TONOS */
    Codepair { ucs: '\u{038c}', keysym: 0x07a7 }, /*         Greek_OMICRONaccent Ό GREEK CAPITAL LETTER OMICRON WITH TONOS */
    Codepair { ucs: '\u{038e}', keysym: 0x07a8 }, /*         Greek_UPSILONaccent Ύ GREEK CAPITAL LETTER UPSILON WITH TONOS */
    Codepair { ucs: '\u{038f}', keysym: 0x07ab }, /*           Greek_OMEGAaccent Ώ GREEK CAPITAL LETTER OMEGA WITH TONOS */
    Codepair { ucs: '\u{0390}', keysym: 0x07b6 }, /*    Greek_iotaaccentdieresis ΐ GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS */
    Codepair { ucs: '\u{0391}', keysym: 0x07c1 }, /*                 Greek_ALPHA Α GREEK CAPITAL LETTER ALPHA */
    Codepair { ucs: '\u{0392}', keysym: 0x07c2 }, /*                  Greek_BETA Β GREEK CAPITAL LETTER BETA */
    Codepair { ucs: '\u{0393}', keysym: 0x07c3 }, /*                 Greek_GAMMA Γ GREEK CAPITAL LETTER GAMMA */
    Codepair { ucs: '\u{0394}', keysym: 0x07c4 }, /*                 Greek_DELTA Δ GREEK CAPITAL LETTER DELTA */
    Codepair { ucs: '\u{0395}', keysym: 0x07c5 }, /*               Greek_EPSILON Ε GREEK CAPITAL LETTER EPSILON */
    Codepair { ucs: '\u{0396}', keysym: 0x07c6 }, /*                  Greek_ZETA Ζ GREEK CAPITAL LETTER ZETA */
    Codepair { ucs: '\u{0397}', keysym: 0x07c7 }, /*                   Greek_ETA Η GREEK CAPITAL LETTER ETA */
    Codepair { ucs: '\u{0398}', keysym: 0x07c8 }, /*                 Greek_THETA Θ GREEK CAPITAL LETTER THETA */
    Codepair { ucs: '\u{0399}', keysym: 0x07c9 }, /*                  Greek_IOTA Ι GREEK CAPITAL LETTER IOTA */
    Codepair { ucs: '\u{039a}', keysym: 0x07ca }, /*                 Greek_KAPPA Κ GREEK CAPITAL LETTER KAPPA */
    Codepair { ucs: '\u{039b}', keysym: 0x07cb }, /*                Greek_LAMBDA Λ GREEK CAPITAL LETTER LAMDA */
    Codepair { ucs: '\u{039c}', keysym: 0x07cc }, /*                    Greek_MU Μ GREEK CAPITAL LETTER MU */
    Codepair { ucs: '\u{039d}', keysym: 0x07cd }, /*                    Greek_NU Ν GREEK CAPITAL LETTER NU */
    Codepair { ucs: '\u{039e}', keysym: 0x07ce }, /*                    Greek_XI Ξ GREEK CAPITAL LETTER XI */
    Codepair { ucs: '\u{039f}', keysym: 0x07cf }, /*               Greek_OMICRON Ο GREEK CAPITAL LETTER OMICRON */
    Codepair { ucs: '\u{03a0}', keysym: 0x07d0 }, /*                    Greek_PI Π GREEK CAPITAL LETTER PI */
    Codepair { ucs: '\u{03a1}', keysym: 0x07d1 }, /*                   Greek_RHO Ρ GREEK CAPITAL LETTER RHO */
    Codepair { ucs: '\u{03a3}', keysym: 0x07d2 }, /*                 Greek_SIGMA Σ GREEK CAPITAL LETTER SIGMA */
    Codepair { ucs: '\u{03a4}', keysym: 0x07d4 }, /*                   Greek_TAU Τ GREEK CAPITAL LETTER TAU */
    Codepair { ucs: '\u{03a5}', keysym: 0x07d5 }, /*               Greek_UPSILON Υ GREEK CAPITAL LETTER UPSILON */
    Codepair { ucs: '\u{03a6}', keysym: 0x07d6 }, /*                   Greek_PHI Φ GREEK CAPITAL LETTER PHI */
    Codepair { ucs: '\u{03a7}', keysym: 0x07d7 }, /*                   Greek_CHI Χ GREEK CAPITAL LETTER CHI */
    Codepair { ucs: '\u{03a8}', keysym: 0x07d8 }, /*                   Greek_PSI Ψ GREEK CAPITAL LETTER PSI */
    Codepair { ucs: '\u{03a9}', keysym: 0x07d9 }, /*                 Greek_OMEGA Ω GREEK CAPITAL LETTER OMEGA */
    Codepair { ucs: '\u{03aa}', keysym: 0x07a5 }, /*         Greek_IOTAdiaeresis Ϊ GREEK CAPITAL LETTER IOTA WITH DIALYTIKA */
    Codepair { ucs: '\u{03ab}', keysym: 0x07a9 }, /*       Greek_UPSILONdieresis Ϋ GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA */
    Codepair { ucs: '\u{03ac}', keysym: 0x07b1 }, /*           Greek_alphaaccent ά GREEK SMALL LETTER ALPHA WITH TONOS */
    Codepair { ucs: '\u{03ad}', keysym: 0x07b2 }, /*         Greek_epsilonaccent έ GREEK SMALL LETTER EPSILON WITH TONOS */
    Codepair { ucs: '\u{03ae}', keysym: 0x07b3 }, /*             Greek_etaaccent ή GREEK SMALL LETTER ETA WITH TONOS */
    Codepair { ucs: '\u{03af}', keysym: 0x07b4 }, /*            Greek_iotaaccent ί GREEK SMALL LETTER IOTA WITH TONOS */
    Codepair { ucs: '\u{03b0}', keysym: 0x07ba }, /* Greek_upsilonaccentdieresis ΰ GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS */
    Codepair { ucs: '\u{03b1}', keysym: 0x07e1 }, /*                 Greek_alpha α GREEK SMALL LETTER ALPHA */
    Codepair { ucs: '\u{03b2}', keysym: 0x07e2 }, /*                  Greek_beta β GREEK SMALL LETTER BETA */
    Codepair { ucs: '\u{03b3}', keysym: 0x07e3 }, /*                 Greek_gamma γ GREEK SMALL LETTER GAMMA */
    Codepair { ucs: '\u{03b4}', keysym: 0x07e4 }, /*                 Greek_delta δ GREEK SMALL LETTER DELTA */
    Codepair { ucs: '\u{03b5}', keysym: 0x07e5 }, /*               Greek_epsilon ε GREEK SMALL LETTER EPSILON */
    Codepair { ucs: '\u{03b6}', keysym: 0x07e6 }, /*                  Greek_zeta ζ GREEK SMALL LETTER ZETA */
    Codepair { ucs: '\u{03b7}', keysym: 0x07e7 }, /*                   Greek_eta η GREEK SMALL LETTER ETA */
    Codepair { ucs: '\u{03b8}', keysym: 0x07e8 }, /*                 Greek_theta θ GREEK SMALL LETTER THETA */
    Codepair { ucs: '\u{03b9}', keysym: 0x07e9 }, /*                  Greek_iota ι GREEK SMALL LETTER IOTA */
    Codepair { ucs: '\u{03ba}', keysym: 0x07ea }, /*                 Greek_kappa κ GREEK SMALL LETTER KAPPA */
    Codepair { ucs: '\u{03bb}', keysym: 0x07eb }, /*                Greek_lambda λ GREEK SMALL LETTER LAMDA */
    Codepair { ucs: '\u{03bc}', keysym: 0x07ec }, /*                    Greek_mu μ GREEK SMALL LETTER MU */
    Codepair { ucs: '\u{03bd}', keysym: 0x07ed }, /*                    Greek_nu ν GREEK SMALL LETTER NU */
    Codepair { ucs: '\u{03be}', keysym: 0x07ee }, /*                    Greek_xi ξ GREEK SMALL LETTER XI */
    Codepair { ucs: '\u{03bf}', keysym: 0x07ef }, /*               Greek_omicron ο GREEK SMALL LETTER OMICRON */
    Codepair { ucs: '\u{03c0}', keysym: 0x07f0 }, /*                    Greek_pi π GREEK SMALL LETTER PI */
    Codepair { ucs: '\u{03c1}', keysym: 0x07f1 }, /*                   Greek_rho ρ GREEK SMALL LETTER RHO */
    Codepair { ucs: '\u{03c2}', keysym: 0x07f3 }, /*       Greek_finalsmallsigma ς GREEK SMALL LETTER FINAL SIGMA */
    Codepair { ucs: '\u{03c3}', keysym: 0x07f2 }, /*                 Greek_sigma σ GREEK SMALL LETTER SIGMA */
    Codepair { ucs: '\u{03c4}', keysym: 0x07f4 }, /*                   Greek_tau τ GREEK SMALL LETTER TAU */
    Codepair { ucs: '\u{03c5}', keysym: 0x07f5 }, /*               Greek_upsilon υ GREEK SMALL LETTER UPSILON */
    Codepair { ucs: '\u{03c6}', keysym: 0x07f6 }, /*                   Greek_phi φ GREEK SMALL LETTER PHI */
    Codepair { ucs: '\u{03c7}', keysym: 0x07f7 }, /*                   Greek_chi χ GREEK SMALL LETTER CHI */
    Codepair { ucs: '\u{03c8}', keysym: 0x07f8 }, /*                   Greek_psi ψ GREEK SMALL LETTER PSI */
    Codepair { ucs: '\u{03c9}', keysym: 0x07f9 }, /*                 Greek_omega ω GREEK SMALL LETTER OMEGA */
    Codepair { ucs: '\u{03ca}', keysym: 0x07b5 }, /*          Greek_iotadieresis ϊ GREEK SMALL LETTER IOTA WITH DIALYTIKA */
    Codepair { ucs: '\u{03cb}', keysym: 0x07b9 }, /*       Greek_upsilondieresis ϋ GREEK SMALL LETTER UPSILON WITH DIALYTIKA */
    Codepair { ucs: '\u{03cc}', keysym: 0x07b7 }, /*         Greek_omicronaccent ό GREEK SMALL LETTER OMICRON WITH TONOS */
    Codepair { ucs: '\u{03cd}', keysym: 0x07b8 }, /*         Greek_upsilonaccent ύ GREEK SMALL LETTER UPSILON WITH TONOS */
    Codepair { ucs: '\u{03ce}', keysym: 0x07bb }, /*           Greek_omegaaccent ώ GREEK SMALL LETTER OMEGA WITH TONOS */
    Codepair { ucs: '\u{0401}', keysym: 0x06b3 }, /*                 Cyrillic_IO Ё CYRILLIC CAPITAL LETTER IO */
    Codepair { ucs: '\u{0402}', keysym: 0x06b1 }, /*                 Serbian_DJE Ђ CYRILLIC CAPITAL LETTER DJE */
    Codepair { ucs: '\u{0403}', keysym: 0x06b2 }, /*               Macedonia_GJE Ѓ CYRILLIC CAPITAL LETTER GJE */
    Codepair { ucs: '\u{0404}', keysym: 0x06b4 }, /*                Ukrainian_IE Є CYRILLIC CAPITAL LETTER UKRAINIAN IE */
    Codepair { ucs: '\u{0405}', keysym: 0x06b5 }, /*               Macedonia_DSE Ѕ CYRILLIC CAPITAL LETTER DZE */
    Codepair { ucs: '\u{0406}', keysym: 0x06b6 }, /*                 Ukrainian_I І CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I */
    Codepair { ucs: '\u{0407}', keysym: 0x06b7 }, /*                Ukrainian_YI Ї CYRILLIC CAPITAL LETTER YI */
    Codepair { ucs: '\u{0408}', keysym: 0x06b8 }, /*                 Cyrillic_JE Ј CYRILLIC CAPITAL LETTER JE */
    Codepair { ucs: '\u{0409}', keysym: 0x06b9 }, /*                Cyrillic_LJE Љ CYRILLIC CAPITAL LETTER LJE */
    Codepair { ucs: '\u{040a}', keysym: 0x06ba }, /*                Cyrillic_NJE Њ CYRILLIC CAPITAL LETTER NJE */
    Codepair { ucs: '\u{040b}', keysym: 0x06bb }, /*                Serbian_TSHE Ћ CYRILLIC CAPITAL LETTER TSHE */
    Codepair { ucs: '\u{040c}', keysym: 0x06bc }, /*               Macedonia_KJE Ќ CYRILLIC CAPITAL LETTER KJE */
    Codepair { ucs: '\u{040e}', keysym: 0x06be }, /*         Byelorussian_SHORTU Ў CYRILLIC CAPITAL LETTER SHORT U */
    Codepair { ucs: '\u{040f}', keysym: 0x06bf }, /*               Cyrillic_DZHE Џ CYRILLIC CAPITAL LETTER DZHE */
    Codepair { ucs: '\u{0410}', keysym: 0x06e1 }, /*                  Cyrillic_A А CYRILLIC CAPITAL LETTER A */
    Codepair { ucs: '\u{0411}', keysym: 0x06e2 }, /*                 Cyrillic_BE Б CYRILLIC CAPITAL LETTER BE */
    Codepair { ucs: '\u{0412}', keysym: 0x06f7 }, /*                 Cyrillic_VE В CYRILLIC CAPITAL LETTER VE */
    Codepair { ucs: '\u{0413}', keysym: 0x06e7 }, /*                Cyrillic_GHE Г CYRILLIC CAPITAL LETTER GHE */
    Codepair { ucs: '\u{0414}', keysym: 0x06e4 }, /*                 Cyrillic_DE Д CYRILLIC CAPITAL LETTER DE */
    Codepair { ucs: '\u{0415}', keysym: 0x06e5 }, /*                 Cyrillic_IE Е CYRILLIC CAPITAL LETTER IE */
    Codepair { ucs: '\u{0416}', keysym: 0x06f6 }, /*                Cyrillic_ZHE Ж CYRILLIC CAPITAL LETTER ZHE */
    Codepair { ucs: '\u{0417}', keysym: 0x06fa }, /*                 Cyrillic_ZE З CYRILLIC CAPITAL LETTER ZE */
    Codepair { ucs: '\u{0418}', keysym: 0x06e9 }, /*                  Cyrillic_I И CYRILLIC CAPITAL LETTER I */
    Codepair { ucs: '\u{0419}', keysym: 0x06ea }, /*             Cyrillic_SHORTI Й CYRILLIC CAPITAL LETTER SHORT I */
    Codepair { ucs: '\u{041a}', keysym: 0x06eb }, /*                 Cyrillic_KA К CYRILLIC CAPITAL LETTER KA */
    Codepair { ucs: '\u{041b}', keysym: 0x06ec }, /*                 Cyrillic_EL Л CYRILLIC CAPITAL LETTER EL */
    Codepair { ucs: '\u{041c}', keysym: 0x06ed }, /*                 Cyrillic_EM М CYRILLIC CAPITAL LETTER EM */
    Codepair { ucs: '\u{041d}', keysym: 0x06ee }, /*                 Cyrillic_EN Н CYRILLIC CAPITAL LETTER EN */
    Codepair { ucs: '\u{041e}', keysym: 0x06ef }, /*                  Cyrillic_O О CYRILLIC CAPITAL LETTER O */
    Codepair { ucs: '\u{041f}', keysym: 0x06f0 }, /*                 Cyrillic_PE П CYRILLIC CAPITAL LETTER PE */
    Codepair { ucs: '\u{0420}', keysym: 0x06f2 }, /*                 Cyrillic_ER Р CYRILLIC CAPITAL LETTER ER */
    Codepair { ucs: '\u{0421}', keysym: 0x06f3 }, /*                 Cyrillic_ES С CYRILLIC CAPITAL LETTER ES */
    Codepair { ucs: '\u{0422}', keysym: 0x06f4 }, /*                 Cyrillic_TE Т CYRILLIC CAPITAL LETTER TE */
    Codepair { ucs: '\u{0423}', keysym: 0x06f5 }, /*                  Cyrillic_U У CYRILLIC CAPITAL LETTER U */
    Codepair { ucs: '\u{0424}', keysym: 0x06e6 }, /*                 Cyrillic_EF Ф CYRILLIC CAPITAL LETTER EF */
    Codepair { ucs: '\u{0425}', keysym: 0x06e8 }, /*                 Cyrillic_HA Х CYRILLIC CAPITAL LETTER HA */
    Codepair { ucs: '\u{0426}', keysym: 0x06e3 }, /*                Cyrillic_TSE Ц CYRILLIC CAPITAL LETTER TSE */
    Codepair { ucs: '\u{0427}', keysym: 0x06fe }, /*                Cyrillic_CHE Ч CYRILLIC CAPITAL LETTER CHE */
    Codepair { ucs: '\u{0428}', keysym: 0x06fb }, /*                Cyrillic_SHA Ш CYRILLIC CAPITAL LETTER SHA */
    Codepair { ucs: '\u{0429}', keysym: 0x06fd }, /*              Cyrillic_SHCHA Щ CYRILLIC CAPITAL LETTER SHCHA */
    Codepair { ucs: '\u{042a}', keysym: 0x06ff }, /*           Cyrillic_HARDSIGN Ъ CYRILLIC CAPITAL LETTER HARD SIGN */
    Codepair { ucs: '\u{042b}', keysym: 0x06f9 }, /*               Cyrillic_YERU Ы CYRILLIC CAPITAL LETTER YERU */
    Codepair { ucs: '\u{042c}', keysym: 0x06f8 }, /*           Cyrillic_SOFTSIGN Ь CYRILLIC CAPITAL LETTER SOFT SIGN */
    Codepair { ucs: '\u{042d}', keysym: 0x06fc }, /*                  Cyrillic_E Э CYRILLIC CAPITAL LETTER E */
    Codepair { ucs: '\u{042e}', keysym: 0x06e0 }, /*                 Cyrillic_YU Ю CYRILLIC CAPITAL LETTER YU */
    Codepair { ucs: '\u{042f}', keysym: 0x06f1 }, /*                 Cyrillic_YA Я CYRILLIC CAPITAL LETTER YA */
    Codepair { ucs: '\u{0430}', keysym: 0x06c1 }, /*                  Cyrillic_a а CYRILLIC SMALL LETTER A */
    Codepair { ucs: '\u{0431}', keysym: 0x06c2 }, /*                 Cyrillic_be б CYRILLIC SMALL LETTER BE */
    Codepair { ucs: '\u{0432}', keysym: 0x06d7 }, /*                 Cyrillic_ve в CYRILLIC SMALL LETTER VE */
    Codepair { ucs: '\u{0433}', keysym: 0x06c7 }, /*                Cyrillic_ghe г CYRILLIC SMALL LETTER GHE */
    Codepair { ucs: '\u{0434}', keysym: 0x06c4 }, /*                 Cyrillic_de д CYRILLIC SMALL LETTER DE */
    Codepair { ucs: '\u{0435}', keysym: 0x06c5 }, /*                 Cyrillic_ie е CYRILLIC SMALL LETTER IE */
    Codepair { ucs: '\u{0436}', keysym: 0x06d6 }, /*                Cyrillic_zhe ж CYRILLIC SMALL LETTER ZHE */
    Codepair { ucs: '\u{0437}', keysym: 0x06da }, /*                 Cyrillic_ze з CYRILLIC SMALL LETTER ZE */
    Codepair { ucs: '\u{0438}', keysym: 0x06c9 }, /*                  Cyrillic_i и CYRILLIC SMALL LETTER I */
    Codepair { ucs: '\u{0439}', keysym: 0x06ca }, /*             Cyrillic_shorti й CYRILLIC SMALL LETTER SHORT I */
    Codepair { ucs: '\u{043a}', keysym: 0x06cb }, /*                 Cyrillic_ka к CYRILLIC SMALL LETTER KA */
    Codepair { ucs: '\u{043b}', keysym: 0x06cc }, /*                 Cyrillic_el л CYRILLIC SMALL LETTER EL */
    Codepair { ucs: '\u{043c}', keysym: 0x06cd }, /*                 Cyrillic_em м CYRILLIC SMALL LETTER EM */
    Codepair { ucs: '\u{043d}', keysym: 0x06ce }, /*                 Cyrillic_en н CYRILLIC SMALL LETTER EN */
    Codepair { ucs: '\u{043e}', keysym: 0x06cf }, /*                  Cyrillic_o о CYRILLIC SMALL LETTER O */
    Codepair { ucs: '\u{043f}', keysym: 0x06d0 }, /*                 Cyrillic_pe п CYRILLIC SMALL LETTER PE */
    Codepair { ucs: '\u{0440}', keysym: 0x06d2 }, /*                 Cyrillic_er р CYRILLIC SMALL LETTER ER */
    Codepair { ucs: '\u{0441}', keysym: 0x06d3 }, /*                 Cyrillic_es с CYRILLIC SMALL LETTER ES */
    Codepair { ucs: '\u{0442}', keysym: 0x06d4 }, /*                 Cyrillic_te т CYRILLIC SMALL LETTER TE */
    Codepair { ucs: '\u{0443}', keysym: 0x06d5 }, /*                  Cyrillic_u у CYRILLIC SMALL LETTER U */
    Codepair { ucs: '\u{0444}', keysym: 0x06c6 }, /*                 Cyrillic_ef ф CYRILLIC SMALL LETTER EF */
    Codepair { ucs: '\u{0445}', keysym: 0x06c8 }, /*                 Cyrillic_ha х CYRILLIC SMALL LETTER HA */
    Codepair { ucs: '\u{0446}', keysym: 0x06c3 }, /*                Cyrillic_tse ц CYRILLIC SMALL LETTER TSE */
    Codepair { ucs: '\u{0447}', keysym: 0x06de }, /*                Cyrillic_che ч CYRILLIC SMALL LETTER CHE */
    Codepair { ucs: '\u{0448}', keysym: 0x06db }, /*                Cyrillic_sha ш CYRILLIC SMALL LETTER SHA */
    Codepair { ucs: '\u{0449}', keysym: 0x06dd }, /*              Cyrillic_shcha щ CYRILLIC SMALL LETTER SHCHA */
    Codepair { ucs: '\u{044a}', keysym: 0x06df }, /*           Cyrillic_hardsign ъ CYRILLIC SMALL LETTER HARD SIGN */
    Codepair { ucs: '\u{044b}', keysym: 0x06d9 }, /*               Cyrillic_yeru ы CYRILLIC SMALL LETTER YERU */
    Codepair { ucs: '\u{044c}', keysym: 0x06d8 }, /*           Cyrillic_softsign ь CYRILLIC SMALL LETTER SOFT SIGN */
    Codepair { ucs: '\u{044d}', keysym: 0x06dc }, /*                  Cyrillic_e э CYRILLIC SMALL LETTER E */
    Codepair { ucs: '\u{044e}', keysym: 0x06c0 }, /*                 Cyrillic_yu ю CYRILLIC SMALL LETTER YU */
    Codepair { ucs: '\u{044f}', keysym: 0x06d1 }, /*                 Cyrillic_ya я CYRILLIC SMALL LETTER YA */
    Codepair { ucs: '\u{0451}', keysym: 0x06a3 }, /*                 Cyrillic_io ё CYRILLIC SMALL LETTER IO */
    Codepair { ucs: '\u{0452}', keysym: 0x06a1 }, /*                 Serbian_dje ђ CYRILLIC SMALL LETTER DJE */
    Codepair { ucs: '\u{0453}', keysym: 0x06a2 }, /*               Macedonia_gje ѓ CYRILLIC SMALL LETTER GJE */
    Codepair { ucs: '\u{0454}', keysym: 0x06a4 }, /*                Ukrainian_ie є CYRILLIC SMALL LETTER UKRAINIAN IE */
    Codepair { ucs: '\u{0455}', keysym: 0x06a5 }, /*               Macedonia_dse ѕ CYRILLIC SMALL LETTER DZE */
    Codepair { ucs: '\u{0456}', keysym: 0x06a6 }, /*                 Ukrainian_i і CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I */
    Codepair { ucs: '\u{0457}', keysym: 0x06a7 }, /*                Ukrainian_yi ї CYRILLIC SMALL LETTER YI */
    Codepair { ucs: '\u{0458}', keysym: 0x06a8 }, /*                 Cyrillic_je ј CYRILLIC SMALL LETTER JE */
    Codepair { ucs: '\u{0459}', keysym: 0x06a9 }, /*                Cyrillic_lje љ CYRILLIC SMALL LETTER LJE */
    Codepair { ucs: '\u{045a}', keysym: 0x06aa }, /*                Cyrillic_nje њ CYRILLIC SMALL LETTER NJE */
    Codepair { ucs: '\u{045b}', keysym: 0x06ab }, /*                Serbian_tshe ћ CYRILLIC SMALL LETTER TSHE */
    Codepair { ucs: '\u{045c}', keysym: 0x06ac }, /*               Macedonia_kje ќ CYRILLIC SMALL LETTER KJE */
    Codepair { ucs: '\u{045e}', keysym: 0x06ae }, /*         Byelorussian_shortu ў CYRILLIC SMALL LETTER SHORT U */
    Codepair { ucs: '\u{045f}', keysym: 0x06af }, /*               Cyrillic_dzhe џ CYRILLIC SMALL LETTER DZHE */
    Codepair { ucs: '\u{0490}', keysym: 0x06bd }, /*   Ukrainian_GHE_WITH_UPTURN Ґ CYRILLIC CAPITAL LETTER GHE WITH UPTURN */
    Codepair { ucs: '\u{0491}', keysym: 0x06ad }, /*   Ukrainian_ghe_with_upturn ґ CYRILLIC SMALL LETTER GHE WITH UPTURN */
    Codepair { ucs: '\u{05d0}', keysym: 0x0ce0 }, /*                hebrew_aleph א HEBREW LETTER ALEF */
    Codepair { ucs: '\u{05d1}', keysym: 0x0ce1 }, /*                  hebrew_bet ב HEBREW LETTER BET */
    Codepair { ucs: '\u{05d2}', keysym: 0x0ce2 }, /*                hebrew_gimel ג HEBREW LETTER GIMEL */
    Codepair { ucs: '\u{05d3}', keysym: 0x0ce3 }, /*                hebrew_dalet ד HEBREW LETTER DALET */
    Codepair { ucs: '\u{05d4}', keysym: 0x0ce4 }, /*                   hebrew_he ה HEBREW LETTER HE */
    Codepair { ucs: '\u{05d5}', keysym: 0x0ce5 }, /*                  hebrew_waw ו HEBREW LETTER VAV */
    Codepair { ucs: '\u{05d6}', keysym: 0x0ce6 }, /*                 hebrew_zain ז HEBREW LETTER ZAYIN */
    Codepair { ucs: '\u{05d7}', keysym: 0x0ce7 }, /*                 hebrew_chet ח HEBREW LETTER HET */
    Codepair { ucs: '\u{05d8}', keysym: 0x0ce8 }, /*                  hebrew_tet ט HEBREW LETTER TET */
    Codepair { ucs: '\u{05d9}', keysym: 0x0ce9 }, /*                  hebrew_yod י HEBREW LETTER YOD */
    Codepair { ucs: '\u{05da}', keysym: 0x0cea }, /*            hebrew_finalkaph ך HEBREW LETTER FINAL KAF */
    Codepair { ucs: '\u{05db}', keysym: 0x0ceb }, /*                 hebrew_kaph כ HEBREW LETTER KAF */
    Codepair { ucs: '\u{05dc}', keysym: 0x0cec }, /*                hebrew_lamed ל HEBREW LETTER LAMED */
    Codepair { ucs: '\u{05dd}', keysym: 0x0ced }, /*             hebrew_finalmem ם HEBREW LETTER FINAL MEM */
    Codepair { ucs: '\u{05de}', keysym: 0x0cee }, /*                  hebrew_mem מ HEBREW LETTER MEM */
    Codepair { ucs: '\u{05df}', keysym: 0x0cef }, /*             hebrew_finalnun ן HEBREW LETTER FINAL NUN */
    Codepair { ucs: '\u{05e0}', keysym: 0x0cf0 }, /*                  hebrew_nun נ HEBREW LETTER NUN */
    Codepair { ucs: '\u{05e1}', keysym: 0x0cf1 }, /*               hebrew_samech ס HEBREW LETTER SAMEKH */
    Codepair { ucs: '\u{05e2}', keysym: 0x0cf2 }, /*                 hebrew_ayin ע HEBREW LETTER AYIN */
    Codepair { ucs: '\u{05e3}', keysym: 0x0cf3 }, /*              hebrew_finalpe ף HEBREW LETTER FINAL PE */
    Codepair { ucs: '\u{05e4}', keysym: 0x0cf4 }, /*                   hebrew_pe פ HEBREW LETTER PE */
    Codepair { ucs: '\u{05e5}', keysym: 0x0cf5 }, /*            hebrew_finalzade ץ HEBREW LETTER FINAL TSADI */
    Codepair { ucs: '\u{05e6}', keysym: 0x0cf6 }, /*                 hebrew_zade צ HEBREW LETTER TSADI */
    Codepair { ucs: '\u{05e7}', keysym: 0x0cf7 }, /*                 hebrew_qoph ק HEBREW LETTER QOF */
    Codepair { ucs: '\u{05e8}', keysym: 0x0cf8 }, /*                 hebrew_resh ר HEBREW LETTER RESH */
    Codepair { ucs: '\u{05e9}', keysym: 0x0cf9 }, /*                 hebrew_shin ש HEBREW LETTER SHIN */
    Codepair { ucs: '\u{05ea}', keysym: 0x0cfa }, /*                  hebrew_taw ת HEBREW LETTER TAV */
    Codepair { ucs: '\u{060c}', keysym: 0x05ac }, /*                Arabic_comma ، ARABIC COMMA */
    Codepair { ucs: '\u{061b}', keysym: 0x05bb }, /*            Arabic_semicolon ؛ ARABIC SEMICOLON */
    Codepair { ucs: '\u{061f}', keysym: 0x05bf }, /*        Arabic_question_mark ؟ ARABIC QUESTION MARK */
    Codepair { ucs: '\u{0621}', keysym: 0x05c1 }, /*                Arabic_hamza ء ARABIC LETTER HAMZA */
    Codepair { ucs: '\u{0622}', keysym: 0x05c2 }, /*          Arabic_maddaonalef آ ARABIC LETTER ALEF WITH MADDA ABOVE */
    Codepair { ucs: '\u{0623}', keysym: 0x05c3 }, /*          Arabic_hamzaonalef أ ARABIC LETTER ALEF WITH HAMZA ABOVE */
    Codepair { ucs: '\u{0624}', keysym: 0x05c4 }, /*           Arabic_hamzaonwaw ؤ ARABIC LETTER WAW WITH HAMZA ABOVE */
    Codepair { ucs: '\u{0625}', keysym: 0x05c5 }, /*       Arabic_hamzaunderalef إ ARABIC LETTER ALEF WITH HAMZA BELOW */
    Codepair { ucs: '\u{0626}', keysym: 0x05c6 }, /*           Arabic_hamzaonyeh ئ ARABIC LETTER YEH WITH HAMZA ABOVE */
    Codepair { ucs: '\u{0627}', keysym: 0x05c7 }, /*                 Arabic_alef ا ARABIC LETTER ALEF */
    Codepair { ucs: '\u{0628}', keysym: 0x05c8 }, /*                  Arabic_beh ب ARABIC LETTER BEH */
    Codepair { ucs: '\u{0629}', keysym: 0x05c9 }, /*           Arabic_tehmarbuta ة ARABIC LETTER TEH MARBUTA */
    Codepair { ucs: '\u{062a}', keysym: 0x05ca }, /*                  Arabic_teh ت ARABIC LETTER TEH */
    Codepair { ucs: '\u{062b}', keysym: 0x05cb }, /*                 Arabic_theh ث ARABIC LETTER THEH */
    Codepair { ucs: '\u{062c}', keysym: 0x05cc }, /*                 Arabic_jeem ج ARABIC LETTER JEEM */
    Codepair { ucs: '\u{062d}', keysym: 0x05cd }, /*                  Arabic_hah ح ARABIC LETTER HAH */
    Codepair { ucs: '\u{062e}', keysym: 0x05ce }, /*                 Arabic_khah خ ARABIC LETTER KHAH */
    Codepair { ucs: '\u{062f}', keysym: 0x05cf }, /*                  Arabic_dal د ARABIC LETTER DAL */
    Codepair { ucs: '\u{0630}', keysym: 0x05d0 }, /*                 Arabic_thal ذ ARABIC LETTER THAL */
    Codepair { ucs: '\u{0631}', keysym: 0x05d1 }, /*                   Arabic_ra ر ARABIC LETTER REH */
    Codepair { ucs: '\u{0632}', keysym: 0x05d2 }, /*                 Arabic_zain ز ARABIC LETTER ZAIN */
    Codepair { ucs: '\u{0633}', keysym: 0x05d3 }, /*                 Arabic_seen س ARABIC LETTER SEEN */
    Codepair { ucs: '\u{0634}', keysym: 0x05d4 }, /*                Arabic_sheen ش ARABIC LETTER SHEEN */
    Codepair { ucs: '\u{0635}', keysym: 0x05d5 }, /*                  Arabic_sad ص ARABIC LETTER SAD */
    Codepair { ucs: '\u{0636}', keysym: 0x05d6 }, /*                  Arabic_dad ض ARABIC LETTER DAD */
    Codepair { ucs: '\u{0637}', keysym: 0x05d7 }, /*                  Arabic_tah ط ARABIC LETTER TAH */
    Codepair { ucs: '\u{0638}', keysym: 0x05d8 }, /*                  Arabic_zah ظ ARABIC LETTER ZAH */
    Codepair { ucs: '\u{0639}', keysym: 0x05d9 }, /*                  Arabic_ain ع ARABIC LETTER AIN */
    Codepair { ucs: '\u{063a}', keysym: 0x05da }, /*                Arabic_ghain غ ARABIC LETTER GHAIN */
    Codepair { ucs: '\u{0640}', keysym: 0x05e0 }, /*              Arabic_tatweel ـ ARABIC TATWEEL */
    Codepair { ucs: '\u{0641}', keysym: 0x05e1 }, /*                  Arabic_feh ف ARABIC LETTER FEH */
    Codepair { ucs: '\u{0642}', keysym: 0x05e2 }, /*                  Arabic_qaf ق ARABIC LETTER QAF */
    Codepair { ucs: '\u{0643}', keysym: 0x05e3 }, /*                  Arabic_kaf ك ARABIC LETTER KAF */
    Codepair { ucs: '\u{0644}', keysym: 0x05e4 }, /*                  Arabic_lam ل ARABIC LETTER LAM */
    Codepair { ucs: '\u{0645}', keysym: 0x05e5 }, /*                 Arabic_meem م ARABIC LETTER MEEM */
    Codepair { ucs: '\u{0646}', keysym: 0x05e6 }, /*                 Arabic_noon ن ARABIC LETTER NOON */
    Codepair { ucs: '\u{0647}', keysym: 0x05e7 }, /*                   Arabic_ha ه ARABIC LETTER HEH */
    Codepair { ucs: '\u{0648}', keysym: 0x05e8 }, /*                  Arabic_waw و ARABIC LETTER WAW */
    Codepair { ucs: '\u{0649}', keysym: 0x05e9 }, /*          Arabic_alefmaksura ى ARABIC LETTER ALEF MAKSURA */
    Codepair { ucs: '\u{064a}', keysym: 0x05ea }, /*                  Arabic_yeh ي ARABIC LETTER YEH */
    Codepair { ucs: '\u{064b}', keysym: 0x05eb }, /*             Arabic_fathatan ً ARABIC FATHATAN */
    Codepair { ucs: '\u{064c}', keysym: 0x05ec }, /*             Arabic_dammatan ٌ ARABIC DAMMATAN */
    Codepair { ucs: '\u{064d}', keysym: 0x05ed }, /*             Arabic_kasratan ٍ ARABIC KASRATAN */
    Codepair { ucs: '\u{064e}', keysym: 0x05ee }, /*                Arabic_fatha َ ARABIC FATHA */
    Codepair { ucs: '\u{064f}', keysym: 0x05ef }, /*                Arabic_damma ُ ARABIC DAMMA */
    Codepair { ucs: '\u{0650}', keysym: 0x05f0 }, /*                Arabic_kasra ِ ARABIC KASRA */
    Codepair { ucs: '\u{0651}', keysym: 0x05f1 }, /*               Arabic_shadda ّ ARABIC SHADDA */
    Codepair { ucs: '\u{0652}', keysym: 0x05f2 }, /*                Arabic_sukun ْ ARABIC SUKUN */
    Codepair { ucs: '\u{0e01}', keysym: 0x0da1 }, /*                  Thai_kokai ก THAI CHARACTER KO KAI */
    Codepair { ucs: '\u{0e02}', keysym: 0x0da2 }, /*                Thai_khokhai ข THAI CHARACTER KHO KHAI */
    Codepair { ucs: '\u{0e03}', keysym: 0x0da3 }, /*               Thai_khokhuat ฃ THAI CHARACTER KHO KHUAT */
    Codepair { ucs: '\u{0e04}', keysym: 0x0da4 }, /*               Thai_khokhwai ค THAI CHARACTER KHO KHWAI */
    Codepair { ucs: '\u{0e05}', keysym: 0x0da5 }, /*                Thai_khokhon ฅ THAI CHARACTER KHO KHON */
    Codepair { ucs: '\u{0e06}', keysym: 0x0da6 }, /*             Thai_khorakhang ฆ THAI CHARACTER KHO RAKHANG */
    Codepair { ucs: '\u{0e07}', keysym: 0x0da7 }, /*                 Thai_ngongu ง THAI CHARACTER NGO NGU */
    Codepair { ucs: '\u{0e08}', keysym: 0x0da8 }, /*                Thai_chochan จ THAI CHARACTER CHO CHAN */
    Codepair { ucs: '\u{0e09}', keysym: 0x0da9 }, /*               Thai_choching ฉ THAI CHARACTER CHO CHING */
    Codepair { ucs: '\u{0e0a}', keysym: 0x0daa }, /*               Thai_chochang ช THAI CHARACTER CHO CHANG */
    Codepair { ucs: '\u{0e0b}', keysym: 0x0dab }, /*                   Thai_soso ซ THAI CHARACTER SO SO */
    Codepair { ucs: '\u{0e0c}', keysym: 0x0dac }, /*                Thai_chochoe ฌ THAI CHARACTER CHO CHOE */
    Codepair { ucs: '\u{0e0d}', keysym: 0x0dad }, /*                 Thai_yoying ญ THAI CHARACTER YO YING */
    Codepair { ucs: '\u{0e0e}', keysym: 0x0dae }, /*                Thai_dochada ฎ THAI CHARACTER DO CHADA */
    Codepair { ucs: '\u{0e0f}', keysym: 0x0daf }, /*                Thai_topatak ฏ THAI CHARACTER TO PATAK */
    Codepair { ucs: '\u{0e10}', keysym: 0x0db0 }, /*                Thai_thothan ฐ THAI CHARACTER THO THAN */
    Codepair { ucs: '\u{0e11}', keysym: 0x0db1 }, /*          Thai_thonangmontho ฑ THAI CHARACTER THO NANGMONTHO */
    Codepair { ucs: '\u{0e12}', keysym: 0x0db2 }, /*             Thai_thophuthao ฒ THAI CHARACTER THO PHUTHAO */
    Codepair { ucs: '\u{0e13}', keysym: 0x0db3 }, /*                  Thai_nonen ณ THAI CHARACTER NO NEN */
    Codepair { ucs: '\u{0e14}', keysym: 0x0db4 }, /*                  Thai_dodek ด THAI CHARACTER DO DEK */
    Codepair { ucs: '\u{0e15}', keysym: 0x0db5 }, /*                  Thai_totao ต THAI CHARACTER TO TAO */
    Codepair { ucs: '\u{0e16}', keysym: 0x0db6 }, /*               Thai_thothung ถ THAI CHARACTER THO THUNG */
    Codepair { ucs: '\u{0e17}', keysym: 0x0db7 }, /*              Thai_thothahan ท THAI CHARACTER THO THAHAN */
    Codepair { ucs: '\u{0e18}', keysym: 0x0db8 }, /*               Thai_thothong ธ THAI CHARACTER THO THONG */
    Codepair { ucs: '\u{0e19}', keysym: 0x0db9 }, /*                   Thai_nonu น THAI CHARACTER NO NU */
    Codepair { ucs: '\u{0e1a}', keysym: 0x0dba }, /*               Thai_bobaimai บ THAI CHARACTER BO BAIMAI */
    Codepair { ucs: '\u{0e1b}', keysym: 0x0dbb }, /*                  Thai_popla ป THAI CHARACTER PO PLA */
    Codepair { ucs: '\u{0e1c}', keysym: 0x0dbc }, /*               Thai_phophung ผ THAI CHARACTER PHO PHUNG */
    Codepair { ucs: '\u{0e1d}', keysym: 0x0dbd }, /*                   Thai_fofa ฝ THAI CHARACTER FO FA */
    Codepair { ucs: '\u{0e1e}', keysym: 0x0dbe }, /*                Thai_phophan พ THAI CHARACTER PHO PHAN */
    Codepair { ucs: '\u{0e1f}', keysym: 0x0dbf }, /*                  Thai_fofan ฟ THAI CHARACTER FO FAN */
    Codepair { ucs: '\u{0e20}', keysym: 0x0dc0 }, /*             Thai_phosamphao ภ THAI CHARACTER PHO SAMPHAO */
    Codepair { ucs: '\u{0e21}', keysym: 0x0dc1 }, /*                   Thai_moma ม THAI CHARACTER MO MA */
    Codepair { ucs: '\u{0e22}', keysym: 0x0dc2 }, /*                  Thai_yoyak ย THAI CHARACTER YO YAK */
    Codepair { ucs: '\u{0e23}', keysym: 0x0dc3 }, /*                  Thai_rorua ร THAI CHARACTER RO RUA */
    Codepair { ucs: '\u{0e24}', keysym: 0x0dc4 }, /*                     Thai_ru ฤ THAI CHARACTER RU */
    Codepair { ucs: '\u{0e25}', keysym: 0x0dc5 }, /*                 Thai_loling ล THAI CHARACTER LO LING */
    Codepair { ucs: '\u{0e26}', keysym: 0x0dc6 }, /*                     Thai_lu ฦ THAI CHARACTER LU */
    Codepair { ucs: '\u{0e27}', keysym: 0x0dc7 }, /*                 Thai_wowaen ว THAI CHARACTER WO WAEN */
    Codepair { ucs: '\u{0e28}', keysym: 0x0dc8 }, /*                 Thai_sosala ศ THAI CHARACTER SO SALA */
    Codepair { ucs: '\u{0e29}', keysym: 0x0dc9 }, /*                 Thai_sorusi ษ THAI CHARACTER SO RUSI */
    Codepair { ucs: '\u{0e2a}', keysym: 0x0dca }, /*                  Thai_sosua ส THAI CHARACTER SO SUA */
    Codepair { ucs: '\u{0e2b}', keysym: 0x0dcb }, /*                  Thai_hohip ห THAI CHARACTER HO HIP */
    Codepair { ucs: '\u{0e2c}', keysym: 0x0dcc }, /*                Thai_lochula ฬ THAI CHARACTER LO CHULA */
    Codepair { ucs: '\u{0e2d}', keysym: 0x0dcd }, /*                   Thai_oang อ THAI CHARACTER O ANG */
    Codepair { ucs: '\u{0e2e}', keysym: 0x0dce }, /*               Thai_honokhuk ฮ THAI CHARACTER HO NOKHUK */
    Codepair { ucs: '\u{0e2f}', keysym: 0x0dcf }, /*              Thai_paiyannoi ฯ THAI CHARACTER PAIYANNOI */
    Codepair { ucs: '\u{0e30}', keysym: 0x0dd0 }, /*                  Thai_saraa ะ THAI CHARACTER SARA A */
    Codepair { ucs: '\u{0e31}', keysym: 0x0dd1 }, /*             Thai_maihanakat ั THAI CHARACTER MAI HAN-AKAT */
    Codepair { ucs: '\u{0e32}', keysym: 0x0dd2 }, /*                 Thai_saraaa า THAI CHARACTER SARA AA */
    Codepair { ucs: '\u{0e33}', keysym: 0x0dd3 }, /*                 Thai_saraam ำ THAI CHARACTER SARA AM */
    Codepair { ucs: '\u{0e34}', keysym: 0x0dd4 }, /*                  Thai_sarai ิ THAI CHARACTER SARA I */
    Codepair { ucs: '\u{0e35}', keysym: 0x0dd5 }, /*                 Thai_saraii ี THAI CHARACTER SARA II */
    Codepair { ucs: '\u{0e36}', keysym: 0x0dd6 }, /*                 Thai_saraue ึ THAI CHARACTER SARA UE */
    Codepair { ucs: '\u{0e37}', keysym: 0x0dd7 }, /*                Thai_sarauee ื THAI CHARACTER SARA UEE */
    Codepair { ucs: '\u{0e38}', keysym: 0x0dd8 }, /*                  Thai_sarau ุ THAI CHARACTER SARA U */
    Codepair { ucs: '\u{0e39}', keysym: 0x0dd9 }, /*                 Thai_sarauu ู THAI CHARACTER SARA UU */
    Codepair { ucs: '\u{0e3a}', keysym: 0x0dda }, /*                Thai_phinthu ฺ THAI CHARACTER PHINTHU */
    Codepair { ucs: '\u{0e3e}', keysym: 0x0dde }, /*      Thai_maihanakat_maitho ฾ ??? */
    Codepair { ucs: '\u{0e3f}', keysym: 0x0ddf }, /*                   Thai_baht ฿ THAI CURRENCY SYMBOL BAHT */
    Codepair { ucs: '\u{0e40}', keysym: 0x0de0 }, /*                  Thai_sarae เ THAI CHARACTER SARA E */
    Codepair { ucs: '\u{0e41}', keysym: 0x0de1 }, /*                 Thai_saraae แ THAI CHARACTER SARA AE */
    Codepair { ucs: '\u{0e42}', keysym: 0x0de2 }, /*                  Thai_sarao โ THAI CHARACTER SARA O */
    Codepair { ucs: '\u{0e43}', keysym: 0x0de3 }, /*          Thai_saraaimaimuan ใ THAI CHARACTER SARA AI MAIMUAN */
    Codepair { ucs: '\u{0e44}', keysym: 0x0de4 }, /*         Thai_saraaimaimalai ไ THAI CHARACTER SARA AI MAIMALAI */
    Codepair { ucs: '\u{0e45}', keysym: 0x0de5 }, /*            Thai_lakkhangyao ๅ THAI CHARACTER LAKKHANGYAO */
    Codepair { ucs: '\u{0e46}', keysym: 0x0de6 }, /*               Thai_maiyamok ๆ THAI CHARACTER MAIYAMOK */
    Codepair { ucs: '\u{0e47}', keysym: 0x0de7 }, /*              Thai_maitaikhu ็ THAI CHARACTER MAITAIKHU */
    Codepair { ucs: '\u{0e48}', keysym: 0x0de8 }, /*                  Thai_maiek ่ THAI CHARACTER MAI EK */
    Codepair { ucs: '\u{0e49}', keysym: 0x0de9 }, /*                 Thai_maitho ้ THAI CHARACTER MAI THO */
    Codepair { ucs: '\u{0e4a}', keysym: 0x0dea }, /*                 Thai_maitri ๊ THAI CHARACTER MAI TRI */
    Codepair { ucs: '\u{0e4b}', keysym: 0x0deb }, /*            Thai_maichattawa ๋ THAI CHARACTER MAI CHATTAWA */
    Codepair { ucs: '\u{0e4c}', keysym: 0x0dec }, /*            Thai_thanthakhat ์ THAI CHARACTER THANTHAKHAT */
    Codepair { ucs: '\u{0e4d}', keysym: 0x0ded }, /*               Thai_nikhahit ํ THAI CHARACTER NIKHAHIT */
    Codepair { ucs: '\u{0e50}', keysym: 0x0df0 }, /*                 Thai_leksun ๐ THAI DIGIT ZERO */
    Codepair { ucs: '\u{0e51}', keysym: 0x0df1 }, /*                Thai_leknung ๑ THAI DIGIT ONE */
    Codepair { ucs: '\u{0e52}', keysym: 0x0df2 }, /*                Thai_leksong ๒ THAI DIGIT TWO */
    Codepair { ucs: '\u{0e53}', keysym: 0x0df3 }, /*                 Thai_leksam ๓ THAI DIGIT THREE */
    Codepair { ucs: '\u{0e54}', keysym: 0x0df4 }, /*                  Thai_leksi ๔ THAI DIGIT FOUR */
    Codepair { ucs: '\u{0e55}', keysym: 0x0df5 }, /*                  Thai_lekha ๕ THAI DIGIT FIVE */
    Codepair { ucs: '\u{0e56}', keysym: 0x0df6 }, /*                 Thai_lekhok ๖ THAI DIGIT SIX */
    Codepair { ucs: '\u{0e57}', keysym: 0x0df7 }, /*                Thai_lekchet ๗ THAI DIGIT SEVEN */
    Codepair { ucs: '\u{0e58}', keysym: 0x0df8 }, /*                Thai_lekpaet ๘ THAI DIGIT EIGHT */
    Codepair { ucs: '\u{0e59}', keysym: 0x0df9 }, /*                 Thai_lekkao ๙ THAI DIGIT NINE */
    Codepair { ucs: '\u{11a8}', keysym: 0x0ed4 }, /*             Hangul_J_Kiyeog ᆨ HANGUL JONGSEONG KIYEOK */
    Codepair { ucs: '\u{11a9}', keysym: 0x0ed5 }, /*        Hangul_J_SsangKiyeog ᆩ HANGUL JONGSEONG SSANGKIYEOK */
    Codepair { ucs: '\u{11aa}', keysym: 0x0ed6 }, /*         Hangul_J_KiyeogSios ᆪ HANGUL JONGSEONG KIYEOK-SIOS */
    Codepair { ucs: '\u{11ab}', keysym: 0x0ed7 }, /*              Hangul_J_Nieun ᆫ HANGUL JONGSEONG NIEUN */
    Codepair { ucs: '\u{11ac}', keysym: 0x0ed8 }, /*         Hangul_J_NieunJieuj ᆬ HANGUL JONGSEONG NIEUN-CIEUC */
    Codepair { ucs: '\u{11ad}', keysym: 0x0ed9 }, /*         Hangul_J_NieunHieuh ᆭ HANGUL JONGSEONG NIEUN-HIEUH */
    Codepair { ucs: '\u{11ae}', keysym: 0x0eda }, /*             Hangul_J_Dikeud ᆮ HANGUL JONGSEONG TIKEUT */
    Codepair { ucs: '\u{11af}', keysym: 0x0edb }, /*              Hangul_J_Rieul ᆯ HANGUL JONGSEONG RIEUL */
    Codepair { ucs: '\u{11b0}', keysym: 0x0edc }, /*        Hangul_J_RieulKiyeog ᆰ HANGUL JONGSEONG RIEUL-KIYEOK */
    Codepair { ucs: '\u{11b1}', keysym: 0x0edd }, /*         Hangul_J_RieulMieum ᆱ HANGUL JONGSEONG RIEUL-MIEUM */
    Codepair { ucs: '\u{11b2}', keysym: 0x0ede }, /*         Hangul_J_RieulPieub ᆲ HANGUL JONGSEONG RIEUL-PIEUP */
    Codepair { ucs: '\u{11b3}', keysym: 0x0edf }, /*          Hangul_J_RieulSios ᆳ HANGUL JONGSEONG RIEUL-SIOS */
    Codepair { ucs: '\u{11b4}', keysym: 0x0ee0 }, /*         Hangul_J_RieulTieut ᆴ HANGUL JONGSEONG RIEUL-THIEUTH */
    Codepair { ucs: '\u{11b5}', keysym: 0x0ee1 }, /*        Hangul_J_RieulPhieuf ᆵ HANGUL JONGSEONG RIEUL-PHIEUPH */
    Codepair { ucs: '\u{11b6}', keysym: 0x0ee2 }, /*         Hangul_J_RieulHieuh ᆶ HANGUL JONGSEONG RIEUL-HIEUH */
    Codepair { ucs: '\u{11b7}', keysym: 0x0ee3 }, /*              Hangul_J_Mieum ᆷ HANGUL JONGSEONG MIEUM */
    Codepair { ucs: '\u{11b8}', keysym: 0x0ee4 }, /*              Hangul_J_Pieub ᆸ HANGUL JONGSEONG PIEUP */
    Codepair { ucs: '\u{11b9}', keysym: 0x0ee5 }, /*          Hangul_J_PieubSios ᆹ HANGUL JONGSEONG PIEUP-SIOS */
    Codepair { ucs: '\u{11ba}', keysym: 0x0ee6 }, /*               Hangul_J_Sios ᆺ HANGUL JONGSEONG SIOS */
    Codepair { ucs: '\u{11bb}', keysym: 0x0ee7 }, /*          Hangul_J_SsangSios ᆻ HANGUL JONGSEONG SSANGSIOS */
    Codepair { ucs: '\u{11bc}', keysym: 0x0ee8 }, /*              Hangul_J_Ieung ᆼ HANGUL JONGSEONG IEUNG */
    Codepair { ucs: '\u{11bd}', keysym: 0x0ee9 }, /*              Hangul_J_Jieuj ᆽ HANGUL JONGSEONG CIEUC */
    Codepair { ucs: '\u{11be}', keysym: 0x0eea }, /*              Hangul_J_Cieuc ᆾ HANGUL JONGSEONG CHIEUCH */
    Codepair { ucs: '\u{11bf}', keysym: 0x0eeb }, /*             Hangul_J_Khieuq ᆿ HANGUL JONGSEONG KHIEUKH */
    Codepair { ucs: '\u{11c0}', keysym: 0x0eec }, /*              Hangul_J_Tieut ᇀ HANGUL JONGSEONG THIEUTH */
    Codepair { ucs: '\u{11c1}', keysym: 0x0eed }, /*             Hangul_J_Phieuf ᇁ HANGUL JONGSEONG PHIEUPH */
    Codepair { ucs: '\u{11c2}', keysym: 0x0eee }, /*              Hangul_J_Hieuh ᇂ HANGUL JONGSEONG HIEUH */
    Codepair { ucs: '\u{11eb}', keysym: 0x0ef8 }, /*            Hangul_J_PanSios ᇫ HANGUL JONGSEONG PANSIOS */
    Codepair { ucs: '\u{11f0}', keysym: 0x0ef9 }, /*  Hangul_J_KkogjiDalrinIeung ᇰ HANGUL JONGSEONG YESIEUNG */
    Codepair { ucs: '\u{11f9}', keysym: 0x0efa }, /*        Hangul_J_YeorinHieuh ᇹ HANGUL JONGSEONG YEORINHIEUH */
    Codepair { ucs: '\u{2002}', keysym: 0x0aa2 }, /*                     enspace   EN SPACE */
    Codepair { ucs: '\u{2003}', keysym: 0x0aa1 }, /*                     emspace   EM SPACE */
    Codepair { ucs: '\u{2004}', keysym: 0x0aa3 }, /*                    em3space   THREE-PER-EM SPACE */
    Codepair { ucs: '\u{2005}', keysym: 0x0aa4 }, /*                    em4space   FOUR-PER-EM SPACE */
    Codepair { ucs: '\u{2007}', keysym: 0x0aa5 }, /*                  digitspace   FIGURE SPACE */
    Codepair { ucs: '\u{2008}', keysym: 0x0aa6 }, /*                  punctspace   PUNCTUATION SPACE */
    Codepair { ucs: '\u{2009}', keysym: 0x0aa7 }, /*                   thinspace   THIN SPACE */
    Codepair { ucs: '\u{200a}', keysym: 0x0aa8 }, /*                   hairspace   HAIR SPACE */
    Codepair { ucs: '\u{2012}', keysym: 0x0abb }, /*                     figdash ‒ FIGURE DASH */
    Codepair { ucs: '\u{2013}', keysym: 0x0aaa }, /*                      endash – EN DASH */
    Codepair { ucs: '\u{2014}', keysym: 0x0aa9 }, /*                      emdash — EM DASH */
    Codepair { ucs: '\u{2015}', keysym: 0x07af }, /*              Greek_horizbar ― HORIZONTAL BAR */
    Codepair { ucs: '\u{2017}', keysym: 0x0cdf }, /*        hebrew_doublelowline ‗ DOUBLE LOW LINE */
    Codepair { ucs: '\u{2018}', keysym: 0x0ad0 }, /*         leftsinglequotemark ‘ LEFT SINGLE QUOTATION MARK */
    Codepair { ucs: '\u{2019}', keysym: 0x0ad1 }, /*        rightsinglequotemark ’ RIGHT SINGLE QUOTATION MARK */
    Codepair { ucs: '\u{201a}', keysym: 0x0afd }, /*          singlelowquotemark ‚ SINGLE LOW-9 QUOTATION MARK */
    Codepair { ucs: '\u{201c}', keysym: 0x0ad2 }, /*         leftdoublequotemark “ LEFT DOUBLE QUOTATION MARK */
    Codepair { ucs: '\u{201d}', keysym: 0x0ad3 }, /*        rightdoublequotemark ” RIGHT DOUBLE QUOTATION MARK */
    Codepair { ucs: '\u{201e}', keysym: 0x0afe }, /*          doublelowquotemark „ DOUBLE LOW-9 QUOTATION MARK */
    Codepair { ucs: '\u{2020}', keysym: 0x0af1 }, /*                      dagger † DAGGER */
    Codepair { ucs: '\u{2021}', keysym: 0x0af2 }, /*                doubledagger ‡ DOUBLE DAGGER */
    Codepair { ucs: '\u{2022}', keysym: 0x0ae6 }, /*          enfilledcircbullet • BULLET */
    Codepair { ucs: '\u{2025}', keysym: 0x0aaf }, /*             doubbaselinedot ‥ TWO DOT LEADER */
    Codepair { ucs: '\u{2026}', keysym: 0x0aae }, /*                    ellipsis … HORIZONTAL ELLIPSIS */
    Codepair { ucs: '\u{2030}', keysym: 0x0ad5 }, /*                    permille ‰ PER MILLE SIGN */
    Codepair { ucs: '\u{2032}', keysym: 0x0ad6 }, /*                     minutes ′ PRIME */
    Codepair { ucs: '\u{2033}', keysym: 0x0ad7 }, /*                     seconds ″ DOUBLE PRIME */
    Codepair { ucs: '\u{2038}', keysym: 0x0afc }, /*                       caret ‸ CARET */
    Codepair { ucs: '\u{203e}', keysym: 0x047e }, /*                    overline ‾ OVERLINE */
    Codepair { ucs: '\u{20a0}', keysym: 0x20a0 }, /*                     EcuSign ₠ EURO-CURRENCY SIGN */
    Codepair { ucs: '\u{20a1}', keysym: 0x20a1 }, /*                   ColonSign ₡ COLON SIGN */
    Codepair { ucs: '\u{20a2}', keysym: 0x20a2 }, /*                CruzeiroSign ₢ CRUZEIRO SIGN */
    Codepair { ucs: '\u{20a3}', keysym: 0x20a3 }, /*                  FFrancSign ₣ FRENCH FRANC SIGN */
    Codepair { ucs: '\u{20a4}', keysym: 0x20a4 }, /*                    LiraSign ₤ LIRA SIGN */
    Codepair { ucs: '\u{20a5}', keysym: 0x20a5 }, /*                    MillSign ₥ MILL SIGN */
    Codepair { ucs: '\u{20a6}', keysym: 0x20a6 }, /*                   NairaSign ₦ NAIRA SIGN */
    Codepair { ucs: '\u{20a7}', keysym: 0x20a7 }, /*                  PesetaSign ₧ PESETA SIGN */
    Codepair { ucs: '\u{20a8}', keysym: 0x20a8 }, /*                   RupeeSign ₨ RUPEE SIGN */
    Codepair { ucs: '\u{20a9}', keysym: 0x0eff }, /*                  Korean_Won ₩ WON SIGN */
    Codepair { ucs: '\u{20a9}', keysym: 0x20a9 }, /*                     WonSign ₩ WON SIGN */
    Codepair { ucs: '\u{20aa}', keysym: 0x20aa }, /*               NewSheqelSign ₪ NEW SHEQEL SIGN */
    Codepair { ucs: '\u{20ab}', keysym: 0x20ab }, /*                    DongSign ₫ DONG SIGN */
    Codepair { ucs: '\u{20ac}', keysym: 0x13a4 }, /*                        Euro € EURO SIGN */
    Codepair { ucs: '\u{20ac}', keysym: 0x20ac }, /*                    EuroSign € EURO SIGN */
    Codepair { ucs: '\u{2105}', keysym: 0x0ab8 }, /*                      careof ℅ CARE OF */
    Codepair { ucs: '\u{2116}', keysym: 0x06b0 }, /*                  numerosign № NUMERO SIGN */
    Codepair { ucs: '\u{2117}', keysym: 0x0afb }, /*         phonographcopyright ℗ SOUND RECORDING COPYRIGHT */
    Codepair { ucs: '\u{211e}', keysym: 0x0ad4 }, /*                prescription ℞ PRESCRIPTION TAKE */
    Codepair { ucs: '\u{2122}', keysym: 0x0ac9 }, /*                   trademark ™ TRADE MARK SIGN */
    Codepair { ucs: '\u{2153}', keysym: 0x0ab0 }, /*                    onethird ⅓ VULGAR FRACTION ONE THIRD */
    Codepair { ucs: '\u{2154}', keysym: 0x0ab1 }, /*                   twothirds ⅔ VULGAR FRACTION TWO THIRDS */
    Codepair { ucs: '\u{2155}', keysym: 0x0ab2 }, /*                    onefifth ⅕ VULGAR FRACTION ONE FIFTH */
    Codepair { ucs: '\u{2156}', keysym: 0x0ab3 }, /*                   twofifths ⅖ VULGAR FRACTION TWO FIFTHS */
    Codepair { ucs: '\u{2157}', keysym: 0x0ab4 }, /*                 threefifths ⅗ VULGAR FRACTION THREE FIFTHS */
    Codepair { ucs: '\u{2158}', keysym: 0x0ab5 }, /*                  fourfifths ⅘ VULGAR FRACTION FOUR FIFTHS */
    Codepair { ucs: '\u{2159}', keysym: 0x0ab6 }, /*                    onesixth ⅙ VULGAR FRACTION ONE SIXTH */
    Codepair { ucs: '\u{215a}', keysym: 0x0ab7 }, /*                  fivesixths ⅚ VULGAR FRACTION FIVE SIXTHS */
    Codepair { ucs: '\u{215b}', keysym: 0x0ac3 }, /*                   oneeighth ⅛ VULGAR FRACTION ONE EIGHTH */
    Codepair { ucs: '\u{215c}', keysym: 0x0ac4 }, /*                threeeighths ⅜ VULGAR FRACTION THREE EIGHTHS */
    Codepair { ucs: '\u{215d}', keysym: 0x0ac5 }, /*                 fiveeighths ⅝ VULGAR FRACTION FIVE EIGHTHS */
    Codepair { ucs: '\u{215e}', keysym: 0x0ac6 }, /*                seveneighths ⅞ VULGAR FRACTION SEVEN EIGHTHS */
    Codepair { ucs: '\u{2190}', keysym: 0x08fb }, /*                   leftarrow ← LEFTWARDS ARROW */
    Codepair { ucs: '\u{2191}', keysym: 0x08fc }, /*                     uparrow ↑ UPWARDS ARROW */
    Codepair { ucs: '\u{2192}', keysym: 0x08fd }, /*                  rightarrow → RIGHTWARDS ARROW */
    Codepair { ucs: '\u{2193}', keysym: 0x08fe }, /*                   downarrow ↓ DOWNWARDS ARROW */
    Codepair { ucs: '\u{21d2}', keysym: 0x08ce }, /*                     implies ⇒ RIGHTWARDS DOUBLE ARROW */
    Codepair { ucs: '\u{21d4}', keysym: 0x08cd }, /*                    ifonlyif ⇔ LEFT RIGHT DOUBLE ARROW */
    Codepair { ucs: '\u{2202}', keysym: 0x08ef }, /*           partialderivative ∂ PARTIAL DIFFERENTIAL */
    Codepair { ucs: '\u{2207}', keysym: 0x08c5 }, /*                       nabla ∇ NABLA */
    Codepair { ucs: '\u{2218}', keysym: 0x0bca }, /*                         jot ∘ RING OPERATOR */
    Codepair { ucs: '\u{221a}', keysym: 0x08d6 }, /*                     radical √ SQUARE ROOT */
    Codepair { ucs: '\u{221d}', keysym: 0x08c1 }, /*                   variation ∝ PROPORTIONAL TO */
    Codepair { ucs: '\u{221e}', keysym: 0x08c2 }, /*                    infinity ∞ INFINITY */
    Codepair { ucs: '\u{2227}', keysym: 0x08de }, /*                  logicaland ∧ LOGICAL AND */
    Codepair { ucs: '\u{2227}', keysym: 0x0ba9 }, /*                     upcaret ∧ LOGICAL AND */
    Codepair { ucs: '\u{2228}', keysym: 0x08df }, /*                   logicalor ∨ LOGICAL OR */
    Codepair { ucs: '\u{2228}', keysym: 0x0ba8 }, /*                   downcaret ∨ LOGICAL OR */
    Codepair { ucs: '\u{2229}', keysym: 0x08dc }, /*                intersection ∩ INTERSECTION */
    Codepair { ucs: '\u{2229}', keysym: 0x0bc3 }, /*                      upshoe ∩ INTERSECTION */
    Codepair { ucs: '\u{222a}', keysym: 0x08dd }, /*                       union ∪ UNION */
    Codepair { ucs: '\u{222a}', keysym: 0x0bd6 }, /*                    downshoe ∪ UNION */
    Codepair { ucs: '\u{222b}', keysym: 0x08bf }, /*                    integral ∫ INTEGRAL */
    Codepair { ucs: '\u{2234}', keysym: 0x08c0 }, /*                   therefore ∴ THEREFORE */
    Codepair { ucs: '\u{223c}', keysym: 0x08c8 }, /*                 approximate ∼ TILDE OPERATOR */
    Codepair { ucs: '\u{2243}', keysym: 0x08c9 }, /*                similarequal ≃ ASYMPTOTICALLY EQUAL TO */
    Codepair { ucs: '\u{2260}', keysym: 0x08bd }, /*                    notequal ≠ NOT EQUAL TO */
    Codepair { ucs: '\u{2261}', keysym: 0x08cf }, /*                   identical ≡ IDENTICAL TO */
    Codepair { ucs: '\u{2264}', keysym: 0x08bc }, /*               lessthanequal ≤ LESS-THAN OR EQUAL TO */
    Codepair { ucs: '\u{2265}', keysym: 0x08be }, /*            greaterthanequal ≥ GREATER-THAN OR EQUAL TO */
    Codepair { ucs: '\u{2282}', keysym: 0x08da }, /*                  includedin ⊂ SUBSET OF */
    Codepair { ucs: '\u{2282}', keysym: 0x0bda }, /*                    leftshoe ⊂ SUBSET OF */
    Codepair { ucs: '\u{2283}', keysym: 0x08db }, /*                    includes ⊃ SUPERSET OF */
    Codepair { ucs: '\u{2283}', keysym: 0x0bd8 }, /*                   rightshoe ⊃ SUPERSET OF */
    Codepair { ucs: '\u{22a2}', keysym: 0x0bdc }, /*                    lefttack ⊢ RIGHT TACK */
    Codepair { ucs: '\u{22a3}', keysym: 0x0bfc }, /*                   righttack ⊣ LEFT TACK */
    Codepair { ucs: '\u{22a4}', keysym: 0x0bc2 }, /*                    downtack ⊤ DOWN TACK */
    Codepair { ucs: '\u{22a5}', keysym: 0x0bce }, /*                      uptack ⊥ UP TACK */
    Codepair { ucs: '\u{2308}', keysym: 0x0bd3 }, /*                     upstile ⌈ LEFT CEILING */
    Codepair { ucs: '\u{230a}', keysym: 0x0bc4 }, /*                   downstile ⌊ LEFT FLOOR */
    Codepair { ucs: '\u{2315}', keysym: 0x0afa }, /*           telephonerecorder ⌕ TELEPHONE RECORDER */
    Codepair { ucs: '\u{2320}', keysym: 0x08a4 }, /*                 topintegral ⌠ TOP HALF INTEGRAL */
    Codepair { ucs: '\u{2321}', keysym: 0x08a5 }, /*                 botintegral ⌡ BOTTOM HALF INTEGRAL */
    Codepair { ucs: '\u{2395}', keysym: 0x0bcc }, /*                        quad ⎕ APL FUNCTIONAL SYMBOL QUAD (Unicode 3.0) */
    Codepair { ucs: '\u{239b}', keysym: 0x08ab }, /*               topleftparens ⎛ ??? */
    Codepair { ucs: '\u{239d}', keysym: 0x08ac }, /*               botleftparens ⎝ ??? */
    Codepair { ucs: '\u{239e}', keysym: 0x08ad }, /*              toprightparens ⎞ ??? */
    Codepair { ucs: '\u{23a0}', keysym: 0x08ae }, /*              botrightparens ⎠ ??? */
    Codepair { ucs: '\u{23a1}', keysym: 0x08a7 }, /*            topleftsqbracket ⎡ ??? */
    Codepair { ucs: '\u{23a3}', keysym: 0x08a8 }, /*            botleftsqbracket ⎣ ??? */
    Codepair { ucs: '\u{23a4}', keysym: 0x08a9 }, /*           toprightsqbracket ⎤ ??? */
    Codepair { ucs: '\u{23a6}', keysym: 0x08aa }, /*           botrightsqbracket ⎦ ??? */
    Codepair { ucs: '\u{23a8}', keysym: 0x08af }, /*        leftmiddlecurlybrace ⎨ ??? */
    Codepair { ucs: '\u{23ac}', keysym: 0x08b0 }, /*       rightmiddlecurlybrace ⎬ ??? */
    Codepair { ucs: '\u{23b7}', keysym: 0x08a1 }, /*                 leftradical ⎷ ??? */
    Codepair { ucs: '\u{23ba}', keysym: 0x09ef }, /*              horizlinescan1 ⎺ HORIZONTAL SCAN LINE-1 (Unicode 3.2 draft) */
    Codepair { ucs: '\u{23bb}', keysym: 0x09f0 }, /*              horizlinescan3 ⎻ HORIZONTAL SCAN LINE-3 (Unicode 3.2 draft) */
    Codepair { ucs: '\u{23bc}', keysym: 0x09f2 }, /*              horizlinescan7 ⎼ HORIZONTAL SCAN LINE-7 (Unicode 3.2 draft) */
    Codepair { ucs: '\u{23bd}', keysym: 0x09f3 }, /*              horizlinescan9 ⎽ HORIZONTAL SCAN LINE-9 (Unicode 3.2 draft) */
    Codepair { ucs: '\u{2409}', keysym: 0x09e2 }, /*                          ht ␉ SYMBOL FOR HORIZONTAL TABULATION */
    Codepair { ucs: '\u{240a}', keysym: 0x09e5 }, /*                          lf ␊ SYMBOL FOR LINE FEED */
    Codepair { ucs: '\u{240b}', keysym: 0x09e9 }, /*                          vt ␋ SYMBOL FOR VERTICAL TABULATION */
    Codepair { ucs: '\u{240c}', keysym: 0x09e3 }, /*                          ff ␌ SYMBOL FOR FORM FEED */
    Codepair { ucs: '\u{240d}', keysym: 0x09e4 }, /*                          cr ␍ SYMBOL FOR CARRIAGE RETURN */
    Codepair { ucs: '\u{2423}', keysym: 0x0aac }, /*                 signifblank ␣ OPEN BOX */
    Codepair { ucs: '\u{2424}', keysym: 0x09e8 }, /*                          nl ␤ SYMBOL FOR NEWLINE */
    Codepair { ucs: '\u{2500}', keysym: 0x08a3 }, /*              horizconnector ─ BOX DRAWINGS LIGHT HORIZONTAL */
    Codepair { ucs: '\u{2500}', keysym: 0x09f1 }, /*              horizlinescan5 ─ BOX DRAWINGS LIGHT HORIZONTAL */
    Codepair { ucs: '\u{2502}', keysym: 0x08a6 }, /*               vertconnector │ BOX DRAWINGS LIGHT VERTICAL */
    Codepair { ucs: '\u{2502}', keysym: 0x09f8 }, /*                     vertbar │ BOX DRAWINGS LIGHT VERTICAL */
    Codepair { ucs: '\u{250c}', keysym: 0x08a2 }, /*              topleftradical ┌ BOX DRAWINGS LIGHT DOWN AND RIGHT */
    Codepair { ucs: '\u{250c}', keysym: 0x09ec }, /*                upleftcorner ┌ BOX DRAWINGS LIGHT DOWN AND RIGHT */
    Codepair { ucs: '\u{2510}', keysym: 0x09eb }, /*               uprightcorner ┐ BOX DRAWINGS LIGHT DOWN AND LEFT */
    Codepair { ucs: '\u{2514}', keysym: 0x09ed }, /*               lowleftcorner └ BOX DRAWINGS LIGHT UP AND RIGHT */
    Codepair { ucs: '\u{2518}', keysym: 0x09ea }, /*              lowrightcorner ┘ BOX DRAWINGS LIGHT UP AND LEFT */
    Codepair { ucs: '\u{251c}', keysym: 0x09f4 }, /*                       leftt ├ BOX DRAWINGS LIGHT VERTICAL AND RIGHT */
    Codepair { ucs: '\u{2524}', keysym: 0x09f5 }, /*                      rightt ┤ BOX DRAWINGS LIGHT VERTICAL AND LEFT */
    Codepair { ucs: '\u{252c}', keysym: 0x09f7 }, /*                        topt ┬ BOX DRAWINGS LIGHT DOWN AND HORIZONTAL */
    Codepair { ucs: '\u{2534}', keysym: 0x09f6 }, /*                        bott ┴ BOX DRAWINGS LIGHT UP AND HORIZONTAL */
    Codepair { ucs: '\u{253c}', keysym: 0x09ee }, /*               crossinglines ┼ BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL */
    Codepair { ucs: '\u{2592}', keysym: 0x09e1 }, /*                checkerboard ▒ MEDIUM SHADE */
    Codepair { ucs: '\u{25aa}', keysym: 0x0ae7 }, /*            enfilledsqbullet ▪ BLACK SMALL SQUARE */
    Codepair { ucs: '\u{25ab}', keysym: 0x0ae1 }, /*          enopensquarebullet ▫ WHITE SMALL SQUARE */
    Codepair { ucs: '\u{25ac}', keysym: 0x0adb }, /*            filledrectbullet ▬ BLACK RECTANGLE */
    Codepair { ucs: '\u{25ad}', keysym: 0x0ae2 }, /*              openrectbullet ▭ WHITE RECTANGLE */
    Codepair { ucs: '\u{25ae}', keysym: 0x0adf }, /*                emfilledrect ▮ BLACK VERTICAL RECTANGLE */
    Codepair { ucs: '\u{25af}', keysym: 0x0acf }, /*             emopenrectangle ▯ WHITE VERTICAL RECTANGLE */
    Codepair { ucs: '\u{25b2}', keysym: 0x0ae8 }, /*           filledtribulletup ▲ BLACK UP-POINTING TRIANGLE */
    Codepair { ucs: '\u{25b3}', keysym: 0x0ae3 }, /*             opentribulletup △ WHITE UP-POINTING TRIANGLE */
    Codepair { ucs: '\u{25b6}', keysym: 0x0add }, /*        filledrighttribullet ▶ BLACK RIGHT-POINTING TRIANGLE */
    Codepair { ucs: '\u{25b7}', keysym: 0x0acd }, /*           rightopentriangle ▷ WHITE RIGHT-POINTING TRIANGLE */
    Codepair { ucs: '\u{25bc}', keysym: 0x0ae9 }, /*         filledtribulletdown ▼ BLACK DOWN-POINTING TRIANGLE */
    Codepair { ucs: '\u{25bd}', keysym: 0x0ae4 }, /*           opentribulletdown ▽ WHITE DOWN-POINTING TRIANGLE */
    Codepair { ucs: '\u{25c0}', keysym: 0x0adc }, /*         filledlefttribullet ◀ BLACK LEFT-POINTING TRIANGLE */
    Codepair { ucs: '\u{25c1}', keysym: 0x0acc }, /*            leftopentriangle ◁ WHITE LEFT-POINTING TRIANGLE */
    Codepair { ucs: '\u{25c6}', keysym: 0x09e0 }, /*                soliddiamond ◆ BLACK DIAMOND */
    Codepair { ucs: '\u{25cb}', keysym: 0x0ace }, /*                emopencircle ○ WHITE CIRCLE */
    Codepair { ucs: '\u{25cb}', keysym: 0x0bcf }, /*                      circle ○ WHITE CIRCLE */
    Codepair { ucs: '\u{25cf}', keysym: 0x0ade }, /*              emfilledcircle ● BLACK CIRCLE */
    Codepair { ucs: '\u{25e6}', keysym: 0x0ae0 }, /*            enopencircbullet ◦ WHITE BULLET */
    Codepair { ucs: '\u{2606}', keysym: 0x0ae5 }, /*                    openstar ☆ WHITE STAR */
    Codepair { ucs: '\u{260e}', keysym: 0x0af9 }, /*                   telephone ☎ BLACK TELEPHONE */
    Codepair { ucs: '\u{2613}', keysym: 0x0aca }, /*               signaturemark ☓ SALTIRE */
    Codepair { ucs: '\u{261c}', keysym: 0x0aea }, /*                 leftpointer ☜ WHITE LEFT POINTING INDEX */
    Codepair { ucs: '\u{261e}', keysym: 0x0aeb }, /*                rightpointer ☞ WHITE RIGHT POINTING INDEX */
    Codepair { ucs: '\u{2640}', keysym: 0x0af8 }, /*                femalesymbol ♀ FEMALE SIGN */
    Codepair { ucs: '\u{2642}', keysym: 0x0af7 }, /*                  malesymbol ♂ MALE SIGN */
    Codepair { ucs: '\u{2663}', keysym: 0x0aec }, /*                        club ♣ BLACK CLUB SUIT */
    Codepair { ucs: '\u{2665}', keysym: 0x0aee }, /*                       heart ♥ BLACK HEART SUIT */
    Codepair { ucs: '\u{2666}', keysym: 0x0aed }, /*                     diamond ♦ BLACK DIAMOND SUIT */
    Codepair { ucs: '\u{266d}', keysym: 0x0af6 }, /*                 musicalflat ♭ MUSIC FLAT SIGN */
    Codepair { ucs: '\u{266f}', keysym: 0x0af5 }, /*                musicalsharp ♯ MUSIC SHARP SIGN */
    Codepair { ucs: '\u{2713}', keysym: 0x0af3 }, /*                   checkmark ✓ CHECK MARK */
    Codepair { ucs: '\u{2717}', keysym: 0x0af4 }, /*                 ballotcross ✗ BALLOT X */
    Codepair { ucs: '\u{271d}', keysym: 0x0ad9 }, /*                  latincross ✝ LATIN CROSS */
    Codepair { ucs: '\u{2720}', keysym: 0x0af0 }, /*                maltesecross ✠ MALTESE CROSS */
    Codepair { ucs: '\u{27e8}', keysym: 0x0abc }, /*            leftanglebracket ⟨ MATHEMATICAL LEFT ANGLE BRACKET */
    Codepair { ucs: '\u{27e9}', keysym: 0x0abe }, /*           rightanglebracket ⟩ MATHEMATICAL RIGHT ANGLE BRACKET */
    Codepair { ucs: '\u{3001}', keysym: 0x04a4 }, /*                  kana_comma 、 IDEOGRAPHIC COMMA */
    Codepair { ucs: '\u{3002}', keysym: 0x04a1 }, /*               kana_fullstop 。 IDEOGRAPHIC FULL STOP */
    Codepair { ucs: '\u{300c}', keysym: 0x04a2 }, /*         kana_openingbracket 「 LEFT CORNER BRACKET */
    Codepair { ucs: '\u{300d}', keysym: 0x04a3 }, /*         kana_closingbracket 」 RIGHT CORNER BRACKET */
    Codepair { ucs: '\u{309b}', keysym: 0x04de }, /*                 voicedsound ゛ KATAKANA-HIRAGANA VOICED SOUND MARK */
    Codepair { ucs: '\u{309c}', keysym: 0x04df }, /*             semivoicedsound ゜ KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK */
    Codepair { ucs: '\u{30a1}', keysym: 0x04a7 }, /*                      kana_a ァ KATAKANA LETTER SMALL A */
    Codepair { ucs: '\u{30a2}', keysym: 0x04b1 }, /*                      kana_A ア KATAKANA LETTER A */
    Codepair { ucs: '\u{30a3}', keysym: 0x04a8 }, /*                      kana_i ィ KATAKANA LETTER SMALL I */
    Codepair { ucs: '\u{30a4}', keysym: 0x04b2 }, /*                      kana_I イ KATAKANA LETTER I */
    Codepair { ucs: '\u{30a5}', keysym: 0x04a9 }, /*                      kana_u ゥ KATAKANA LETTER SMALL U */
    Codepair { ucs: '\u{30a6}', keysym: 0x04b3 }, /*                      kana_U ウ KATAKANA LETTER U */
    Codepair { ucs: '\u{30a7}', keysym: 0x04aa }, /*                      kana_e ェ KATAKANA LETTER SMALL E */
    Codepair { ucs: '\u{30a8}', keysym: 0x04b4 }, /*                      kana_E エ KATAKANA LETTER E */
    Codepair { ucs: '\u{30a9}', keysym: 0x04ab }, /*                      kana_o ォ KATAKANA LETTER SMALL O */
    Codepair { ucs: '\u{30aa}', keysym: 0x04b5 }, /*                      kana_O オ KATAKANA LETTER O */
    Codepair { ucs: '\u{30ab}', keysym: 0x04b6 }, /*                     kana_KA カ KATAKANA LETTER KA */
    Codepair { ucs: '\u{30ad}', keysym: 0x04b7 }, /*                     kana_KI キ KATAKANA LETTER KI */
    Codepair { ucs: '\u{30af}', keysym: 0x04b8 }, /*                     kana_KU ク KATAKANA LETTER KU */
    Codepair { ucs: '\u{30b1}', keysym: 0x04b9 }, /*                     kana_KE ケ KATAKANA LETTER KE */
    Codepair { ucs: '\u{30b3}', keysym: 0x04ba }, /*                     kana_KO コ KATAKANA LETTER KO */
    Codepair { ucs: '\u{30b5}', keysym: 0x04bb }, /*                     kana_SA サ KATAKANA LETTER SA */
    Codepair { ucs: '\u{30b7}', keysym: 0x04bc }, /*                    kana_SHI シ KATAKANA LETTER SI */
    Codepair { ucs: '\u{30b9}', keysym: 0x04bd }, /*                     kana_SU ス KATAKANA LETTER SU */
    Codepair { ucs: '\u{30bb}', keysym: 0x04be }, /*                     kana_SE セ KATAKANA LETTER SE */
    Codepair { ucs: '\u{30bd}', keysym: 0x04bf }, /*                     kana_SO ソ KATAKANA LETTER SO */
    Codepair { ucs: '\u{30bf}', keysym: 0x04c0 }, /*                     kana_TA タ KATAKANA LETTER TA */
    Codepair { ucs: '\u{30c1}', keysym: 0x04c1 }, /*                    kana_CHI チ KATAKANA LETTER TI */
    Codepair { ucs: '\u{30c3}', keysym: 0x04af }, /*                    kana_tsu ッ KATAKANA LETTER SMALL TU */
    Codepair { ucs: '\u{30c4}', keysym: 0x04c2 }, /*                    kana_TSU ツ KATAKANA LETTER TU */
    Codepair { ucs: '\u{30c6}', keysym: 0x04c3 }, /*                     kana_TE テ KATAKANA LETTER TE */
    Codepair { ucs: '\u{30c8}', keysym: 0x04c4 }, /*                     kana_TO ト KATAKANA LETTER TO */
    Codepair { ucs: '\u{30ca}', keysym: 0x04c5 }, /*                     kana_NA ナ KATAKANA LETTER NA */
    Codepair { ucs: '\u{30cb}', keysym: 0x04c6 }, /*                     kana_NI ニ KATAKANA LETTER NI */
    Codepair { ucs: '\u{30cc}', keysym: 0x04c7 }, /*                     kana_NU ヌ KATAKANA LETTER NU */
    Codepair { ucs: '\u{30cd}', keysym: 0x04c8 }, /*                     kana_NE ネ KATAKANA LETTER NE */
    Codepair { ucs: '\u{30ce}', keysym: 0x04c9 }, /*                     kana_NO ノ KATAKANA LETTER NO */
    Codepair { ucs: '\u{30cf}', keysym: 0x04ca }, /*                     kana_HA ハ KATAKANA LETTER HA */
    Codepair { ucs: '\u{30d2}', keysym: 0x04cb }, /*                     kana_HI ヒ KATAKANA LETTER HI */
    Codepair { ucs: '\u{30d5}', keysym: 0x04cc }, /*                     kana_FU フ KATAKANA LETTER HU */
    Codepair { ucs: '\u{30d8}', keysym: 0x04cd }, /*                     kana_HE ヘ KATAKANA LETTER HE */
    Codepair { ucs: '\u{30db}', keysym: 0x04ce }, /*                     kana_HO ホ KATAKANA LETTER HO */
    Codepair { ucs: '\u{30de}', keysym: 0x04cf }, /*                     kana_MA マ KATAKANA LETTER MA */
    Codepair { ucs: '\u{30df}', keysym: 0x04d0 }, /*                     kana_MI ミ KATAKANA LETTER MI */
    Codepair { ucs: '\u{30e0}', keysym: 0x04d1 }, /*                     kana_MU ム KATAKANA LETTER MU */
    Codepair { ucs: '\u{30e1}', keysym: 0x04d2 }, /*                     kana_ME メ KATAKANA LETTER ME */
    Codepair { ucs: '\u{30e2}', keysym: 0x04d3 }, /*                     kana_MO モ KATAKANA LETTER MO */
    Codepair { ucs: '\u{30e3}', keysym: 0x04ac }, /*                     kana_ya ャ KATAKANA LETTER SMALL YA */
    Codepair { ucs: '\u{30e4}', keysym: 0x04d4 }, /*                     kana_YA ヤ KATAKANA LETTER YA */
    Codepair { ucs: '\u{30e5}', keysym: 0x04ad }, /*                     kana_yu ュ KATAKANA LETTER SMALL YU */
    Codepair { ucs: '\u{30e6}', keysym: 0x04d5 }, /*                     kana_YU ユ KATAKANA LETTER YU */
    Codepair { ucs: '\u{30e7}', keysym: 0x04ae }, /*                     kana_yo ョ KATAKANA LETTER SMALL YO */
    Codepair { ucs: '\u{30e8}', keysym: 0x04d6 }, /*                     kana_YO ヨ KATAKANA LETTER YO */
    Codepair { ucs: '\u{30e9}', keysym: 0x04d7 }, /*                     kana_RA ラ KATAKANA LETTER RA */
    Codepair { ucs: '\u{30ea}', keysym: 0x04d8 }, /*                     kana_RI リ KATAKANA LETTER RI */
    Codepair { ucs: '\u{30eb}', keysym: 0x04d9 }, /*                     kana_RU ル KATAKANA LETTER RU */
    Codepair { ucs: '\u{30ec}', keysym: 0x04da }, /*                     kana_RE レ KATAKANA LETTER RE */
    Codepair { ucs: '\u{30ed}', keysym: 0x04db }, /*                     kana_RO ロ KATAKANA LETTER RO */
    Codepair { ucs: '\u{30ef}', keysym: 0x04dc }, /*                     kana_WA ワ KATAKANA LETTER WA */
    Codepair { ucs: '\u{30f2}', keysym: 0x04a6 }, /*                     kana_WO ヲ KATAKANA LETTER WO */
    Codepair { ucs: '\u{30f3}', keysym: 0x04dd }, /*                      kana_N ン KATAKANA LETTER N */
    Codepair { ucs: '\u{30fb}', keysym: 0x04a5 }, /*            kana_conjunctive ・ KATAKANA MIDDLE DOT */
    Codepair { ucs: '\u{30fc}', keysym: 0x04b0 }, /*              prolongedsound ー KATAKANA-HIRAGANA PROLONGED SOUND MARK */
    Codepair { ucs: '\u{3131}', keysym: 0x0ea1 }, /*               Hangul_Kiyeog ㄱ HANGUL LETTER KIYEOK */
    Codepair { ucs: '\u{3132}', keysym: 0x0ea2 }, /*          Hangul_SsangKiyeog ㄲ HANGUL LETTER SSANGKIYEOK */
    Codepair { ucs: '\u{3133}', keysym: 0x0ea3 }, /*           Hangul_KiyeogSios ㄳ HANGUL LETTER KIYEOK-SIOS */
    Codepair { ucs: '\u{3134}', keysym: 0x0ea4 }, /*                Hangul_Nieun ㄴ HANGUL LETTER NIEUN */
    Codepair { ucs: '\u{3135}', keysym: 0x0ea5 }, /*           Hangul_NieunJieuj ㄵ HANGUL LETTER NIEUN-CIEUC */
    Codepair { ucs: '\u{3136}', keysym: 0x0ea6 }, /*           Hangul_NieunHieuh ㄶ HANGUL LETTER NIEUN-HIEUH */
    Codepair { ucs: '\u{3137}', keysym: 0x0ea7 }, /*               Hangul_Dikeud ㄷ HANGUL LETTER TIKEUT */
    Codepair { ucs: '\u{3138}', keysym: 0x0ea8 }, /*          Hangul_SsangDikeud ㄸ HANGUL LETTER SSANGTIKEUT */
    Codepair { ucs: '\u{3139}', keysym: 0x0ea9 }, /*                Hangul_Rieul ㄹ HANGUL LETTER RIEUL */
    Codepair { ucs: '\u{313a}', keysym: 0x0eaa }, /*          Hangul_RieulKiyeog ㄺ HANGUL LETTER RIEUL-KIYEOK */
    Codepair { ucs: '\u{313b}', keysym: 0x0eab }, /*           Hangul_RieulMieum ㄻ HANGUL LETTER RIEUL-MIEUM */
    Codepair { ucs: '\u{313c}', keysym: 0x0eac }, /*           Hangul_RieulPieub ㄼ HANGUL LETTER RIEUL-PIEUP */
    Codepair { ucs: '\u{313d}', keysym: 0x0ead }, /*            Hangul_RieulSios ㄽ HANGUL LETTER RIEUL-SIOS */
    Codepair { ucs: '\u{313e}', keysym: 0x0eae }, /*           Hangul_RieulTieut ㄾ HANGUL LETTER RIEUL-THIEUTH */
    Codepair { ucs: '\u{313f}', keysym: 0x0eaf }, /*          Hangul_RieulPhieuf ㄿ HANGUL LETTER RIEUL-PHIEUPH */
    Codepair { ucs: '\u{3140}', keysym: 0x0eb0 }, /*           Hangul_RieulHieuh ㅀ HANGUL LETTER RIEUL-HIEUH */
    Codepair { ucs: '\u{3141}', keysym: 0x0eb1 }, /*                Hangul_Mieum ㅁ HANGUL LETTER MIEUM */
    Codepair { ucs: '\u{3142}', keysym: 0x0eb2 }, /*                Hangul_Pieub ㅂ HANGUL LETTER PIEUP */
    Codepair { ucs: '\u{3143}', keysym: 0x0eb3 }, /*           Hangul_SsangPieub ㅃ HANGUL LETTER SSANGPIEUP */
    Codepair { ucs: '\u{3144}', keysym: 0x0eb4 }, /*            Hangul_PieubSios ㅄ HANGUL LETTER PIEUP-SIOS */
    Codepair { ucs: '\u{3145}', keysym: 0x0eb5 }, /*                 Hangul_Sios ㅅ HANGUL LETTER SIOS */
    Codepair { ucs: '\u{3146}', keysym: 0x0eb6 }, /*            Hangul_SsangSios ㅆ HANGUL LETTER SSANGSIOS */
    Codepair { ucs: '\u{3147}', keysym: 0x0eb7 }, /*                Hangul_Ieung ㅇ HANGUL LETTER IEUNG */
    Codepair { ucs: '\u{3148}', keysym: 0x0eb8 }, /*                Hangul_Jieuj ㅈ HANGUL LETTER CIEUC */
    Codepair { ucs: '\u{3149}', keysym: 0x0eb9 }, /*           Hangul_SsangJieuj ㅉ HANGUL LETTER SSANGCIEUC */
    Codepair { ucs: '\u{314a}', keysym: 0x0eba }, /*                Hangul_Cieuc ㅊ HANGUL LETTER CHIEUCH */
    Codepair { ucs: '\u{314b}', keysym: 0x0ebb }, /*               Hangul_Khieuq ㅋ HANGUL LETTER KHIEUKH */
    Codepair { ucs: '\u{314c}', keysym: 0x0ebc }, /*                Hangul_Tieut ㅌ HANGUL LETTER THIEUTH */
    Codepair { ucs: '\u{314d}', keysym: 0x0ebd }, /*               Hangul_Phieuf ㅍ HANGUL LETTER PHIEUPH */
    Codepair { ucs: '\u{314e}', keysym: 0x0ebe }, /*                Hangul_Hieuh ㅎ HANGUL LETTER HIEUH */
    Codepair { ucs: '\u{314f}', keysym: 0x0ebf }, /*                    Hangul_A ㅏ HANGUL LETTER A */
    Codepair { ucs: '\u{3150}', keysym: 0x0ec0 }, /*                   Hangul_AE ㅐ HANGUL LETTER AE */
    Codepair { ucs: '\u{3151}', keysym: 0x0ec1 }, /*                   Hangul_YA ㅑ HANGUL LETTER YA */
    Codepair { ucs: '\u{3152}', keysym: 0x0ec2 }, /*                  Hangul_YAE ㅒ HANGUL LETTER YAE */
    Codepair { ucs: '\u{3153}', keysym: 0x0ec3 }, /*                   Hangul_EO ㅓ HANGUL LETTER EO */
    Codepair { ucs: '\u{3154}', keysym: 0x0ec4 }, /*                    Hangul_E ㅔ HANGUL LETTER E */
    Codepair { ucs: '\u{3155}', keysym: 0x0ec5 }, /*                  Hangul_YEO ㅕ HANGUL LETTER YEO */
    Codepair { ucs: '\u{3156}', keysym: 0x0ec6 }, /*                   Hangul_YE ㅖ HANGUL LETTER YE */
    Codepair { ucs: '\u{3157}', keysym: 0x0ec7 }, /*                    Hangul_O ㅗ HANGUL LETTER O */
    Codepair { ucs: '\u{3158}', keysym: 0x0ec8 }, /*                   Hangul_WA ㅘ HANGUL LETTER WA */
    Codepair { ucs: '\u{3159}', keysym: 0x0ec9 }, /*                  Hangul_WAE ㅙ HANGUL LETTER WAE */
    Codepair { ucs: '\u{315a}', keysym: 0x0eca }, /*                   Hangul_OE ㅚ HANGUL LETTER OE */
    Codepair { ucs: '\u{315b}', keysym: 0x0ecb }, /*                   Hangul_YO ㅛ HANGUL LETTER YO */
    Codepair { ucs: '\u{315c}', keysym: 0x0ecc }, /*                    Hangul_U ㅜ HANGUL LETTER U */
    Codepair { ucs: '\u{315d}', keysym: 0x0ecd }, /*                  Hangul_WEO ㅝ HANGUL LETTER WEO */
    Codepair { ucs: '\u{315e}', keysym: 0x0ece }, /*                   Hangul_WE ㅞ HANGUL LETTER WE */
    Codepair { ucs: '\u{315f}', keysym: 0x0ecf }, /*                   Hangul_WI ㅟ HANGUL LETTER WI */
    Codepair { ucs: '\u{3160}', keysym: 0x0ed0 }, /*                   Hangul_YU ㅠ HANGUL LETTER YU */
    Codepair { ucs: '\u{3161}', keysym: 0x0ed1 }, /*                   Hangul_EU ㅡ HANGUL LETTER EU */
    Codepair { ucs: '\u{3162}', keysym: 0x0ed2 }, /*                   Hangul_YI ㅢ HANGUL LETTER YI */
    Codepair { ucs: '\u{3163}', keysym: 0x0ed3 }, /*                    Hangul_I ㅣ HANGUL LETTER I */
    Codepair { ucs: '\u{316d}', keysym: 0x0eef }, /*     Hangul_RieulYeorinHieuh ㅭ HANGUL LETTER RIEUL-YEORINHIEUH */
    Codepair { ucs: '\u{3171}', keysym: 0x0ef0 }, /*    Hangul_SunkyeongeumMieum ㅱ HANGUL LETTER KAPYEOUNMIEUM */
    Codepair { ucs: '\u{3178}', keysym: 0x0ef1 }, /*    Hangul_SunkyeongeumPieub ㅸ HANGUL LETTER KAPYEOUNPIEUP */
    Codepair { ucs: '\u{317f}', keysym: 0x0ef2 }, /*              Hangul_PanSios ㅿ HANGUL LETTER PANSIOS */
    Codepair { ucs: '\u{3184}', keysym: 0x0ef4 }, /*   Hangul_SunkyeongeumPhieuf ㆄ HANGUL LETTER KAPYEOUNPHIEUPH */
    Codepair { ucs: '\u{3186}', keysym: 0x0ef5 }, /*          Hangul_YeorinHieuh ㆆ HANGUL LETTER YEORINHIEUH */
    Codepair { ucs: '\u{318d}', keysym: 0x0ef6 }, /*                Hangul_AraeA ㆍ HANGUL LETTER ARAEA */
    Codepair { ucs: '\u{318e}', keysym: 0x0ef7 }, /*               Hangul_AraeAE ㆎ HANGUL LETTER ARAEAE */
];
