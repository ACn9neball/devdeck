use std::{cmp::max, error::Error, fs};

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Projects {
    id: i64,
    title: String,
    b_description: String,
    d_description: String,
    progress: String,
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

pub fn view(id: i64) -> Result<(), Box<dyn Error>> {
    let name = "devdeck";
    let mut path = dirs::config_dir().ok_or("No system config file")?;
    path.push(name);
    fs::create_dir_all(&path)?;

    let data = "markdown.db";
    path.push(data);
    let db: &str = path.to_str().unwrap();
    let c = Connection::open(db)?;
    let mut projects = c.prepare("SELECT * FROM projects")?;
    let mut languages = c.prepare("SELECT * FROM languages")?;
    let mut features = c.prepare("SELECT * FROM features")?;
    let project_iter = projects.query_map([], |row| {
        Ok(Projects {
            id: row.get(0)?,
            title: row.get(1)?,
            b_description: row.get(2)?,
            d_description: row.get(3)?,
            progress: row.get(4)?,
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
        let unique_id: i64 = p.id;
        if id == unique_id {
            println!("{:=^40}", " Project Details ");
            println!("{:<20} {}", "Title:", p.title);
            println!("{:<20} {}", "Progress:", p.progress);
            println!("{:<20} {}", "Purpose:", p.purpose);
            println!("{:-<40}", "");
            println!("Basic Description:\n  {}", p.b_description);
            println!("\nDetailed Description:\n  {}", p.d_description);
            println!("{:-<40}", "");
            println!("{:<20} {}", "Directory:", p.path);
            if p.repository.is_empty() {
                println!("{:<20} None", "Repository:");
            } else {
                println!("{:<20} {}", "Repository:", p.repository);
            }
            println!("{:-<40}", "");
        }
    }

    let mut language_vec: Vec<String> = vec![];
    for language in language_iter {
        let l = language.unwrap();
        let unique_id: i64 = l.unique_id;
        if id == unique_id {
            language_vec.push(l.language);
        }
    }

    let mut feature_vec: Vec<String> = vec![];
    for feature in feature_iter {
        let f = feature.unwrap();
        let unique_id: i64 = f.unique_id;
        if id == unique_id {
            feature_vec.push(f.feature);
        }
    }

    println!("{:<25} {:<25}", "LANGUAGES", "FEATURES");
    println!("{:<25} {:<25}", "----------", "---------");
    let max_len = max(language_vec.len(), feature_vec.len());
    for i in 0..max_len {
        let language = language_vec.get(i).map(|s| s.as_str()).unwrap_or("");
        let feature = feature_vec.get(i).map(|s| s.as_str()).unwrap_or("");

        println!("{:<25} {:<25}", language, feature);
    }

    println!("{:=^40}", "");

    Ok(())
}
