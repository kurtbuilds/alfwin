#![allow(unused)]
#![allow(non_camel_case_types)]
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::io::Read;
use std::io::{BufReader, Cursor, Stdout, Write};
use std::path;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{env, fs, io, thread, time::Duration};

use clap::{App, AppSettings, Arg, ArgMatches};
use dirs::home_dir;
use regex::Regex;

use crate::windows::get_window_names;
use alfred::{AlfredItem, AlfredItemType, AlfredItems, ItemIcon, ItemIconType};

mod alfred;
mod windows;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const SUBCOMMAND_DIR: &'static str = env!("SUBCOMMAND_DIR");
const SCRIPT_DIR: &'static str = env!("SCRIPT_DIR");

#[derive(Debug)]
enum TabLauncher {
    Chrome,
    iTerm,
}

struct LaunchTab {
    name: String,
    window: usize,
    tab: usize,
}

struct LaunchWindow {
    process: String,
    window_name: String,
}

enum LaunchAction {
    Chrome(LaunchTab),
    iTerm(LaunchTab),
    Window(LaunchWindow),
}

impl LaunchAction {
    fn uid(&self) -> String {
        match &self {
            LaunchAction::Chrome(chrome) => format!(
                "{}-{}-{}",
                chrome.window,
                chrome.tab,
                chrome.name.to_ascii_lowercase()
            ),
            LaunchAction::iTerm(iterm) => format!(
                "{}-{}-{}",
                iterm.window,
                iterm.tab,
                iterm.name.to_ascii_lowercase()
            ),
            LaunchAction::Window(window) => format!(
                "{}-{}",
                window.process.to_ascii_lowercase(),
                window.window_name.to_ascii_lowercase()
            ),
        }
    }

    fn name(&self) -> String {
        match &self {
            LaunchAction::Chrome(chrome) => format!("{} - Chrome Tab", chrome.name),
            LaunchAction::iTerm(iterm) => format!("{} - iTerm Session", iterm.name),
            LaunchAction::Window(window) => format!("{} - {}", window.window_name, window.process),
        }
    }

    fn process_name(&self) -> String {
        match &self {
            LaunchAction::Chrome(chrome) => format!("Google Chrome"),
            LaunchAction::iTerm(iterm) => format!("iTerm"),
            LaunchAction::Window(window) => window.process.to_string(),
        }
    }

    fn args(&self) -> String {
        match &self {
            LaunchAction::Chrome(chrome) => format!("open-chrome {} {}", chrome.window, chrome.tab),
            LaunchAction::iTerm(iterm) => format!("open-iterm {} {}", iterm.window, iterm.tab),
            LaunchAction::Window(window) => {
                format!("open \"{}\" \"{}\"", window.process, window.window_name)
            }
        }
    }

    fn icon(&self, icon_lookup: &BTreeMap<String, String>) -> ItemIcon {
        match &self {
            LaunchAction::Chrome(_) => ItemIcon {
                typ: ItemIconType::IconForFileAtPath,
                path: "/Applications/Google Chrome.app".to_string(),
            },
            LaunchAction::iTerm(_) => ItemIcon {
                typ: ItemIconType::IconForFileAtPath,
                path: "/Applications/iTerm.app".to_string(),
            },
            LaunchAction::Window(window) => ItemIcon {
                typ: ItemIconType::IconForFileAtPath,
                path: icon_lookup
                    .get(&window.process)
                    .unwrap_or(&"".to_string())
                    .to_string(),
            },
        }
    }
}

