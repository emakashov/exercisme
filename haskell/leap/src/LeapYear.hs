module LeapYear (isLeapYear) where

isLeapYear :: Integer -> Bool
isLeapYear year =
    if divBy 100
        then divBy 400
        else divBy 4
    where divBy x = (==) 0 $ year `mod` x
