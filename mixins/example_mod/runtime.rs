// Put testable mod logic here. This file is both the standalone Cargo library
// and the injected `crate::game::example_mod` module; no generated copy exists.

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub const CATEGORIES: [&str; 5] = ["Combat", "Movement", "Visuals", "Player", "Misc"];
pub const MODULES: [[&str; 6]; 5] = [
    [
        "Attack Aura",
        "No Velocity",
        "Trigger Bot",
        "Aim Assist",
        "Auto Explosion",
        "Auto Swap",
    ],
    ["Speed", "Fly", "Sprint", "Step", "No Slow", "Air Jump"],
    [
        "ESP",
        "Chams",
        "Full Bright",
        "Tracers",
        "Name Tags",
        "View Model",
    ],
    [
        "Auto Totem",
        "Auto Eat",
        "Inventory Sort",
        "Fast Use",
        "No Fall",
        "Auto Tool",
    ],
    [
        "Sprint Reset",
        "Tape Mouse",
        "Backtrack",
        "Web Trap",
        "Anti Bot",
        "No Friend Damage",
    ],
];

pub const MANAGER: [&str; 2] = ["Binds", "Config"];

/// Which window sits to the right of the panel; mirrors `prism_ui::SIDE_*`.
pub const SIDE_NONE: usize = 0;
pub const SIDE_MODULE: usize = 1;
pub const SIDE_BINDS: usize = 2;
pub const SIDE_CONFIG: usize = 3;

/// Where the config system keeps its files, relative to the game directory.
pub const CONFIG_DIR: &str = "config/prism";

pub struct GuiState {
    pub category: usize,
    pub enabled: [[bool; 6]; 5],
    pub animation: [[f32; 6]; 5],
    pub binds: [[Option<String>; 6]; 5],
    pub values: [[u8; 6]; 5],
    pub selected: Option<(usize, usize)>,
    pub waiting_bind: bool,
    pub side: usize,
    pub query: String,
    pub search_focus: bool,
    pub configs: Vec<String>,
    pub config_selected: Option<usize>,
    pub status: String,
    pub loaded: bool,
    /// Panel fade/slide-in, stepped by the renderer.
    pub open: f32,
    /// Right-hand window slide-in, stepped by the renderer.
    pub side_anim: f32,
    /// True while the VALUE slider is grabbed.
    pub dragging: bool,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            category: 0,
            enabled: [[false; 6]; 5],
            animation: [[0.0; 6]; 5],
            binds: [const { [const { None }; 6] }; 5],
            values: [[50; 6]; 5],
            selected: None,
            waiting_bind: false,
            side: SIDE_NONE,
            query: String::new(),
            search_focus: false,
            configs: Vec::new(),
            config_selected: None,
            status: String::new(),
            loaded: false,
            open: 0.0,
            side_anim: 0.0,
            dragging: false,
        }
    }
}

pub static GUI: Mutex<GuiState> = Mutex::new(GuiState {
    category: 0,
    enabled: [[false; 6]; 5],
    animation: [[0.0; 6]; 5],
    binds: [const { [const { None }; 6] }; 5],
    values: [[50; 6]; 5],
    selected: None,
    waiting_bind: false,
    side: SIDE_NONE,
    query: String::new(),
    search_focus: false,
    configs: Vec::new(),
    config_selected: None,
    status: String::new(),
    loaded: false,
    open: 0.0,
    side_anim: 0.0,
    dragging: false,
});

impl GuiState {
    pub fn activate_bind(&mut self, key: &str) -> usize {
        let mut count = 0;
        for category in 0..5 {
            for module in 0..6 {
                if self.binds[category][module].as_deref() == Some(key) {
                    self.enabled[category][module] = !self.enabled[category][module];
                    count += 1;
                }
            }
        }
        count
    }

