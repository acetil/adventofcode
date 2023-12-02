from typing import List, Tuple

amphiprods = "ABCD"

costs = {
    "A" : 1,
    "B" : 10,
    "C" : 100,
    "D" : 1000
}


class GameState:
    def __init__ (self, rooms: List[str], hallway: List[str]):
        self.rooms = rooms
        self.hallway = hallway
        self.roomStr = "".join(i if i is not None else "." for j in self.rooms for i in j) + "".join(i if i is not None else "." for i in self.hallway)
    
    def __hash__(self) -> int:
        return hash(self.roomStr)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, GameState):
            return False
        
        return self.roomStr == other.roomStr
    
    def accessibleFromRoom (self, room: int, hallway: int) -> bool:
        roomPos = room + 2
        if hallway < roomPos:
            return all(i is None for i in self.hallway[hallway:roomPos])
        else:
            return all(i is None for i in self.hallway[roomPos:hallway + 1])

    def roomEnterable (self, room: int) -> bool:
        amphiType = amphiprods[room]
        return all(i is None or i == amphiType for i in self.rooms[room])
    
    def accessibleFromHallway (self, room: int, hallway: int) -> bool:
        if self.hallway[hallway] is None:
            return False
        elif not self.roomEnterable(room) or self.hallway[hallway] != amphiprods[room]:
            return False
        
        return self.accessibleFromRoom(room, hallway)

    def getRoomDepth (self, room: int) -> int:
        pass

    def getNextGamestates (self) -> List[Tuple['GameState', int]]:
        nextStates: List[Tuple[GameState, int]] = []

        for i in range(len(self.hallway)):
            if self.hallway[i] is not None:
                for j in range(len(self.rooms)):
                    if self.accessibleFromHallway(j, i):
                        pass


rooms = [[] for i in range(0, 4)]

with open("day23.txt") as f:
    f.readline()
    f.readline()

    line1 = f.readline()
    line2 = f.readline()

    lines = [line1, line2]

    for i in lines:
        for j, x in enumerate([k for k in i.strip().split("#") if k != ""]):
            rooms[j].append(x)

