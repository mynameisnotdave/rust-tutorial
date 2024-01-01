use std::collections::HashMap;

fn hashmap() -> () {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score: Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}

fn convert_tuple_arr_to_hashmap() -> () {
    // This is how to define an array of tuples.
    // Define the types inside the tuple and then define how many
    // tuples there are in the array.
    let teams: [(&str, i32);3] = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1: HashMap<&str, i32> = HashMap::new();
    // This is how to convert this array of tuples into a hash map.
    // There is however another way of doing it.
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // This is a simpler way of doing it. Relatively self explanatory.
    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);

    // This is another way of doing it, however the data structure that teams is
    // being 'collected' into must be explicitly defined.
    let _teams_map3: HashMap<&str, i32> = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);
}

fn check_if_exists() -> () {
    let mut player_stats: HashMap<&str, i32> = HashMap::new();

    // This is how to insert a new value based on whether the key exists currently or not.
    // Use or_insert_with(FUNCTION NAME) to apply a value based on what a function returns.
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);


}