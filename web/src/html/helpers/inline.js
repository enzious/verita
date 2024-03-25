import fs from 'fs';

export default function (filename) {
  return fs.readFileSync(filename, 'utf-8');
}
