use crab_text::formats::legacy::LegacyTextFormat;
use crab_text::formats::TextFormat;

#[test]
fn test_legacy() {
    let text = "&0&lCr&r&1ab&r&2Cr&r&3af&r&4t &r&4&ni&r&5s &r&6&oth&r&7e &r&7&mf&r&8as&r&9te&r&ast&r&b &r&b&kMi&r&cne&r&dcr&r&eaft &rser&r&fver!&r";
    let serialized = LegacyTextFormat::ampersand().parse(&text.to_string()).unwrap();
    let cycle = LegacyTextFormat::ampersand().write(&serialized).unwrap();

    assert_eq!(text, cycle);
}
