interface Point {
    x: number;
    y: number;
}

const main = async () => {
    const rocks = await getRocksFromFile();

    const answer1 = solution1(rocks);
    const answer2 = solution2(rocks);

    console.log(`The answer to part 1 is ${answer1}`);
    console.log(`The answer to part 2 is ${answer2}`);
};

const getRocksFromFile = async () => {
    const rocks: Point[] = [];
    const INPUT_FILE = "./test.txt";
    const input = await Deno.readTextFile(INPUT_FILE);
    const lines = input.split("\n");

    for (const line of lines) {
        const points: Point[] = line.split("->").map(point => {
            const [x, y] = point.split(",");
            return { x: parseInt(x), y: parseInt(y) };
        });

        for (let i = 0; i < points.length - 1; i++) {
            rocks.push(points[i]);

            const xDiff = Math.abs(points[i + 1].x - points[i].x);
            const yDiff = Math.abs(points[i + 1].y - points[i].y);

            if (xDiff >= yDiff) {
                for (let j = 1; j < xDiff; j++) {
                    const x = points[i].x + j * Math.sign(points[i + 1].x - points[i].x);
                    const y = points[i].y + j * Math.sign(points[i + 1].y - points[i].y);
                    rocks.push({ x, y });
                }
            } else {
                for (let j = 1; j < yDiff; j++) {
                    const x = points[i].x + j * Math.sign(points[i + 1].x - points[i].x);
                    const y = points[i].y + j * Math.sign(points[i + 1].y - points[i].y);
                    rocks.push({ x, y });
                }
            }
        }

        rocks.push(points[points.length - 1]);
    }

    return rocks;
};

const solution1 = (rocks: Point[]) => {
    const farthestPositiveX = Math.max(...rocks.map(r => r.x));
    const farthestNegativeX = Math.min(...rocks.map(r => r.x));
    const farthestY = Math.max(...rocks.map(r => r.y));
    let sandCount = 0;

    while (true) {
        const { newSand, overflow } = drop1Sand(rocks, farthestPositiveX, farthestNegativeX, farthestY);
        if (!overflow) {
            sandCount++;
            rocks.push(newSand);
        } else {
            break;
        }
    }

    return sandCount;
};

const solution2 = (rocks: Point[]) => {
    const floorIndex = Math.max(...rocks.map(r => r.y)) + 2;
    let sandCount = 0;

    while (true) {
        const newSand = drop1SandSolution2(rocks, floorIndex);
        sandCount++;

        if (newSand.x == 500 && newSand.y == 0) {
            break;
        } else {
            rocks.push(newSand);
        }
    }

    return sandCount;
};

const drop1Sand = (obstacles: Point[], farthestPositiveX: number, farthestNegativeX: number, farthestY: number) => {
    const newSand: Point = { x: 500, y: 0 };
    let hasNextPoint = true;
    let overflow = false;

    while (hasNextPoint) {
        if (newSand.x > farthestPositiveX || newSand.x < farthestNegativeX || newSand.y > farthestY) {
            hasNextPoint = false;
            overflow = true;
        } else {
            if (!obstacles.filter(o => o.x === newSand.x && o.y === newSand.y + 1).length) {
                newSand.y++;
            } else if (!obstacles.filter(o => o.x === newSand.x - 1 && o.y === newSand.y + 1).length) {
                newSand.x--;
                newSand.y++;
            } else if (!obstacles.filter(o => o.x === newSand.x + 1 && o.y === newSand.y + 1).length) {
                newSand.x++;
                newSand.y++;
            } else {
                hasNextPoint = false;
            }
        }
    }

    return { newSand, overflow };
};

const drop1SandSolution2 = (obstacles: Point[], floorIndex: number) => {
    const newSand: Point = { x: 500, y: 0 };

    while (true) {
        if (newSand.y < floorIndex - 1) {
            if (!obstacles.filter(o => o.x === newSand.x && o.y === newSand.y + 1).length) {
                newSand.y++;
            } else if (!obstacles.filter(o => o.x === newSand.x - 1 && o.y === newSand.y + 1).length) {
                newSand.x--;
                newSand.y++;
            } else if (!obstacles.filter(o => o.x === newSand.x + 1 && o.y === newSand.y + 1).length) {
                newSand.x++;
                newSand.y++;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    return newSand;
};

main();
