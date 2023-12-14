import input from "../input.txt?raw";

class SpecialMap {
    constructor(mapFrom, mapTo, ranges) {
        this.mapFrom = mapFrom;
        this.mapTo = mapTo;
        this.starts = Object.keys(ranges).map(k => parseInt(k));
        this.starts.sort((a, b) => b - a);
        this.ranges = ranges;
    }

    get(value) {
        let index = this.starts.findIndex(x => x <= value);

        while (index >= 0) {
            const start = this.starts[index--];
            const range = this.ranges[start];
            if (value - start <= range.count) {
                return range.start + value - start;
            }
        }
        
        return value;
    }

    getRange(range) {
        const result = [];
        let start = range.start;
        let lastStart = range.start;
        while (start < range.start + range.count) {
            let myStart, myRange;
            let index = this.starts.findIndex(x => x <= start);

            if (index == -1) {
                // we are before the first range
                index = this.starts.length - 1;
                myStart = this.starts[index];
                myRange = this.ranges[myStart];
                if (myStart >= range.start + range.count) {
                    // console.log("map all to identity");
                    result.push(range);
                    break;
                } else {
                    // console.log("identity mapping");
                    result.push({ start: start, count: myStart - start });
                    start = myStart;
                }
            } else {
                myStart = this.starts[index];
                myRange = this.ranges[myStart];
            }
            
            if (start >= myStart + myRange.count) {
                // we are between ranges
                index--;
                if (index < 0) {
                    // we are after the end range
                    // console.log("last range");
                    result.push({ start: start, count: range.start + range.count - start });
                    break;
                } else {
                    // we found a larger range
                    myStart = this.starts[index];
                    myRange = this.ranges[myStart];
                    // console.log("identity mapping");
                    result.push({ start: start, count: myStart - start });
                    start = myStart;
                }
            }
            // at this point, start is at the beginning of the next largest range
            const end = Math.min(range.start + range.count, myStart + myRange.count);
            // console.log("regular mapping");
            result.push({ start: myRange.start + start - myStart, count: end - start });
            start = end;

            if (lastStart >= start) {
                console.error("The algorithm didn't advance. Start was " + start);
                break;
            }

            lastStart = start;
        }

        return result;
    }
}

function parseMap(rawMap) {
    const mappings = rawMap.split(/\r?\n/);
    const rawMapName = mappings.shift();
    const ranges = {};

    const [mapFrom, _, mapTo] = rawMapName.split(" ")[0].split("-");

    for (const line of mappings) {
        const [start, leftStart, count] = line.split(" ").map(x => parseInt(x));
        ranges[leftStart] = { start, count };
    }

    return { [mapFrom]: new SpecialMap(mapFrom, mapTo, ranges) };
}

export const maps = {};

const rawMaps = input.split(/\r?\n\r?\n/);
export const seeds = rawMaps.shift().split(": ")[1].split(" ").map(x => parseInt(x));

rawMaps.forEach(m => Object.assign(maps, parseMap(m)));


const testMappings = {
    5: { start: 10, count: 5 },
    20: { start: 50, count: 8 },
};

const testMap = new SpecialMap("a", "b", testMappings);

console.log(testMap.getRange({ start: 0, count: 100 }));
const cross = [
    { start: 0, count: 5 },
    { start: 10, count: 5 },
    { start: 10, count: 10 },
    { start: 50, count: 8 },
    { start: 28, count: 72 },
];

console.log(maps);