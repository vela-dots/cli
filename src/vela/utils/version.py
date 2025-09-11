import shutil
import subprocess

from vela.utils.paths import config_dir


def print_version() -> None:
    if shutil.which("pacman"):
        print("Packages:")
        pkgs = ["vela-shell-git", "vela-cli-git", "vela-meta"]
        versions = subprocess.run(
            ["pacman", "-Q", *pkgs], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True
        ).stdout

        for pkg in pkgs:
            if pkg not in versions:
                print(f"    {pkg} not installed")
        print("\n".join(f"    {pkg}" for pkg in versions.splitlines()))
    else:
        print("Packages: not on Arch")

    print()
    try:
        vela_dir = (config_dir / "hypr").resolve().parent
        vela_ver = subprocess.check_output(
            ["git", "--git-dir", vela_dir / ".git", "rev-list", "--format=%B", "--max-count=1", "HEAD"], text=True
        )
        print("Vela:")
        print("    Last commit:", vela_ver.split()[1])
        print("    Commit message:", *vela_ver.splitlines()[1:])
    except subprocess.CalledProcessError:
        print("Vela: not installed")

    print()
    if shutil.which("qs"):
        print("Quickshell:")
        print("   ", subprocess.check_output(["qs", "--version"], text=True).strip())
    else:
        print("Quickshell: not in PATH")

    local_shell_dir = config_dir / "quickshell/vela"
    if local_shell_dir.exists():
        print("\nLocal copy of shell found:")

        try:
            shell_ver = subprocess.check_output(
                [
                    "git",
                    "--git-dir",
                    local_shell_dir / ".git",
                    "rev-list",
                    "--format=%B",
                    "--max-count=1",
                    "upstream/main",
                ],
                text=True,
                stderr=subprocess.DEVNULL,
            )
            print("    Last merged upstream commit:", shell_ver.split()[1])
            print("    Commit message:", *shell_ver.splitlines()[1:])
        except subprocess.CalledProcessError:
            print("    Unable to determine last merged upstream commit.")

        shell_ver = subprocess.check_output(
            ["git", "--git-dir", local_shell_dir / ".git", "rev-list", "--format=%B", "--max-count=1", "HEAD"],
            text=True,
        )
        print("\n    Last commit:", shell_ver.split()[1])
        print("    Commit message:", *shell_ver.splitlines()[1:])
