use std::fs;
use std::path::Path;
extern crate regex;
use regex::Regex;

include!("src/code/raw_artifact.rs");

fn main() {
    let path_pattern = Regex::new(r"src/artifacts/projects.*\.yaml").unwrap();

    let articles: Vec<RawArticle> = fs::read_dir("src/artifacts/projects/")
        .unwrap()
        .map(|f| f.unwrap())
        .filter(|f| f.metadata().unwrap().is_file())
        .map(|f| f.path())
        .map(|path| {
            let path_str = path.to_str().unwrap();
            match path_pattern.captures(path.to_str().unwrap()) {
                Some(_) => path_str.to_string(),
                None => {
                    panic!("Invalid file detected under artifact: {}", path_str);
                }
            }
        })
        .map(|path| {
            let raw_artifact: RawArticle = serde_yaml::from_str(
                fs::read_to_string(path)
                    .expect("Failed to read the file")
                    .as_str(),
            )
            .unwrap();

            RawArticle {
                title: raw_artifact.title,
                language: raw_artifact.language,
                status: raw_artifact.status,
                tags: raw_artifact.tags,
                preview: raw_artifact.preview,
                desc: raw_artifact.desc,
            }
            // make dir if not exists
        })
        .collect();

    // build is millis since epoch
    let meta = MetaYaml {
        build: chrono::Utc::now().timestamp_millis().to_string(),
    };

    let built_yaml = BuiltYaml {
        artifacts: articles,
        meta,
    };

    let output_path = Path::new("src/artifacts/build/compiled_projects.yaml");
    let parent = output_path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent).expect("Failed to create the dir");
    }
    match fs::write(output_path, serde_yaml::to_string(&built_yaml).unwrap()) {
        Ok(_) => println!("Successfully wrote to {}", output_path.display()),
        Err(e) => panic!("Failed to write to {}: {}", output_path.display(), e),
    }
}
