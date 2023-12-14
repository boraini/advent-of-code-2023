import { maps, seeds } from "./parsed-input"

const START = "seed";
const END = "location";

function part1(maps, seeds) {
  let min = Infinity;
  for (let seed of seeds) {
    let currentObject = START;
    let current = seed;
    while (currentObject != END) {
      const map = maps[currentObject];
      currentObject = map.mapTo;
      current = map.get(current);
    }
    min = Math.min(min, current);
  }
  return min;
}

function part2(maps, seeds) {
  const mySeeds = [];
  for (let i = 0; i < seeds.length; i += 2) {
    mySeeds.push({ start: seeds[i], count: seeds[i + 1]});
  }

  console.log(mySeeds);

  let min = Infinity;
  for (let seed of mySeeds) {
    let currentObject = START;
    let current = [seed];
    while (currentObject != END) {
      const map = maps[currentObject];
      currentObject = map.mapTo;
      current = current.flatMap(c => map.getRange(c));
    }
    for (let location of current) {
      min = Math.min(min, location.start);
    }
  }

  return min;
}

document.querySelector('#app').innerHTML = `<ol>
  <li>${part1(maps, seeds)}</li>
  <li>${part2(maps, seeds)}</li>
</ol>`
