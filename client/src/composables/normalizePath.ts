export function normalizePath(pathArr: string[]) {
  return `/${pathArr.join('/')}`;
}
