//! Shell script generation for wrapper functions and tab completions.
//!
//! This module is used by `enx setup` to materialize shell integration
//! files that get sourced from the user's shell startup file.

use clap::CommandFactory;
use clap_complete::Shell;

use crate::cli::Cli;

/// Build the shell wrapper and completions script for the given shell name.
///
/// Bash and zsh share the same wrapper function, while fish uses a dedicated
/// wrapper implementation.
pub fn generate_script(shell_name: &str) -> anyhow::Result<String> {
    let script = match shell_name {
        "bash" => generate_bash(),
        "zsh" => generate_zsh(),
        "fish" => generate_fish(),
        _ => anyhow::bail!("unsupported shell: {shell_name}"),
    };

    Ok(script)
}

/// Generate completions as a String for the given shell.
///
/// `clap_complete` normally writes to a `impl Write`. We give it
/// a `Vec<u8>` (which implements Write — it's just an in-memory buffer)
/// and then convert to a String.
fn completions_for(shell: Shell) -> String {
    let mut cmd = Cli::command();

    // `Vec<u8>` implements `std::io::Write`, so we can use it as a buffer.
    // This is a common Rust pattern: Vec<u8> is a growable byte buffer,
    // and it implements Write by just appending bytes.
    let mut buf: Vec<u8> = Vec::new();
    clap_complete::generate(shell, &mut cmd, "enx", &mut buf);

    // Convert bytes to String. `from_utf8_lossy` replaces any invalid
    // UTF-8 sequences with the replacement character. Completion scripts
    // should always be valid UTF-8, so this is just defensive coding.
    String::from_utf8_lossy(&buf).into_owned()
}

/// Generate the wrapper function shared by bash and zsh.
///
/// The wrapper function is identical for both shells — only the
/// completions differ (bash uses `complete`, zsh uses `compdef`
/// and its own completion system).
fn bash_zsh_wrapper(completions: &str) -> String {
    format!(
        r#"
enx() {{
    local tmpfile
    tmpfile=$(mktemp "${{TMPDIR:-/tmp}}/enx-finalizer.XXXXXX")
    trap "rm -f '$tmpfile'" EXIT

    ENX_FINALIZER_FILE="$tmpfile" command enx "$@"
    local exit_code=$?

    if [ $exit_code -eq 0 ] && [ -f "$tmpfile" ]; then
        while IFS= read -r line; do
            case "$line" in
                cd:*)
                    cd "${{line#cd:}}" || true
                    ;;
                setenv:*)
                    export "${{line#setenv:}}"
                    ;;
            esac
        done < "$tmpfile"
    fi

    rm -f "$tmpfile"
    trap - EXIT
    return $exit_code
}}

{completions}
"#
    )
}

fn generate_bash() -> String {
    let completions = completions_for(Shell::Bash);
    bash_zsh_wrapper(&completions)
}

fn generate_zsh() -> String {
    let completions = completions_for(Shell::Zsh);
    bash_zsh_wrapper(&completions)
}

fn fish_wrapper(completions: &str) -> String {
    format!(
        r#"
function enx
    set -l tmpdir /tmp
    if set -q TMPDIR
        set tmpdir "$TMPDIR"
    end

    set -l tmpfile (mktemp "$tmpdir/enx-finalizer.XXXXXX")
    if test -z "$tmpfile"
        return 1
    end

    env ENX_FINALIZER_FILE="$tmpfile" command enx $argv
    set -l exit_code $status

    if test $exit_code -eq 0; and test -f "$tmpfile"
        while read -l line
            if string match -q "cd:*" -- $line
                cd (string replace -r '^cd:' '' -- $line); or true
            else if string match -q "setenv:*" -- $line
                set -l kv (string replace -r '^setenv:' '' -- $line)
                set -l parts (string split -m1 '=' -- $kv)
                if test (count $parts) -eq 2
                    set -gx $parts[1] $parts[2]
                end
            end
        end < "$tmpfile"
    end

    rm -f "$tmpfile"
    return $exit_code
end

{completions}
"#
    )
}

fn generate_fish() -> String {
    let completions = completions_for(Shell::Fish);
    fish_wrapper(&completions)
}
