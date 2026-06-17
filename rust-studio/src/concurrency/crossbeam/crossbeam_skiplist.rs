fn main() {
    use crossbeam_skiplist::SkipMap;
    use crossbeam_utils::thread::scope;
    let person_ages = SkipMap::new();
    scope(|s| {
                s.spawn(|_| {
            person_ages.insert("Spike Garrett", 22);
            person_ages.insert("Stan Hancock", 47);
            person_ages.insert("Rea Bryan", 234);
            assert_eq!(person_ages.get("Spike Garrett").unwrap().value(), &22);
        });
        s.spawn(|_| {
            person_ages.insert("Bryon Conroy", 65);
            person_ages.insert("Lauren Reilly", 2);
        });
    })
    .unwrap();
    assert!(person_ages.contains_key("Spike Garrett"));
    person_ages.remove("Rea Bryan");
    assert!(!person_ages.contains_key("Rea Bryan"));
}
