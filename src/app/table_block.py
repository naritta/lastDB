class TableBlock:
    def __init__(self, header={}, line_pointer=[], heap_tuple=[]):
        self.header = header
        self.line_pointer = line_pointer
        self.heap_tuple = heap_tuple

    def sequencialScan(start, end):
        return self.heap_tuple[start:end]

    def getLogId():
        if "logId" in self.header:
            return self.header["logId"]
        else:
            return None
