#[derive(Clone, Default)]
pub struct Theme {
    pub separator_fg: (u8, u8, u8),

    pub home_bg: (u8, u8, u8),
    pub home_fg: (u8, u8, u8),
    pub path_bg: (u8, u8, u8),
    pub path_fg: (u8, u8, u8),
    pub cwd_fg: (u8, u8, u8),

    pub username_bg: (u8, u8, u8),
    pub username_fg: (u8, u8, u8),
    pub username_root_bg: (u8, u8, u8),
    pub username_root_fg: (u8, u8, u8),
    pub hostname_bg: (u8, u8, u8),
    pub hostname_fg: (u8, u8, u8),

    pub jobs_bg: (u8, u8, u8),
    pub jobs_fg: (u8, u8, u8),

    pub time_bg: (u8, u8, u8),
    pub time_fg: (u8, u8, u8),

    pub ssh_bg: (u8, u8, u8),
    pub ssh_fg: (u8, u8, u8),

    pub ssh_char: char,

    pub ro_bg: (u8, u8, u8),
    pub ro_fg: (u8, u8, u8),

    pub ro_char: char,

    pub git_clean_bg: (u8, u8, u8),
    pub git_clean_fg: (u8, u8, u8),
    pub git_dirty_bg: (u8, u8, u8),
    pub git_dirty_fg: (u8, u8, u8),
    pub git_ahead_bg: (u8, u8, u8),
    pub git_ahead_fg: (u8, u8, u8),
    pub git_behind_bg: (u8, u8, u8),
    pub git_behind_fg: (u8, u8, u8),
    pub git_conflicted_bg: (u8, u8, u8),
    pub git_conflicted_fg: (u8, u8, u8),
    pub git_notstaged_bg: (u8, u8, u8),
    pub git_notstaged_fg: (u8, u8, u8),
    pub git_staged_bg: (u8, u8, u8),
    pub git_staged_fg: (u8, u8, u8),
    pub git_untracked_bg: (u8, u8, u8),
    pub git_untracked_fg: (u8, u8, u8),

    pub git_ahead_char: char,
    pub git_behind_char: char,
    pub git_staged_char: char,
    pub git_notstaged_char: char,
    pub git_untracked_char: char,
    pub git_conflicted_char: char,

    pub cmd_passed_bg: (u8, u8, u8),
    pub cmd_passed_fg: (u8, u8, u8),
    pub cmd_failed_bg: (u8, u8, u8),
    pub cmd_failed_fg: (u8, u8, u8),

    pub ps_bg: (u8, u8, u8),
    pub ps_fg: (u8, u8, u8),

    pub virtual_env_bg: (u8, u8, u8),
    pub virtual_env_fg: (u8, u8, u8),

    pub nixshell_bg: (u8, u8, u8),
    pub nixshell_fg: (u8, u8, u8),
}

use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct ErrCorrupt;

impl StdError for ErrCorrupt {}
impl fmt::Display for ErrCorrupt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Corrupt theme file")
    }
}

pub fn load(file: &str) -> Result<Theme, Box<dyn StdError>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut theme = Theme::default();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.chars().all(char::is_whitespace) {
            continue;
        }
        let mut parts = line.splitn(2, '=');

        let variable = parts
            .next()
            .map(|inner| inner.trim())
            .ok_or_else(|| ErrCorrupt)?;
        let value = parts
            .next()
            .map(|inner| inner.trim())
            .ok_or_else(|| ErrCorrupt)?;

        if variable.ends_with("char") {
            let index = theme_index_char(&mut theme, variable).ok_or_else(|| ErrCorrupt)?;

            if value.chars().count() == 1 {
                *index = value.parse()?;
            } else {
                let codepoint = u32::from_str_radix(value, 16)?;
                *index = std::char::from_u32(codepoint).ok_or_else(|| ErrCorrupt)?;
            }
        } else {
            let index = theme_index_rgb(&mut theme, variable).ok_or_else(|| ErrCorrupt)?;
            let rgb = value
                .split(',')
                .map(|color| color.parse())
                .collect::<Result<Vec<u8>, _>>()?;
            if rgb.len() != 3 {
                return Err(ErrCorrupt.into());
            }
            *index = (rgb[0], rgb[1], rgb[2]);
        }
    }

    Ok(theme)
}

