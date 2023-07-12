use regex::{Captures, Regex, Replacer};

pub fn handle_content(s: &str) -> String {
    let s = RankReplacer::reg().replace_all(s, RankReplacer);
    s.to_string()
}

pub struct RankReplacer;

impl RankReplacer {
    /// If this were any other application, lazy_static or similar would be ideal, but it doesn't
    /// matter for this, so no
    pub fn reg() -> Regex {
        // Groups:          v-1-v  v---2---v    v---3---v
        Regex::new(r#"\{\{#r(ank)? ([a-z_]+) *?([^ }].+?)?}}"#).unwrap()
    }
}

impl Replacer for RankReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let rank = caps.get(2).unwrap();
        let text = caps.get(3);

        if let Some(text) = text {
            dst.push_str(&format!(
                r#"<span class="name {}">{}</span>"#,
                rank.as_str(),
                text.as_str()
            ));
        } else {
            let rank_name = match rank.as_str() {
                "jr_dev" => "Jr. Dev".into(),
                "jr_mod" => "Jr. Mod".into(),
                "jr_builder" => "Jr. Builder".into(),
                s => {
                    let mut s = s.to_string();
                    if let Some(r) = s.get_mut(..1) {
                        r.make_ascii_uppercase();
                    }
                    s
                }
            };
            dst.push_str(&format!(
                r#"<span class="label {}">{}</span>"#,
                rank.as_str(),
                rank_name
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use super::handle_content;

    #[test]
    fn rank() {
        let r = handle_content("{{#rank adept  }}");
        assert_eq!(r, r#"<span class="label adept">Adept</span>"#);
        let r = handle_content("{{#rank adept}}");
        assert_eq!(r, r#"<span class="label adept">Adept</span>"#);
        let r = handle_content("{{#rank adept  }} {{}}");
        assert_eq!(r, r#"<span class="label adept">Adept</span> {{}}"#);
    }

    #[test]
    fn name() {
        let r = handle_content("{{#rank adept bob}}");
        assert_eq!(r, r#"<span class="name adept">bob</span>"#);
        let r = handle_content("{{#rank adept  bob}}");
        assert_eq!(r, r#"<span class="name adept">bob</span>"#);
    }

    #[test]
    fn none() {
        let r = handle_content("{{#rank adept bob}");
        assert_eq!(r, r#"{{#rank adept bob}"#);
        let r = handle_content("{{rank adept  bob}}");
        assert_eq!(r, r#"{{rank adept  bob}}"#);
    }
}
