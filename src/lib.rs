#[macro_use] extern crate lazy_static;
use literally::hmap;




#[derive(Debug, PartialEq, Clone)]
pub enum Knownness {
    Known,
    Unknown,
    BiUnknown,
    GenUnknown,
}

//lazy_static! {
//    
//}

#[derive(Debug, PartialEq, Clone)]
pub struct SReading {
    baseform: String,
    tags: Vec<String>,
}

impl ToString for SReading {
    fn to_string(&self) -> String {
        String::from("!!!")
    }
}

fn readings_to_string(&[SReading]) -> String {
    String::from("!!!")   
}

fn subreadings_to_string(&[SReading]) -> String {
    String::from("!!!")   
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexicalUnit {
    pub lexical_unit: String,
    pub wordform: String,
    pub wordbound_blank: String,
    pub readings: Vec<Vec<SReading>>,
    pub knownness: Knownness,
}

impl ToString for LexicalUnit {
    fn to_string(&self) -> String {
        String::from("@@@")
    }
}

pub fn parse(text: &str) -> Vec<LexicalUnit> {
    vec![LexicalUnit {
        lexical_unit: "@@@".to_owned(),
        wordform: "@@@".to_owned(),
        wordbound_blank: "@@@".to_owned(),
        readings: vec![vec![]],
        knownness: Knownness::Known,
    }]
}

pub fn parse_with_text(text: &str) -> Vec<(String, LexicalUnit)> {
    vec![(String::from("!!!"), LexicalUnit {
        lexical_unit: "@@@".to_owned(),
        wordform: "@@@".to_owned(),
        wordbound_blank: "@@@".to_owned(),
        readings: vec![vec![]],
        knownness: Knownness::Known,
    })]
}

pub fn main_pos(x: &[SReading]) -> String {
    "XXX".to_owned()
}

pub fn main_pos_with_ltr(x: &[SReading]) -> String {
    "XXX".to_owned()
}

//        lexical_units = list(parse(self.s1))
//        self.assertEqual(len(lexical_units), 1)

#[cfg(test)]
mod tests {
    use super::*;
    // lazy_static! {
    //     static ref S1: &'static str = r"[\^keep<escapes>\$] \^ \$ \/ \[ \] ^x\/y\^\$\<z\>å/A\$\^B<tag><tag2>/A\/S<tag><#1-\>2>$";
    // }
    static S1: &'static str = r"[\^keep<escapes>\$] \^ \$ \/ \[ \] ^x\/y\^\$\<z\>å/A\$\^B<tag><tag2>/A\/S<tag><#1-\>2>$";
    static S2: &'static str = "^hypercholesterolemia/*hypercholesterolemia$";
    static S3: &'static str = "$^vino/vino<n><m><sg>/venir<vblex><ifi><p3><sg>$";
    static S4: &'static str = "^dímelo/decir<vblex><imp><p2><sg>+me<prn><enc><p1><mf><sg>+lo<prn><enc><p3><nt>/decir<vblex><imp><p2><sg>+me<prn><enc><p1><mf><sg>+lo<prn><enc><p3><m><sg>$";
    static S5: &'static str = r"[] [[t:b:123456]]^My/My<det><pos><sp>$ [bl] ^test/testlem<tags1><tags2>$ [\[] [\]blank] [[t:i:12asda; t:p:1abc76]]^name/name<n><sg>/name<vblex><inf>/name<vblex><pres>$";

    #[test]
    fn test_parse() {        
        let lexical_units = parse(S1);
        assert_eq!(lexical_units.len(), 1);
        let lexical_unit = &lexical_units[0];
        assert_eq!(lexical_unit.to_string(), r"x\/y\^\$\<z\>å/A\$\^B<tag><tag2>/A\/S<tag><#1-\>2>");
        let readings = &lexical_unit.readings;
        assert_eq!(readings, &vec![vec![SReading {baseform: String::from("A\\$\\^B"),
                                                  tags: vec![String::from("tag"),
                                                             String::from("tag2")]}],
                                   vec![SReading {baseform: String::from("A\\/S"),
                                                  tags: vec![String::from("tag"),
                                                             String::from("#1-\\>2")]}]]);
        assert_eq!(&lexical_unit.wordform, r"x\/y\^\$\<z\>å");
        assert_eq!(lexical_unit.knownness, Knownness::Known)
    }

    #[test]
    fn test_parse_with_text() {
        let lexical_units_with_blanks = parse_with_text(S1);
        assert_eq!(lexical_units_with_blanks.len(), 1);
        let (blank, _lexical_unit) = lexical_units_with_blanks[0].clone();
        assert_eq!(blank, r"[\^keep<escapes>\$] \^ \$ \/ \[ \] ");
    }


    #[test]
    fn test_parse_unknown() {
        let lexical_units = parse(S2);
        assert_eq!(lexical_units.len(), 1);
        assert_eq!(lexical_units[0].knownness, Knownness::Unknown);
    }
    
    #[test]
    fn test_parse_subreadings() {
        let lexical_units = parse(S4);
        assert_eq!(lexical_units.len(), 1);
        assert_eq!(&lexical_units[0].readings,
                   &vec![
                       vec![
                           SReading { baseform: String::from("decir"),
                                      tags: vec![String::from("vblex"),
                                                 String::from("imp"),
                                                 String::from("p2"),
                                                 String::from("sg")] },
                           SReading { baseform: String::from("me"),
                                      tags: vec![String::from("prn"),
                                                 String::from("enc"),
                                                 String::from("p1"),
                                                 String::from("mf"),
                                                 String::from("sg")] },
                           SReading { baseform: String::from("lo"),
                                      tags: vec![String::from("prn"),
                                                 String::from("enc"),
                                                 String::from("p3"),
                                                 String::from("nt")] }
                       ],
                       vec![
                           SReading { baseform: String::from("decir"),
                                      tags: vec![ String::from("vblex"),
                                                  String::from("imp"),
                                                  String::from("p2"),
                                                  String::from("sg")] },
                           SReading { baseform: String::from("me"),
                                      tags: vec![String::from("prn"),
                                                 String::from("enc"),
                                                 String::from("p1"),
                                                 String::from("mf"),
                                                 String::from("sg")] },
                           SReading { baseform: String::from("lo"),
                                      tags: vec![String::from("prn"),
                                                 String::from("enc"),
                                                 String::from("p3"),
                                                 String::from("m"),
                                                 String::from("sg")] },
                       ],
                   ],
        )
    }

    #[test]
    fn test_mainpos() {
        let lexical_units = parse(S4);
        assert_eq!(lexical_units.len(), 1);
        let pos = main_pos(&lexical_units[0].readings[0]);
        assert_eq!(&pos, "prn");
    }

    #[test]
    fn test_mainpos_ltr() {
        let lexical_units = parse(S4);
        assert_eq!(lexical_units.len(), 1);
        let pos = main_pos_with_ltr(&lexical_units[0].readings[0]);
        assert_eq!(&pos, "vblex")
    }

    #[test]   
    fn test_reading_to_string() {
        let lexical_units = parse(S4);
        assert_eq!(lexical_units.len(), 1);
        assert_eq!(readings_to_string(&lexical_units[0].readings[0]), "decir<vblex><imp><p2><sg>+me<prn><enc><p1><mf><sg>+lo<prn><enc><p3><nt>");
    }

    #[test]   
    fn test_subreading_to_string() {
        let lexical_units = parse(S4);
        assert_eq!(lexical_units.len(), 1);
        assert_eq!(subreadings_to_string(&lexical_units[0].readings[0]), "decir<vblex><imp><p2><sg>");
    }

    #[test]
    fn test_blanks_with_wordbound_blanks() {    
        let lexical_units_with_blanks = parse_with_text(S5);
        assert_eq!(lexical_units_with_blanks.len(), 3);
        let (blank, _lexical_unit) = &lexical_units_with_blanks[0];
        assert_eq!(blank, r"[] ");
        let (blank, _lexical_unit) = &lexical_units_with_blanks[1];
        assert_eq!(blank, r" [bl] ");
        let (blank, _lexical_unit) = &lexical_units_with_blanks[2];
        assert_eq!(blank, r" [\[] [\]blank] ");
    }
}