    /// The modules shown in the grid: the current category, or every match
    /// for the current search query in category order.
    pub fn visible(&self) -> Vec<(usize, usize)> {
        let query = self.query.trim().to_lowercase();
        if query.is_empty() {
            return (0..6).map(|module| (self.category, module)).collect();
        }
        let mut matches = Vec::new();
        for (category, modules) in MODULES.iter().enumerate() {
            for (module, name) in modules.iter().enumerate() {
                if name.to_lowercase().contains(&query) {
                    matches.push((category, module));
                }
            }
        }
        matches
    }

    /// Bound modules as (category, module, key), in grid order.
    pub fn bound(&self) -> Vec<(usize, usize, String)> {
        let mut rows = Vec::new();
        for category in 0..5 {
            for module in 0..6 {
                if let Some(key) = &self.binds[category][module] {
                    rows.push((category, module, key.clone()));
                }
            }
        }
        rows
    }

    pub fn open_module(&mut self, category: usize, module: usize) {
        if self.side == SIDE_NONE {
            self.side_anim = 0.0;
        }
        self.side = SIDE_MODULE;
        self.selected = Some((category, module));
        self.waiting_bind = false;
        self.dragging = false;
    }

    pub fn close_side(&mut self) {
        self.side = SIDE_NONE;
        self.selected = None;
        self.waiting_bind = false;
        self.dragging = false;
        self.side_anim = 0.0;
    }

    pub fn reset_ui(&mut self) {
        self.close_side();
        self.search_focus = false;
        self.query.clear();
        self.open = 0.0;
    }

    pub fn serialize(&self) -> String {
        let mut out = String::from("# prism config v1\n");
        out.push_str(&format!("category={}\n", self.category));
        for category in 0..5 {
            for module in 0..6 {
                if self.enabled[category][module] {
                    out.push_str(&format!("enabled={category},{module}\n"));
                }
            }
        }
        for category in 0..5 {
            for module in 0..6 {
                if let Some(key) = &self.binds[category][module] {
                    out.push_str(&format!("bind={category},{module},{key}\n"));
                }
            }
        }
        for category in 0..5 {
            for module in 0..6 {
                out.push_str(&format!(
                    "value={category},{module},{}\n",
                    self.values[category][module]
                ));
            }
        }
        out
    }

