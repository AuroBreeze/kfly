# 🚀 kfly
The Linux Kernel Patch Submission Pilot.

`kfly` is a CLI automation tool designed for Linux kernel contributors. It streamlines the repetitive workflow of checking, profiling, and submitting patches by using a configurable, pipeline-based engine.

## ✨ Features
- Pipeline-driven: Define your own workflow in kfly.toml.
- Intelligent Variable Injection: Use {patch} placeholders to dynamically inject patch paths into any command.
- Fail-fast Mechanism: Stop the execution immediately if a critical task (like checkpatch.pl) fails.
- Interactive Gates: Protect sensitive operations (like git send-email) with mandatory y/N confirmations.
- Minimalist & Professional UI: Color-coded status reports for high-glanceability.

## 🛠️ Prerequisites

Before taking off, ensure you have the following in your environment:

- Rust (2021 edition or later)
- Git (configured with send-email)
- Perl (required for Linux kernel scripts)
- Linux Kernel Source Tree

## 📦 Installation

```Bash
git clone https://github.com/AuroBreeze/kfly.git
cd kfly
cargo build --release
### Optionally link to your bin path
sudo ln -s $(pwd)/target/release/kfly /usr/local/bin/kfly
```


## ⚙️ Configuration (kfly.toml)
Create a `kfly.toml` in your project root or working directory:



```Ini, TOML
[settings]
kernel_root = "/path/to/your/linux/kernel"
test_email = "AuroBreeze@outlook.com"

[[workflow]]
name = "Checkpatch"
command = "scripts/checkpatch.pl"
args = ["--no-tree", "--strict", "{patch}"]
interactive = false
fail_fast = true

[[workflow]]
name = "Get Maintainers"
command = "scripts/get_maintainer.pl"
args = ["{patch}"]
interactive = false
fail_fast = true

[[workflow]]
name = "Send Email"
command = "git send-email"
args = ["--suppress-cc=all", "{patch}"]
interactive = true
fail_fast = false
```

## 🚀 Usage
Simply point kfly to your .patch file:

```Bash
kfly --patch ./0001-fix-logic-error.patch
```

Options:
- -p, --patch <PATH>: Path to the patch file (Required).
- -k, --kernel-root <PATH>: Override the kernel root path.
- -t, --test: Run in test mode (redirects to test_email).
- -d, --dry-run: Preview commands without executing.

## 🛡️ Workflow Logic

1. Validation: kfly resolves the absolute path of your patch.
2. Configuration: Loads the kfly.toml pipeline.
3. Execution:
  - Replaces {patch} with the actual path.
  - Checks for interactive flags.
  - Monitors exit codes; if a fail_fast task returns non-zero, the mission is aborted.
