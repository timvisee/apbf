extern crate itertools;
extern crate pbr;
extern crate permutator;

use std::cmp;
use std::process::Command;

use itertools::Itertools;
use pbr::ProgressBar;
use permutator::Permutation;

/// The dots we should generate patterns on for brute forcing.
///
/// The indices are row-by-row based, for example:
///
/// ```
/// 0 1 2    00 01 02 03    00 01 02 03 04
/// 3 4 5    04 05 06 07    05 06 07 08 09
/// 6 7 8    08 09 10 11    10 11 12 13 14
///          12 13 14 15    15 16 17 18 19
///                         20 21 22 23 24
/// ```
const DOTS: [u16; 12] = [1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15];

/// The size of the pattern grid, probably 3, 4 or 5.
const GRID_SIZE: u16 = 4;

/// The minimum length of patterns to attempt.
const PATTERN_LEN_MIN: u16 = 4;

/// The maximum length of patterns to attempt.
const PATTERN_LEN_MAX: u16 = 6;

/// The maximum distance between dots in a pattern.
///
/// Imagine the following distances for each dot, where `X` is the center:
/// ```
/// 2 2 2 2 2
/// 2 1 1 1 2
/// 2 1 X 1 2
/// 2 1 1 1 2
/// 2 2 2 2 2
/// ```
const PATTERN_DISTANCE_MAX: u16 = 1;

/// Output normally returned to stdout for a decryption attempt.
///
/// The tool will stop if anything else was returned.
const STDOUT_NORMAL: &str = "Attempting to decrypt data partition via command line.\n";

/// Application entry point.
fn main() {
    // Get a list of dots we can use
    let dots = DOTS;

    // Generate all possible patterns
    println!("Generating possible patterns...");
    let patterns: Vec<_> = (PATTERN_LEN_MIN..=PATTERN_LEN_MAX)
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

/// Test whether the distance between all dots are allowed based on `PATTERN_DISTANCE_MAX`.
///
/// If the distance for some dots is greater, `false` is returned and the pattern should be
/// skipped.
fn valid_distance(dots: &Vec<&u16>) -> bool {
    dots.windows(2)
        .all(|dots| distance(*dots[0], *dots[1]) <= PATTERN_DISTANCE_MAX)
}

/// Determine the distance between two dots.
///
/// See `PATTERN_DISTANCE_MAX`.
fn distance(a: u16, b: u16) -> u16 {
    // Get the dot coordinates
    let a = dot_pos(a);
    let b = dot_pos(b);

    // Determine the distance and return
    cmp::max(
        (a.0 as i32 - b.0 as i32).abs(),
        (a.1 as i32 - b.1 as i32).abs(),
    ) as u16
}

/// Find the (x, y) position for a given dot index.
///
/// If the `GRID_SIZE` is 4, a dot index of `6` will return `(2, 1)`.
fn dot_pos(dot: u16) -> (u16, u16) {
    (dot / GRID_SIZE, dot % GRID_SIZE)
}

/// Generate the pass phrase for the given pattern.
fn gen_phrase(pattern: Vec<&u16>) -> String {
    pattern.iter().map(|p| dot_char(**p)).collect()
}

/// Find the character to use in the passphrase for a given dot index.
fn dot_char(pos: u16) -> char {
    ('1' as u8 + pos as u8) as char
}

/// Render the given pattern in the terminal.
fn render_pat(pattern: &Vec<&u16>) {
    // Create a pattern slug and print it
    let slug = pattern.iter().map(|p| format!("{}", p)).join("-");
    println!("\nPattern: {}", slug);

    // Render the pattern grid
    (0..GRID_SIZE).for_each(|y| {
        (0..GRID_SIZE).for_each(|x| {
            if pattern.contains(&&(y * GRID_SIZE + x)) {
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
    println!("Attempting: '{}'", phrase);

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
