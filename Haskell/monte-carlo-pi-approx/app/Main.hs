module Main where

import System.Random
import System.IO

piApprox :: Int -> Int -> Int -> IO Float
piApprox currentIter maxIter numPointsInside = 
  if currentIter < maxIter then do 
    x <- randomRIO (0::Float, 1::Float)
    y <- randomRIO (0::Float, 1::Float)
    let result = numPointsInside + fromEnum( x*y + y*y < 1)
    piApprox (currentIter + 1) maxIter result
  else 
    return (4 * fromIntegral numPointsInside / fromIntegral maxIter)

main :: IO ()
main = do
  putStrLn "Monte-Carlo Pi Approximation"
  putStr "Number of Iterations: "
  hFlush stdout
  inputString <- getLine
  let maxIter = read inputString::Int
  pi <- piApprox 0 maxIter 0 
  putStrLn ("Pi Approximation with " ++ inputString ++ " iterations is " ++ show pi)
