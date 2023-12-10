import input from "../input.txt?raw";

/**
 * @param {string} input 
 */
function parseInput(input) {
  return input.split(/\r?\n/).filter(l => l != "");
}

/**
 * @param {string} line 
 */
function extractNumberDigitsOnly(line) {
  const matches = [...line.matchAll(/[0-9]/gd)];
  const first = matches[0];
  const last = matches[matches.length - 1];
  return parseInt(first + last);
}

/**
 * @param {string} line 
 */
function extractNumberAlsoWords(line) {
  const conversion = {
    one: "1",
    two: "2",
    three: "3",
    four: "4",
    five: "5",
    six: "6",
    seven: "7",
    eight: "8",
    nine: "9",
  };

  const conversionReverse = {};

  Object.entries(conversion).forEach(([key, value]) => conversionReverse[key.split("").reverse().join("")] = value);

  function convertToDigit(str, conversion) {
    if (str in conversion) {
      return conversion[str];
    } else {
      return str;
    }
  }

  const re = new RegExp("[0-9]|" + Object.keys(conversion).join("|"), "g");
  const reReverse = new RegExp("[0-9]|" + Object.keys(conversionReverse).join("|"), "g");

  const matches = line.match(re);
  const matchesReverse = line.split("").reverse().join("").match(reReverse);

  const first = convertToDigit(matches[0], conversion);
  const last = convertToDigit(matchesReverse[0], conversionReverse);

  return parseInt(first + last);
}

/**
 * @param {string[]} input 
 * @param {(line: string) => number} numberExtractor
 */
function solve(input, numberExtractor) {
  let result = 0;
  for (let line of input) {
    const n = numberExtractor(line);
    result += n;
  }

  return result;
}

const problem = parseInput(input);

const solution1 = solve(problem, extractNumberDigitsOnly);
const solution2 = solve(problem, extractNumberAlsoWords);

document.querySelector('#app').innerHTML = `<ol>
    <li>${solution1}</li>
    <li>${solution2}</li>
</ol>`;