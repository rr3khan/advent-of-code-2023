# Advent of Code 2023 - Day 2: Cube Conundrum

## Part One

You find yourself on Snow Island, a floating island in the sky. An Elf greets you and invites you to play a game involving cubes of different colors. The Elf reveals subsets of cubes from a bag, and your goal is to deduce information about the number of cubes. Your task is to determine which games would have been possible if the bag had specific counts of red, green, and blue cubes.

### Problem Description

You are provided with records of several games, each identified by an ID number. Each record consists of subsets of cubes revealed during the game. Your goal is to identify the games that would have been possible if the bag contained a specific configuration of red, green, and blue cubes.

#### Example

For instance, if the bag contained 12 red cubes, 13 green cubes, and 14 blue cubes, games 1, 2, and 5 would be possible, while games 3 and 4 would not be possible. The sum of the IDs of the possible games is 8.

### Solution

The solution involves parsing the game records, checking if each game is possible given the cube counts, and summing the IDs of the possible games.

## Part Two

The Elf informs you that they've stopped producing snow due to a lack of water. To address this issue, you need to determine the fewest number of cubes of each color required for each game to be possible.

### Problem Description

For each game, find the minimum set of cubes (minimum counts for red, green, and blue) that must have been present for the game to be possible. Calculate the power of each set, which is the product of the counts of red, green, and blue cubes. Sum up these powers for all games.

#### Example

For the example games, calculate the minimum set for each game and find the sum of their powers: 2286.

### Solution

The solution involves determining the minimum set of cubes for each game and calculating the power of each set.

Your puzzle answer for Part Two is the sum of the powers of the minimum sets.

Enjoy solving the Cube Conundrum on Snow Island!