    pub fn apply(&mut self, text: &str) {
        self.enabled = [[false; 6]; 5];
        self.binds = [const { [const { None }; 6] }; 5];
        self.values = [[50; 6]; 5];
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, rest)) = line.split_once('=') else {
                continue;
            };
            match key.trim() {
                "category" => {
                    if let Some(category) = rest.trim().parse::<usize>().ok().filter(|c| *c < 5) {
                        self.category = category;
                    }
                }
                "enabled" => {
                    if let Some((category, module)) = parse_slot(rest) {
                        self.enabled[category][module] = true;
                    }
                }
                "bind" => {
                    let mut parts = rest.splitn(3, ',');
                    let (Some(category), Some(module), Some(key)) =
                        (parts.next(), parts.next(), parts.next())
                    else {
                        continue;
                    };
                    let (Ok(category), Ok(module)) = (
                        category.trim().parse::<usize>(),
                        module.trim().parse::<usize>(),
                    ) else {
                        continue;
                    };
                    let key = key.trim();
                    if category < 5 && module < 6 && !key.is_empty() {
                        self.binds[category][module] = Some(key.to_string());
                    }
                }
                "value" => {
                    let mut parts = rest.splitn(3, ',');
                    let (Some(category), Some(module), Some(value)) =
                        (parts.next(), parts.next(), parts.next())
                    else {
                        continue;
                    };
                    let (Ok(category), Ok(module)) = (
                        category.trim().parse::<usize>(),
                        module.trim().parse::<usize>(),
                    ) else {
                        continue;
                    };
                    if category < 5
                        && module < 6
                        && let Ok(value) = value.trim().parse::<u8>()
                    {
                        self.values[category][module] = value.min(100);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn refresh_configs(&mut self) {
        self.configs = config_files();
        if self
            .config_selected
            .is_some_and(|index| index >= self.configs.len())
        {
            self.config_selected = self.configs.len().checked_sub(1);
        }
    }

    pub fn next_config_name(&self) -> String {
        let mut index = 1;
        loop {
            let name = format!("config-{index}");
            if !self.configs.contains(&name) && !config_path(&name).exists() {
                return name;
            }
            index += 1;
        }
    }

    pub fn save_to(&mut self, name: &str) -> bool {
        let name = sanitize_name(name);
        let dir = config_dir();
        if fs::create_dir_all(&dir).is_err() {
            self.status = "CANNOT CREATE CONFIG DIR".into();
            return false;
        }
        match fs::write(dir.join(format!("{name}.cfg")), self.serialize()) {
            Ok(()) => {
                self.status = format!("SAVED {name}");
                self.refresh_configs();
                if let Some(index) = self.configs.iter().position(|config| *config == name) {
                    self.config_selected = Some(index);
                }
                true
            }
            Err(_) => {
                self.status = format!("FAILED TO SAVE {name}");
                false
            }
        }
    }

    pub fn load_from(&mut self, name: &str) -> bool {
        match fs::read_to_string(config_path(name)) {
            Ok(text) => {
                self.apply(&text);
                self.status = format!("LOADED {name}");
                true
            }
            Err(_) => {
                self.status = format!("FAILED TO LOAD {name}");
                false
            }
        }
    }

    pub fn delete(&mut self, name: &str) -> bool {
        match fs::remove_file(config_path(name)) {
            Ok(()) => {
                self.status = format!("DELETED {}", name.to_uppercase());
                self.refresh_configs();
                if self.configs.is_empty() {
                    self.config_selected = None;
                }
                true
            }
            Err(_) => {
                self.status = "DELETE FAILED".into();
                false
            }
        }
    }

    /// Load the autosaved session once, then list the saved configs.
    pub fn ensure_loaded(&mut self) {
        if self.loaded {
            return;
        }
        self.loaded = true;
        if let Ok(text) = fs::read_to_string(config_path("session")) {
            self.apply(&text);
        }
        self.refresh_configs();
    }

    pub fn save_session(&mut self) {
        let dir = config_dir();
        if fs::create_dir_all(&dir).is_err() {
            return;
        }
        let _ = fs::write(dir.join("session.cfg"), self.serialize());
    }

    pub fn config_action(&mut self, which: usize) {
        let selected = self
            .config_selected
            .and_then(|index| self.configs.get(index))
            .cloned();
        match which {
            0 => {
                let name = self.next_config_name();
                self.save_to(&name);
            }
            1 => {
                let name = selected.unwrap_or_else(|| self.next_config_name());
                self.save_to(&name);
            }
            2 => match selected {
                Some(name) => {
                    self.load_from(&name);
                }
                None => self.status = "SELECT A CONFIG TO LOAD".into(),
            },
            3 => match selected {
                Some(name) => {
                    self.delete(&name);
                }
                None => self.status = "SELECT A CONFIG TO DELETE".into(),
            },
            _ => {}
        }
    }
}

fn parse_slot(text: &str) -> Option<(usize, usize)> {
    let (category, module) = text.split_once(',')?;
    let category = category.trim().parse::<usize>().ok()?;
    let module = module.trim().parse::<usize>().ok()?;
    (category < 5 && module < 6).then_some((category, module))
}

/// Only plain file names reach the disk: `save_to("../x")` stays inside.
pub fn sanitize_name(name: &str) -> String {
    let mut out: String = name
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
        .take(24)
        .collect();
    if out.is_empty() {
        out.push_str("config");
    }
    out
}

pub fn config_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_default().join(CONFIG_DIR)
}

pub fn config_path(name: &str) -> PathBuf {
    config_dir().join(format!("{}.cfg", sanitize_name(name)))
}

pub fn config_files() -> Vec<String> {
    let Ok(entries) = fs::read_dir(config_dir()) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? != "cfg" {
                return None;
            }
            let name = path.file_stem()?.to_string_lossy().into_owned();
            (name != "session").then_some(name)
        })
        .collect();
    names.sort();
    names
}

