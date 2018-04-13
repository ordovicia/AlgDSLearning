import Control.Applicative
import Control.Monad
import System.IO


main :: IO ()
main = do
    n_temp <- getLine
    let n = read n_temp :: Integer
    print $ factorial n

factorial :: Integer -> Integer
factorial 1 = 1
factorial n = n * factorial (n - 1)

getMultipleLines :: Int -> IO [String]
getMultipleLines n
    | n <= 0 = return []
    | otherwise = do
        x <- getLine
        xs <- getMultipleLines (n-1)
        let ret = (x:xs)
        return ret
