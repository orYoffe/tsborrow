type Resource<T> = T;

const stream: Resource<Stream> = openStream();
const reader = borrow(stream);
dispose(stream);
reader.read();
