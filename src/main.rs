use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::result::Result;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TableRow {
    #[header("tense")]
    tense: String,
    #[header("yo")]
    form1s: String,
    #[header("tú")]
    form2s: String,
    #[header("él/ella/Ud.")]
    form3s: String,
    #[header("nosotros")]
    form1p: String,
    #[header("vosotros")]
    form2p: String,
    #[header("ellos/ellas/Uds.")]
    form3p: String,
}

#[derive(Debug, Clone)]
struct VerbForms {
    form1s: String,
    form2s: String,
    form3s: String,
    form1p: String,
    form2p: String,
    form3p: String,
}

#[derive(Debug, Clone)]
struct Verb {
    infinitive: String,
    infinitive_english: String,
    verb_english: String,
    mood: String,
    tense: String,
    forms: VerbForms,
}

fn read_verbs_csv() -> Vec<Verb> {
    let csv = include_str!("jehle_verb_database.csv");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .quoting(true)
        .from_reader(csv.as_bytes());
    let records = reader.records().map(|row| {
        let rowc = row?.clone();
        Ok(Verb {
            infinitive: rowc.get(0).unwrap_or_default().to_string(),
            infinitive_english: rowc.get(1).unwrap_or_default().to_string(),
            mood: rowc.get(2).unwrap_or_default().to_string(),
            tense: rowc.get(4).unwrap_or_default().to_string(),
            verb_english: rowc.get(6).unwrap_or_default().to_string(),
            forms: VerbForms {
                form1s: rowc.get(7).unwrap_or_default().to_string(),
                form2s: rowc.get(8).unwrap_or_default().to_string(),
                form3s: rowc.get(9).unwrap_or_default().to_string(),
                form1p: rowc.get(10).unwrap_or_default().to_string(),
                form2p: rowc.get(11).unwrap_or_default().to_string(),
                form3p: rowc.get(12).unwrap_or_default().to_string(),
            },
        })
    });
    let result: Result<Vec<Verb>, Box<dyn Error>> = records.collect();
    result.ok().expect("Failed to parse")
}

fn group_verbs(verbs: Vec<Verb>, f: fn(&Verb) -> String) -> HashMap<String, Vec<Verb>> {
    verbs.into_iter().fold(HashMap::new(), |mut map, verb| {
        let value = map.entry(f(&verb).clone()).or_insert(Vec::new());
        value.push(verb);
        map
    })
}

fn verbs() -> HashMap<String, Vec<Verb>> {
    let verbs = read_verbs_csv();
    group_verbs(verbs, |verb| verb.infinitive.clone())
}

fn print_verb(verbs: HashMap<String, Vec<Verb>>, verb: &str, mood: &str) {
    let conjugations = verbs.get(verb).expect("¿Cómo?");
    let verbs_for_mood = conjugations
        .into_iter()
        .cloned()
        .filter(|verb| verb.mood == mood)
        .collect();

    let verbs_by_tense = group_verbs(verbs_for_mood, |verb| verb.tense.clone());

    let mut data = Vec::new();
    for (tense, verbs) in verbs_by_tense {
        for verb in verbs {
            let row = TableRow {
                tense: tense.clone(),
                form1s: verb.forms.form1s,
                form2s: verb.forms.form2s,
                form3s: verb.forms.form3s,
                form1p: verb.forms.form1p,
                form2p: verb.forms.form2p,
                form3p: verb.forms.form3p,
            };
            data.push(row);
        }
    }
    let forms_table = Table::new(&data).to_string();
    print!("{}", forms_table);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verb = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("¿Qué verbo?");
        std::process::exit(1);
    });
    let verbs = verbs();
    Ok(print_verb(verbs, &verb, "Indicativo"))
}
