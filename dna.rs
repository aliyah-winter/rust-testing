fn main() {
  println!("{}", dna_strand("GTAT"))
}

fn dna_strand(dna: &str) -> String {
  dna.chars().map(|char| 
  match char {
    'A' => 'T',
    'T' => 'A',
    'G' => 'C',
    'C' => 'G',
    _ => panic!(),
  }).collect()
}