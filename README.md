# Boraini's Advent of Code 2023

My solutions for (some) Advent of Code 2023 challenges.

Each challenge folder may have solutions with multiple programming languages and their toolchains.

- rust: these are Cargo crates. cd to these folderz and use `cargo run`.
- javascript: these use Vite and are npm packages. cd to these folders and first use `npm i`, then use use `npm run dev`, and finally open the provided link in your web browser.
- python: these are vanilla Python projects. cd to these folders, install the pip packages found in requirements.txt, then use `python .`
- kodit: these use the Kodit interpreter that I have built. Build the kodit executable from the source found in <https://github.com/boraini/kodit/>. cd to these folders and run `python build.py`. A dist.kdt file will appear. Run this file according to that repository's README.

**Each solution expects you to put the input.txt to where the language folders are located** (so while the code is running its path will be `../input.txt`).