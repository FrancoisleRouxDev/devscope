# Security policy

## Reporting a vulnerability

Please **do not open a public issue** for security problems.

Report it privately instead:

1. Go to the repository's **Security** tab and choose **Report a vulnerability**
   (GitHub private vulnerability reporting), or
2. email **francoislerouxdev@gmail.com** with "devscope security" in the subject.

Include what you found, how to reproduce it, and which version you used. I aim to reply within
a week, and to fix confirmed problems in the next release, crediting you if you wish.

## Supported versions

devscope is an early preview. Only the latest release receives security fixes.

## What devscope does with your data

devscope reads file names, sizes, dates and (for the duplicate and secrets checks) file contents,
all on your own computer. It never uploads anything, has no telemetry, and never modifies or
deletes your files. The only network request the installed app makes is a check for a newer
version on `github.com`.

## Scope

Of particular interest:

- Anything that lets a folder or file that devscope analyses run code or write to disk.
- Any way for another program or web page to use devscope's local engine without the app's token.
- Any way to escape the app window's sandbox.

The design and the protections in place are described in the "Security and privacy" section of
the README.
