interface Block {
  id: number;
  workingNode: {
    id: number;
    path: string;
  };
  isCommand?: boolean;
  command?: string;
  data?: any;
  error?: any;
  loading?: boolean;
  ready?: boolean;
  createdAt: Date;
}
