## ed-debby

A personal learning project rewriting the venerable `ed` line editor in Rust. 

## Why `ed`? 

- CLI editors are cool, and are useful for things like non-interactive text twiddling, big search/replace operations, and zero-distraction editing.
- Most of the behavior fits on one man page.
- Limited programming scope to focus on doing it right.
- Getting practice with parsing and domain-specific language implementation.


## Why is the project called ed-debby?

First commits were made during hurricane Debby, 2024.

## Differences so Far

It probably won't be possible to get full compatibility, particularly in the area of regular expression processing. If compatibility is a concern, please use the default `ed`. Here are some differences so far: 

- Gnu `ed` uses a linked list for the buffer structure. Rust sources strongly recommend Rust's dynamic vector or other data structures instead. For now that seems like a safe bet.
- Command history, editing, and other quality-of-life features provided by `rustyline`.

# Implemented and Planned Features

## Ranges

- `.` - Represents the current line in the buffer.
- `$` - Refers to the last line in the buffer.
- `n` - Targets the nth line in the buffer, where n is a number in the range [0,$].
- TODO `-` or `^` - Moves to the previous line. This is equivalent to -1 and may be repeated with cumulative effect.
- TODO `-n` or `^n` - Moves to the nth previous line, where n is a non-negative number.
- TODO `+` - Moves to the next line. This is equivalent to +1 and may be repeated with cumulative effect.
- TODO `+n` - Moves to the nth next line, where n is a non-negative number.
- `,` or `%` - Selects the first through last lines in the buffer. This is equivalent to the address range 1,$.
- TODO `;` - Selects from the current through last lines in the buffer. This is equivalent to the address range .,$.
- TODO `/re/` - Searches for the next line containing the regular expression `re`. The search wraps to the beginning of the buffer and continues down to the current line, if necessary. The second slash can be omitted if it ends a line. `//` repeats the last search.
- TODO `?re?` - Searches for the previous line containing the regular expression `re`. The search wraps to the end of the buffer and continues up to the current line, if necessary. The second question mark can be omitted if it ends a line. `??` repeats the last search.
- TODO `'lc` - Navigates to the line previously marked by a `k` (mark) command, where `lc` is a lowercase letter.

## Commands

### Editing Commands
- `(.)a`: Appends text to the buffer after the addressed line. Text is entered in input mode.
- `(.,.)c`: Changes lines in the buffer. The addressed lines are deleted, and text is appended in their place.
- `(.,.)d`: Deletes the addressed lines from the buffer.
- TODO `e file`: Edits the specified file and sets it as the default filename.
- TODO `E file`: Edits the specified file unconditionally, discarding any unsaved changes.

### Display Commands
- `(.,.)p`: Prints the addressed lines.
- TODO `(.,.)n`: Prints the addressed lines with their line numbers.
- TODO `(.,.)l`: Prints the addressed lines unambiguously.

### File Operations
- `f file`: Sets or displays the default filename.
- `(1,$)w file`: Writes the addressed lines to the specified file, replacing the file's contents. (Only whole buffer)
- TODO `(1,$)W file`: Appends the addressed lines to the specified file.
- `(1,$)wq file`: Saves the addressed lines to a file and quits `ed`. (Only whole buffer)

### Buffer Modification
-  `(.)i`: Inserts text before the current line.
-  `(.)a`: Appends text after the current line.
-  `(.,.)c`: Prompts for input and replaces the addressed lines.
-  `(.,.)d`: Deletes line range.
- TODO `(.,+)j`: Joins the addressed lines into a single line.
- TODO `(.,.)m(.)`: Moves the addressed lines to the specified destination address.
- TODO `(.,.)t(.)`: Copies the addressed lines to the specified destination address.

### Search and Replace
- TODO `(.,.)s/re/replacement/`: Performs substitution on the addressed lines.
- TODO `(1,$)g/re/command-list`: Executes commands on lines matching a regex.
- TODO `(1,$)v/re/command-list`: Applies command-list to lines not matching the regex.

### Regular Expressions
- Regular expressions are used to match patterns in text.

### Miscellaneous Commands
- TODO `H`: Toggles the printing of error explanations.
- TODO `(+)zn`: Scrolls n lines at a time starting at addressed line.
- TODO `($)=`: Prints the line number of the current or specified line.
- TODO `!command`: Executes the specified command via the shell.

### Exiting Commands
-  `q`: Quits `ed`, prompting if there are unsaved changes.
- TODO `Q`: Quits `ed` immediately without prompting for unsaved changes.
