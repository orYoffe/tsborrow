type Owned<T> = T;

const token: Owned<Token> = acquireToken();
for (const job of jobs) {
  submit(job, move(token));
}
