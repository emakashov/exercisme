module Bob (responseFor) where
import Data.Char (isAlpha, isUpper, isSpace)


responseFor :: String -> String
responseFor xs
    | isAnything = "Fine. Be that way!"
    | isYell     = "Whoa, chill out!"
    | isAsk      = "Sure."
    | otherwise  = "Whatever."
    where
        isWeirCharacter x = elem x "%^*@#$(*^"
        isAsk =
            (==) '?' . head . filter (\x -> not $ isSpace x) . reverse $ xs
        isYell =
            (any isAlpha xs) && (all isUpper $ filter isAlpha xs)
            || any isWeirCharacter xs
        isAnything =
            length xs == 0
            || all isSpace xs
