import functools

class DataInput:
    def __init__  (self, dataStr):
        self.bytes = [int(dataStr[i:i+2], 16) for i in range(0, len(dataStr), 2)]

        self.bytePos = 0
        self.bitPos = 0

    def readCurrBits (self, n):
        bits = min(8 - self.bitPos, n)

        val = (self.bytes[self.bytePos] >> (8 - self.bitPos - bits)) & ((1 << bits) - 1)

        self.bitPos += bits

        if self.bitPos == 8:
            self.bytePos += 1
            self.bitPos = 0

        return val, bits
    
    def hasBits (self):
        return self.bytePos < len(self.bytes)
    
    def readBits (self, n):
        num = 0

        bitsToGo = n

        while self.hasBits() and bitsToGo > 0:
            bits, readBits = self.readCurrBits(bitsToGo)

            bitsToGo -= readBits

            num = num << readBits | bits
        
        return num

    def getLocation (self):
        return self.bytePos * 8 + self.bitPos

class Packet:
    def __init__ (self, version, typeId):
        self.version = version
        self.typeId = typeId

    def getVersion (self):
        return self.version
    
    def getTypeId (self):
        return self.typeId

    def getValue (self):
        raise NotImplementedError("Need to implement this!")

    def printPacket (self, indent=""):
        raise NotImplementedError("Need to implement this!")
    
    def getVersionTotal (self):
        return self.getVersion()

def parsePacket (data: DataInput):
    version = data.readBits(3)
    typeId = data.readBits(3)

    if typeId == 4:
        return LiteralPacket(data, version, typeId)
    else:
        return OperatorPacket(data, version, typeId)   

class LiteralPacket(Packet):
    def __init__ (self, data: DataInput, version, typeId):
        super(LiteralPacket, self).__init__(version, typeId)
        self.value = 0

        while True:
            readNum = data.readBits(5)
            self.value = self.value << 4 | (readNum & 0xF)
            if readNum >> 4 == 0:
                break
        
    def getValue (self):
        return self.value
    
    def printPacket (self, indent=""):
        indentStr = indent
        if indent != "":
            indentStr += "=>"
        print(indentStr + f"version: {self.version} type: {self.typeId} (literal) value: {self.value}")

class OperatorPacket(Packet):
        def __init__ (self, data: DataInput, version, typeId):
            super(OperatorPacket, self).__init__(version, typeId)
            self.readType = data.readBits(1)

            if self.readType:
                self.children = [parsePacket(data) for i in range(0, data.readBits(11))]
            else:
                self.children = []
                length = data.readBits(15)
                start = data.getLocation()
                while data.getLocation() < start + length:
                    self.children.append(parsePacket(data))

            if self.typeId == 0:
                self.opType = "sum"
                self.op = lambda l: sum(l)
            elif self.typeId == 1:
                self.opType = "product"
                self.op = lambda l: functools.reduce(lambda a, b: a * b, l, 1)
            elif self.typeId == 2:
                self.opType = "minimum"
                self.op = lambda l: min(l)
            elif self.typeId == 3:
               self.opType = "maximum"
               self.op = lambda l: max(l)
            elif self.typeId == 5:
                self.opType = "greater-than"
                self.op = lambda l: 1 if l[0] > l[1] else 0
            elif self.typeId == 6:
                self.opType = "less-than"
                self.op = lambda l: 1 if l[0] < l[1] else 0
            else:
                self.opType = "equal"
                self.op = lambda l: 1 if l[0] == l[1] else 0
        
        def printPacket(self, indent=""):
            indentStr = indent
            if indent != "":
                indentStr += "=>"

            print(indentStr + f"version: {self.version} type: {self.typeId} (operator={self.opType}):")
            for i in self.children:
                i.printPacket(indent + "  ") 
                

        def getVersionTotal(self):
            return self.version + sum(i.getVersionTotal() for i in self.children)
        
        def getValue (self):
            return self.op([i.getValue() for i in self.children])
        
f = open("data/day16.txt")

start = True

for i in f.readlines():
    if i.strip() != "":
        if not start:
            print("\n")
        start = False
        print(i.strip())
        data = DataInput(i.strip())

        packet = parsePacket(data)

        packet.printPacket()

        print(f"Version total: {packet.getVersionTotal()}")
        print(f"Value: {packet.getValue()}")

