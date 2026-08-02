type Resource<T> = T;

const file: Resource<FileHandle> = openFile("data.bin");
dispose(file);
file.read();
