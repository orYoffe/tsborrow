interface Avatar {}

interface Buffer {
  clear(): void;
  write(value: string): void;
}

interface Cache {
  read(): void;
  write(): void;
}

interface Config {
  read(): void;
}

interface Connection {
  query(statement: string): void;
}

interface DataRecord {
  read(): void;
  update(): void;
}

interface Document {
  read(): void;
}

interface File {
  read(): void;
}

interface FileHandle extends File {
  write(value: string): void;
}

interface Job {}

interface Queue {
  push(value: string): void;
}

interface Secret {}

interface Session {
  close(): void;
  inspect(): void;
  send(value: string): void;
}

interface Socket {
  read(): void;
  send(value: string): void;
}

interface State {
  get(key: string): unknown;
  set(key: string, value: unknown): void;
}

interface Store {
  get(key: string): unknown;
  set(key: string, value: unknown): void;
}

interface Stream {
  read(): void;
}

interface Token {}

interface Transaction {
  commitWork(): void;
}

declare const console: {
  log(...values: unknown[]): void;
};

declare const jobs: readonly Job[];

declare function acquireToken(): Token;
declare function allocate(): Buffer;
declare function beginTransaction(): Transaction;
declare function borrow<T>(value: T): T;
declare function borrowMut<T>(value: T): T;
declare function connect(): Socket;
declare function createCache(): Cache;
declare function createQueue(): Queue;
declare function createSession(): Session;
declare function createState(): State;
declare function createStore(): Store;
declare function dispose<T>(value: T): void;
declare function endBorrow<T>(value: T): void;
declare function loadConfig(): Config;
declare function loadRecord(): DataRecord;
declare function loadSecret(): Secret;
declare function loadUser(): User;
declare function move<T>(value: T): T;
declare function openConnection(): Connection;
declare function openDocument(): Document;
declare function openFile(path: string): FileHandle;
declare function openStream(): Stream;
declare function render(avatar: Avatar): void;
declare function save(user: User): void;
declare function schedule(): Promise<void>;
declare function setTimeout(callback: () => void, delay: number): number;
declare function shouldTransfer(): boolean;
declare function submit(job: Job, token: Token): void;
declare function upload(avatar: Avatar): void;

interface User {
  name: string;
  profile: {
    avatar: Avatar;
  };
}
