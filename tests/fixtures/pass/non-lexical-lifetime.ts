type Owned<T> = T;

const record: Owned<Record<string, string>> = loadRecord();
const reader = borrow(record);
reader.read();
const writer = borrowMut(record);
writer.update();
