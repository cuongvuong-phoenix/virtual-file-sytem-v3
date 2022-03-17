/* ----------------------------------------------------------------
Node
---------------------------------------------------------------- */
interface VfsNode {
  id: number;
  path: string[];
  isFolder: boolean;
  created_at?: Date;
}

interface VfsNodeLsItem {
  id: number;
  path: string[];
  isFolder: boolean;
  created_at: Date;
  size: number;
}

/* ----------------------------------------------------------------
Block
---------------------------------------------------------------- */
interface ResponseError {
  code: string;
  message: string;
}

interface ParsedArgv {
  _: string[];
  [key: string]: any;
}

interface Block {
  id: number;
  workingNode: VfsNode;

  isCommand?: boolean;
  command?: string;
  parsedArgv?: ParsedArgv;

  data?: any;
  loading?: boolean;
  error?: ResponseError;

  createdAt: Date;
}
