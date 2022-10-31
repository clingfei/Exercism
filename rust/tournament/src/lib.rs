use std::collections::HashMap;
use std::cmp::Ordering;

static HEADER: &'static str ="Team                           | MP |  W |  D |  L |  P";

enum MatchResult {
    Won,
    Drawn,
    Lost,
}

use MatchResult::*;
#[derive(Clone, PartialEq, PartialOrd, Eq)]
struct Team {
    name: String,
    mp: i32,
    w: i32,
    d: i32,
    l: i32,
    score: i32
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.into(), 
            mp: 0, 
            w: 0, 
            d: 0, 
            l: 0, 
            score: 0
        }
    }
    fn set_match_result(&mut self, result: MatchResult) {
        self.mp += 1;
        match result {
            Won => {
                self.w+=1;
                self.score += 3;
            }
            Drawn => {
                self.d += 1;
                self.score += 1;
            }
            Lost => {
                self.l += 1;
            }
        }
    }
    fn to_row(&self) -> String {
        format!(
            "{:<30} |  {} |  {} |  {} |  {} |  {}",
            self.name, self.mp, self.w, self.d, self.l, self.score
        )
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Team) -> Ordering{
        self
            .score
            .cmp(&other.score)
            .then(other.name.cmp(&self.name))
    }
}

pub fn tally(match_results: &str) -> String {
    let mut output: Vec<String> = Vec::new();
    output.push(HEADER.to_string());
    if match_results.len() == 0 {
        return HEADER.to_string();
    }
    let mut hashmap = HashMap::new();
    let vec :Vec<&str> = match_results.split("\n").collect();
    let mut i = 0;
    while i < vec.len() {
        let line: Vec<&str> = vec[i].split(";").collect();    
        if line[2] == "draw" {
            let team = hashmap.entry(line[0]).or_insert(Team::new(line[0]));
            team.set_match_result(Drawn);
            let team = hashmap.entry(line[1]).or_insert(Team::new(line[1]));
            team.set_match_result(Drawn);
        } else if line[2] == "win" {
            let team = hashmap.entry(line[0]).or_insert(Team::new(line[0]));
            team.set_match_result(Won);
            let team = hashmap.entry(line[1]).or_insert(Team::new(line[1]));
            team.set_match_result(Lost);
        } else if line[2] == "loss" {
            let team = hashmap.entry(line[0]).or_insert(Team::new(line[0]));
            team.set_match_result(Lost);
            let team = hashmap.entry(line[1]).or_insert(Team::new(line[1]));
            team.set_match_result(Won);
        }
        i += 1;
    }
    let mut teams: Vec<Team> = Vec::new();
    for iter in hashmap.into_iter() {
        teams.push(iter.1);
    }
    teams.sort_by(|a, b| b.cmp(&a));
    for team in teams {
        output.push(team.to_row());
    }
    output.join("\n")
}
