#!/usr/bin/python
import argparse
import shutil
from pathlib import Path


class ProjectCreator:
    dirs = [
        "src",
        "data",
        ".github",
        "build-aux",
        "po",
        ".vscode",
    ]
    files = [
        ".gitignore",
        ".editorconfig",
        "meson.build",
        "meson_options.txt",
        "Cargo.toml",
        "Cargo.lock",
        "local.sh",
        "README.md",
    ]
    source_id = "org.mydomain.MyRustApp"
    source_name = "my_rust_app"
    source_obj_path = "/org/mydomain/MyRustApp"

    def __init__(self, args):
        self.name = args.name
        self.id = args.id
        self.obj_path = "/" + "/".join(self.id.split("."))
        print(f"ID paths: {self.obj_path}")
        self.root = Path(args.root).expanduser().resolve()
        self.project_path = self.root / self.name
        self.source_path = Path(__file__).parent

    def copy_directories(self):
        for dir_name in self.dirs:
            print(f"Copying {dir_name}...")
            shutil.copytree(
                self.source_path / dir_name,
                self.project_path / dir_name,
                dirs_exist_ok=True,
            )

    def copy_files(self):
        for file_name in self.files:
            print(f"Copying {file_name}...")
            shutil.copy2(self.source_path / file_name, self.project_path / file_name)

    def rename_ids(self):
        for dir in self.dirs:
            dir_path = self.project_path / dir
            for file_path in dir_path.glob(f"**/{self.source_id}*"):
                print(f"Renaming file {file_path}...")
                new_file_path = file_path.with_name(
                    file_path.name.replace(self.source_id, self.id)
                )
                file_path.rename(new_file_path)

    def patch_files(self):
        patch_extensions = [
            "rs",
            "toml",
            "md",
            "sh",
            "json",
            "desktop",
            "in",
            "build",
            "xml",
        ]
        for file_name in self.project_path.glob("**/*"):
            if file_name.is_file() and file_name.suffix.lstrip(".") in patch_extensions:
                # print(f"Patching file {file_name}...")
                with open(file_name, "r") as f:
                    content = f.read()
                current_content = content
                content = content.replace(self.source_id, self.id)
                content = content.replace(self.source_name, self.name)
                content = content.replace(self.source_obj_path, self.obj_path)
                if content != current_content:
                    print(f"Updating content in {file_name}...")
                    with open(file_name, "w") as f:
                        f.write(content)

    def create_project(self):
        self.project_path.mkdir(parents=True, exist_ok=True)
        print(f"Creating project at: {self.project_path}")
        self.copy_directories()
        self.copy_files()
        self.rename_ids()
        self.patch_files()
        print(f"Project '{self.name}' created with ID '{self.id}'.")


def main():
    parser = argparse.ArgumentParser(
        description="Create a new project directory with standard structure."
    )
    parser.add_argument("name", type=str, help="Name of the new project (ex: my_app)")
    parser.add_argument(
        "id",
        type=str,
        help="id of the new project (ex: com.example.project)",
    )
    parser.add_argument(
        "root",
        type=str,
        help="Root directory where the project will be created (ex: /home/user/projects)",
    )
    args = parser.parse_args()
    print(args)
    creator = ProjectCreator(args)
    creator.create_project()


if __name__ == "__main__":
    main()
