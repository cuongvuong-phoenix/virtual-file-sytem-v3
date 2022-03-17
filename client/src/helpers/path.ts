export function decodePath(pathArr: string[]) {
  return `/${pathArr.join('/')}`;
}
