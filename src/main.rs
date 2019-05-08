extern crate itertools;
extern crate pbr;
extern crate permutator;

use std::process::Command;

use itertools::Itertools;
use pbr::ProgressBar;
use permutator::Permutation;

const DOTS: [u16; 8] = [2, 3, 6, 7, 10, 11, 14, 15];
const GRID_SIZE: usize = 4;
const PAT_LEN_MIN: usize = 3;
const PAT_LEN_MAX: usize = 6;

const FAIL_OUTPUT: &str = "Attempting to decrypt data partition via command line.\n";

fn main() {
    let dots = DOTS.clone();

    println!("Generating possible patterns...");
    let patterns: Vec<_> = (PAT_LEN_MIN..=PAT_LEN_MAX)
        .flat_map(|n| {
            dots.iter().combinations(n).flat_map(|mut dots| {
                dots.permutation()
                    .map(|pat| pat.clone())
                    .collect::<Vec<_>>()
            })
        })
        .collect();

    println!("Patterns to try: {}", patterns.len());
    let mut pb = ProgressBar::new(patterns.len() as u64);

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
            if dots.contains(&&((y * GRID_SIZE + x) as u16)) {
                print!("●");
            } else {
                print!("○");
            }
        });
        println!();
    })
}

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

    if status.success() && stdout == FAIL_OUTPUT {
        return;
    }

    println!("status: {}", stdout);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    panic!("got unexpected output");
}