fn parse_tabs(o: Output, typ: TabLauncher) -> Result<Vec<LaunchAction>, Box<dyn Error>> {
    let mut seen = BTreeSet::new();
    String::from_utf8(o.stdout)
        .map_err(|e| Box::new(e) as Box<dyn Error>)
        .map(|s| {
            s.trim()
                .split("\n")
                .map(String::from)
                .filter_map(|line| {
                    if line == "" {
                        return None;
                    }
                    let args: Vec<String> = line.splitn(3, ",").map(String::from).collect();
                    let window = args[0]
                        .parse::<usize>()
                        .expect(&format!("Failed to parse window. {:#?}: {:#?}", typ, line));
                    let tab = args[1]
                        .parse::<usize>()
                        .expect(&format!("Failed to parse tab. {:#?}: {:#?}", typ, line));
                    let name = args[2].to_owned();
                    if seen.contains(&name) {
                        return None;
                    }
                    seen.insert(name.to_owned());
                    Some(match typ {
                        TabLauncher::Chrome => {
                            LaunchAction::Chrome(LaunchTab { name, window, tab })
                        }
                        TabLauncher::iTerm => LaunchAction::iTerm(LaunchTab { name, window, tab }),
                    })
                })
                .collect()
        })
}

fn icon_lookup() -> BTreeMap<String, String> {
    let home_dir = dirs::home_dir().unwrap();

    let mut apps = vec![
        fs::read_dir("/System/Applications/"),
        fs::read_dir("/Applications"),
        fs::read_dir(home_dir.join("Applications")),
        fs::read_dir(home_dir.join("Applications").join("JetBrains Toolbox")),
    ]
    .into_iter()
    .filter_map(|x| x.ok())
    .flatten();
    let idea_path = home_dir
        .join("Applications")
        .join("JetBrains Toolbox")
        .join("IntelliJ IDEA Ultimate.app");
    let mut r = BTreeMap::from_iter(
        vec![
            (
                "IntelliJ IDEA".to_string(),
                idea_path.to_str().unwrap().to_string(),
            ),
            (
                "Alfred Preferences".to_string(),
                "/Applications/Alfred 4.app".to_string(),
            ),
        ]
        .into_iter(),
    );
    r.extend(apps.filter_map(|r| {
        r.ok().map(|entry| {
            (
                entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                entry.path().to_str().unwrap().to_string(),
            )
        })
    }));
    r
}

