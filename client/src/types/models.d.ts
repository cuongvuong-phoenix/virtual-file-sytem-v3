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
interface Block {
  id: number;
  workingNode: VfsNode;
  isCommand?: boolean;
  command?: string;
  loading?: boolean;
  ready?: boolean;
  data?: any;
  error?: any;
  createdAt: Date;
}
