module Main where

import System.Random
import System.IO

piApprox :: Int -> IO Float
piApprox maxIter = 
  let iterMonteCarlo currentIter numPointsInside = 
        if currentIter < maxIter then do 
          x <- randomRIO (0::Float, 1::Float)
          y <- randomRIO (0::Float, 1::Float)
          let result = numPointsInside + fromEnum( x*y + y*y < 1)
          iterMonteCarlo (currentIter + 1) result
        else 
          return (4 * fromIntegral numPointsInside / fromIntegral maxIter)
  in iterMonteCarlo 0 0

main :: IO ()
main = do
  putStrLn "Monte-Carlo Pi Approximation"
  putStr "Number of Iterations: "
  hFlush stdout
  inputString <- getLine
  let maxIter = read inputString::Int
  pi <- piApprox maxIter
  putStrLn ("Pi Approximation with " ++ inputString ++ " iterations is " ++ show pi)
