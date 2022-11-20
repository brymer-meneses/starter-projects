
-- Caesar Cipher
-- This is my first haskell program
-- November 20, 2022


import System.IO
import Data.Char


letterToIndex :: Char -> Int
letterToIndex letter
  | 'a' <= letter && letter <= 'z' = ord letter - ord 'a'
  | 'A' <= letter && letter <= 'Z' = ord letter - ord 'A'
  | otherwise = undefined

shiftLetter :: Char -> Int -> Char
shiftLetter letter shift
  | 'a' <= letter && letter <= 'z' = chr ( index + ord 'a' )
  | 'A' <= letter && letter <= 'Z' = chr ( index + ord 'A' )
  | otherwise = letter
  where index = ( letterToIndex letter + shift ) `mod` 26


main = do
  putStrLn "Caesar Cipher"
  putStr "Input String: "
  hFlush stdout
  inputString <- getLine
  putStr "Key: "
  hFlush stdout
  keyStr <- getLine
  let key  = read keyStr :: Int
  let encrypted = map (`shiftLetter` key ) inputString
  putStr "Encrypted String: "
  putStrLn encrypted




