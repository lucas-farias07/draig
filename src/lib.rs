use std::fs;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

pub fn create(repository: String) -> io::Result<()> {
    let repo_path = Path::new(&repository);

    if !repo_path.exists() {
        if let Err(e) = fs::create_dir(&repo_path) {
            eprintln!(
                "Failed to create repository directory '{}': {}",
                repo_path.display(),
                e
            );
            return Err(e);
        }
    }

    let directory = repo_path.join(".draig");

    if let Err(e) = fs::create_dir(&directory) {
        eprintln!(
            "Failed to create '.draig' directory at '{}': {}",
            directory.display(),
            e
        );
        return Err(e);
    }

    let snapshots_path = directory.join("snapshots");
    if let Err(e) = fs::create_dir(&snapshots_path) {
        eprintln!(
            "Failed to create 'snapshots' directory at '{}': {}",
            snapshots_path.display(),
            e
        );
        return Err(e);
    }

    let default_rules_path = Path::new("layout/rules");
    let defaultrules_file = match fs::File::open(default_rules_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Failed to open default rules file at '{}': {}",
                default_rules_path.display(),
                e
            );
            return Err(e);
        }
    };
    let mut default_rules = BufReader::new(defaultrules_file);

    let rules_path = directory.join("rules");
    let rules_file = match fs::File::create(&rules_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Failed to create rules file at '{}': {}",
                rules_path.display(),
                e
            );
            return Err(e);
        }
    };
    let mut rules = BufWriter::new(rules_file);

    if let Err(e) = io::copy(&mut default_rules, &mut rules) {
        eprintln!("Failed to copy rules to '{}': {}", rules_path.display(), e);
        return Err(e);
    }

    Ok(())
}

