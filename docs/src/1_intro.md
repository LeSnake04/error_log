# Introduction

## Terminologie

- entries: Vector of errors and log messages
- features: cargo features

## Why does this exist?

Error handling in Rust mostly works by eighter handeling the error and a message
on the terminal or panicing. This works for most cases - but not when writing a
GUI or TUI application, where you want to inform the user messages inside the
App.

For example: You made a tool called `cool_util` with a CLI interface that adds 2
numbers and start making a GUI or TUI for the app: `cool_util_gui`. For the Cli
tool you used log messages and println to inform the user. Now you decide to
make an error dialog for the GUI when something wents wrong. But how do you get
the messages? Redirecting and catching stdout is very tedious and Writing a
logger is not so easy, catches a lot of unessesary info, e.g. `wgpu` log
messages. These ways would also restrict you from using stderr or log for
debugging of the UI.

I created this library for cases like this.

## How it works.

This crate works by proving `ErrorLog` a struct that allows you to simply store
errors and log messages, so you can decide how to process them.

You can uilize it by using `ErrorLog` instead of ´Result´ in the parent crate
(In this case, `cool_util`). It provides the same features as ´Result´. They can
be easily merged to keep the configuration, but sync ok values and entries.

By default, it just prints to the console, but you can use `log` macros with
just one function. When in CLI mode, you should let it print to the console. You
can use the `instant_display` function to show messages the moment you add them.
This makes it behave just like it does when using log and prinln.

Meanwhile in the GUI application, you have 3 options:

1. Use the build in `native_dialog` feature
2. Write your own dialog and add it via `ErrorLog.set_display_fn()`.
3. Read the entries e.g. with `ErrorLog.entries()` and display them.

Option 1+2 and allow the use of `instant_display`.

## Functionalities

- comprehensive entry management
- direct access to underlying data
- build in presets (see Features)
- 100% safe code
- simple usage with `+=`, `*`, `+=`
- option to instantly display new entries
- [no_std support](./1_1_no_std.md)

## Features

- default
- std
- default_no_std
  - default features but without std
- instant-display
- messages
  - toggles managing of log messages
- errors
  - toggles managing of errors
