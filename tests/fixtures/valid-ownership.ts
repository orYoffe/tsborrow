type Owned<T> = T;

const file: Owned<File> = openFile("report.txt");
const movedFile = move(file);
movedFile.read();
