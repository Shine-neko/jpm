use clap::App;
use clap::SubCommand;
use clap::ArgMatches;
use std::fs;
use serde_json;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Repository {
    pub(crate) url: String
}

#[derive(Deserialize, Debug)]
struct Package {
    repository: Repository,
    #[serde(default)]
    dependencies: HashMap<String, String>
}

pub(crate) fn command_config() -> App<'static, 'static, 'static, 'static, 'static, 'static> {
    return SubCommand::with_name("install")
        .about("install all dependencies for a project")
}

pub(crate) fn install(_args: &ArgMatches) {
    let config = fs::read_to_string("package.json").unwrap();

    fs::create_dir_all("node_modules/.bin").unwrap();

    let result: Result<Config, serde_json::Error> = serde_json::from_str(&config);
    let packages = result.unwrap();

    for (name, version) in packages.dependencies {
        download_dependency(name, version);
    }
}

fn download_dependency(name: String, version: String) {
    let package = load_package_json(name.clone(), version);

    println!("Installing {}", package.repository.url);

    let _command_output = Command::new("git")
        .arg("clone")
        .arg(package.repository.url)
        .arg(format!("node_modules/{}", &name))
        .output()
        .expect("failed to execute process");

    for (n, v)  in package.dependencies {
        download_dependency(n, v);
    }
}


fn load_package_json(name: String, version: String) -> Package {
    let tag = version
        .replace("^", "")
        .replace("~", "");

    let repository_url = format!("https://registry.npmjs.org/{}/{}", name, tag);

    println!("Fetch {}", repository_url);

    match ureq::get(&repository_url).call() {
        Ok(response) => {

            let content = response.into_string().unwrap();

            let result: Result<Package, serde_json::Error> = serde_json::from_str(&content);
            let mut package = result.unwrap();
            package.repository.url = package.repository.url.replace("git+", "");

            return package;

        },
        Err(_) => { panic!("Unable to load package") }
    }
}
