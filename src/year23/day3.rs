use std::cmp::Ordering::Equal;
use std::fs;
use regex::bytes::Regex;
use super::super::utils;

pub fn solve() {
    let mut file = fs::read("src/year23/day3.txt").unwrap();

    let part_regex = Regex::new(r"[^.\d\n]").unwrap();
    let num_regex = Regex::new(r"\d").unwrap();

    //Calculate grid width.
    let mut width = 0;

    for char in &file {
        if char == &b'\n' {
            break
        }
        width += 1;
    }

    //Remove newlines.
    file.retain(|char| char != &b'\n');

    //A 2D grid representation of the file.
    let map: Vec<&[u8]> = file
        .chunks(width)
        .collect();

    //A list of all the character locations.
    let mut targets: Vec<(usize, usize)> = vec![];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let char = map[row][col];

            if part_regex.is_match(&[char]) {
                targets.push((row, col));
            }
        }
    }

    //List of numbers adjacent to the characters.
    let mut adjacent: Vec<(usize, usize)> = vec![];

    for target in targets {
        for row_off in 0..3 {
            let row = target.0 + row_off - 1;

            //Bounds check.
            if target.0 == 0 && row_off == 0 { continue }
            if row >= map.len() { continue }

            for col_off in 0..3 {
                let col = target.1 + col_off - 1;

                //Bounds check.
                if target.1 == 0 && col_off == 0 { continue }
                if col >= map[row].len() { continue }

                //If this passes, there is a number at (row, col) that is adjacent to a character.
                if num_regex.is_match(&[map[row][col]]) {
                    adjacent.push((row, col));
                }
            }
        }
    }

    adjacent.sort_by(|a, b| {
       if a.0.cmp(&b.0) == Equal {
           return a.1.cmp(&b.1);
       }
        a.0.cmp(&b.0)
    });

    //Removes SOME duplicate pointers.
    adjacent.dedup_by(|a, b| {
        if a.0 != b.0 {
            return false;
        }
        if a.1 == b.1 + 1 || a == b {
            return true;
        }
        false
    });

    let mut total = 0;

    let mut last = (0, 0);

    for mut loc in adjacent {
        let mut string = "".to_string();

        //Move to front of string.
        loop {
            if loc.1 <= 0 {
                break
            }

            if !num_regex.is_match(&[map[loc.0][loc.1 - 1]]) {
                break
            }
            loc.1 -= 1;
        }

        //Checks if this number was already counted.
        if last == loc {
            continue
        }

        last = loc;
        string.push(char::from(map[loc.0][loc.1]));

        loop {
            if loc.1 >= width - 1 {
                break
            }

            if !num_regex.is_match(&[map[loc.0][loc.1 + 1]]) {
                break
            }
            loc.1 += 1;
            string.push(char::from(map[loc.0][loc.1]));
        }

        if string.len() != 0 {
            total += utils::get_num(string.as_str());
        }
    }

    println!("{}", total);
}