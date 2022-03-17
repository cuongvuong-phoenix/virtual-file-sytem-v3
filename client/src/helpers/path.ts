export function decodePath(pathArr: string[]) {
  return `/${pathArr.join('/')}`;
}

export function encodePath(path: string, workingPath: string[]) {
  if (path.startsWith('/')) {
    workingPath.splice(0, workingPath.length);
  }

  const pathArr = path.split('/');

  if (path.startsWith('/')) {
    pathArr.shift();
  }
  if (path.endsWith('/')) {
    pathArr.pop();
  }

  while (pathArr.length > 0) {
    const pathSegment = pathArr.shift() as string;

    if (pathSegment === '.') {
      continue;
    } else if (pathSegment === '..') {
      workingPath.pop();
    } else {
      workingPath.push(pathSegment);
    }
  }

  return workingPath;
}
