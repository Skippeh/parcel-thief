export type Vector3 = [number, number, number];

export const distanceBetweenSquared = (a: Vector3, b: Vector3) => {
  const x = a[0] - b[0];
  const y = a[1] - b[1];
  const z = a[2] - b[2];
  return Math.abs(x * x + y * y + z * z);
};

export const distanceBetween = (a: Vector3, b: Vector3) => {
  return Math.sqrt(distanceBetweenSquared(a, b));
};
