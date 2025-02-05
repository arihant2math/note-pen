use itertools::Itertools;
use note_pen::prelude::*;

pub struct Solution {
    pub soprano: Vec<Note>,
    pub alto: Vec<Note>,
    pub tenor: Vec<Note>,
    pub bass: Vec<Note>,
}

fn get_solution_score(key: &Key, solution: &Solution) -> i32 {
    let mut score = 0;
    let lines = [&solution.soprano, &solution.alto, &solution.tenor];
    for line in lines {
        let result = line.iter()
            .zip(line.iter().skip(1))
            .map(|(a, b)| {
                let mut a_scale_degree = ScaleDegree::from_note(a, key);
                let mut b_scale_degree = ScaleDegree::from_note(b, key);
                while a_scale_degree.is_none() {
                    a_scale_degree = ScaleDegree::from_note(&a.increment_by(1), key);
                }
                while b_scale_degree.is_none() {
                    b_scale_degree = ScaleDegree::from_note(&b.increment_by(1), key);
                }
                (a_scale_degree.unwrap().degree.get() as i32 - b_scale_degree.unwrap().degree.get() as i32).abs()
            })
            .map(|diff| diff.pow(2).min(1))
            .sum::<i32>();
        score += result;
    }
    score
}

fn get_solutions_inner(key: &Key, progression: &[RomanNumeral]) -> Vec<Solution> {
    let mut solutions = vec![];
    let mut bass_line = vec![];
    for attem_chord in progression {
        bass_line.push(attem_chord.chord(&key).notes.first().unwrap().clone());
    }

    let mut chord = progression.first().unwrap().chord(&key);
    // 7th chords need to be shortened
    if chord.notes.len() == 4 {
        chord.notes.remove(0);
    }
    for option in chord.notes.iter().permutations(chord.notes.len()).unique() {
        let soprano = option[0];
        let alto = option[1];
        let tenor = option[2];
        if progression.len() == 1 {
            let bass = bass_line.first().unwrap();
            solutions.push(Solution {
                soprano: vec![soprano.clone()],
                alto: vec![alto.clone()],
                tenor: vec![tenor.clone()],
                bass: vec![bass.clone()],
            });
        } else {
            let mut inner_solutions = get_solutions_inner(key, &progression[1..]);
            for solution in inner_solutions.iter_mut() {
                solution.soprano.insert(0, soprano.clone());
                solution.alto.insert(0, alto.clone());
                solution.tenor.insert(0, tenor.clone());
                solution.bass.insert(0, bass_line.first().unwrap().clone());
            }
            solutions.append(&mut inner_solutions);
        }
    }

    solutions
}

fn get_solutions(key: &Key, progression: &[RomanNumeral]) -> Vec<Solution> {
    get_solutions_inner(key, progression)
}

fn main() {
    let g_major = Key::new_major(Note::new(Alphabet::G, Accidental::Natural, 4)).unwrap();
    let chords = vec![RomanNumeral::major_chord(1, Inversion::ROOT),
                      RomanNumeral::major_chord(5, Inversion::FIRST),
                      RomanNumeral::major_chord(4, Inversion::FIRST),
                      RomanNumeral::major_chord(6, Inversion::ROOT),
                      RomanNumeral::seventh_chord(5, Tonality::Major, Inversion::ROOT),
                      RomanNumeral::major_chord(1, Inversion::ROOT)
    ];

    let solutions = get_solutions(&g_major, &chords);
    let mut best_score = i32::MAX;
    let mut best_solution = None;
    for solution in solutions {
        let score = get_solution_score(&g_major, &solution);
        if score < best_score {
            best_score = score;
            best_solution = Some(solution);
        }
    }

    let best_solution = best_solution.unwrap();
    let voices = [("soprano", best_solution.soprano),
        ("alto", best_solution.alto),
        ("tenor", best_solution.tenor),
        ("bass", best_solution.bass)];
    for (voice_name, voice) in voices.iter() {
        print!("{}: ", voice_name);
        for note in voice {
            print!("{}{} ", note.alphabet, note.accidental.unicode());
        }
        println!();
    }
}