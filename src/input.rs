use crate::readme::{create, split};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;
use rusqlite::Connection;
use std::fs;

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

pub fn start(function: i64, id: i64) -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let result = run(&mut terminal, function, id);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal, function: i64, id: i64) -> color_eyre::Result<()> {
    let mut switch: usize = 0;
    let mut complete = false;
    let mut edt_title = TextArea::default();
    let mut edt_b = TextArea::default();
    let mut edt_d = TextArea::default();
    let mut edt_features = TextArea::default();
    let mut edt_languages = TextArea::default();
    let mut edt_progress = TextArea::default();
    let mut edt_path = TextArea::default();
    let mut edt_repository = TextArea::default();
    let mut edt_purpose = TextArea::default();

    if function == 1 {
        let name = "devdeck";
        let mut path = dirs::config_dir().ok_or("No system config file").unwrap();
        path.push(name);
        fs::create_dir_all(&path).unwrap();

        let data = "markdown.db";
        path.push(data);
        let db: &str = path.to_str().unwrap();
        let c = Connection::open(db)?;
        let mut projects = c.prepare("SELECT * FROM projects WHERE id = ?1")?;
        let mut languages = c.prepare("SELECT * FROM languages")?;
        let mut features = c.prepare("SELECT * FROM features")?;
        let project_iter = projects.query_map([id], |row| {
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
            edt_title.insert_str(p.title);
            edt_b.insert_str(p.b_description);
            edt_d.insert_str(split(p.d_description));
            edt_progress.insert_str(p.progress);
            edt_path.insert_str(p.path);
            edt_repository.insert_str(p.repository);
            edt_purpose.insert_str(p.purpose);
        }

        for language in language_iter {
            let l = language.unwrap();
            let unique_id: i64 = l.unique_id;
            if id == unique_id {
                edt_languages.insert_str(l.language);
                edt_languages.insert_newline();
            }
        }

        for feature in feature_iter {
            let f = feature.unwrap();
            let unique_id: i64 = f.unique_id;
            if id == unique_id {
                edt_features.insert_str(f.feature);
                edt_features.insert_newline();
            }
        }
    }

    let colors = vec![
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
    ];

    loop {
        terminal.draw(|frame| {
            render(
                frame,
                &mut edt_title,
                &mut edt_b,
                &mut edt_d,
                &mut edt_features,
                &mut edt_languages,
                &mut edt_progress,
                &mut edt_path,
                &mut edt_repository,
                &mut edt_purpose,
                complete,
                switch,
                colors.clone(),
            )
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        if switch != 8 {
                            switch += 1;
                        } else {
                            switch = 0;
                        }
                        continue;
                    }
                    KeyCode::BackTab => {
                        if switch != 0 {
                            switch -= 1;
                        } else {
                            switch = 8;
                        }
                        continue;
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        let title = edt_title.lines().join("\n");
                        let basic = edt_b.lines().join("\n");
                        let detailed = edt_d.lines().join("\n");
                        let features = edt_features.lines().join("\n");
                        let languages = edt_languages.lines().join("\n");
                        let progress = edt_progress.lines().join("\n");
                        let path = edt_path.lines().join("\n");
                        let repository = edt_repository.lines().join("\n");
                        let purpose = edt_purpose.lines().join("\n");

                        if !title.is_empty()
                            && !basic.is_empty()
                            && !detailed.is_empty()
                            && !features.is_empty()
                            && !languages.is_empty()
                            && !progress.is_empty()
                            && !purpose.is_empty()
                        {
                            complete = true;
                        } else {
                            complete = false;
                        }

                        if complete == true {
                            let name = "devdeck";
                            let mut p = dirs::config_dir().ok_or("No system config file").unwrap();
                            p.push(name);
                            fs::create_dir_all(&p).unwrap();

                            let data = "markdown.db";
                            p.push(data);
                            let db: &str = p.to_str().unwrap();
                            let c = Connection::open(db)?;
                            let project = Projects {
                                id: 0,
                                title: title,
                                b_description: basic,
                                d_description: detailed,
                                progress: progress,
                                path: path,
                                repository: repository,
                                purpose: purpose,
                            };
                            let mut projects = c.prepare("SELECT * FROM projects")?;
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
                            if function == 0 {
                                c.execute(
                                "INSERT INTO projects (title, bDescription, dDescription, progress, path, repository, purpose) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                                (&project.title, &project.b_description, split(project.d_description), &project.progress, &project.path, &project.repository, &project.purpose),
                            )?;
                            } else if function == 1 {
                                c.execute(
                                    "UPDATE projects SET 
                                    title = COALESCE(NULLIF(?2, ''), title),
                                    bDescription = COALESCE(NULLIF(?3, 0), bDescription),
                                    dDescription = COALESCE(NULLIF(?4, ''),  dDescription),
                                    progress = COALESCE(NULLIF(?5, ''), progress),
                                    path = COALESCE(NULLIF(?6, ''), path),
                                    repository = COALESCE(NULLIF(?7, ''), repository),
                                    purpose = COALESCE(NULLIF(?8, ''), purpose)
                                    WHERE id = ?1",
                                    (
                                        id,
                                        &project.title,
                                        &project.b_description,
                                        split(project.d_description),
                                        &project.progress,
                                        &project.path,
                                        &project.repository,
                                        &project.purpose,
                                    ),
                                )?;
                            }
                            if function == 1 {
                                let name = "devdeck";
                                let mut p =
                                    dirs::config_dir().ok_or("No system config file").unwrap();
                                p.push(name);
                                fs::create_dir_all(&p).unwrap();

                                let data = "markdown.db";
                                p.push(data);
                                let db: &str = p.to_str().unwrap();
                                let c = Connection::open(db)?;
                                let mut languages = c.prepare("SELECT * FROM languages")?;
                                let language_iter = languages.query_map([], |row| {
                                    Ok(Languages {
                                        _id: row.get(0)?,
                                        language: row.get(1)?,
                                        unique_id: row.get(2)?,
                                    })
                                })?;
                                for language in language_iter {
                                    let l = language.unwrap();
                                    let unique_id = l.unique_id;
                                    if unique_id == id {
                                        c.execute("DELETE FROM languages WHERE id = ?1", (l._id,))?;
                                    }
                                }

                                let mut features = c.prepare("SELECT * FROM features")?;
                                let feature_iter = features.query_map([], |row| {
                                    Ok(Features {
                                        _id: row.get(0)?,
                                        feature: row.get(1)?,
                                        unique_id: row.get(2)?,
                                    })
                                })?;
                                for feature in feature_iter {
                                    let f = feature.unwrap();
                                    let unique_id = f.unique_id;
                                    if unique_id == id {
                                        c.execute("DELETE FROM features WHERE id = ?1", (f._id,))?;
                                    }
                                }
                            }

                            let mut id: i64 = 0;
                            for pro in project_iter {
                                let p = pro.unwrap();
                                id = p.id;
                            }

                            for i in 0..features.lines().count() {
                                let mut feat = features.lines();
                                let feature = Features {
                                    _id: 0,
                                    feature: feat.nth(i).expect("").to_string(),
                                    unique_id: id,
                                };
                                c.execute(
                                    "INSERT INTO features (feature, unique_id) VALUES (?1, ?2)",
                                    (&feature.feature, &feature.unique_id),
                                )?;
                            }

                            for i in 0..languages.lines().count() {
                                let mut lang = languages.lines();
                                let language = Languages {
                                    _id: 0,
                                    language: lang.nth(i).expect("").to_string(),
                                    unique_id: id,
                                };
                                c.execute(
                                    "INSERT INTO languages (language, unique_id) VALUES (?1, ?2)",
                                    (&language.language, &language.unique_id),
                                )?;
                            }
                            create(id).expect("");
                            break;
                        }
                    }
                    _ => {
                        let title = edt_title.lines().join("\n");
                        let basic = edt_b.lines().join("\n");
                        let detailed = edt_d.lines().join("\n");
                        let features = edt_features.lines().join("\n");
                        let languages = edt_languages.lines().join("\n");
                        let progress = edt_progress.lines().join("\n");
                        let purpose = edt_purpose.lines().join("\n");

                        if !title.is_empty()
                            && !basic.is_empty()
                            && !detailed.is_empty()
                            && !features.is_empty()
                            && !languages.is_empty()
                            && !progress.is_empty()
                            && !purpose.is_empty()
                        {
                            complete = true;
                        } else {
                            complete = false;
                        }

                        match switch {
                            0 => {
                                edt_title.input(key);
                            }
                            1 => {
                                edt_b.input(key);
                            }
                            2 => {
                                edt_d.input(key);
                            }
                            3 => {
                                edt_features.input(key);
                            }
                            4 => {
                                edt_languages.input(key);
                            }
                            5 => {
                                edt_progress.input(key);
                            }
                            6 => {
                                edt_purpose.input(key);
                            }
                            7 => {
                                edt_path.input(key);
                            }
                            8 => {
                                edt_repository.input(key);
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
    }
    Ok(())
}

fn render(
    frame: &mut Frame,
    edt_title: &mut TextArea,
    edt_b: &mut TextArea,
    edt_d: &mut TextArea,
    edt_features: &mut TextArea,
    edt_languages: &mut TextArea,
    edt_progress: &mut TextArea,
    edt_path: &mut TextArea,
    edt_repo: &mut TextArea,
    edt_purpose: &mut TextArea,
    complete: bool,
    switch: usize,
    colors: Vec<Color>,
) {
    let area = frame.area();

    let border = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    frame.render_widget(border.clone(), area);
    let inner_area = border.inner(area);

    let split = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(2),
        Constraint::Fill(2),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ]);
    let [
        top,
        input_title,
        input_b_description,
        input_d_description,
        middle,
        input_progress,
        input_purpose,
        input_path,
        input_repository,
        bottom,
    ] = split.areas(inner_area);

    let top_spilt = Layout::horizontal([Constraint::Fill(1), Constraint::Ratio(1, 10)]);
    let [main, count] = top_spilt.areas(top);

    frame.render_widget(
        Paragraph::new("> SYSTEM: Enter project metadata below").alignment(Alignment::Left),
        main,
    );
    frame.render_widget(Paragraph::new("[1/100]").alignment(Alignment::Right), count);

    edt_title.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title("[1] Project Title")
            .fg(color_switch(switch, colors.clone())[0]),
    );
    frame.render_widget(edt_title.widget(), input_title);

    edt_b.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title("[2] Basic Description")
            .fg(color_switch(switch, colors.clone())[1]),
    );
    frame.render_widget(edt_b.widget(), input_b_description);

    edt_d.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("[3] Detailed Description")
            .fg(color_switch(switch, colors.clone())[2]),
    );
    frame.render_widget(edt_d.widget(), input_d_description);

    let middle_spilt = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [features, languages] = middle_spilt.areas(middle);
    edt_features.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("[4] Features")
            .fg(color_switch(switch, colors.clone())[3]),
    );
    frame.render_widget(edt_features.widget(), features);

    edt_languages.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("[5] Languages")
            .fg(color_switch(switch, colors.clone())[4]),
    );
    frame.render_widget(edt_languages.widget(), languages);

    edt_progress.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title("[6] Progress")
            .fg(color_switch(switch, colors.clone())[5]),
    );
    frame.render_widget(edt_progress.widget(), input_progress);

    edt_purpose.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title("[7] Purpose")
            .fg(color_switch(switch, colors.clone())[6]),
    );
    frame.render_widget(edt_purpose.widget(), input_purpose);

    edt_path.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title("[8] Full Path")
            .fg(color_switch(switch, colors.clone())[7]),
    );
    frame.render_widget(edt_path.widget(), input_path);

    edt_repo.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title("[9] Repository Link")
            .fg(color_switch(switch, colors.clone())[8]),
    );
    frame.render_widget(edt_repo.widget(), input_repository);

    let bottom_spilt = Layout::horizontal([Constraint::Fill(1), Constraint::Ratio(1, 10)]);
    let [main, save] = bottom_spilt.areas(bottom);
    let status = if complete == true {
        "Completed"
    } else {
        "InCompleted"
    };
    frame.render_widget(
        Paragraph::new(format!("{}", status)).alignment(Alignment::Center),
        main,
    );
    frame.render_widget(
        Paragraph::new("CTRL+S TO SAVE ").alignment(Alignment::Right),
        save,
    );
}

fn color_switch(switch: usize, mut colors: Vec<Color>) -> Vec<Color> {
    for i in 0..colors.len() {
        if i == switch {
            colors[i] = Color::Blue
        } else {
            colors[i] = Color::Reset
        }
    }
    return colors;
}
