use itertools::Itertools;
use std::collections::HashSet;
use std::io::{prelude::*, stdin};

struct Record {
    label: String,
    sequence: Vec<u8>,
}

impl Record {
    fn distance(&self, other: &Record) -> f32 {
        let percent_difference = Record::percent_difference_of_nucleotides(self, other);
        -0.75 * (1.0 - (percent_difference * (4.0 / 3.0))).log2()
    }

    fn percent_difference_of_nucleotides(&self, other: &Record) -> f32 {
        let diff = self
            .sequence
            .iter()
            .zip(other.sequence.iter())
            .filter(|(a, b)| Record::is_nucleotide(a) && Record::is_nucleotide(b))
            .filter(|(a, b)| a != b)
            .count() as f32;

        let len = self
            .sequence
            .iter()
            .zip(other.sequence.iter())
            .filter(|(a, b)| Record::is_nucleotide(a) && Record::is_nucleotide(b))
            .count() as f32;
        diff / len
    }

    fn is_nucleotide(c: &u8) -> bool {
        let nuc: HashSet<u8> = vec![65, 67, 71, 84].into_iter().collect();
        nuc.contains(c)
    }
}

fn main() {
    read_fasta_records()
        .iter()
        .combinations(2)
        .map(|record_pair| {
            (
                record_pair[0],
                record_pair[1],
                record_pair[0].distance(record_pair[1]),
            )
        })
        .filter(|(.., distance)| *distance < 0.04)
        .for_each(|(record1, record2, distance)| {
            println!("{} {} {}", record1.label, record2.label, distance)
        });
}

fn read_fasta_records() -> Vec<Record> {
    stdin()
        .lock()
        .lines()
        .tuples()
        .map(|(label, sequence)| Record {
            label: label.unwrap().trim().to_owned(),
            sequence: sequence.unwrap().trim().as_bytes().to_vec(),
        })
        .collect()
}
