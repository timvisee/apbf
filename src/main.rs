extern crate itertools;
extern crate pbr;
extern crate permutator;

mod config;

use std::cmp;
use std::process::{self, Command};
use std::thread;
use std::time::Duration;

use itertools::Itertools;
use pbr::ProgressBar;
use permutator::Permutation;
use took::Timer;

use config::*;

/// Output normally returned to stdout for a decryption attempt.
///
/// The tool will stop if anything else was returned.
const STDOUT_NORMAL: &str = "Attempting to decrypt data partition via command line.\n";

/// Partial output returned to stdout on successful decryption.
const STDOUT_SUCCESS: &str = "Data successfully decrypted";

/// Application entry point.
fn main() {
    // Start a timer for the whole process
    let timer = Timer::new();

    // Start brute forcing
    match CODE_TYPE {
        Code::Pattern => brute_force_pattern(&timer),
        Code::Pin => brute_force_pin(&timer),
    }

    // We did not find any code
    println!("\nDone! No code found. Took {}.", timer);
}

/// Brute force pattern.
fn brute_force_pattern(timer: &Timer) {
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

    // Try all patterns, start a timer
    patterns
        .into_iter()
        .inspect(render_pattern)
        .map(|pattern| (generate_phrase(&pattern), pattern))
        .for_each(|(code, pattern)| {
            // Report the phrase to try, show progress bar
            println!("Passphrase: '{}'", code);
            pb.inc();
            println!();

            // Try the phrase, report on success
            if try_phrase(&code) {
                println!("\nSuccess! Took {}.", timer);
                println!("Here is your pattern in order:");
                render_pattern_steps(&pattern);
                process::exit(0);
            }

            // Wait for the next attempt
            thread::sleep(Duration::from_millis(ATTEMPT_TIMEOUT));
        });
}

/// Brute force pin.
fn brute_force_pin(timer: &Timer) {
    // Set the minimum and maximum
    let min = 0;
    let max = 9999;
    let count = max - min + 1;

    // Initialse brute forcing
    println!("Codes to try: {}", max - min + 1);
    let mut pb = ProgressBar::new(count);

    // Try all pins
    for pin in min..=max {
        let code = format!("{:04}", pin);

        // Report the phrase to try, show progress bar
        println!("Passphrase: '{}'", code);
        pb.inc();
        println!();

        // Try the phrase, report on success
        if try_phrase(&code) {
            println!("\nSuccess! Took {}.", timer);
            println!("Here is your PIN code: {}", code);
            process::exit(0);
        }

        // Wait for the next attempt
        thread::sleep(Duration::from_millis(ATTEMPT_TIMEOUT));
    }
}

/// Code type.
pub enum Code {
    /// A pattern.
    Pattern,

    /// A code.
    Pin,
}

/// Try the given passphrase generated based on a pattern.
///
/// Returns `true` if decryption succeeded, false if not.
///
/// Panics when unexpected output is returned (possibly when an item is found).
fn try_phrase(phrase: &str) -> bool {
    // Build and invoke the decrypt command, collect results
    let out = Command::new("adb")
        .arg("shell")
        .arg(format!("twrp decrypt '{}'", phrase))
        .output()
        .expect("failed to invoke decrypt command");
    let status = out.status;
    let stdout = String::from_utf8(out.stdout).expect("output is not in valid UTF-8 format");
    let stderr = String::from_utf8(out.stderr).expect("output is not in valid UTF-8 format");

    // Check for success
    if status.success() && stdout.contains(STDOUT_SUCCESS) && stderr == "" {
        return true;
    }

    // Regular output, continue
    if status.success() && stdout == STDOUT_NORMAL && stderr == "" {
        return false;
    }

    // Report and exit
    println!("An error occurred, heres the output for the decryption attempt:");
    println!("- status: {}", status);
    println!("- stdout: {}", stdout);
    println!("- stderr: {}", stderr);
    process::exit(1);
}

/// Find the character to use in the passphrase for a given dot index.
fn dot_char(pos: u16) -> char {
    ('1' as u8 + pos as u8) as char
}

/// Generate the pass phrase for the given pattern.
fn generate_phrase(pattern: &[&u16]) -> String {
    pattern.iter().map(|p| dot_char(**p)).collect()
}

