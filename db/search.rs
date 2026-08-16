use crate::ui::search::{Filter, SearchResult};
use rusqlite::{Connection, Result, params};

const MAX_RESULTS: i64 = 25;

pub fn search_commands(
    conn: &Connection,
    query: &str,
    filter: &Filter,
    project_dir: &str,
) -> Result<Vec<SearchResult>> {
    let trimmed_query = query.trim();
    let like_query = format!("%{}%", trimmed_query);

    let mut sql = String::from(
        "
        SELECT command, dir, COALESCE(output, ''), exit_code, timestamp
        FROM commands
        WHERE 1 = 1
        ",
    );

    if !trimmed_query.is_empty() {
        sql.push_str(" AND (command LIKE ?1 OR output LIKE ?1 OR dir LIKE ?1)");
    }

    match filter {
        Filter::All => {}
        Filter::Today => {
            sql.push_str(" AND timestamp >= strftime('%s', 'now', 'start of day')");
        }
        Filter::ThisWeek => {
            sql.push_str(" AND timestamp >= strftime('%s', 'now', '-7 days')");
        }
        Filter::Errors => {
            sql.push_str(" AND exit_code != 0");
        }
        Filter::Project => {
            if trimmed_query.is_empty() {
                sql.push_str(" AND dir = ?1");
            } else {
                sql.push_str(" AND dir = ?2");
            }
        }
    }

    sql.push_str(" ORDER BY timestamp DESC LIMIT ");
    sql.push_str(&MAX_RESULTS.to_string());

    let mut statement = conn.prepare(&sql)?;
    let rows = match (trimmed_query.is_empty(), matches!(filter, Filter::Project)) {
        (true, true) => statement.query_map(params![project_dir], row_to_search_result)?,
        (false, true) => {
            statement.query_map(params![like_query, project_dir], row_to_search_result)?
        }
        (true, false) => statement.query_map([], row_to_search_result)?,
        (false, false) => statement.query_map(params![like_query], row_to_search_result)?,
    };

    rows.collect()
}

fn row_to_search_result(row: &rusqlite::Row<'_>) -> Result<SearchResult> {
    let output: String = row.get(2)?;
    let timestamp: i64 = row.get(4)?;

    Ok(SearchResult {
        command: row.get(0)?,
        dir: row.get(1)?,
        output: preview_output(&output),
        exit_code: row.get(3)?,
        time_ago: time_ago(timestamp),
    })
}

fn preview_output(output: &str) -> String {
    let preview = output
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim();

    if preview.chars().count() > 90 {
        format!("{}...", preview.chars().take(87).collect::<String>())
    } else {
        preview.to_string()
    }
}

fn time_ago(timestamp: i64) -> String {
    let now = chrono::Utc::now().timestamp();
    let elapsed = now.saturating_sub(timestamp);

    match elapsed {
        0..=59 => String::from("just now"),
        60..=3599 => format!("{}m ago", elapsed / 60),
        3600..=86399 => format!("{}h ago", elapsed / 3600),
        86400..=604799 => format!("{}d ago", elapsed / 86400),
        _ => format!("{}w ago", elapsed / 604800),
    }
}
