module SpaceAge (Planet(..), ageOn) where

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune


periodInEarthYears :: Planet -> Float
periodInEarthYears Mercury = 0.2408467
periodInEarthYears Venus = 0.61519726
periodInEarthYears Earth = 1
periodInEarthYears Mars = 1.8808158
periodInEarthYears Jupiter = 11.862615
periodInEarthYears Saturn = 29.447498
periodInEarthYears Uranus = 84.016846
periodInEarthYears Neptune = 164.79132


ageOn :: Planet -> Float -> Float
ageOn planet seconds =
    ageOnEarth / periodInEarthYears planet
    where ageOnEarth = seconds / 31557600
