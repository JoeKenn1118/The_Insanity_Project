ls
Hello,
Jebus this is getting hairy.
I think I added too many things too quickly.

Need to make changing functions maybe?
Problem with that is structs in structs gets dirty

wrapper means that making struct sub values pub safer

Something I want,

Represent the map as a 3D array, with each box having paths to boxes around it
    - Start as a 2D array
    - Each box has a quick and long description
        - one is read when player is considering options
        - one is read when player is entering area
            - if the player has a certain skill then they can get the long description instead of short