use std::fs;
use std::path::Path;
extern crate regex;
include!("src/models/mod.rs");

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut prev_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }

    slug.trim_matches('-').to_owned()
}

fn parse_markdown_blog(contents: &str, path: &str) -> RawBlog {
    let mut sections = contents.splitn(3, "---");
    let prefix = sections.next().unwrap_or_default();
    let front_matter = sections.next().unwrap_or_default();
    let body = sections.next().unwrap_or_default();

    if !prefix.trim().is_empty() || front_matter.trim().is_empty() || body.trim().is_empty() {
        panic!(
            "Markdown blog files must start with YAML front matter delimited by ---: {}",
            path
        );
    }

    #[derive(serde::Deserialize)]
    struct BlogFrontMatter {
        time: String,
        title: String,
        tags: Vec<String>,
    }

    let meta: BlogFrontMatter = serde_yaml::from_str(front_matter)
        .unwrap_or_else(|_| panic!("Failed to parse markdown front matter: {}", path));

    RawBlog {
        slug: Path::new(path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(slugify)
            .filter(|slug| !slug.is_empty())
            .unwrap_or_else(|| panic!("Failed to derive slug from path: {}", path)),
        time: meta.time,
        title: meta.title,
        tags: meta.tags,
        body: body.trim().to_owned(),
    }
}

fn parse_article(path: &str) -> OneOfArticle {
    let contents = fs::read_to_string(path).expect("Failed to read the file");

    match Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "md" | "markdown" => OneOfArticle::Blog(parse_markdown_blog(&contents, path)),
        "yaml" | "yml" => {
            let mut article: OneOfArticle = serde_yaml::from_str(contents.as_str())
                .unwrap_or_else(|_| panic!("Failed to parse the file: {}", path));
            if let OneOfArticle::Blog(blog) = &mut article {
                if blog.slug.trim().is_empty() {
                    blog.slug = slugify(&blog.title);
                }
            }
            article
        }
        _ => panic!("Unsupported artifact file extension: {}", path),
    }
}

fn parse_dir(input_dirs: Vec<&str>, output_dir: &str) {
    let all_articles: Vec<OneOfArticle> = input_dirs
        .iter()
        .flat_map(|input_dir| {
            let articles: Vec<_> = fs::read_dir(input_dir)
                .unwrap()
                .map(|f| f.unwrap())
                .filter(|f| f.metadata().unwrap().is_file())
                .map(|f| f.path())
                .map(|path| path.to_str().unwrap().to_string())
                .map(|path| parse_article(path.as_str()))
                .map(|art| match art {
                    OneOfArticle::Project(proj) => OneOfArticle::Project(RawProject {
                        desc: markdown::to_html(proj.desc.as_str()),
                        ..proj
                    }),
                    OneOfArticle::Experience(exp) => OneOfArticle::Experience(RawExperience {
                        desc: markdown::to_html(exp.desc.as_str()),
                        ..exp
                    }),
                    OneOfArticle::Blog(blog) => OneOfArticle::Blog(blog),
                })
                .collect();

            articles
        })
        .collect();

    // build is millis since epoch
    let meta = MetaYaml {
        build: chrono::Utc::now().timestamp_millis().to_string(),
    };

    let built_yaml = BuiltYaml {
        artifacts: all_articles,
        meta,
    };

    let output_path = Path::new(output_dir);
    let parent = output_path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent).expect("Failed to create the dir");
    }
    match fs::write(output_path, serde_yaml::to_string(&built_yaml).unwrap()) {
        Ok(_) => println!("Successfully wrote to {}", output_path.display()),
        Err(e) => panic!("Failed to write to {}: {}", output_path.display(), e),
    }
}

fn main() {
    parse_dir(
        vec![
            "src/artifacts/projects",
            "src/artifacts/experiences",
            "src/artifacts/blogs",
        ],
        "src/artifacts/build/compiled.yaml",
    );
}
