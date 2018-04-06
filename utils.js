export function rand() {
  var array = new Uint32Array(1);
  window.crypto.getRandomValues(array);
  return array[0];
}