/// Render the given pattern in the terminal.
fn render_pattern(pattern: &Vec<&u16>) {
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

/// Render the steps for performing the pattern to the user in the terminal.
fn render_pattern_steps(pattern: &Vec<&u16>) {
    // Render the pattern grid
    (0..GRID_SIZE).for_each(|y| {
        (0..GRID_SIZE).for_each(|x| {
            let index = pattern.iter().position(|p| p == &&(y * GRID_SIZE + x));
            if let Some(index) = index {
                print!("{} ", index + 1);
            } else {
                print!("· ");
            }
        });
        println!();
    })
}

/// Find the (x, y) position for a given dot index.
///
/// If the `GRID_SIZE` is 4, a dot index of `6` will return `(2, 1)`.
fn dot_position(dot: u16) -> (u16, u16) {
    (dot / GRID_SIZE, dot % GRID_SIZE)
}

/// Determine the distance between two dots.
///
/// See `PATTERN_DISTANCE_MAX`.
fn distance(a: u16, b: u16) -> u16 {
    // Get the dot coordinates
    let a = dot_position(a);
    let b = dot_position(b);

    // Determine the distance and return
    cmp::max(
        (a.0 as i32 - b.0 as i32).abs(),
        (a.1 as i32 - b.1 as i32).abs(),
    ) as u16
}

/// Test whether the distance between all dots are allowed based on `PATTERN_DISTANCE_MAX`.
///
/// If the distance for some dots is greater, `false` is returned and the pattern should be
/// skipped.
fn valid_distance(dots: &Vec<&u16>) -> bool {
    dots.windows(2)
        .all(|dots| distance(*dots[0], *dots[1]) <= PATTERN_DISTANCE_MAX)
}

/// Unit tests to validate some of the functions above.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_char() {
        assert_eq!(dot_char(0), '1');
        assert_eq!(dot_char(1), '2');
        assert_eq!(dot_char(2), '3');
        assert_eq!(dot_char(8), '9');
        assert_eq!(dot_char(9), ':');
        assert_eq!(dot_char(15), '@');
    }

    #[test]
    fn test_generate_phrase() {
        assert_eq!(generate_phrase(&[]), "");
        assert_eq!(generate_phrase(&[&0, &1, &2]), "123");
        assert_eq!(
            generate_phrase(&[&0, &1, &2, &3, &4, &5, &6, &7, &8]),
            "123456789"
        );
        assert_eq!(generate_phrase(&[&9, &15]), ":@");
    }

    #[test]
    fn test_dot_position() {
        assert_eq!(dot_position(0), (0, 0));
        assert_eq!(dot_position(GRID_SIZE), (1, 0));
        if GRID_SIZE >= 3 {
            assert_eq!(dot_position(GRID_SIZE + 2), (1, 2));
        }
        assert_eq!(dot_position(GRID_SIZE * 2), (2, 0));
        assert_eq!(
            dot_position(GRID_SIZE * GRID_SIZE - 1),
            (GRID_SIZE - 1, GRID_SIZE - 1)
        );
    }

    #[test]
    fn test_distance() {
        // Distance of 0
        assert_eq!(distance(0, 0), 0);
        assert_eq!(distance(1, 1), 0);
        assert_eq!(distance(99, 99), 0);
        assert_eq!(distance(100, 100), 0);

        // Distance of 1
        assert_eq!(distance(0, 1), 1);
        assert_eq!(distance(0, GRID_SIZE), 1);

        // Distance of 2
        if GRID_SIZE >= 3 {
            assert_eq!(distance(0, 2), 2);
        }
        assert_eq!(distance(0, GRID_SIZE * 2), 2);
        assert_eq!(distance(0, GRID_SIZE * GRID_SIZE - 1), GRID_SIZE - 1);
    }

    #[test]
    fn test_valid_distance() {
        assert!(valid_distance(&vec![&0, &0, &PATTERN_DISTANCE_MAX]));
        if PATTERN_DISTANCE_MAX <= GRID_SIZE {
            assert!(!valid_distance(&vec![&0, &0, &(PATTERN_DISTANCE_MAX + 1)]));
        }
        if GRID_SIZE >= 3 && PATTERN_DISTANCE_MAX >= 1 {
            assert!(valid_distance(&vec![&0, &1, &2]));
        }
    }
}
