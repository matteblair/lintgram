extern crate yaml_rust;

pub mod linter {

    use yaml_rust::{Yaml, YamlLoader};
    use std::error::Error;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum IssueSeverity {
        Warning,
        Error,
    }

    pub struct Issue {
        severity: IssueSeverity,
        description: String,
        context: String,
        line_number: usize,
        column_number: usize,
    }

    impl Issue {
        pub fn severity(&self) -> IssueSeverity {
            self.severity
        }
        pub fn description(&self) -> &str {
            &self.description
        }
        pub fn context(&self) -> &str {
            &self.context
        }
        pub fn line_number(&self) -> usize {
            self.line_number
        }
        pub fn column_number(&self) -> usize {
            self.column_number
        }
    }

    pub fn lint_scene_file(scene_file_string: &str) -> Result<Vec<Issue>, Box<Error>> {
        let documents = YamlLoader::load_from_str(&scene_file_string)?;
        let yaml = &documents[0];
        // Do stuff with the YAML!
        let mut issues : Vec<Issue> = Vec::new();
        // Check that the root element is a mapping.
        if let &Yaml::Hash(ref _mapping) = yaml {
            // Ok, let's check the elements of the mapping.
            println!("Found a mapping :)");
        } else {
            // That's an error!
            println!("Oops, root node should be a mapping.");
            let issue = Issue {
                severity: IssueSeverity::Error,
                description: String::from("Root node should be a mapping."),
                context: String::new(),
                line_number: 0,
                column_number: 0,
            };
            issues.push(issue);
        }
        Ok(issues)
    }
}

#[cfg(test)]
mod tests {
    use linter::{IssueSeverity, lint_scene_file};
    #[test]
    fn root_must_be_mapping() {
        let issues = lint_scene_file("[]").unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity(), IssueSeverity::Error);
    }
}
