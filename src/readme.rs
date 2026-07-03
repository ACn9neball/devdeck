use std::{
    error::Error,
    fs::{self, remove_file, write},
    path::PathBuf,
};

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Projects {
    _id: i64,
    title: String,
    b_description: String,
    d_description: String,
    _progress: String,
    path: String,
    repository: String,
    purpose: String,
}

#[derive(Debug)]
struct Features {
    _id: i64,
    feature: String,
    unique_id: i64,
}

#[derive(Debug)]
struct Languages {
    _id: i64,
    language: String,
    unique_id: i64,
}

pub fn create(id: i64) -> Result<(), Box<dyn Error>> {
    let mut title = String::new();
    let mut basic_description = String::new();
    let mut detailed_description = String::new();
    let mut directory = String::new();
    let mut repository = String::new();
    let mut purpose = String::new();
    let mut language_list: Vec<String> = vec![];
    let mut feature_list: Vec<String> = vec![];

    let name = "devdeck";
    let mut path = dirs::config_dir().ok_or("No system config file")?;
    path.push(name);
    fs::create_dir_all(&path)?;

    let data = "markdown.db";
    path.push(data);
    let db: &str = path.to_str().unwrap();
    let c = Connection::open(db)?;
    let mut projects = c.prepare("SELECT * FROM projects WHERE id = ?1")?;
    let mut languages = c.prepare("SELECT * FROM languages")?;
    let mut features = c.prepare("SELECT * FROM features")?;
    let project_iter = projects.query_map([id], |row| {
        Ok(Projects {
            _id: row.get(0)?,
            title: row.get(1)?,
            b_description: row.get(2)?,
            d_description: row.get(3)?,
            _progress: row.get(4)?,
            path: row.get(5)?,
            repository: row.get(6)?,
            purpose: row.get(7)?,
        })
    })?;

    let language_iter = languages.query_map([], |row| {
        Ok(Languages {
            _id: row.get(0)?,
            language: row.get(1)?,
            unique_id: row.get(2)?,
        })
    })?;

    let feature_iter = features.query_map([], |row| {
        Ok(Features {
            _id: row.get(0)?,
            feature: row.get(1)?,
            unique_id: row.get(2)?,
        })
    })?;

    for project in project_iter {
        let p = project.unwrap();
        title = p.title;
        basic_description = p.b_description;
        detailed_description = p.d_description;
        directory = p.path;
        repository = p.repository;
        purpose = p.purpose;
    }

    for language in language_iter {
        let l = language.unwrap();
        let unique_id: i64 = l.unique_id;
        if id == unique_id {
            language_list.push(l.language);
        }
    }

    for feature in feature_iter {
        let f = feature.unwrap();
        let unique_id: i64 = f.unique_id;
        if id == unique_id {
            feature_list.push(f.feature);
        }
    }

    let contents = readme_writer(
        title,
        basic_description,
        detailed_description,
        language_list,
        feature_list,
        repository.clone(),
        purpose,
    );

    if !directory.is_empty() && !repository.is_empty() {
        let mut path = PathBuf::from(directory);
        if path.exists() {
            path.push("README.md");
            if !path.exists() {
                write(path, contents).expect("");
            } else {
                remove_file(&path).expect("");
                write(path, contents).expect("");
            }
        }
    }
    Ok(())
}

fn readme_writer(
    title: String,
    b_description: String,
    d_description: String,
    language_list: Vec<String>,
    feature_list: Vec<String>,
    repo: String,
    purpose: String,
) -> String {
    let mut languages = String::new();
    for language in language_list {
        languages = format!("{}\n | **{}** |", languages, language);
    }

    let mut features = String::new();
    for i in 0..feature_list.len() {
        features = format!(
            "{}\n* **Core Feature {}:** {}",
            features,
            i + 1,
            feature_list[i]
        );
    }
    let content = match purpose.as_str() {
        "Personal" => {
            format!(
                r#"# {}

---

## 📱 Basic Description

{}

---

## 🔍 Detailed Description

{}

---

## 🚀 Features

{}

---

## 🛠️ Languages & Tools Used

List the programming languages, primary libraries, or frameworks that power the project.

| :--- |
{}
| ---: |

---

## 💻 Quick Start

```bash
# Command to clone, build, or run the app
git clone {}
cd {}/
cargo install --path
{}
```"#,
                title,
                b_description,
                split(d_description),
                features,
                languages,
                repo,
                title,
                title.to_lowercase()
            )
        }
        "School" => String::new(),
        "Work" => {
            format!(
                r#"
# {}

{}

## Description

{}

## Getting Started

### Dependencies

* Describe any prerequisites, libraries, OS version, etc., needed before installing program.
* ex. Windows 10

### Installing

* How/where to download your program
* Any modifications needed to be made to files/folders

### Executing program

* How to run the program
* Step-by-step bullets
```
git clone {}
cd {}/
cargo install --path
{}
```

## Help

Any advise for common problems or issues.
```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. N9neball
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

* 0.2
    * Various bug fixes and optimizations
    * See [commit change]() or See [release history]()
* 0.1
    * Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
            "#,
                title,
                b_description,
                split(d_description),
                repo,
                title,
                title.to_lowercase(),
            )
        }
        _ => String::new(),
    };

    content
}

pub fn split(description: String) -> String {
    let mut result = String::new();
    let mut line_length = 0;

    for word in description.split_whitespace() {
        let word_len = word.len();

        if line_length == 0 {
            result.push_str(word);
            line_length = word_len;
        } else if line_length + 1 + word_len > 150 {
            result.push('\n');
            result.push_str(word);
            line_length = word_len;
        } else {
            result.push(' ');
            result.push_str(word);
            line_length += 1 + word_len;
        }
    }

    result
}
