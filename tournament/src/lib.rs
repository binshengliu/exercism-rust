use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone)]
struct TeamResult {
    name: String,
    win: u32,
    draw: u32,
    loss: u32,
}

impl TeamResult {
    pub fn new(name: &str) -> TeamResult {
        TeamResult {
            name: name.to_string(),
            win: 0,
            draw: 0,
            loss: 0,
        }
    }

    fn points(&self) -> u32 {
        self.win * 3 + self.draw
    }

    fn played_matches(&self) -> u32 {
        self.win + self.draw + self.loss
    }
}

pub fn tally(input: &str) -> String {
    let reader = BufReader::new(std::io::Cursor::new(input));
    let mut table: HashMap<String, TeamResult> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let splits: Vec<&str> = line.split(";").collect();
        let team1 = splits[0];
        let team2 = splits[1];
        let result = splits[2];

        table.entry(team1.to_string()).or_insert(
            TeamResult::new(team1),
        );

        table.entry(team2.to_string()).or_insert(
            TeamResult::new(team2),
        );

        match result {
            "win" => {
                table.get_mut(team1).unwrap().win += 1;
                table.get_mut(team2).unwrap().loss += 1;
            }
            "draw" => {
                table.get_mut(team1).unwrap().draw += 1;
                table.get_mut(team2).unwrap().draw += 1;
            }
            "loss" => {
                table.get_mut(team1).unwrap().loss += 1;
                table.get_mut(team2).unwrap().win += 1;
            }
            _ => {}
        }
    }

    // calling table.values().cloned() has to incur copying the values
    let mut table = table
        .into_iter()
        .map(|(_key, value)| value)
        .collect::<Vec<TeamResult>>();

    // Sort by points, then by name
    table.sort_by(|team1, team2| {
        let ord = team2.points().cmp(&team1.points());
        if ord == std::cmp::Ordering::Equal {
            team1.name.cmp(&team2.name)
        } else {
            ord
        }
    });

    // "Team                           | MP |  W |  D |  L |  P"
    let head = format!(
        "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
        "Team",
        "MP",
        "W",
        "D",
        "L",
        "P"
    );

    let mut s: Vec<String> = table
        .iter()
        .map(|team| {
            format!(
                "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                team.name,
                team.played_matches(),
                team.win,
                team.draw,
                team.loss,
                team.points()
            )
        })
        .collect();
    s.insert(0, head);

    s.join("\n")
}
