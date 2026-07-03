use cli_table::{Cell, Style, Table, format::Justify, print_stdout};
use rusqlite::Connection;
use std::{error::Error, fs};

#[derive(Debug)]
struct Projects {
    id: i64,
    title: String,
    b_description: String,
    d_description: String,
    progress: String,
    purpose: String,
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let name = "devdeck";
    let mut path = dirs::config_dir().ok_or("No system config file")?;
    path.push(name);
    fs::create_dir_all(&path)?;

    let data = "markdown.db";
    path.push(data);
    let db: &str = path.to_str().unwrap();
    let c = Connection::open(db)?;

    let mut all = c.prepare("SELECT * FROM projects")?;
    let project_iter = all.query_map([], |row| {
        Ok(Projects {
            id: row.get(0)?,
            title: row.get(1)?,
            b_description: row.get(2)?,
            d_description: row.get(3)?,
            progress: row.get(4)?,
            purpose: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for project in project_iter {
        let p = project.unwrap();
        let wrapped_b_desc = textwrap::fill(&p.b_description, 30);
        let wrapped_d_desc = textwrap::fill(&p.d_description, 65);

        data.push(vec![
            p.id.cell().justify(Justify::Right),
            p.title.cell(),
            wrapped_b_desc.cell(),
            wrapped_d_desc.cell(),
            p.progress.cell(),
            p.purpose.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "BASIC DESCRIPTION".cell().bold(true),
            "DETAILED DESCRIPTION".cell().bold(true),
            "PROGRESS".cell().bold(true),
            "MADE FOR".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}
