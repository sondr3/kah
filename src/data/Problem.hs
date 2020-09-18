readInput = (map read) . words
writeOutput = unlines . (map show)

main = interact (writeOutput . solve . readInput)
