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
}

pub fn display(name: String) -> Result<(), Box<dyn Error>> {
    let n = "devdeck";
    let mut path = dirs::config_dir().ok_or("No system config file")?;
    path.push(n);
    fs::create_dir_all(&path)?;

    let data = "markdown.db";
    path.push(data);
    let db: &str = path.to_str().unwrap();
    let c = Connection::open(db)?;

    let mut all = c.prepare("SELECT * FROM projects WHERE title LIKE ?1")?;
    let sp = format!("%{}%", name);
    let project_iter = all.query_map([&sp], |row| {
        Ok(Projects {
            id: row.get(0)?,
            title: row.get(1)?,
            b_description: row.get(2)?,
            d_description: row.get(3)?,
            progress: row.get(4)?,
        })
    })?;
    let mut data = Vec::new();
    for project in project_iter {
        let p = project.unwrap();
        let wrapped_b_desc = textwrap::fill(&p.b_description, 30);
        let wrapped_d_desc = textwrap::fill(&p.d_description, 70);

        data.push(vec![
            p.id.cell().justify(Justify::Right),
            p.title.cell(),
            wrapped_b_desc.cell(),
            wrapped_d_desc.cell(),
            p.progress.cell(),
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
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}
