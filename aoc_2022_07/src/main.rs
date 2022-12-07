use nom::IResult;
use std::{collections::HashMap, io, mem};

#[derive(Debug)]
enum Line<'a> {
    Cd(&'a str),
    Ls,
    Dir(&'a str),
    File(u64, &'a str),
}

fn parse_line(line: &str) -> IResult<&str, Line<'_>> {
    use nom::{branch, bytes, character, combinator, sequence};

    branch::alt((
        sequence::preceded(
            sequence::pair(bytes::complete::tag("$"), character::complete::multispace1),
            branch::alt((
                sequence::preceded(
                    sequence::tuple((bytes::complete::tag("cd"), character::complete::multispace1)),
                    combinator::map(
                        bytes::complete::take_while1(|b: char| {
                            b.is_alphanumeric() || b == '/' || b == '.'
                        }),
                        Line::Cd,
                    ),
                ),
                combinator::map(bytes::complete::tag("ls"), |_| Line::Ls),
            )),
        ),
        combinator::map(
            sequence::separated_pair(
                bytes::complete::tag("dir"),
                character::complete::multispace1,
                character::complete::not_line_ending,
            ),
            |(_, name)| Line::Dir(name),
        ),
        combinator::map(
            sequence::separated_pair(
                character::complete::u64,
                character::complete::multispace1,
                bytes::complete::take_while1(|b: char| b.is_alphanumeric() || b == '.'),
            ),
            |(size, name)| Line::File(size, name),
        ),
    ))(line)
}

fn dir_size(name: &str, filesystem: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    let contents = filesystem.get(name).expect("no contents for directory");

    let parent_name = name.to_string() + ":";
    contents.iter().map(|(size, _)| size).sum::<u64>()
        + filesystem
            .iter()
            .filter_map(|(name, contents)| {
                name.starts_with(&parent_name)
                    .then(|| contents.iter().map(|(size, _)| size).sum::<u64>())
            })
            .sum::<u64>()
}

fn main() -> io::Result<()> {
    let filesystem = itertools::process_results(io::stdin().lines(), |it| {
        let mut filesystem: HashMap<String, Vec<(u64, String)>> = HashMap::default();
        let mut path = Vec::new();
        let mut dir_contents = Vec::default();

        for line in it {
            let (remaining, line) = parse_line(&line)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "could not parse line"))?;
            assert_eq!("", remaining);

            match line {
                Line::Cd(dir) => {
                    if !path.is_empty() {
                        let existing_contents = mem::take(&mut dir_contents);
                        let path_str = path.join(":");
                        filesystem
                            .entry(path_str)
                            .or_default()
                            .extend(existing_contents);
                    }

                    if dir == ".." {
                        path.pop();
                    } else {
                        path.push(dir.to_string());
                    }
                }
                Line::Ls => {}
                Line::Dir(_name) => {}
                Line::File(size, name) => {
                    dir_contents.push((size, name.to_string()));
                }
            }
        }

        if !path.is_empty() {
            let existing_contents = mem::take(&mut dir_contents);
            filesystem.insert(path.join(":"), existing_contents);
        }

        Ok::<_, io::Error>(filesystem)
    })??;

    let filesystem_size = dir_size("/", &filesystem);
    let needed_size = 30_000_000 - (70_000_000 - filesystem_size);
    let answer = filesystem
        .keys()
        .map(|name| dir_size(name, &filesystem))
        .filter(|size| *size >= needed_size)
        .min()
        .expect("directory not found");

    println!("{answer}");

    Ok(())
}