static OPEN: AtomicBool = AtomicBool::new(false);

pub fn is_open() -> bool {
    OPEN.load(Ordering::Relaxed)
}

pub fn toggle() -> bool {
    !OPEN.fetch_xor(true, Ordering::Relaxed)
}

pub fn close() {
    OPEN.store(false, Ordering::Relaxed);
}

pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[example_mod] Breaking block at {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

pub fn before_break(pos: [i32; 3]) {
    eprintln!("{}", block_break_message(pos));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[example_mod] Breaking block at -3, 64, 12"
        );
    }

    #[test]
    fn bind_toggles_matching_placeholders_only() {
        let mut state = GuiState::default();
        state.binds[0][1] = Some("KeyK".into());
        state.binds[2][3] = Some("KeyK".into());
        assert_eq!(state.activate_bind("KeyJ"), 0);
        assert_eq!(state.activate_bind("KeyK"), 2);
        assert!(state.enabled[0][1] && state.enabled[2][3]);
        assert!(!state.enabled[0][0]);
        assert_eq!(state.activate_bind("KeyK"), 2);
        assert!(!state.enabled[0][1] && !state.enabled[2][3]);
    }

    #[test]
    fn search_filters_modules_across_categories() {
        let mut state = GuiState::default();
        assert_eq!(state.visible().len(), 6);
        assert!(state.visible().iter().all(|(category, _)| *category == 0));

        state.query = "fly".into();
        assert_eq!(state.visible(), vec![(1, 1)]);

        state.query = "auto".into();
        let matches = state.visible();
        assert!(matches.contains(&(0, 4)) && matches.contains(&(3, 0)));
        assert!(matches.iter().all(|(category, module)| {
            MODULES[*category][*module].to_lowercase().contains("auto")
        }));

        state.query = "zzz".into();
        assert!(state.visible().is_empty());
    }

    #[test]
    fn binds_are_listed_in_grid_order() {
        let mut state = GuiState::default();
        state.binds[4][5] = Some("KeyK".into());
        state.binds[0][2] = Some("KeyJ".into());
        assert_eq!(
            state.bound(),
            vec![(0, 2, "KeyJ".to_string()), (4, 5, "KeyK".to_string())]
        );
    }

    #[test]
    fn configs_round_trip_enabled_binds_and_values() {
        let mut state = GuiState {
            category: 2,
            ..GuiState::default()
        };
        state.enabled[2][3] = true;
        state.enabled[0][5] = true;
        state.binds[2][3] = Some("KeyK".into());
        state.values[0][1] = 75;
        state.values[4][5] = 100;

        let mut restored = GuiState::default();
        restored.apply(&state.serialize());

        assert_eq!(restored.category, 2);
        assert!(restored.enabled[2][3] && restored.enabled[0][5]);
        assert!(!restored.enabled[1][1]);
        assert_eq!(restored.binds[2][3].as_deref(), Some("KeyK"));
        assert_eq!(restored.binds[0][0], None);
        assert_eq!(restored.values[0][1], 75);
        assert_eq!(restored.values[4][5], 100);
        assert_eq!(restored.values[3][2], 50);
    }

    #[test]
    fn config_parser_ignores_bad_lines_and_clamps() {
        let mut state = GuiState::default();
        state.apply(
            "garbage\ncategory=9\nenabled=5,0\nenabled=0,6\nvalue=0,0,240\nbind=0,0,\nvalue=1,1,10\n",
        );
        assert_eq!(state.category, 0);
        assert!(!state.enabled[0][0]);
        assert_eq!(state.values[0][0], 100);
        assert_eq!(state.values[1][1], 10);
        assert_eq!(state.binds[0][0], None);
    }

    #[test]
    fn config_names_cannot_escape_the_directory() {
        assert_eq!(sanitize_name("../../etc/passwd"), "etcpasswd");
        assert_eq!(sanitize_name(""), "config");
        assert_eq!(sanitize_name("my config!"), "myconfig");
    }
}