fn theme_index_rgb<'a>(theme: &'a mut Theme, name: &str) -> Option<&'a mut (u8, u8, u8)> {
    match name {
        "separator_fg" => Some(&mut theme.separator_fg),

        "home_bg" => Some(&mut theme.home_bg),
        "home_fg" => Some(&mut theme.home_fg),
        "path_bg" => Some(&mut theme.path_bg),
        "path_fg" => Some(&mut theme.path_fg),
        "cwd_fg" => Some(&mut theme.cwd_fg),

        "username_bg" => Some(&mut theme.username_bg),
        "username_fg" => Some(&mut theme.username_fg),
        "username_root_bg" => Some(&mut theme.username_root_bg),
        "username_root_fg" => Some(&mut theme.username_root_fg),
        "hostname_bg" => Some(&mut theme.hostname_bg),
        "hostname_fg" => Some(&mut theme.hostname_fg),

        "jobs_bg" => Some(&mut theme.jobs_bg),
        "jobs_fg" => Some(&mut theme.jobs_fg),

        "time_bg" => Some(&mut theme.time_bg),
        "time_fg" => Some(&mut theme.time_fg),

        "ssh_bg" => Some(&mut theme.ssh_bg),
        "ssh_fg" => Some(&mut theme.ssh_fg),

        "ro_bg" => Some(&mut theme.ro_bg),
        "ro_fg" => Some(&mut theme.ro_fg),

        "git_clean_bg" => Some(&mut theme.git_clean_bg),
        "git_clean_fg" => Some(&mut theme.git_clean_fg),
        "git_dirty_bg" => Some(&mut theme.git_dirty_bg),
        "git_dirty_fg" => Some(&mut theme.git_dirty_fg),
        "git_ahead_bg" => Some(&mut theme.git_ahead_bg),
        "git_ahead_fg" => Some(&mut theme.git_ahead_fg),
        "git_behind_bg" => Some(&mut theme.git_behind_bg),
        "git_behind_fg" => Some(&mut theme.git_behind_fg),
        "git_conflicted_bg" => Some(&mut theme.git_conflicted_bg),
        "git_conflicted_fg" => Some(&mut theme.git_conflicted_fg),
        "git_notstaged_bg" => Some(&mut theme.git_notstaged_bg),
        "git_notstaged_fg" => Some(&mut theme.git_notstaged_fg),
        "git_staged_bg" => Some(&mut theme.git_staged_bg),
        "git_staged_fg" => Some(&mut theme.git_staged_fg),
        "git_untracked_bg" => Some(&mut theme.git_untracked_bg),
        "git_untracked_fg" => Some(&mut theme.git_untracked_fg),

        "cmd_passed_bg" => Some(&mut theme.cmd_passed_bg),
        "cmd_passed_fg" => Some(&mut theme.cmd_passed_fg),
        "cmd_failed_bg" => Some(&mut theme.cmd_failed_bg),
        "cmd_failed_fg" => Some(&mut theme.cmd_failed_fg),

        "virtual_env_bg" => Some(&mut theme.virtual_env_bg),
        "virtual_env_fg" => Some(&mut theme.virtual_env_fg),

        "nixshell_bg" => Some(&mut theme.nixshell_bg),
        "nixshell_fg" => Some(&mut theme.nixshell_fg),

        _ => None,
    }
}

fn theme_index_char<'a>(theme: &'a mut Theme, name: &str) -> Option<&'a mut char> {
    match name {
        "ssh_char" => Some(&mut theme.ssh_char),
        "ro_char" => Some(&mut theme.ro_char),

        "git_ahead_char" => Some(&mut theme.git_ahead_char),
        "git_behind_char" => Some(&mut theme.git_behind_char),
        "git_staged_char" => Some(&mut theme.git_staged_char),
        "git_notstaged_char" => Some(&mut theme.git_notstaged_char),
        "git_untracked_char" => Some(&mut theme.git_untracked_char),
        "git_conflicted_char" => Some(&mut theme.git_conflicted_char),

        _ => None,
    }
}
