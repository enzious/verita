export default function(content) {
  var out = JSON.stringify(content);

  return out.substring(1, out.length - 2);
}
