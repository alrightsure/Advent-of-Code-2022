const main = async () => {
    const pairs = await getPairsFromFile();
    
    const answer1 = solution1(pairs);
    const answer2 = solution2(pairs);

    console.log(`The answer to question 1 is ${answer1}`);
    console.log(`The answer to question 2 is ${answer2}`);
};

const getPairsFromFile = async (): Promise<any[][]> => {
    const INPUT_FILE = "./input.txt";
    const input = await Deno.readTextFile(INPUT_FILE);
    const lines = input.split("\n");

    const pairs = [];

    let i = 0;
    while (i < lines.length) {
        const packet1 = lines[i];
        const packet2 = lines[i + 1];

        pairs.push([eval(packet1), eval(packet2)]);
        i += 3;
    }

    return pairs;
};

const comparePairs = (packet1: any, packet2: any): number => {
    for (let i = 0; i < packet1.length; i++) {
        // A is longer than B, not valid signal
        if (i >= packet2.length) {
            return 1;
        }
        const first = packet1[i];
        const second = packet2[i];
        if (Array.isArray(first) && Array.isArray(second)) {
            const result = comparePairs(first, second);
            if (result !== 0) {
                return result;
            }
        } else if (Array.isArray(first)) {
            const result = comparePairs(first, [second]);
            if (result !== 0) {
                return result;
            }
        } else if (Array.isArray(second)) {
            const result = comparePairs([first], second);
            if (result !== 0) {
                return result;
            }
        } else if (first !== second) {
            return first - second;
        }
    }
    if (packet1.length < packet2.length) {
        return -1;
    }
    return 0;
};

const solution1 = (pairs: any[][]): number => {
    return (
        pairs.reduce((acc, pair, i) => {
            if (comparePairs(pair[0], pair[1]) < 0) {
                return acc + i + 1;
            }
            return acc;
        }, 0)
    );
};

const solution2 = (pairs: any[][]): number => {
    const dividerPackets = [[[2]], [[6]]];
    const packets = pairs.flat();
    packets.push(...dividerPackets);
    packets.sort(comparePairs);

    const dividerIndices = dividerPackets.map(
        (packet) => packets.indexOf(packet) + 1
    );

    return dividerIndices.reduce((acc, index) => acc * index, 1);
}

main();
