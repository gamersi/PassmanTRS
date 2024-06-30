export type Block = {
  nonce: string;
  data: string;
};

export type Password = {
  id: number;
  name: string;
  username: string;
  password: Block;
  url: string;
  notes: string;
  decrypted_password?: string;
};
