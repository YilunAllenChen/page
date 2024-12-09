use std::fs;
use std::path::Path;

use markdown::Options;
extern crate regex;
include!("src/models/mod.rs");

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
                .map(|path| {
                    let raw_artifact = serde_yaml::from_str(
                        fs::read_to_string(&path)
                            .expect("Failed to read the file")
                            .as_str(),
                    )
                    .unwrap_or_else(|_| panic!("Failed to parse the file: {}", path));
                    raw_artifact
                    // make dir if not exists
                })
                .map(|art| match art {
                    OneOfArticle::Project(proj) => OneOfArticle::Project(RawProject {
                        desc: markdown::to_html(proj.desc.as_str()),
                        ..proj
                    }),
                    OneOfArticle::Experience(exp) => OneOfArticle::Experience(RawExperience {
                        desc: markdown::to_html(exp.desc.as_str()),
                        ..exp
                    }),
                    OneOfArticle::Blog(blog) => OneOfArticle::Blog(RawBlog {
                        body: markdown::to_html_with_options(blog.body.as_str(), &Options::gfm()).unwrap(),
                        ..blog
                    }),
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
