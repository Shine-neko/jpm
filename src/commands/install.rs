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
    dependencies: HashMap<String, String>,
    #[serde(default)]
    versions: Vec<String>
}

#[derive(Deserialize, Debug)]
struct PackageInfo {
    name: String,
    versions: HashMap<String, PackageVersion>
}

#[derive(Deserialize, Debug)]
struct PackageVersion {
    version: String,
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

    let info_json_content = get_json_content(format!("https://registry.npmjs.org/{}", name));
    let package_info = serde_json::from_str::<PackageInfo>(&info_json_content).unwrap();

    let repository_url = format!("https://registry.npmjs.org/{}/{}", name, tag);
    println!("Fetch {}", repository_url);

    let json_content = get_json_content(repository_url);
    let mut package = serde_json::from_str::<Package>(&json_content).unwrap();
    package.repository.url = package.repository.url.replace("git+", "");

    for version in package_info.versions.keys() {
        package.versions.push(version.to_string());
    }

    return package;
}

fn get_json_content(url: String) -> String {
    return match ureq::get(&url).call() {
        Ok(response) => {
            response.into_string().unwrap()
        },
        Err(_) => { panic!("Unable to load content") }
    }
}