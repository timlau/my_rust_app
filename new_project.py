#!/usr/bin/python
import argparse
import shutil
import shlex
import subprocess
from pathlib import Path


class ProjectCreator:
    source_dirs = [
        "src",
        "data",
        ".github",
        "build-aux",
        "po",
        ".vscode",
    ]
    source_files = [
        ".gitignore",
        ".editorconfig",
        "meson.build",
        "meson_options.txt",
        "Cargo.toml",
        "Cargo.lock",
        "local.sh",
        "README.md",
        "Makefile",
        "my_rust_app.spec",
        "LICENSE",
    ]
    source_id = "org.mydomain.MyRustApp"
    source_name = "my_rust_app"
    source_obj_path = "/org/mydomain/MyRustApp"

    def __init__(self, args):
        self.name = args.name
        self.id = args.id
        self.obj_path = "/" + "/".join(self.id.split("."))
        self.root = Path(args.root).expanduser().resolve()
        self.project_path = self.root / self.name
        self.source_path = Path(__file__).parent

    def cleanup_unwanted(self):
        self.project_path.joinpath("src", "config.rs").unlink(missing_ok=True)
        self.project_path.joinpath("po", f"{self.source_name}.pot").unlink(
            missing_ok=True
        )

    def copy_directories(self):
        for dir_name in self.source_dirs:
            print(f" --> Copying directory : {dir_name}")
            shutil.copytree(
                self.source_path / dir_name,
                self.project_path / dir_name,
                dirs_exist_ok=True,
            )

    def copy_files(self):
        for file_name in self.source_files:
            print(f" --> Copying file : {file_name}")
            shutil.copy2(self.source_path / file_name, self.project_path / file_name)

    def rename_files(self, key, new_key):
        dir_path = self.project_path
        for file_path in dir_path.glob(f"**/*{key}*"):
            print(f" --> Renaming file : {file_path}...")
            new_file_path = file_path.with_name(file_path.name.replace(key, new_key))
            file_path.rename(new_file_path)

    def patch_files(self):
        patch_extensions = [
            ".rs",
            ".toml",
            ".md",
            ".sh",
            ".json",
            ".desktop",
            ".in",
            ".build",
            ".xml",
            ".yml",
            ".spec",
        ]
        for file_name in self.project_path.rglob("*"):
            if file_name.is_file() and file_name.suffix in patch_extensions:
                # print(f"Patching file {file_name}...")
                with open(file_name, "r") as f:
                    content = f.read()
                current_content = content
                content = content.replace(self.source_id, self.id)
                content = content.replace(self.source_name, self.name)
                content = content.replace(self.source_obj_path, self.obj_path)
                if content != current_content:
                    print(f" --> Updating content in {file_name}...")
                    with open(file_name, "w") as f:
                        f.write(content)

    def post_actions(self):
        print(" --> Generating translation template...")
        subprocess.run(
            shlex.split(
                f"xgettext --package-name={self.name} --package-version=main --files-from=po/POTFILES.in --output=po/{self.name}.pot"
            ),
            cwd=self.project_path,
        )
        print(" --> Initializing git repository...")
        subprocess.run(shlex.split("git init -q"), cwd=self.project_path)
        subprocess.run(shlex.split("git add *"), cwd=self.project_path)
        subprocess.run(
            shlex.split("git commit -m 'Initial import' -q"), cwd=self.project_path
        )

    def create_project(self):
        self.project_path.mkdir(parents=True, exist_ok=True)
        print(f"Creating project at: {self.project_path}")
        self.copy_directories()
        self.cleanup_unwanted()
        self.copy_files()
        self.rename_files(self.source_name, self.name)
        self.rename_files(self.source_id, self.id)
        self.patch_files()
        self.post_actions()
        print(
            f"Project '{self.name}' created with ID '{self.id}'. in {self.project_path}"
        )


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