fn display_results() {
    let icon_map = icon_lookup();

    let mut chrome = Command::new("osascript")
        .arg(PathBuf::from(SCRIPT_DIR).join("get_chrome_tabs.scpt"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("get_chrome_tabs.scpt failed to start.");

    let mut iterm = Command::new("osascript")
        .arg(PathBuf::from(SCRIPT_DIR).join("get_iterm_tabs.scpt"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("get_iterm_tabs.scpt failed to start.");

    let windows = get_window_names()
        .into_iter()
        .map(|w| {
            LaunchAction::Window(LaunchWindow {
                process: w.app_name,
                window_name: w.win_name,
            })
        })
        .collect::<Vec<_>>();

    let chrome = chrome
        .wait_with_output()
        .map(|o| parse_tabs(o, TabLauncher::Chrome).unwrap())
        .unwrap();
    let iterm = iterm
        .wait_with_output()
        .map(|o| parse_tabs(o, TabLauncher::iTerm).unwrap())
        .unwrap();

    let windows_and_tabs = windows
        .into_iter()
        .chain(iterm.into_iter())
        .chain(chrome.into_iter());

    let result = AlfredItems {
        items: windows_and_tabs
            .map(|launch| AlfredItem {
                uid: launch.uid(),
                typ: AlfredItemType::Default,
                title: launch.name(),
                subtitle: launch.process_name(),
                arg: launch.args(),
                autocomplete: launch.name(),
                icon: launch.icon(&icon_map),
            })
            .collect(),
    };
    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);
}

fn display_intellij() {
    let fpath = home_dir().unwrap().join(
        "Library/Application Support/JetBrains/IntelliJIdea2021.3/options/recentProjects.xml",
    );
    let items = fs::read(&fpath)
        .unwrap()
        .split(|x| *x == b'\n')
        .filter_map(|line| {
            let line = String::from_utf8(line.to_vec()).unwrap();
            let re = Regex::new(r#"^\s*<entry key="([$a-zA-Z/.0-9_]+)">\s*$"#).unwrap();
            re.captures(&line).map(|caps| caps[1].to_string())
        })
        .map(|s| s.replace("$USER_HOME$", home_dir().unwrap().to_str().unwrap()))
        .map(|x| AlfredItem {
            uid: x.to_owned(),
            typ: AlfredItemType::Default,
            title: x
                .rsplitn(3, '/')
                .take(2)
                .collect::<Vec<&str>>()
                .into_iter()
                .rev()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .join(" ")
                // .replace("/Users/kurt/work/", "")
                // .replace("/", " ")
                .to_owned(),
            subtitle: "IntelliJ".to_owned(),
            arg: format!("open-intellij \"{}\"", x),
            autocomplete: format!(
                " {} ",
                x.rsplitn(3, '/')
                    .skip(1)
                    .take(1)
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>()
                    .join(" ")
                    .to_string()
            ),
            icon: ItemIcon {
                typ: ItemIconType::IconForFileAtPath,
                path: PathBuf::from("/Applications/IntelliJ IDEA.app")
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
        })
        .collect::<Vec<AlfredItem>>();
    let result = AlfredItems { items };
    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", json);
}

fn main() {
    let start_time = Instant::now();

    let com_match = App::new("alfwin")
        .version(VERSION)
        .about("Provide results and launch functionality as an Alfred extension.")
        .subcommand(
            App::new("open")
                .arg(Arg::new("process"))
                .arg(Arg::new("window")),
        )
        .subcommand(
            App::new("open-chrome")
                .arg(Arg::new("window"))
                .arg(Arg::new("tab")),
        )
        .subcommand(
            App::new("open-iterm")
                .arg(Arg::new("window"))
                .arg(Arg::new("tab")),
        )
        .subcommand(App::new("open-intellij").arg(Arg::new("path")))
        .subcommand(App::new("list-intellij"))
        .subcommand(App::new("debug"))
        .get_matches();

    match com_match.subcommand() {
        None => display_results(),
        Some((subcommand, matches)) => match subcommand {
            "open" => {
                let process = matches.value_of("process").unwrap();
                let window = matches.value_of("window").unwrap();
                Command::new("osascript")
                    .arg(PathBuf::from(SCRIPT_DIR).join("activate_application_window.scpt"))
                    .arg(process)
                    .arg(window)
                    .output()
                    .expect("activate_application_window.scpt failed to start.");
            }
            "open-chrome" => {
                let window = matches.value_of("window").unwrap();
                let tab = matches.value_of("tab").unwrap();
                Command::new("osascript")
                    .arg(PathBuf::from(SCRIPT_DIR).join("activate_chrome_tab.scpt"))
                    .arg(window)
                    .arg(tab)
                    .output()
                    .expect("activate_chrome_tab.scpt failed to start.");
            }
            "open-iterm" => {
                let window = matches.value_of("window").unwrap();
                let tab = matches.value_of("tab").unwrap();
                Command::new("osascript")
                    .arg(PathBuf::from(SCRIPT_DIR).join("activate_iterm_tab.scpt"))
                    .arg(window)
                    .arg(tab)
                    .output()
                    .expect("activate_iterm.scpt failed to start.");
            }
            "open-intellij" => {
                let path = matches.value_of("path").unwrap();
                eprintln!("path {}", path);
                Command::new("/usr/local/bin/idea")
                    .current_dir(path)
                    .arg(path)
                    .output()
                    .unwrap();
            }
            "list-intellij" => display_intellij(),
            "debug" => {
                let result = AlfredItems {
                    items: vec![AlfredItem {
                        uid: "desktop".to_string(),
                        typ: AlfredItemType::File,
                        title: "Desktop".to_string(),
                        subtitle: "~/Desktop".to_string(),
                        arg: "~/Desktop".to_string(),
                        autocomplete: "Desktop".to_string(),
                        icon: ItemIcon {
                            typ: ItemIconType::IconForFileAtPath,
                            path: "~/Desktop".to_string(),
                        },
                    }],
                };
                let json = serde_json::to_string_pretty(&result).unwrap();
                println!("{}", json);
            }
            _ => panic!("Subcommand {} not recognized.", subcommand),
        },
    }
    eprintln!("Process took {}ms.", start_time.elapsed().as_millis());
}
