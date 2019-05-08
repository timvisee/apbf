extern crate itertools;
extern crate pbr;
extern crate permutator;

use std::cmp;
use std::process::Command;

use itertools::Itertools;
use pbr::ProgressBar;
use permutator::Permutation;

const DOTS: [u16; 12] = [1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15];
const GRID_SIZE: u16 = 4;
const PAT_LEN_MIN: u16 = 4;
const PAT_DIST_MAX: u16 = 1;
const PAT_LEN_MAX: u16 = 6;

const STDOUT_NORMAL: &str = "Attempting to decrypt data partition via command line.\n";

fn main() {
    // Get a list of dots we can use
    let dots = DOTS;

    // Generate all possible patterns
    println!("Generating possible patterns...");
    let patterns: Vec<_> = (PAT_LEN_MIN..=PAT_LEN_MAX)
        .flat_map(|n| {
            dots.iter().combinations(n as usize).flat_map(|mut dots| {
                dots.permutation()
                    .filter(valid_distance)
                    .collect::<Vec<_>>()
            })
        })
        .collect();

    // Initialse brute forcing
    println!("Patterns to try: {}", patterns.len());
    let mut pb = ProgressBar::new(patterns.len() as u64);

    // Try all patterns
    patterns
        .into_iter()
        .map(|pat| pat.clone())
        .inspect(render_pat)
        .map(gen_phrase)
        .for_each(|code| {
            try_phrase(&code);
            pb.inc();
        });

    println!("\nDone!");
}

fn valid_distance(dots: &Vec<&u16>) -> bool {
    dots.windows(2)
        .all(|dots| distance(*dots[0], *dots[1]) <= PAT_DIST_MAX)
}

fn distance(a: u16, b: u16) -> u16 {
    let a = dot_pos(a);
    let b = dot_pos(b);

    cmp::max(
        (a.0 as i32 - b.0 as i32).abs(),
        (a.1 as i32 - b.1 as i32).abs(),
    ) as u16
}

fn dot_pos(dot: u16) -> (u16, u16) {
    (dot / GRID_SIZE, dot % GRID_SIZE)
}

fn gen_phrase(dots: Vec<&u16>) -> String {
    dots.iter().map(|p| dot_char(**p)).collect()
}

fn dot_char(pos: u16) -> char {
    ('1' as u8 + pos as u8) as char
}

fn render_pat(dots: &Vec<&u16>) {
    println!();
    (0..GRID_SIZE).for_each(|y| {
        (0..GRID_SIZE).for_each(|x| {
            if dots.contains(&&(y * GRID_SIZE + x)) {
                print!("●");
            } else {
                print!("○");
            }
        });
        println!();
    })
}

/// Try the given passphrase generated based on a pattern.
///
/// Panics when unexpected output is returned (possibly when an item is found).
fn try_phrase(phrase: &str) {
    println!("Attempting: {}", phrase);

    // Build the decrypt command
    let out = Command::new("adb")
        .arg("shell")
        .arg("twrp")
        .arg("decrypt")
        .arg(format!("'{}'", phrase))
        .output()
        .expect("failed to run decrypt command");

    let status = out.status;
    let stdout = String::from_utf8(out.stdout).expect("output is not in valid UTF-8 format");
    let stderr = String::from_utf8(out.stderr).expect("output is not in valid UTF-8 format");

    if status.success() && stdout == STDOUT_NORMAL {
        return;
    }

    println!("status: {}", stdout);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    panic!("got unexpected output");
}
