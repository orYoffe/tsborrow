type Resource<T> = T;

const file: Resource<FileHandle> = openFile("report.txt");
file.write("complete");
dispose(file);
