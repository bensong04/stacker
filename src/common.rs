use chrono::{self, DateTime, Local};
use std::{collections::HashMap, fmt::Display, hash::Hash};
use toml::{Table, Value};

pub struct Player {
    pub name: String,
    pub buyin: i64,
    pub ishost: bool,
    pub clockin: DateTime<Local>,
    pub log: Vec<(DateTime<Local>, i64)>,
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ishost {
            write!(f, "(HOST) ")?;
        }
        let dur = Local::now() - self.clockin;
        write!(
            f,
            "NAME: {} | BUYIN: {} | PLAYING FOR: {}h {}m\n",
            self.name,
            self.buyin,
            dur.num_hours(),
            dur.num_minutes() % 60
        )?;
        self.log
            .iter()
            .try_for_each(|(time, addon)| write!(f, "(@ {}) added on for {}\n", time, addon))?;
        Ok(())
    }
}

impl Player {
    pub fn new(name: String, buyin: i64, ishost: bool) -> Self {
        let now = Local::now();
        Self {
            name,
            buyin: buyin,
            ishost: ishost,
            clockin: now,
            log: vec![(now, buyin)],
        }
    }

    /// Returns (final cashout, rake paid)
    /// The rake is considered in the final cashout.
    pub fn calculate_cashout(&self, stack: i64, rake_rate: u32) -> (i64, i64) {
        if !self.ishost {
            let rake =
                (Local::now() - self.clockin).num_seconds() as f32 * rake_rate as f32 / 3600.0;
            let rounded_rake = rake.floor() as i64;
            return (stack - rounded_rake, rounded_rake);
        }
        (stack, 0)
    }

    pub fn add_on(&mut self, addon: i64) {
        let now = Local::now();
        self.buyin += addon;
        self.log.push((now, addon));
    }
}

pub struct Game {
    blinds: (u32, u32),
    min_bi: Option<u32>,
    max_bi: Option<u32>,
    ledger: HashMap<String, Player>,
    timed_rake: Option<u32>, // dollars per hour
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}/${} NLH", self.blinds.0, self.blinds.1)?;
        if let Some(min_bi) = self.min_bi {
            write!(f, " | Min. buyin ${}", min_bi)?;
        }
        if let Some(max_bi) = self.max_bi {
            write!(f, " | Max. buyin ${}", max_bi)?;
        }
        if let Some(rake) = self.timed_rake {
            write!(f, " | Timed rake ${}/hr", rake)?;
        }
        self.ledger
            .values()
            .try_for_each(|player| write!(f, "\n-----\n{}", player))?;
        write!(f, "TOTAL: ${}", self.total_money())?;
        Ok(())
    }
}

impl Game {
    pub fn from_config(config: &Table) -> Result<Self, &'static str> {
        // ugly lol
        let sb = config
            .get("sb")
            .ok_or("Expected small blind")?
            .as_integer()
            .ok_or("Small blind must be integer")?;
        let bb = config
            .get("bb")
            .ok_or("Expected small blind")?
            .as_integer()
            .ok_or("Small blind must be integer")?;
        let min_bi = config.get("min_bi");
        let max_bi = config.get("max_bi");
        let rake = config.get("rake");
        if let Some(v) = min_bi
            && !v.is_integer()
        {
            return Err("Min buyin must be integer");
        }
        if let Some(v) = max_bi
            && !v.is_integer()
        {
            return Err("Max buyin must be integer");
        }
        if let Some(v) = rake
            && !v.is_integer()
        {
            return Err("Timed, hourly rake must be integer");
        }
        let map_int = |v: &Value| v.as_integer().unwrap() as u32;
        Ok(Self {
            blinds: (sb as u32, bb as u32),
            min_bi: min_bi.map(map_int),
            max_bi: max_bi.map(map_int),
            ledger: HashMap::new(),
            timed_rake: rake.map(map_int),
        })
    }

    pub fn add_player(&mut self, name: String, buyin: i64, ishost: bool) -> Result<(), String> {
        if let Some(min_bi) = self.min_bi
            && buyin < min_bi.into()
        {
            return Err(format!(
                "{} attempted to buy in for ${} but min. buyin is ${}",
                name, buyin, min_bi
            ));
        }
        if let Some(min_bi) = self.max_bi
            && buyin > min_bi.into()
        {
            return Err(format!(
                "{} attempted to buy in for ${} but max. buyin is ${}",
                name, buyin, min_bi
            ));
        }
        self.ledger
            .insert(name.clone(), Player::new(name, buyin, ishost));
        Ok(())
    }

    pub fn addon_player(&mut self, name: &String, amt: i64) -> Result<(), String> {
        let player = self.get_player(name)?;
        player.add_on(amt);
        Ok(())
    }

    /// Returns (buyin, cashout, ledger, paid rake)
    pub fn cashout_player(
        &mut self,
        name: &String,
        stack: i64,
    ) -> Result<(i64, i64, i64, i64), String> {
        let player = self
            .ledger
            .remove(name)
            .ok_or(&format!("Player {} doesn't exist", name))?;
        let (cashout, rake) = player.calculate_cashout(stack, self.timed_rake.unwrap_or(0));
        Ok((player.buyin, cashout, cashout - player.buyin, rake))
    }

    pub fn get_player(&mut self, name: &String) -> Result<&mut Player, String> {
        self.ledger
            .get_mut(name)
            .ok_or(format!("Player {} doesn't exist", name))
    }

    pub fn total_money(&self) -> i64 {
        self.ledger.values().map(|player| player.buyin).sum()
    }
}
