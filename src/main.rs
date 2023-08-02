use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    // Read the feature name from the command-line argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: {} <feature-name> <class-name> <path>", args[0]);
        return;
    }

    let feature_name = &args[1];
    let class_name = &args[2];
    let path = &args[3];

    // Create the main feature folder
    if let Err(err) = fs::create_dir(&feature_name) {
        eprintln!("Error creating the main folder: {}", err);
        return;
    }

    // Create the required subfolders
    let subfolders = ["application", "data", "domain", "presentation"];
    for folder in &subfolders {
        let folder_path = format!("{}/{}", &feature_name, folder);
        if let Err(err) = fs::create_dir_all(&folder_path) {
            eprintln!("Error creating subfolder {}: {}", folder, err);
            return;
        }
    }

    // Create the files inside the respective folders
    let files = [
        ("application", format!("{}_service.dart", feature_name)),
        ("data", format!("{}_repository.dart", feature_name)),
        ("domain", format!("{}_model.dart", feature_name)),
        ("presentation", format!("{}_view.dart", feature_name)),
    ];

    for (folder, file) in &files {
        let file_path = format!("{}/{}/{}", &feature_name, folder, file);
        let file_content = generate_file_content(folder, feature_name, class_name , path);
        if let Err(err) = write_to_file(&file_path, &file_content) {
            eprintln!("Error creating file {}: {}", file, err);
            return;
        }
    }

    // Create the export.dart file with the desired content
    let export_file_path = format!("{}/export.dart", &feature_name);
    let export_content = format!(
        "export 'domain/{}_model.dart';\nexport 'data/{}_repository.dart';\nexport 'application/{}_service.dart';\nexport 'presentation/{}_view.dart';",
        feature_name, feature_name, feature_name, feature_name
    );

    if let Err(err) = write_to_file(&export_file_path, &export_content) {
        eprintln!("Error creating export.dart file: {}", err);
        return;
    }

    println!("Feature folders and files successfully created!");
}

fn generate_file_content(folder: &str, feature_name: &str, class_name: &str , path: &str) -> String {
    match folder {
        "application" => format!(
            "import 'package:flutter_riverpod/flutter_riverpod.dart';\nimport '{}/{}/export.dart';\n\nfinal {}ServiceProvider = Provider<{}Service>((ref) {{\n  return {}Service(ref);\n}});\n\nclass {}Service {{\n  ProviderRef ref;\n  {}Service(this.ref);\n}}",
            path, feature_name, feature_name, class_name, class_name, class_name, class_name
        ),
        "data" => format!(
            "import 'package:flutter_riverpod/flutter_riverpod.dart';\nimport '{}/{}/export.dart';\n\nabstract class _{}Repository {{}}\n\nclass {}Repository implements _{}Repository {{}}\n\nfinal {}RepositoryProvider = Provider<{}Repository>((ref) {{\n  return {}Repository();\n}});",
            path, feature_name, class_name, class_name, class_name, feature_name, class_name, class_name
        ),
        "domain" => format!(
            "import 'package:flutter/foundation.dart';\n\nimport 'package:freezed_annotation/freezed_annotation.dart';\n\npart '{}_model.freezed.dart';\npart '{}_model.g.dart';\n\n@freezed\nclass {}Model with _${}Model {{\n  const factory {}Model() = _{}Model;\n\n  factory {}Model.fromJson(Map<String, dynamic> json) => _${}ModelFromJson(json);\n}}",
            feature_name, feature_name, class_name, class_name, class_name, class_name, class_name, class_name
        ),
        "presentation" => format!(
            "import 'package:flutter/material.dart';\nimport 'package:flutter_riverpod/flutter_riverpod.dart';\n\nclass {}View extends ConsumerWidget {{\n  static MaterialPageRoute<dynamic> route() => MaterialPageRoute(\n        settings: const RouteSettings(name: \"{}View\"),\n        builder: (context) => const {}View(),\n      );\n\n  const {}View({{super.key}});\n\n  @override\n  Widget build(BuildContext context, ref) {{\n    return Container();\n  }}\n}}",
            class_name, class_name, class_name, class_name, 
        ),
        _ => String::new(),
    }
}

fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
