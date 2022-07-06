use std::{env::{self, set_current_dir}, fs::{self, File}, path::Path, process::Command, io::{Write, Read}};
use libdlta::{database::initialise::initialise, base::structs::Package, database::add::add};

mod structs;
use structs::ConfigFile;
mod scripts;
use scripts::*;
mod help;
use help::help;

fn main() {
    // grabs the arguments
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    if args.is_empty() {
        println!("Usage: sgma <command> <package>");
        return;
    }
    
    // sorts arguments into either flags or packages
    let mut flags: Vec<String> = Vec::new();
    let mut modifiers: Vec<String> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            flags.push(arg.to_string());
        } else {
            modifiers.push(arg.to_string());
        }
    }
    
    // sets the 0th argument (e.g. sgma **install**) as "oper"
    let oper = args[0].clone();
    
    if oper.as_str() == "init-repo" {
        let path = &modifiers[0];
        fs::create_dir_all(path).unwrap_or_else(
            |err| {
                eprintln!("Could not create directory: {}", err);
                std::process::exit(1);
            }
        );
        fs::write(
            path.to_string() + "/sgma.toml",
            toml::to_string(&ConfigFile::default()).unwrap_or_else(|err| {
                eprintln!("Could not write default config file: {}", err);
                std::process::exit(1);
            })
        ).unwrap_or_else(|err| {
            eprintln!("Could not write config file: {}", err);
            std::process::exit(1);
        });
        fs::create_dir(path.to_string() + "/out").unwrap_or_else(
            |err| {
                eprintln!("Could not create directory: {}", err);
                std::process::exit(1);
            }
        );
        fs::create_dir(path.to_string() + "/src").unwrap_or_else(
            |err| {
                eprintln!("Could not create directory: {}", err);
                std::process::exit(1);
            }
        );
        initialise(Path::new(&(path.to_owned() + &"/out/db.sqlite".to_string())), false).unwrap_or_else(
            |err| {
                eprintln!("Could not initialise database: {}", err);
                std::process::exit(1);
            }
        );
        println!("Initialised new repository at {}", path);
        std::process::exit(0);
    }

    // parse config file
    let pkg = &modifiers[0];
    let config = toml::from_str::<ConfigFile>(&fs::read_to_string("./sgma.toml").unwrap_or_else(|_| {
        eprintln!("Could not find sgma.toml");
        std::process::exit(1);
    })).unwrap_or_else(|err| {
        eprintln!("Problem parsing config file: {}", err);
        std::process::exit(1);
    });
    let srcpath = config.srcpath;
    let outpath = config.outpath;
    #[allow(unused_variables)]
    let exclude = config.exclude;

    match oper.as_str() {
        "build" => {
            // initial prep work, defining variables and entering the package source directory
            let pkg = &modifiers[0];
            let spath = format!("{}/{}", &srcpath, &pkg);
            std::env::set_current_dir(spath).unwrap_or_else(
                |err| {
                    eprintln!("Could not enter source directory: {}", err);
                    std::process::exit(1);
                }
            );

            // create relevant directories
            fs::create_dir_all("out").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir_all("src").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir_all("built").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir_all("built/overlay").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir_all("built/scripts").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );

            // run build script and then rsync things around
            Command::new("sh")
                .arg("./build.sh")
                .spawn()
                .unwrap_or_else(|err| {
                    eprintln!("Could not execute build script: {}", err);
                    std::process::exit(1);
                }).wait().unwrap();
            let package = toml::from_str::<Package>(&fs::read_to_string("md.toml").unwrap_or_else(|_| {
                eprintln!("Could not find md.toml");
                std::process::exit(1);
            })).unwrap_or_else(|err| {
                eprintln!("Problem parsing md.toml: {}", err);
                std::process::exit(1);
            });
            Command::new("rsync")
                .arg("-r")
                .arg("scripts/")
                .arg("built/scripts/")
                .status()
                .unwrap_or_else(|err| {
                    eprintln!("Could not execute rsync: {}", err);
                    std::process::exit(1);
                });
            Command::new("rsync")
                .arg("-r")
                .arg("out/overlay/")
                .arg("built/overlay/")
                .status()
                .unwrap_or_else(|err| {
                    eprintln!("Could not execute rsync: {}", err);
                    std::process::exit(1);
                });

            // parse md.toml for package metadata
            let mut mdfile = File::create("built/md.toml").unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            );
            mdfile.write_all(toml::to_string(&package).unwrap_or_else(|err| {
                eprintln!("Could not write to file: {}", err);
                std::process::exit(1);
            }).as_bytes()).unwrap_or_else(
                |err| {
                    eprintln!("Could not write to file: {}", err);
                    std::process::exit(1);
                }
            );

            // tar up the built/ directory
            let mut ar = tar::Builder::new(File::create(format!("out/{}.tar", pkg)).unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            ));
            ar.append_dir_all(".", "built").unwrap_or_else(
                |err| {
                    eprintln!("Could not append directory: {}", err);
                    std::process::exit(1);
                }
            );

            // compress the tarball accordingly
            let mut zstd = zstd_util::ZstdContext::new(11, Some(&[]));
            let mut f = File::open(format!("out/{}.tar", pkg)).unwrap_or_else(
                |err| {
                    eprintln!("Could not open file: {}", err);
                    std::process::exit(1);
                }
            );
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap_or_else(
                |err| {
                    eprintln!("Could not read file: {}", err);
                    std::process::exit(1);
                }
            );
            let deflated = zstd.compress(&buffer).unwrap_or_else(
                |err| {
                    eprintln!("Could not compress file: {}", err);
                    std::process::exit(1);
                }
            );
            let mut outfile = File::create(format!("out/{}.tar.zst", pkg)).unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            );
            outfile.write_all(&deflated).unwrap_or_else(
                |err| {
                    eprintln!("Could not write to file: {}", err);
                    std::process::exit(1);
                }
            );

            // return to root repo directory
            set_current_dir("../../").unwrap_or_else(
                |_| {
                    eprintln!("Could not enter repository directory (how?)");
                    std::process::exit(1);
                }
            );

            // copy the baked package file to the repository directory
            Command::new("rsync")
                .arg("-r")
                .arg(format!("src/{}/out/{}.tar.zst", pkg, pkg))
                .arg("out/")
                .status()
                .unwrap_or_else(|err| {
                    eprintln!("Could not execute rsync: {}", err);
                    std::process::exit(1);
                });

            // clean up after the build process
            fs::remove_dir_all(format!("src/{}/built", pkg)).unwrap_or_else(
                |err| {
                    eprintln!("Could not remove intermediate build directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::remove_dir_all(format!("src/{}/out", pkg)).unwrap_or_else(
                |err| {
                    eprintln!("Could not remove out directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir_all(format!("src/{}/out", pkg)).unwrap_or_else(
                |err| {
                    eprintln!("Could not recreate out directory: {}", err);
                    std::process::exit(1);
                }
            );

            // either update the repository with the new package, or do nothing if the --dry parameter is passed
            if !flags.contains(&"--dry".to_string()) && !exclude.contains(pkg) {
                add(package, Path::new("out/db.sqlite")).unwrap_or_else(
                    |err| {
                        eprintln!("Could not add package to database: {}", err);
                        std::process::exit(1);
                    }
                );
                println!("Packaging of {} succeeded!", pkg);
            } else {
                println!("Packaging of {} succeeded! (dry run)", pkg);
            }
            std::process::exit(0);
            
        }
        "new-srcpkg" => {
            // make sure we're actually in a repository and that the config is valid
            if !Path::exists(Path::new(&"./sgma.toml")) {
                eprintln!("Could not find sgma.toml, have you initialised a repository?");
                std::process::exit(1);
            }
            if srcpath.is_empty() {
                eprintln!("No srcpath specified in sgma.toml");
                std::process::exit(1);
            }
            if outpath.is_empty() {
                eprintln!("No outpath specified in sgma.toml");
                std::process::exit(1);
            }

            // create the directory for the source package, and the relevant subdirectories
            fs::create_dir(srcpath.to_string() + "/" + pkg).unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir(srcpath.to_string() + "/" + pkg + "/scripts").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );
            fs::create_dir(srcpath.to_string() + "/" + pkg + "/config").unwrap_or_else(
                |err| {
                    eprintln!("Could not create directory: {}", err);
                    std::process::exit(1);
                }
            );

            // define default config for the source package
            let default_srcpkg = Package {
                name: pkg.to_string(),
                version: "0.1.0".to_string(),
                description: Some("".to_string()),
                authors: Vec::new(),
                license: Some("".to_string()),
                tracked_files: Vec::new(),
                dependencies: Some(Vec::new()),
                provides: Some(Vec::new()),
                conflicts: Some(Vec::new()),
                reccomendations: Some(Vec::new()),
                arch: "any".to_string(),
            };

            // writes build.sh to the package root and the post/pre/hook scripts to the scripts directory
            let mut buildsh = File::create(srcpath.to_string() + "/" + pkg + "/build.sh").unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            );
            buildsh.write_all(BUILD_SH.as_bytes()).unwrap_or_else(
                |err| {
                    eprintln!("Could not write to file: {}", err);
                    std::process::exit(1);
                }
            );
            let mut presh = File::create(srcpath.to_string() + "/" + pkg + "/scripts/pre.sh").unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            );
            presh.write_all(PRE_SH.as_bytes()).unwrap_or_else(
                |err| {
                    eprintln!("Could not write to file: {}", err);
                    std::process::exit(1);
                }
            );
            let mut postsh = File::create(srcpath.to_string() + "/" + pkg + "/scripts/post.sh").unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            );
            postsh.write_all(POST_SH.as_bytes()).unwrap_or_else(
                |err| {
                    eprintln!("Could not write to file: {}", err);
                    std::process::exit(1);
                }
            );
            let mut hooksh = File::create(srcpath.to_string() + "/" + pkg + "/scripts/hook.sh").unwrap_or_else(
                |err| {
                    eprintln!("Could not create file: {}", err);
                    std::process::exit(1);
                }
            );
            hooksh.write_all(HOOK_SH.as_bytes()).unwrap_or_else(
                |err| {
                    eprintln!("Could not write to file: {}", err);
                    std::process::exit(1);
                }
            );

            // write default config to md.toml
            fs::write(srcpath + "/" + pkg + "/md.toml", toml::to_string(&default_srcpkg).unwrap_or_else(|err| {
                eprintln!("Could not write default config file: {}", err);
                std::process::exit(1);
            })).unwrap_or_else(|err| {
                eprintln!("Could not write config file: {}", err);
                std::process::exit(1);
            });
            println!("Created new source package {}", pkg);       
        }
        "query" => {
            // make sure we're actually in a repository and that the config is valid
            if !Path::exists(Path::new(&"./sgma.toml")) {
                eprintln!("Could not find sgma.toml, have you initialised a repository?");
                std::process::exit(1);
            }
            if srcpath.is_empty() {
                eprintln!("No srcpath specified in sgma.toml");
                std::process::exit(1);
            }
            if outpath.is_empty() {
                eprintln!("No outpath specified in sgma.toml");
                std::process::exit(1);
            }

            // query the database for the package       
            let res = libdlta::database::query::query(pkg, Path::new(&format!("{}/db.sqlite", outpath)));
            println!("{:?}", res);
        }
        "remove" => {
            // make sure we're actually in a repository and that the config is valid
            if !Path::exists(Path::new(&"./sgma.toml")) {
                eprintln!("Could not find sgma.toml, have you initialised a repository?");
                std::process::exit(1);
            }
            if srcpath.is_empty() {
                eprintln!("No srcpath specified in sgma.toml");
                std::process::exit(1);
            }
            if outpath.is_empty() {
                eprintln!("No outpath specified in sgma.toml");
                std::process::exit(1);
            }

            // remove the package from the database
            let query = libdlta::database::query::query(pkg, Path::new(&format!("{}/db.sqlite", outpath)));
            let res = libdlta::database::remove::remove(query, Path::new(&format!("{}/db.sqlite", outpath))).unwrap_or_else(
                |err| {
                    eprintln!("Could not remove package from database: {}", err);
                    std::process::exit(1);
                }
            );
            println!("{:?}", res);
        }
        _ => {
            help();
        }
    }
}
