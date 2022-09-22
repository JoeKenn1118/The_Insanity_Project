wrapper functions safer than making struct vars pubs

Need a world variable? 4D array of areas? [plane, x, y, z]

Need a Room/Area Variable, contains information on area and encounters, can pass in and pull out player.

Need an encounter variable, contains monsters (puzzles?)

Start with encounter, build it so it works, then build the room, then build the world

Future Development:
Represent the map as a 3D array, with each box having paths to boxes around it
    - Start as a 2D array
    - Each box has a quick and long description
        - one is read when player is considering options
        - one is read when player is entering area
            - if the player has a certain skill then they can get the long description instead of short