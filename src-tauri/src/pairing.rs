use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
    process::{Command, ExitStatus},
};

pub async fn execute_bbp(
    input_file_path: &Path,
    bbp_exec_path: &Path,
    output_file_path: &Path,
) -> Result<ExitStatus> {
    Command::new(bbp_exec_path)
        .arg("--dutch")
        .arg(input_file_path)
        .arg("-p")
        .arg(output_file_path)
        .status()
}

pub fn parse_bbp_output(output_file: &mut File) -> Result<Vec<(u16, u16)>> {
    let mut buf: String = String::new();
    output_file.read_to_string(&mut buf)?;
    let mut lines = buf.lines();
    let mut ids: Vec<(u16, u16)> = Vec::new();
    if let Some(_) = lines.next() {
        lines.for_each(|l| {
            let mut pairs = l.split_ascii_whitespace();
            let white_id: Option<u16> = pairs.next().and_then(|id| {
                if let Ok(n) = id.parse() {
                    Some(n)
                } else {
                    None
                }
            });
            let black_id: Option<u16> = pairs.next().and_then(|id| {
                if let Ok(n) = id.parse() {
                    Some(n)
                } else {
                    None
                }
            });
            ids.push((white_id.unwrap_or_default(), black_id.unwrap_or_default()))
        })
    }
    Ok(ids)
}
