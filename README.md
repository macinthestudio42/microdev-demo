# Your first MicroDev task

[![Open in MicroDev](https://microdev.dev/assets/open-in-microdev.svg)](https://microdev.dev/start?repository=microdev-dev/microdev-demo)

Open the sample in your browser, sign in with GitHub, and launch it with your SSH
public key. The link selects this repository; it does not launch compute until
you confirm. Prefer the terminal? Follow the CLI steps below.

A small Rust checklist tool for the `- [ ]` lists in Markdown files, with no
external dependencies, so it builds in seconds. MicroDev supplies an isolated
Linux environment, Rust, Git, SSH, and the Codex, Claude Code, and Pi CLIs.

From your own terminal:

```sh
curl -fsSL https://microdev.dev/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
microdev login
microdev demo
```

Sign in with GitHub during `microdev login`. No card or GitHub App installation is needed
for this sample. In the connected environment:

```sh
cd ~/workspace
cargo test
cargo run -- list
cargo run -- done 2
```

## Tasks for your agent

Run `codex`, `claude`, or `pi`, sign in with your own agent account, and hand it
one of these. Each is small enough to finish in a few minutes and comes with
tests to prove it. Agent model usage is supplied by your account and is not
included in the trial.

1. Add `undo NUMBER`, which unchecks an item, with tests.
2. Add `--pending`, which lists only the items still to do.
3. Make `list` print items that are done after the ones still to do, keeping
   their numbers, and keep `TODO.md` in its original order.
4. Add `remove NUMBER`, which deletes an item but keeps the notes around it.

To commit your change, set your own Git name and email **for this checkout**:

```sh
git config --local user.name 'Your Name'
git config --local user.email 'YOUR_GITHUB_EMAIL_OR_NOREPLY_ADDRESS'
git add -A
git commit -m 'Add undo'
```

Replace the name and email with your own values. Repository-local Git settings
survive pause/resume with `.git`; global settings in your home directory do not.
Committing is optional: saved, uncommitted files also survive.

Your checkout is yours to experiment with. The sample's source is public and
read-only through MicroDev; it does not grant push access to this repository.
Connect your own repository from https://microdev.dev/app when ready.

Save your files and run `exit` to return to your own terminal. Exiting SSH leaves
the task running. Use `microdev list` to find its ID, then pause it explicitly:

```sh
microdev pause TASK_ID
# When you want to continue:
microdev ssh TASK_ID
```

Replace `TASK_ID` with the ID from the list. Pause waits until your workspace is
saved; SSH resumes that same task and reconnects. Run `cd ~/workspace`, check
`git status`, and run `cargo test` to pick up where you left off. Start your agent
again and choose its resume/continue option to reopen a saved conversation.

MicroDev preserves your workspace (including untracked and ignored work) and
supported agent session/auth files. Installed packages, other home-directory
files, shell variables, tmux sessions, and running processes are fresh on resume.
Keep task notes and custom state in `~/workspace`. Save editor buffers before
pausing; process memory is not saved.

The no-card trial provides 300 minutes of running time over 14 days, with two
tasks running at once. Only running time counts: paused tasks draw nothing. Use
`microdev trial` to see your remaining minutes.
