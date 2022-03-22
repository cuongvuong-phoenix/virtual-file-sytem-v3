export function decodePath(pathArr: string[]) {
  return `/${pathArr.join('/')}`;
}

export const VALID_PATH_SEGMENT_REGEX = /^[a-zA-z0-9 _-]+$/;

export function encodePath(workingPath: string[], path?: string) {
  const result = workingPath.slice();

  if (path !== undefined) {
    if (path.startsWith('/')) {
      result.splice(0, result.length);
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
        result.pop();
      } else {
        result.push(pathSegment);
      }
    }
  }

  result.forEach((pathSegment) => {
    if (!VALID_PATH_SEGMENT_REGEX.test(pathSegment)) {
      throw new Error(`invalid name in path ("${pathSegment}"), must match the regex /^[a-zA-z0-9 _-]+$/`);
    }
  });

  return result;
}
